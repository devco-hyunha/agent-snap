//! 경로 검사 정책과 거부 순서, 거부 오류 타입만 둡니다(03-3 T1·T3, SRP 분할).
//!
//! 검사 순서를 한 자리에 고정합니다(spec AC 2 "경로 검증 검사 순서", T3 확정):
//! 경로 정규화 → 작업 폴더 경계 → 심볼릭 링크 → 비밀 목록.
//! 모든 파일 도구는 이 파일의 검사 한 자리 [`check_tool_path`]·[`check_write_path`]만
//! 호출하고, 도구 파일마다 검사를 다시 정의하지 않습니다(DRY, dev-principles 3절).
//! 판정 헬퍼는 역할별로 나눠 두며, 여기서는 순서와 거부만 정의합니다.
//!
//! 심볼릭 링크 기준은 "작업 폴더 안에서 생성된 링크만 허용"입니다(plan.md 리스크, spec AC 2).
//! 링크 기준 판정은 `link_guard`에 있고, 그 판정을 이 순서에서 부릅니다.
//! 비밀 이름 판정은 `secret_names`에 있습니다.

use std::path::{Component, Path, PathBuf};

// 도구 바깥에 내놓는 이름은 기존 `path_guard` 창구를 그대로 유지하려고 다시 내보냅니다.
// plain use를 함께 두면(Rust 2018 규칙) 이 파일 안에서 같은 이름을 바로 부릅니다.
pub use super::link_guard::{escapes_symlink, is_symlink_path};
use super::path_norm::{below_working_dir, normalize_path};
pub use super::secret_names::is_secret_name;

/// 경로 검사가 거부하는 자리를 나타내는 타입입니다.
///
/// 라이브러리 오류 타입은 `thiserror`를 쓴다(개발 규칙). Display에 경로와 키 값을 넣지 않아
/// 거부 이유 한 줄만 모델에 보냅니다(dev-principles 8절: 민감한 값을 결과에 남기지 않음).
/// 거부 문구 정본입니다. Display(#[error])와 message()가 같은 상수를 씁니다(DRY, dev 3절).
const OUTSIDE_WORKING_DIR_MESSAGE: &str = "경계 밖: 작업 폴더 아래 경로만 허용합니다";
const SECRET_FILE_MESSAGE: &str = "비밀 파일은 읽을 수도 쓸 수도 없습니다";
const GIT_WRITE_MESSAGE: &str = ".git 아래는 쓸 수 없습니다";

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum PathGuardError {
    /// 경로가 작업 폴더 아래가 아닙니다. 경로는 표시에 넣지 않습니다(spec AC 2).
    #[error("{OUTSIDE_WORKING_DIR_MESSAGE}")]
    OutsideWorkingDir,
    /// 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`) 자리입니다(D17·spec AC 4).
    #[error("{SECRET_FILE_MESSAGE}")]
    SecretFile,
    /// `.git` 아래 쓰기 거부 자리입니다(D49). 읽기 검사는 이 자리를 막지 않습니다.
    #[error("{GIT_WRITE_MESSAGE}")]
    GitWrite,
}

impl PathGuardError {
    /// 도구 오류 한 줄 자리에 쓰는 이유 문장입니다. Display과 message()가 같은 상수를
    /// 쓰므로 문구 정본이 한 곳입니다(DRY).
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::OutsideWorkingDir => OUTSIDE_WORKING_DIR_MESSAGE,
            Self::SecretFile => SECRET_FILE_MESSAGE,
            Self::GitWrite => GIT_WRITE_MESSAGE,
        }
    }
}

/// 읽기 도구가 쓰는 경로 검사입니다(T3 확정 순서: 정규화 → 경계 → 심볼릭 링크 → 비밀 목록, spec AC 2).
///
/// 통과하면 정규화한 절대 경로를 돌려줍니다. 도구는 이 값을 파일 읽기·쓰기에 그대로 씁니다.
/// 상대 경로는 작업 폴더에 붙인 뒤 `.`과 `..`를 없애 정규화하므로, `..`로 작업 폴더 밖으로
/// 나가는 경로는 정규화 뒤에 경계 검사에서 걸러집니다. 절대 경로는 그대로 두고 경계 검사로
/// 걸러냅니다(spec AC 2).
///
/// # 거부
/// - 경로가 비어 있을 때 경계 밖으로 봅니다.
/// - 정규화한 경로가 작업 폴더와 같거나 작업 폴더 아래가 아닐 때 경계 밖.
/// - 경로가 심볼릭 링크를 거쳐 작업 폴더 밖으로 나갈 때 경계 밖.
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
    // 심볼릭 링크 검사는 경계 검사를 통과한 경로만 다룹니다. 순서 고정(spec AC 2).
    if escapes_symlink(working_dir, &normalized) {
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

/// 경로에 `.git` 항목이 있는지 확인합니다. 쓰기 검사에서만 씁니다(D49).
fn has_git_component(path: &Path) -> bool {
    path.components()
        .any(|part| matches!(part, Component::Normal(name) if name == ".git"))
}
