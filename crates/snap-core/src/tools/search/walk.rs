//! 폴더 순회와 자리 거르기 자리
//!
//! 숨김 자리는 요청 시에만 담고, 비밀 파일 자리는 늘 거름
//! 심볼릭 링크 기준은 `path_guard`의 [`is_symlink_path`]·[`escapes_symlink`]를 그대로 씀

use std::path::{Path, PathBuf};

use crate::tools::display::relative_path;
use crate::tools::path_guard::{check_tool_path, escapes_symlink, is_secret_name, is_symlink_path};

/// 순회로 줍은 자리 하나
pub(crate) struct WalkEntry {
    /// 실제 경로. 읽기 도구가 이 자리로 파일을 읽음
    pub(crate) path: PathBuf,
    /// 작업 폴더 기준 표시 경로(구분자는 `/`)
    pub(crate) rel: String,
    /// 폴더 여부
    pub(crate) is_dir: bool,
}

/// 자리 인자를 실제 폴더 경로로 풂
///
/// 없거나 비었거나 `.`이면 작업 폴더 자기자리(목록·검색 기준 자리라 경계 밖이 아님)
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

/// 시작 폴더 아래 자리를 모아 표시 경로 오름차순으로 반환
///
/// `recursive`이면 하위 폴더까지, `include_hidden`이면 숨김 자리도 포함. 읽을 수 없는 자리·UTF-8이 아닌 이름·비밀 파일 이름은 건너뜀
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
            // 밖으로 나가는 링크 자리는 목록·검색 결과에 올리지 않음
            if escapes_symlink(working_dir, &path) {
                continue;
            }
            if !include_hidden && name.starts_with('.') {
                continue;
            }
            let is_dir = meta.is_dir();
            let rel = relative_path(&path, working_dir);
            // 링크 폴더는 어떤 방향이든 순회하지 않음. ripgrep 기본(`--no-follow`)과 같은 규칙
            if recursive && is_dir && !is_symlink_path(&path) {
                stack.push(path.clone());
            }
            out.push(WalkEntry { path, rel, is_dir });
        }
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    out
}
