//! 경로 정규화와 이름 비교 헬퍼만 맡습니다(SRP 분할).
//!
//! `path_guard`·`link_guard`·`secret_names`가 이 헬퍼를 호출합니다. 판정 순서와 거부
//! 문구는 `path_guard`가 정하고, 이 파일은 비교 규칙만 한곳에서 제공합니다(DRY).
//! 이름·경로 항목 비교는 OS 관례를 따르고, Windows는 대소문자를 가리지 않습니다.

use std::path::{Component, Path, PathBuf};

/// `.`을 없애고 `..`는 앞 이름을 없애는 정규화 한 곳입니다.
///
/// 루트까지 `..`로 올라가는 경로는 빈 경로가 됩니다. 빈 경로와 작업 폴더 자기자리는
/// 아래가 아니므로 경계 검사가 거부합니다.
pub(crate) fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(name) => out.push(name),
            Component::Prefix(prefix) => out.push(prefix.as_os_str()),
            Component::RootDir => out.push(Component::RootDir.as_os_str()),
        }
    }
    out
}

/// 경로가 작업 폴더 **아래**에 있는지 확인합니다. 작업 폴더 자기자리는 아래가 아닙니다.
///
/// 항목 비교는 OS 관례를 따릅니다(`names_match`). 앞 부분을 다 맞추고 나서 항목이
/// 더 있어야 아래로 봅니다.
pub(crate) fn below_working_dir(candidate: &Path, root: &Path) -> bool {
    let candidate_parts: Vec<Component<'_>> = candidate.components().collect();
    let root_parts: Vec<Component<'_>> = root.components().collect();
    if candidate_parts.len() <= root_parts.len() {
        return false;
    }
    candidate_parts[..root_parts.len()]
        .iter()
        .zip(&root_parts)
        .all(|(a, b)| same_component(a, b))
}

/// 경로 구성 항목 하나를 OS 관례로 비교합니다.
fn same_component(a: &Component<'_>, b: &Component<'_>) -> bool {
    names_match(
        &a.as_os_str().to_string_lossy(),
        &b.as_os_str().to_string_lossy(),
    )
}

/// 파일 이름·경로 항목을 OS 관례로 비교합니다.
///
/// Windows는 대소문자를 구분하지 않고, 그 밖의 운영체제는 문자 그대로 비교합니다
/// (개발 규칙: 두 OS 동등. `cfg!(windows)` 한 자리에서 갈라 두 OS에서 같은 기준으로
/// 검사하게 둡니다).
pub(crate) fn names_match(a: &str, b: &str) -> bool {
    if cfg!(windows) {
        a.eq_ignore_ascii_case(b)
    } else {
        a == b
    }
}
