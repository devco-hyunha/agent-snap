//! 저장이 만드는 임시 파일: 쓰기·이름 생성

use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::error::{ConfigError, SaveError};
use super::error::TEMP_WRITE;
use super::io_error::{save_io_kind, save_io_target};

/// 임시 파일에 텍스트를 통째로 쓰고 교체 전에 디스크에 반영. `sync_all` 사용
pub(super) fn write_temp(
    temp_path: &Path,
    text: &str,
    file: &'static str,
) -> Result<(), ConfigError> {
    let write_result = (|| -> Result<(), std::io::Error> {
        let mut temp = std::fs::File::create(temp_path)?;
        temp.write_all(text.as_bytes())?;
        temp.sync_all()
    })();
    // 원인은 파일 경로가 섞여 있어 미포함. 종류만 정함
    write_result.map_err(|source| {
        SaveError::SaveIo {
            target: save_io_target(file),
            kind: save_io_kind(TEMP_WRITE, &source),
        }
        .into()
    })
}

/// 기존 파일 이름에 프로세스 번호와 번호를 붙인 임시 파일 이름. 같은 폴더에서 이 프로세스가 만든 임시 파일끼리만 겹치지 않게 함
pub(super) fn temp_name(file: &str) -> String {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{file}-{}-{unique}.tmp", std::process::id())
}
