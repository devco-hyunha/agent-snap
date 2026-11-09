//! 파일 도구가 한곳에서 부르는 경로 검사 1차입니다(`path_guard`, 03-3 T1).
//!
//! 검사 순서를 한 자리에 고정합니다: 경로 정규화 → 작업 폴더 경계 → 비밀 파일 → 쓰기 금지(`.git`).
//! T3가 심볼릭 링크 검사를 같은 순서에 추가해 순서를 확정하고, 모든 파일 도구는 이 파일의
//! 검사 한 자리만 호출합니다. 도구 파일마다 검사를 다시 정의하지 않습니다(DRY, dev-principles 3절).
//!
//! 심볼릭 링크는 여기서 확인하지 않습니다. 링크가 가리키는 자리는 T3가 정하는 규칙이고,
//! T1은 경로 문자열만 다룹니다.

use std::path::{Component, Path, PathBuf};

use crate::config::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME};

/// 경로 검사 1차가 거부하는 자리를 나타내는 타입입니다.
///
/// 라이브러리 오류 타입은 `thiserror`를 쓴다(개발 규칙). Display에 경로와 키 값을 넣지 않아
/// 거부 이유 한 줄만 모델에 보냅니다(dev-principles 8절: 민감한 값을 결과에 남기지 않음).
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum PathGuardError {
    /// 경로가 작업 폴더 아래가 아닙니다. 경로는 표시에 넣지 않습니다(spec AC 2).
    #[error("경계 밖: 작업 폴더 아래 경로만 허용합니다")]
    OutsideWorkingDir,
    /// 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`) 자리입니다(D17·spec AC 4).
    #[error("비밀 파일은 읽을 수도 쓸 수도 없습니다")]
    SecretFile,
    /// `.git` 아래 쓰기 거부 자리입니다(D49). 읽기 검사는 이 자리를 막지 않습니다.
    #[error(".git 아래는 쓸 수 없습니다")]
    GitWrite,
}

impl PathGuardError {
    /// 도구 오류 한 줄 자리에 쓰는 이유 문장입니다. Display와 같은 문장을 한 곳에서만
    /// 정의해 도구마다 문구를 다시 적지 않게 합니다(DRY).
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::OutsideWorkingDir => "경계 밖: 작업 폴더 아래 경로만 허용합니다",
            Self::SecretFile => "비밀 파일은 읽을 수도 쓸 수도 없습니다",
            Self::GitWrite => ".git 아래는 쓸 수 없습니다",
        }
    }
}

/// 읽기 도구가 쓰는 경로 검사입니다(순서 고정: 정규화 → 경계 → 비밀 목록, spec AC 2).
///
/// 통과하면 정규화한 절대 경로를 돌려줍니다. 도구는 이 값을 파일 읽기·쓰기에 그대로 씁니다.
/// 상대 경로는 작업 폴더에 붙인 뒤 `.`과 `..`를 없애 정규화하므로, `..`로 작업 폴더 밖으로
/// 나가는 경로는 정규화 뒤에 경계 검사에서 걸러집니다. 절대 경로는 그대로 두고 경계 검사로
/// 걸러냅니다(spec AC 2).
///
/// # 거부
/// - 경로가 비어 있을 때 경계 밖으로 봅니다.
/// - 정규화한 경로가 작업 폴더와 같거나 작업 폴더 아래가 아닐 때 경계 밖.
/// - 마지막 이름이 비밀 파일일 때 비밀 파일 거부.
pub fn check_tool_path(working_dir: &Path, path: &str) -> Result<PathBuf, PathGuardError> {
    let raw = path.trim();
    if raw.is_empty() {
        return Err(PathGuardError::OutsideWorkingDir);
    }
    let joined = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        working_dir.join(raw)
    };
    let normalized = normalize_path(&joined);
    if !below_working_dir(&normalized, &normalize_path(working_dir)) {
        return Err(PathGuardError::OutsideWorkingDir);
    }
    if is_secret_name(&normalized) {
        return Err(PathGuardError::SecretFile);
    }
    Ok(normalized)
}

/// 쓰기 도구가 쓰는 경로 검사입니다(읽기 검사 + `.git` 아래 쓰기 거부, D49).
///
/// `.git` 아래 쓰기는 거부하고, 나머지는 [`check_tool_path`] 결과를 그대로 씁니다.
pub fn check_write_path(working_dir: &Path, path: &str) -> Result<PathBuf, PathGuardError> {
    let target = check_tool_path(working_dir, path)?;
    if has_git_component(&target) {
        return Err(PathGuardError::GitWrite);
    }
    Ok(target)
}

/// `.`을 없애고 `..`는 앞 이름을 없애는 정규화 한 곳입니다.
///
/// 루트까지 `..`로 올라가는 경로는 빈 경로가 됩니다. 빈 경로와 작업 폴더 자기자리는
/// 아래가 아니므로 경계 검사가 거부합니다.
fn normalize_path(path: &Path) -> PathBuf {
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
fn below_working_dir(candidate: &Path, root: &Path) -> bool {
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

/// 마지막 이름이 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`)인지 확인합니다(D17).
///
/// 정규화가 끝난 경로의 마지막 이름만 비교하므로 폴더 이름과 섞이지 않습니다.
///
/// 목록·검색 도구가 이 판정을 재사용합니다(DRY).
pub fn is_secret_name(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    names_match(name, CONFIG_FILE_NAME) || names_match(name, CREDENTIALS_FILE_NAME)
}

/// 경로에 `.git` 항목이 있는지 확인합니다. 쓰기 검사에서만 씁니다(D49).
fn has_git_component(path: &Path) -> bool {
    path.components()
        .any(|part| matches!(part, Component::Normal(name) if name == ".git"))
}

/// 파일 이름·경로 항목을 OS 관례로 비교합니다.
///
/// Windows는 대소문자를 구분하지 않고, 그 밖의 운영체제는 문자 그대로 비교합니다
/// (개발 규칙: 두 OS 동등. `cfg!(windows)` 한 자리에서 갈라 두 OS에서 같은 기준으로
/// 검사하게 둡니다).
fn names_match(a: &str, b: &str) -> bool {
    if cfg!(windows) {
        a.eq_ignore_ascii_case(b)
    } else {
        a == b
    }
}
