//! 설정 파일·자격 파일의 원자적 교체: 같은 폴더 임시 파일에 전체를 쓰고, 반영한 뒤 기존 파일을 바꿉니다.
//!
//! 저장이 교체 지점을 함수로 받기 때문에, 교체 실패를 시험하는 테스트는 자기가 쓰는 파일 이름으로
//! 교체를 막을 수 있습니다. 교체 지점을 넘기지 않는 저장은 `replace_file`을 쓰므로 D40 규칙
//! 2(임시 파일에 쓰기 → 디스크 반영 → `std::fs::rename`)를 따릅니다.

use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::error::{ConfigError, SaveError};
use super::error::{REPLACE, TEMP_WRITE, save_io_kind, save_io_target};

/// 파일 교체를 맡기는 함수의 타입입니다. `from`에서 `target`으로 옮기는 동작만 담습니다.
///
/// 저장이 이 타입을 받는 이유는 교체 실패를 시험하려는 테스트가 같은 폴더 안의 시험용 파일
/// 이름으로 교체를 막을 수 있게 하기 위해서입니다.
pub(crate) type Replace = dyn Fn(&Path, &Path) -> Result<(), std::io::Error>;

/// 표준 교체 지점: `std::fs::rename`은 같은 폴더의 파일만 바꿉니다(D40 규칙 2).
pub(crate) fn replace_file(from: &Path, target: &Path) -> Result<(), std::io::Error> {
    std::fs::rename(from, target)
}

/// `text`를 같은 폴더의 임시 파일에 쓰고, 디스크에 반영한 뒤 `replace`로 기존 `file`을 바꿉니다.
///
/// 교체에 실패하면 기존 파일은 그대로 두고 이번 저장이 만든 임시 파일만 지웁니다.
///
/// # 오류
/// 임시 파일 쓰기가 실패하거나 교체에 실패하면 [`SaveError::SaveIo`]로 대상 파일 종류와
/// 오류 종류만 밝힙니다. 파일 경로·키 값·base_url은 밝히지 않습니다(02-8 spec 표 2번째 줄).
///
/// [`SaveError::SaveIo`]: super::super::error::SaveError::SaveIo
pub(crate) fn write_replacing(
    folder: &Path,
    file: &'static str,
    text: &str,
    replace: &Replace,
) -> Result<(), ConfigError> {
    let temp_path = folder.join(temp_name(file));
    // 임시 파일부터 만들어야 교체 실패 뒤에 지울 임시 파일 경로가 정해집니다.
    write_temp(&temp_path, text, file)?;
    if let Err(source) = replace(&temp_path, &folder.join(file)) {
        // D40 규칙 2대로 교체 실패는 기존 파일을 건드리지 않습니다. 이번 저장이 만든 임시 파일만
        // 지웁니다. 지우는 대상은 임시 파일 하나이고, 근거는 AC 4(교체 후 남은 임시 파일 정리)와
        // D40 규칙 2(교체 실패 시 임시 파일만 정리)입니다.
        let _ = std::fs::remove_file(&temp_path);
        // 원인은 파일 경로가 섞여 있어 담지 않습니다. 종류만 정합니다(02-8 spec 표 2번째 줄).
        return Err(SaveError::SaveIo {
            target: save_io_target(file),
            kind: save_io_kind(REPLACE, &source),
        }
        .into());
    }
    Ok(())
}

/// 임시 파일에 텍스트를 통째로 쓰고, 교체 전에 디스크에 반영합니다(D40 규칙 2의 `sync_all`).
fn write_temp(temp_path: &Path, text: &str, file: &'static str) -> Result<(), ConfigError> {
    let write_result = (|| -> Result<(), std::io::Error> {
        let mut temp = std::fs::File::create(temp_path)?;
        temp.write_all(text.as_bytes())?;
        temp.sync_all()
    })();
    // 원인은 파일 경로가 섞여 있어 담지 않습니다. 종류만 정합니다(02-8 spec 표 2번째 줄).
    write_result.map_err(|source| {
        SaveError::SaveIo {
            target: save_io_target(file),
            kind: save_io_kind(TEMP_WRITE, &source),
        }
        .into()
    })
}

/// 기존 파일 이름에 프로세스 번호와 번호를 붙인 임시 파일 이름입니다. 같은 폴더에서 이 프로세스가
/// 만든 임시 파일끼리만 겹치지 않게 합니다.
fn temp_name(file: &str) -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{file}-{}-{unique}.tmp", std::process::id())
}
