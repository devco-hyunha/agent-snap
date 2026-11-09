//! 폴더 순회와 자리 거르기를 한곳에 모은 자리입니다(03-3 T2).
//!
//! 목록·glob·grep이 같은 순회를 각자 다시 만들지 않게 여기 한 파일에 둡니다(DRY).
//! 숨김 자리는 요청 시에만 담고, 비밀 파일 자리는 늘 거릅니다.
//! 심볼릭 링크가 가리키는 폴더로는 내려가지 않습니다(순환·탈출 방지).

use std::path::{Path, PathBuf};

use super::display::relative_path;
use super::path_guard::{check_tool_path, is_secret_name};

/// 순회로 줍은 자리 하나입니다.
pub(crate) struct WalkEntry {
    /// 실제 경로. 읽기 도구가 이 자리로 파일을 읽습니다.
    pub(crate) path: PathBuf,
    /// 작업 폴더 기준 표시 경로(구분자는 `/`).
    pub(crate) rel: String,
    /// 폴더 여부.
    pub(crate) is_dir: bool,
}

/// 자리 인자를 실제 폴더 경로로 풉니다.
///
/// 없거나 비었거나 `.`이면 작업 폴더 자기자리입니다(목록·검색 기준 자리라 경계 밖이 아님).
/// 그 밖의 경로는 `path_guard` 한 자리로 검사합니다.
pub(crate) fn resolve_dir(working_dir: &Path, path: Option<&str>) -> Result<PathBuf, &'static str> {
    let Some(raw) = path else {
        return Ok(working_dir.to_path_buf());
    };
    let raw = raw.trim();
    if raw.is_empty() || raw == "." {
        return Ok(working_dir.to_path_buf());
    }
    check_tool_path(working_dir, raw).map_err(|error| error.message())
}

/// 시작 폴더 아래 자리를 모아 표시 경로 오름차순으로 돌려줍니다.
///
/// `recursive`이면 하위 폴더까지, `include_hidden`이면 숨김 자리도 담습니다.
/// 읽을 수 없는 자리·UTF-8이 아닌 이름·비밀 파일 이름은 건너뜁니다.
pub(crate) fn collect(
    working_dir: &Path,
    start: &Path,
    recursive: bool,
    include_hidden: bool,
) -> Vec<WalkEntry> {
    let mut out = Vec::new();
    let mut stack = vec![start.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read) = std::fs::read_dir(&dir) else {
            continue;
        };
        for child in read.flatten() {
            let Ok(meta) = child.file_type() else {
                continue;
            };
            let name = child.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let path = child.path();
            if is_secret_name(&path) {
                continue;
            }
            if !include_hidden && name.starts_with('.') {
                continue;
            }
            let is_dir = meta.is_dir();
            let rel = relative_path(&path, working_dir);
            if recursive && is_dir && !meta.is_symlink() {
                stack.push(path.clone());
            }
            out.push(WalkEntry { path, rel, is_dir });
        }
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    out
}
