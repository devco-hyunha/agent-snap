//! 루프 결과·오류 타입을 모은 자리
//!
//! 루프 동작(`loop/runner.rs`)이 왕복 결과로 받는 타입만 한 파일에 모아 순서 없는 참조를 피함

use crate::agent::fallback::connector::{FallbackConnector, FallbackPoint};
use crate::agent::fallback::tool_call_form::ToolCallFormat;
use crate::llm::TransportError;

/// 왕복 시작 전에 확인하는 문제
#[derive(thiserror::Error, Debug)]
pub enum LoopStartError {
    /// `tool_call` 프로파일이 요청 필드를 덮어쓰려는 설정 오류
    #[error(transparent)]
    Profile(#[from] crate::agent::settings::profile::ProfileError),
    /// 등록한 도구 이름이 서로 같음. 같은 이름은 하나만 등록 가능
    #[error("도구 이름 `{name}`가 등록 목록에 두 번 있습니다")]
    DuplicateToolName {
        /// 문제된 이름
        name: String,
    },
}

/// 왕복이 끝나고 남기는 도구 호출 기록 한 개
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallRecord {
    /// 서버가 준 호출 id
    pub call_id: String,
    /// 등록된 도구 이름
    pub tool_name: String,
    /// 모델이 준 인자 JSON 원문
    pub arguments: String,
    /// 루프가 모델에 넘긴 결과 문자열. 잘림과 오류 한 줄을 함께 담음
    pub result: String,
    /// 실행한 순간의 유닉스 시각(초). 검증은 하한·상한으로만 봄
    pub executed_at_unix_secs: u64,
    /// 호출이 어느 형식 경로에서 나왔는지 밝히는 호출 형식 칸
    pub format: ToolCallFormat,
}

/// 루프가 끝난 뒤 돌려주는 결과
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopResult {
    /// `stop`으로 끝난 최종 답 텍스트
    pub final_text: String,
    /// 전체 왕복에서 쌓은 도구 호출 기록
    pub tool_calls: Vec<ToolCallRecord>,
    /// 모델 왕복 횟수
    pub turns: u32,
    /// 폴백 전환 기록 값. 전환 지점과 후보 자리의 코드만 담고, 도구 호출 왕복·후보 없음·상한·거부 종료는 `None`
    pub fallback_point: Option<FallbackPoint>,
}

/// 왕복을 끝내는 사정 4갈래
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopEnd {
    /// `finish_reason=stop`에 해당하는 응답으로 정상 종료
    Stop {
        /// 그 자리에서 얻은 최종 답과 지금까지 기록
        result: LoopResult,
    },
    /// 반복 상한을 채우고 종료. 마지막 바퀴까지의 결과를 partial에 담음
    TurnLimit {
        /// 상한까지 채운 부분 결과
        partial: LoopResult,
    },
    /// 미승인·거부 결과를 모델에 알리고 정지. 같은 도구·인자 재시도 없음
    ApprovalRejected {
        /// 멈추게 한 도구 호출 기록
        stopped_record: ToolCallRecord,
        /// 그까지 채운 부분 결과
        partial: LoopResult,
    },
}

/// 왕복이 실패한 자리
#[derive(thiserror::Error, Debug)]
pub enum LoopError {
    /// `tool_call` 프로파일 설정이 올바르지 않음
    #[error(transparent)]
    Profile(#[from] crate::agent::settings::profile::ProfileError),
    /// 왕복 전송 실패. 최종 오류의 표시만 그대로 사용
    #[error("모델 요청에 실패했습니다: {reason}")]
    Transport {
        /// 가리기를 거친 전송 오류 문구
        reason: String,
        /// 서버가 준 HTTP 상태 코드. 상태 코드 응답이 아니면 `None`
        ///
        /// 판단에 쓰도록 코드를 보존하는 창구. 본문 글자는 담지 않음
        status: Option<u16>,
    },
    /// 모델이 1안 호출 형식을 처리하지 못함. 폴백 연결 지점으로 넘김
    #[error("모델이 도구 호출을 처리하지 못했습니다. 폴백 연결 지점: {fallback}({point})")]
    Unsupported {
        /// 폴백 연결 지점 표기. 소비하는 자리이고, 판정 결과가 고르는 상수를 사용
        fallback: FallbackConnector,
        /// 왜 폴백으로 넘기는지 밝힌 판단 값. 2안 조립은 폴백이 담당
        point: FallbackPoint,
    },
    /// 왕복 전에 확인한 문제
    #[error(transparent)]
    Start(#[from] LoopStartError),
}

impl LoopError {
    /// 전송 오류를 루프 오류로 변환. 주소·키는 가리기를 이미 거친 값
    ///
    /// 상태 코드 보존만 담당하고 본문 글자는 옮기지 않음
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

    /// 폴백 연결 지점 표기를 고르는 상수로 채운 미지원 오류 생성
    #[must_use]
    pub fn unsupported(point: FallbackPoint) -> Self {
        Self::Unsupported {
            fallback: FallbackConnector,
            point,
        }
    }
}

/// 왕복 한 번의 결과를 받은 자리
#[derive(Debug)]
pub enum LoopOutcome {
    /// 왕복이 끝나고 결과를 남김
    Finished(LoopEnd),
    /// 왕복이 실패함
    Failed(LoopError),
}

impl From<LoopError> for LoopOutcome {
    fn from(value: LoopError) -> Self {
        Self::Failed(value)
    }
}
