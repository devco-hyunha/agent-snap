//! 심볼릭 링크 판정. 링크가 작업 폴더 밖으로 나가는지, 링크 자리인지 확인
//!
//! 기준은 "작업 폴더 안에서 생성된 링크만 허용"

use std::path::{Path, PathBuf};

use super::path_norm::{below_working_dir, normalize_path};

/// 경로가 심볼릭 링크 자리인지 확인
///
/// `walk`은 이 판정이 참인 폴더로 내려가지 않음. ripgrep은 같은 기준을 검색 인자(`--no-follow`)로 맞춤
pub fn is_symlink_path(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.is_symlink())
}

/// 심볼릭 링크가 작업 폴더 밖으로 나가는지 확인
///
/// 존재하지 않는 파일 앞으로의 쓰기도 잡으려고 경로의 가장 깊은 조상까지 표준화하고 남은 이름을 붙여 판정. 표준화 실패(순환 링크·권한)는 거부하지 않음
#[must_use]
pub fn escapes_symlink(working_dir: &Path, normalized: &Path) -> bool {
    let root = resolve_root(working_dir);
    // 목록·검색 기준 자리(작업 폴더 자기자리)는 링크 검사 대상이 아님
    if normalized == root.as_path() {
        return false;
    }
    let Some(escaper) = resolve_prefix(normalized) else {
        return false;
    };
    if escaper == root {
        return false;
    }
    !below_working_dir(&escaper, &root)
}

/// 작업 폴더 기준을 OS가 따라간 자리로 맞춤. 링크로 만들어진 작업 폴더·임시 폴더 표준화를 같은 표기로 맞춤
///
/// 작업 폴더가 실재하는 자리만 표준화. 검사할 경로가 없는 자리에 닿아 부모 폴더까지 표준화되는 일을 차단
fn resolve_root(working_dir: &Path) -> PathBuf {
    let normalized = normalize_path(working_dir);
    if std::fs::symlink_metadata(&normalized).is_ok() {
        return canonicalize_path(&normalized).unwrap_or(normalized);
    }
    normalized
}

/// 경로의 가장 깊은 조상까지 표준화하고 남은 이름을 붙임. 조상이 하나도 표준화되지 못하면 `None`
fn resolve_prefix(path: &Path) -> Option<PathBuf> {
    let deepest_existing = path
        .ancestors()
        .find(|ancestor| std::fs::symlink_metadata(ancestor).is_ok())?;
    let canonical = canonicalize_path(deepest_existing)?;
    let tail = path.strip_prefix(deepest_existing).ok()?;
    Some(canonical.join(tail))
}

/// OS가 따라간 자리로 경로 표준화
#[cfg(windows)]
fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    // Windows 표준화는 `\\?\` 접두를 붙이므로 경계 검사가 같은 표기로 비교하게 접두만 제거
    let resolved = std::fs::canonicalize(path).ok()?;
    let text = resolved.to_string_lossy();
    let trimmed = text
        .strip_prefix(r"\\?\")
        .or_else(|| text.strip_prefix(r"\\.\"))
        .unwrap_or(&text);
    Some(PathBuf::from(trimmed))
}

/// OS가 따라간 자리로 경로 표준화(UNIX는 심볼릭 링크를 따라감)
#[cfg(unix)]
fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}
