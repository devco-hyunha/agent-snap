//! 전송 계층 오류 타입.
//!
//! Display·Debug에 API 키와 base_url 호스트를 담지 않음. 서버 오류 본문은 가린 뒤
//! 길이 제한 요약만 담고, 연결 실패는 헤더 전 연결 세부 6종으로 나눔.

use crate::llm::client::give_up_note::GiveUpNote;
use crate::llm::client::segment::TimeoutSegment;
use crate::llm::error::StreamParseError;
use crate::llm::retry::{PreHeaderConnectKind, RetryGiveUpReason};

/// 전송 계층 오류.
#[derive(thiserror::Error)]
pub enum TransportError {
    /// HTTP 클라이언트를 만들지 못함.
    #[error("전송 클라이언트를 만들지 못했습니다")]
    ClientBuild,
    /// 요청 취소. 호출 전·스트림 도중 취소를 함께 나타냄.
    #[error("취소됨")]
    Cancelled,
    /// 헤더를 받기 전 연결 실패.
    ///
    /// 세부 종류와 연결 설정 초 값만 담고, 키·base_url 호스트는 담지 않음.
    #[error("연결 실패: {kind:?}")]
    ConnectFailed {
        /// 헤더 전 연결 세부 종류.
        kind: PreHeaderConnectKind,
        /// 연결 시간 설정 값(초).
        connect_seconds: u64,
    },
    /// 응답 헤더를 정해진 시간 안에 받지 못함.
    #[error("시간 초과: {segment} (설정 {seconds}초)")]
    Timeout {
        /// 초과가 난 구간.
        segment: TimeoutSegment,
        /// 설정된 초 값.
        seconds: u64,
    },
    /// 서버가 정상 상태가 아닌 응답을 보냄.
    ///
    /// 본문 요약은 어림 가리기와 알고 있는 키·호스트 가리기를 거친 값.
    #[error("서버 응답 상태 {status}: {body_summary}{}", GiveUpNote(*give_up))]
    ServerStatus {
        /// HTTP 상태 코드.
        status: u16,
        /// 키·호스트를 가린 뒤 자른 본문 요약.
        body_summary: String,
        /// 재시도 판단 끝의 포기 이유. 단일 요청 실패면 `None`. 헤더 원문은 담지 않음.
        give_up: Option<RetryGiveUpReason>,
    },
    /// 응답을 읽는 중 실패. 스트림·형식 오류를 함께 담음.
    #[error(transparent)]
    Streaming(#[from] StreamParseError),
    /// 본문 읽기 중 연결 끊김. 주소가 든 라이브러리 오류 텍스트는 담지 않음.
    #[error("본문 읽기 실패")]
    BodyRead,
    /// 모델 목록 응답 형식 오류. 종류만 담고 본문 원문·키·주소는 담지 않음.
    #[error(transparent)]
    ModelsListFormat(#[from] crate::llm::error::models_format::ModelsListFormatError),
}
