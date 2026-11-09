//! 쓰기 결과 표시를 만드는 자리입니다(03-3 T1, dev-principles 3절 DRY).
//!
//! 쓰기·편집 결과가 같은 방식으로 경로를 보여야 하므로 이 파일 한 곳에서만 만듭니다.
//! 경로 판단은 `path_guard`가 맡고, 이 파일은 표시만 맡습니다(책임 분리).

use std::path::Path;

/// 작업 폴더를 기준으로 한 경로 표시를 만들어 줍니다.
#[must_use]
pub fn relative_path(target: &Path, working_dir: &Path) -> String {
    match target.strip_prefix(working_dir) {
        Ok(rel) => display_path(rel),
        Err(_) => display_path(target),
    }
}

/// 경로 구분 기호(`/`와 `\`)를 `/` 하나로 맞춰 한 줄 문자열로 만듭니다.
///
/// Windows와 Linux에서 같은 표시가 나오게 하는 자리입니다(개발 규칙: 두 OS 동등).
fn display_path(path: &Path) -> String {
    let raw = path.to_string_lossy();
    raw.replace('\\', "/")
}
