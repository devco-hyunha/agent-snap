//! 전송 계층 오류 타입입니다.
//!
//! Display·Debug에 API 키와 base_url 호스트를 담지 않습니다. 서버 오류 본문은 가린 뒤
//! 길이를 제한한 요약만 담고(D28), 연결 실패는 헤더 전 연결 세부 6종으로 나눕니다(D26).
//! 포기 이유 한 줄은 `give_up_note`가, Debug 표시는 `error_debug`가 각각 맡습니다(작업 07 SRP 분리).

use crate::llm::client::give_up_note::GiveUpNote;
use crate::llm::client::segment::TimeoutSegment;
use crate::llm::error::StreamParseError;
use crate::llm::retry::{PreHeaderConnectKind, RetryGiveUpReason};

/// 전송 계층 오류입니다.
#[derive(thiserror::Error)]
pub enum TransportError {
    /// HTTP 클라이언트를 만들지 못했습니다.
    #[error("전송 클라이언트를 만들지 못했습니다")]
    ClientBuild,
    /// 요청이 취소되었습니다. 호출 전 취소와 스트림 도중 취소를 함께 나타냅니다.
    #[error("취소됨")]
    Cancelled,
    /// 헤더를 받기 전 연결에 실패했습니다.
    ///
    /// 세부 종류와 연결 설정 초 값만 담고, 키·base_url 호스트는 담지 않습니다(D26).
    #[error("연결 실패: {kind:?}")]
    ConnectFailed {
        /// 헤더 전 연결 세부 종류입니다.
        kind: PreHeaderConnectKind,
        /// 연결 시간 설정 값(초)입니다.
        connect_seconds: u64,
    },
    /// 응답 헤더를 정해진 시간 안에 받지 못했습니다.
    #[error("시간 초과: {segment} (설정 {seconds}초)")]
    Timeout {
        /// 초과가 난 구간입니다.
        segment: TimeoutSegment,
        /// 설정된 초 값입니다.
        seconds: u64,
    },
    /// 서버가 정상 상태가 아닌 응답을 보냈습니다.
    ///
    /// 본문 요약은 어림 가리기와 알고 있는 키·호스트 가리기를 거친 값입니다(D28).
    #[error("서버 응답 상태 {status}: {body_summary}{}", GiveUpNote(*give_up))]
    ServerStatus {
        /// HTTP 상태 코드입니다.
        status: u16,
        /// 키·호스트를 가린 뒤 자른 본문 요약입니다.
        body_summary: String,
        /// 재시도 판단을 거친 끝에 포기한 이유입니다. 판단을 거치지 않은 단일 요청 실패면 `None`입니다.
        ///
        /// 02-4 판단 값만 얹고 새 오류 종류는 만들지 않습니다(02-6 spec 33행). 헤더 원문은 담지 않습니다.
        give_up: Option<RetryGiveUpReason>,
    },
    /// 응답을 읽는 중에 실패했습니다.
    ///
    /// 스트림 오류와 형식 오류를 함께 담고, 02-3 오류 표 규칙을 따릅니다.
    #[error(transparent)]
    Streaming(#[from] StreamParseError),
    /// 응답 본문을 읽는 중에 연결이 끊겼습니다. 주소가 들어 있는 라이브러리 오류 텍스트는 담지 않습니다.
    #[error("본문 읽기 실패")]
    BodyRead,
    /// 모델 목록 응답(`GET /models`)의 형식 오류입니다. 종류만 담고 본문 원문·키·주소는 담지 않습니다.
    #[error(transparent)]
    ModelsListFormat(#[from] crate::llm::error::models_format::ModelsListFormatError),
}
