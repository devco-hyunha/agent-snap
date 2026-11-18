//! 경로 검사 정책과 거부 순서, 거부 오류 타입
//!
//! 검사 순서 고정: 경로 정규화 → 작업 폴더 경계 → 심볼릭 링크 → 비밀 목록
//! 심볼릭 링크 기준은 "작업 폴더 안에서 생성된 링크만 허용"

use std::path::{Component, Path, PathBuf};

// 공개 이름은 `tools.rs`의 기존 창구를 유지하려고 다시 내보냄
pub use super::link_guard::{escapes_symlink, is_symlink_path};
use super::path_norm::{below_working_dir, normalize_path};
pub use super::secret_names::is_secret_name;

/// 경로 검사가 거부하는 자리를 나타내는 타입
///
/// Display에 경로와 키 값을 넣지 않아 거부 이유 한 줄만 모델에 보냄
const OUTSIDE_WORKING_DIR_MESSAGE: &str = "경계 밖: 작업 폴더 아래 경로만 허용합니다";
const SECRET_FILE_MESSAGE: &str = "비밀 파일은 읽을 수도 쓸 수도 없습니다";
const GIT_WRITE_MESSAGE: &str = ".git 아래는 쓸 수 없습니다";

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum PathGuardError {
    /// 경로가 작업 폴더 아래가 아님. 경로는 표시에 넣지 않음
    #[error("{OUTSIDE_WORKING_DIR_MESSAGE}")]
    OutsideWorkingDir,
    /// 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`) 자리
    #[error("{SECRET_FILE_MESSAGE}")]
    SecretFile,
    /// `.git` 아래 쓰기 거부 자리. 읽기 검사는 이 자리를 막지 않음
    #[error("{GIT_WRITE_MESSAGE}")]
    GitWrite,
}

impl PathGuardError {
    /// 도구 오류 한 줄에 쓰는 이유 문장
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::OutsideWorkingDir => OUTSIDE_WORKING_DIR_MESSAGE,
            Self::SecretFile => SECRET_FILE_MESSAGE,
            Self::GitWrite => GIT_WRITE_MESSAGE,
        }
    }
}

/// 읽기 도구가 쓰는 경로 검사(순서: 정규화 → 경계 → 심볼릭 링크 → 비밀 목록)
/// 통과하면 정규화한 절대 경로를 반환하고, 상대·절대 경로 모두 경계 검사로 걸러냄
/// # 거부
/// - 빈 경로 / 작업 폴더 아래가 아닌 경로 / 링크로 밖으로 나가는 경로 / 비밀 파일 이름
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
    // 심볼릭 링크 검사는 경계 검사를 통과한 경로만 다룸. 순서 고정
    if escapes_symlink(working_dir, &normalized) {
        return Err(PathGuardError::OutsideWorkingDir);
    }
    if is_secret_name(&normalized) {
        return Err(PathGuardError::SecretFile);
    }
    Ok(normalized)
}

/// 쓰기 도구가 쓰는 경로 검사(읽기 검사 + `.git` 아래 쓰기 거부)
///
/// `.git` 아래 쓰기는 거부하고, 나머지는 [`check_tool_path`] 결과를 그대로 사용
pub fn check_write_path(working_dir: &Path, path: &str) -> Result<PathBuf, PathGuardError> {
    let target = check_tool_path(working_dir, path)?;
    if has_git_component(&target) {
        return Err(PathGuardError::GitWrite);
    }
    Ok(target)
}

/// 경로에 `.git` 항목이 있는지 확인. 쓰기 검사에서만 사용
fn has_git_component(path: &Path) -> bool {
    path.components()
        .any(|part| matches!(part, Component::Normal(name) if name == ".git"))
}
