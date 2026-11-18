//! 재시도 포기 이유를 Display에 한 줄로 표기.
//!
//! Display·Debug에 API 키와 base_url 호스트를 담지 않음. 판단 없는 실패(`None`)는 문장을 붙이지 않음.

use std::fmt;

use crate::llm::retry::{NotCandidateDetail, RetryGiveUpReason};

/// Display에 포기 이유 한 줄만 얹는 표시 도우미.
///
/// 판단 없는 실패(`None`)는 문장을 붙이지 않음. 헤더 원문·키·호스트는 밝히지 않음.
pub(crate) struct GiveUpNote(pub(crate) Option<RetryGiveUpReason>);

impl fmt::Display for GiveUpNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            None => Ok(()),
            Some(reason) => write!(f, " ({})", format_give_up_reason(&reason)),
        }
    }
}

/// 포기 이유를 사람이 볼 한 문장으로 바꿈.
///
/// 상태 코드·대기 초만 밝히고 헤더 원문·키·호스트는 밝히지 않음.
fn format_give_up_reason(reason: &RetryGiveUpReason) -> String {
    match reason {
        RetryGiveUpReason::NotCandidate(detail) => {
            let detail = match detail {
                NotCandidateDetail::HttpStatus { status } => format!("상태 {status}"),
                NotCandidateDetail::HeaderWaitTimeout => "헤더 대기".to_owned(),
            };
            format!("재시도 대상 아님: {detail}")
        }
        RetryGiveUpReason::AttemptsExceeded { attempt } => format!("재시도 {attempt}회로 포기"),
        RetryGiveUpReason::RetryAfterExceedsCap { wait } => {
            format!("대기 {}초가 상한을 넘어 포기", wait.as_secs())
        }
    }
}
