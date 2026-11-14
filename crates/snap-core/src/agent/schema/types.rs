//! 루프 결과·오류 타입을 모는 자리입니다.
//!
//! 루프 동작(`loop/runner.rs`)이 왕복 결과로 받는 타입만 한 파일에 모아
//! 순서 없는 참조를 피합니다. 설정 값(`LoopConfig`)과 그 기본값은 `settings/config.rs`의 책임입니다.
//! 각 타입은 변경 이유가 하나씩입니다.

use crate::agent::fallback::connector::{FallbackConnector, FallbackPoint};
use crate::agent::fallback::tool_call_form::ToolCallFormat;
use crate::llm::TransportError;

/// 왕복을 시작하기 전에 확인해야 하는 문제가 있습니다.
#[derive(thiserror::Error, Debug)]
pub enum LoopStartError {
    /// `tool_call` 프로파일이 요청 필드를 덮어쓰려는 설정 오류입니다.
    #[error(transparent)]
    Profile(#[from] crate::agent::settings::profile::ProfileError),
    /// 등록한 도구 이름이 서로 같습니다. 같은 이름은 하나만 등록할 수 있습니다.
    #[error("도구 이름 `{name}`가 등록 목록에 두 번 있습니다")]
    DuplicateToolName {
        /// 문제된 이름입니다.
        name: String,
    },
}

/// 왕복이 끝나고 남기는 도구 호출 기록 한 개입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallRecord {
    /// 서버가 준 호출 id입니다.
    pub call_id: String,
    /// 등록된 도구 이름입니다.
    pub tool_name: String,
    /// 모델이 준 인자 JSON 원문입니다.
    pub arguments: String,
    /// 루프가 모델에 넘긴 결과 문자열입니다. 잘림과 오류 한 줄을 함께 담습니다.
    pub result: String,
    /// 실행한 순간의 유닉스 시각(초)입니다. 검증은 하한·상한으로만 봅니다.
    pub executed_at_unix_secs: u64,
    /// 호출이 어느 형식 경로에서 나왔는지 밝히는 호출 형식 칸입니다.
    ///
    /// 03-2 루프는 1안으로만 보내므로 `ToolCallFormat::OpenAi`로 채웁니다. 2안 기록은 03-4가
    /// 만드시고, 03-4 AC 5의 전환 기록 소비 대상입니다(할 일 4).
    pub format: ToolCallFormat,
}

/// 루프가 끝난 뒤 돌려주는 결과입니다.
///
/// 03-2는 1안 경로만 담당하고 03-4로 넘길 판단을 남기지므로, 판정 결과를 이 결과에 담습니다.
/// 03-4는 필드 순서를 그대로 읽고 03-4 AC 5 전환 기록 입력으로 소비합니다(할 일 6, 리뷰 1차 N2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopResult {
    /// `stop`으로 끝난 최종 답 텍스트입니다.
    pub final_text: String,
    /// 전체 왕복에서 쌓은 도구 호출 기록입니다.
    pub tool_calls: Vec<ToolCallRecord>,
    /// 모델 왕복 횟수입니다.
    pub turns: u32,
    /// 03-4 전환 기록 값입니다. `runner`가 채우는 지금은 네 갈래입니다.
    /// 4x 상태 코드 전환은 `Some(UnsupportedStatus)`, 텍스트 2회 연속 전환은 `Some(TextStreak)`,
    /// `auto` 선언의 텍스트 1회 후보는 `Some(NoToolCallText)`이고 앞의 4x 기록이 있으면 그 값을 유지합니다.
    /// 선언 값(`openai`·`text`)로 텍스트 1회에 종료한 첫 왕복도 `Some(NoToolCallText)`입니다.
    /// 도구 호출이 있는 왕복·여러 번째 왕복(후보 없음)·반복 상한·거부 종료는 `None`입니다.
    pub fallback_point: Option<FallbackPoint>,
}

/// 왕복을 끝내는 사정 4갈래입니다(spec 동작 규칙 1~4, AC 2·AC 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopEnd {
    /// `finish_reason=stop`에 해당하는 응답으로 정상 종료했습니다.
    Stop {
        /// 그 자리에서 얻은 최종 답과 지금까지 기록입니다.
        result: LoopResult,
    },
    /// 반복 상한을 채우고 끝났습니다. 마지막 바퀴까지의 결과를 partial에 담습니다.
    TurnLimit {
        /// 상한까지 채운 부분 결과입니다.
        partial: LoopResult,
    },
    /// 미승인·거부 결과를 모델에 알리고 멈췄습니다. 같은 도구·인자 재시도를 하지 않습니다.
    ApprovalRejected {
        /// 멈추게 한 도구 호출 기록입니다.
        stopped_record: ToolCallRecord,
        /// 그까지 채운 부분 결과입니다.
        partial: LoopResult,
    },
}

/// 왕복이 실패한 자리입니다.
#[derive(thiserror::Error, Debug)]
pub enum LoopError {
    /// `tool_call` 프로파일 설정이 올바르지 않습니다.
    #[error(transparent)]
    Profile(#[from] crate::agent::settings::profile::ProfileError),
    /// 왕복 전송이 실패했습니다. 02-6이 낸 최종 오류의 표시만 그대로 씁니다(D13).
    #[error("모델 요청에 실패했습니다: {reason}")]
    Transport {
        /// 가리기를 거친 전송 오류 문구입니다.
        reason: String,
        /// 서버가 준 HTTP 상태 코드입니다. 상태 코드 응답이 아니면 `None`입니다.
        ///
        /// 03-4가 판단에 쓰도록 코드를 보존하는 창구입니다(할 일 1). 본문 글자는 담지 않습니다(D28).
        status: Option<u16>,
    },
    /// 모델이 1안 호출 형식을 처리하지 못했습니다. 03-4 폴백 연결 지점으로 넘깁니다(AC 4 자리).
    #[error("모델이 도구 호출을 처리하지 못했습니다. 폴백 연결 지점: {fallback}({point})")]
    Unsupported {
        /// 폴백 연결 지점 표기입니다. 03-4가 소비하는 자리이고, 판정 결과가 고르는 상수를 씁니다.
        fallback: FallbackConnector,
        /// 왜 폴백으로 넘기는지 밝힌 판단 값입니다. 2안 조립은 03-4가 합니다.
        point: FallbackPoint,
    },
    /// 왕복 전에 확인한 문제가 있습니다.
    #[error(transparent)]
    Start(#[from] LoopStartError),
}

impl LoopError {
    /// 전송 오류를 루프 오류로 바꿉니다. 주소·키는 02-5 가리기를 이미 거친 값입니다.
    ///
    /// 서버가 준 상태 코드가 있으면 그대로 옮깁니다. 4x 판별은 `tool_call_form`의 `is_unsupported_status`가
    /// 하고, 여기서는 코드 보존만 합니다(할 일 1). 본문 글자는 옮기지 않습니다(D28).
    pub fn transport(error: &TransportError) -> Self {
        let status = match error {
            TransportError::ServerStatus { status, .. } => Some(*status),
            _ => None,
        };
        Self::Transport {
            reason: error.to_string(),
            status,
        }
    }

    /// 03-4가 소비하는 폴백 연결 지점 표기를 고르는 상수로 채운 미지원 오류를 만듭니다(AC 4, 03-4 자리).
    #[must_use]
    pub fn unsupported(point: FallbackPoint) -> Self {
        Self::Unsupported {
            fallback: FallbackConnector,
            point,
        }
    }
}

/// 왕복 한 번의 결과를 받은 자리입니다.
#[derive(Debug)]
pub enum LoopOutcome {
    /// 왕복이 끝나고 결과를 남겼습니다.
    Finished(LoopEnd),
    /// 왕복이 실패했습니다.
    Failed(LoopError),
}

impl From<LoopError> for LoopOutcome {
    fn from(value: LoopError) -> Self {
        Self::Failed(value)
    }
}
