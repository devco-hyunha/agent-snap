//! 도구가 돌려주는 실행 결과 타입과 미래 상자로만 보는 자리
//!
//! 결과 문자열 규칙(`오류: ` 접두어)을 이 한 곳에서만 정의

/// 도구가 돌려주는 실행 결과
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolOutcome {
    /// 결과가 문자열 하나. 잘림은 루프가 적용
    Done(String),
    /// 승인자에게 넘길 명령 요약
    ApprovalRequired {
        /// 승인 화면에 보여주는 한 줄 요약
        summary: String,
    },
    /// 모델에 그대로 보내는 한 줄 오류(`오류: 이유` 형식)
    Error(String),
}

impl ToolOutcome {
    /// 오류 한 줄 생성. `오류: `를 앞에 붙임
    pub fn error(reason: &str) -> Self {
        Self::Error(format!("오류: {reason}"))
    }

    /// 모델에 보낼 결과 문자열 반환
    pub fn text(&self) -> &str {
        match self {
            Self::Done(text) | Self::Error(text) => text.as_str(),
            Self::ApprovalRequired { summary } => summary.as_str(),
        }
    }
}

/// 구현체가 같은 미래 타입을 한곳에서 쓰게 하는 자리
pub type ToolCallFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ToolOutcome> + Send + 'a>>;
