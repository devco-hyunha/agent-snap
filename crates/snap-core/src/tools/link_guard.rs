//! 심볼릭 링크 판정만 맡습니다. 링크가 작업 폴더 밖으로 나가는지, 링크 자리인지 봅니다(03-3 T3, SRP 분할).
//!
//! 기준은 "작업 폴더 안에서 생성된 링크만 허용"입니다(plan.md 리스크, spec AC 2).
//! 검사 순서와 거부 문구는 `path_guard`가 정하고, 이 파일의 판정만 그 순서에서 부릅니다.
//! Windows·Linux 동작은 `#[cfg(windows)]`·`#[cfg(unix)]` 게이트로 나눠 같은 커밋에서
//! 두 OS 기준을 함께 봅니다(개발 규칙 이식성).

use std::path::{Path, PathBuf};

use super::path_norm::{below_working_dir, normalize_path};

/// 경로가 심볼릭 링크 자리인지 확인합니다(순회 기준을 한곳에서 맞추는 자리, spec AC 2).
///
/// `walk`은 이 판정이 참인 폴더로 내려가지 않습니다. ripgrep 경로는 같은 기준을
/// 검색 인자(`--no-follow`)로 맞춰 두 순로의 링크 순회 범위를 한 규칙으로 맞춥니다
/// (T2 리뷰 3항).
pub fn is_symlink_path(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.is_symlink())
}

/// 심볼릭 링크가 작업 폴더 밖으로 나가는지 확인합니다(T3 기준: "작업 폴더 안에서 생성된 링크만 허용", spec AC 2).
///
/// 링크가 가리키는 최종 자리가 작업 폴더 아래가 아니면 경계 밖으로 봅니다. 작업 폴더
/// 안 파일을 가리키는 링크는 통과합니다.
///
/// 존재하지 않는 파일 앞으로의 쓰기도 잡으려고 경로의 가장 깊은 조상 자리까지 표준화하고 남은
/// 이름을 붙여 판정합니다. 목록·검색 순회 기준 자리인 작업 폴더 자체는 검사하지 않습니다.
/// 표준화가 실패하는 자리(순환 링크·권한)는 이 검사가 거부하지 않습니다: 읽기·쓰기
/// 단계가 "파일 없음"·"폴더 아님"으로 이유를 내기 때문이고, 거부를 한 곳에서만
/// 정의합니다(dev-principles 7절: 실패 처리는 한 자리).
#[must_use]
pub fn escapes_symlink(working_dir: &Path, normalized: &Path) -> bool {
    let root = resolve_root(working_dir);
    // 목록·검색 기준 자리(작업 폴더 자기자리)는 링크 검사 대상이 아닙니다.
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

/// 작업 폴더 기준을 OS가 따라간 자리로 맞춥니다. 링크로 만들어진 작업 폴더·임시
/// 폴더 표준화(`macOS`의 `/private` 자리)를 같은 표기로 만들려면 같은 함수를
/// 재사용합니다(dev-principles 3절 DRY).
///
/// 작업 폴더가 실재하는 자리만 표준화합니다. 검사할 경로가 없는 자리에 닿아
/// 부모 폴더까지 표준화되는 일을 막습니다.
fn resolve_root(working_dir: &Path) -> PathBuf {
    let normalized = normalize_path(working_dir);
    if std::fs::symlink_metadata(&normalized).is_ok() {
        return canonicalize_path(&normalized).unwrap_or(normalized);
    }
    normalized
}

/// 경로의 가장 깊은 조상까지 표준화하고, 표준화하지 못한 남은 이름을 붙여 줍니다.
///
/// 조상이 하나도 표준화되지 못하면 `None`(표준화 실패 = 이 검사는 거부하지 않음).
fn resolve_prefix(path: &Path) -> Option<PathBuf> {
    let deepest_existing = path
        .ancestors()
        .find(|ancestor| std::fs::symlink_metadata(ancestor).is_ok())?;
    let canonical = canonicalize_path(deepest_existing)?;
    // 가장 깊은까지 표준화한 자리가 작업 폴더를 떠났으면 뒤 부분은 같은 자리에 붙습니다.
    let tail = path.strip_prefix(deepest_existing).ok()?;
    Some(canonical.join(tail))
}

/// OS가 따라간 자리로 경로 표준화를 한 곳에서 줍니다.
#[cfg(windows)]
fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    // Windows 표준화는 `\\?\` 접두를 붙이므로 경계 검사가 같은 표기로
    // 비교하게 접두만 떼어 줍니다.
    let resolved = std::fs::canonicalize(path).ok()?;
    let text = resolved.to_string_lossy();
    let trimmed = text
        .strip_prefix(r"\\?\")
        .or_else(|| text.strip_prefix(r"\\.\"))
        .unwrap_or(&text);
    Some(PathBuf::from(trimmed))
}

/// OS가 따라간 자리로 경로 표준화를 한 곳에서 줍니다(UNIX: 심볼릭 링크를 따라갑니다).
#[cfg(unix)]
fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}
