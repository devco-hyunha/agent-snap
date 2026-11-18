//! 작업 폴더의 `AGENTS.md`를 읽는 책임만 다루는 파일
//!
//! 조립은 `prompt::assemble_messages`를 그대로 사용

use std::path::Path;

/// 기본 지시 파일 상한. Codex `project_doc_max_bytes`의 32 KiB를 따름
pub const DEFAULT_AGENTS_MD_MAX_BYTES: usize = 32 * 1024;

/// 지시 파일 이름. OS 대소문자 관례에 맞춰 찾되 이 이름을 기준으로 사용
pub const AGENTS_MD_FILE_NAME: &str = "AGENTS.md";

/// 조립된 프롬프트 끝에 붙이는 "지시가 길다" 표시 한 줄
///
/// 원문을 잘라 넣지 않는다는 것을 모델에게도 알리는 문구. 한 줄로만 붙임
pub const AGENTS_MD_LONG_NOTICE: &str =
    "[지시가 깁니다: 지시 파일을 잘라 넣지 않고 그대로 담았습니다]";

/// `AGENTS.md`를 읽은 결과 상태
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentsMd {
    /// 지시 원문을 읽은 상태. `has_length_notice`는 상한을 넘으면 true이고 표시를 끝에 붙임
    Loaded {
        /// 읽은 원문 그대로. 잘라 넣지 않음
        text: String,
        /// 상한 초과 여부 표시
        has_length_notice: bool,
    },
    /// 파일이 없는 상태. 오류가 아니라 지시 없이 진행
    Missing,
    /// 파일이 빈 상태. 지시 없는 상태로 봄
    Empty,
    /// 읽기에 실패한 상태. 권한·깨진 링크처럼 읽지 못한 이유만 남김
    ReadFailed {
        /// 화면과 로그에 남길 읽지 못한 이유
        reason: String,
    },
}

impl AgentsMd {
    /// 조립에 넘길 지시 문자열 생성
    ///
    /// 지시가 있는 경우에만 원문 + 상한 초과 표시 한 줄이고, 읽지 않은 네 상태는 빈 문자열
    #[must_use]
    pub fn directives(&self) -> String {
        match self {
            // 상한을 넘은 원문은 그대로 두고 표시 한 줄만 뒤에 붙임
            Self::Loaded {
                text,
                has_length_notice: true,
            } => format!("{text}\n{AGENTS_MD_LONG_NOTICE}"),
            Self::Loaded { text, .. } => text.clone(),
            Self::Missing | Self::Empty | Self::ReadFailed { .. } => String::new(),
        }
    }
}

/// 작업 폴더 최상위의 `AGENTS.md`를 읽어 상태별로 반환
///
/// 읽기 실패는 오류로 멈추지 않고 상태로만 알림(루프는 계속 진행)
#[must_use]
pub fn read_agents_md(working_dir: &Path, max_bytes: usize) -> AgentsMd {
    let target = working_dir.join(AGENTS_MD_FILE_NAME);
    match std::fs::read(&target) {
        Ok(raw) => match String::from_utf8(raw) {
            Ok(text) if text.trim().is_empty() => AgentsMd::Empty,
            Ok(text) => {
                // 상한 초과 표시는 원문 길이를 먼저 재서 정함. 원문은 그대로 둠
                let has_length_notice = text.len() > max_bytes;
                AgentsMd::Loaded {
                    text,
                    has_length_notice,
                }
            }
            // 지시 파일은 텍스트여야 하므로 바이트가 깨지면 읽기 실패로만 봄
            Err(_) => AgentsMd::ReadFailed {
                reason: "utf8 텍스트가 아닙니다".to_owned(),
            },
        },
        // 파일이 아예 없으면 지시 없는 상태, 그 밖의 실패(권한·폴더 이름)는 읽기 실패로 나눔
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => AgentsMd::Missing,
        Err(source) => AgentsMd::ReadFailed {
            reason: source.to_string(),
        },
    }
}

/// 기본 상한(32 KiB)으로 읽는 자리. 설정 값이 없을 때 이 한 자리를 거침
#[must_use]
pub fn read_agents_md_default(working_dir: &Path) -> AgentsMd {
    read_agents_md(working_dir, DEFAULT_AGENTS_MD_MAX_BYTES)
}
