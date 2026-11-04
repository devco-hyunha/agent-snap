//! 모의 서버가 쓰는 스크립트 규격과 요청 기록의 타입입니다.
//!
//! 서버 수명(`super::server`)·본문 스트림 구현(`super::handler`)과 분리했습니다.
//! 테스트가 응답 형식을 조립할 때 이 타입들을 먼저 만납니다.

use std::time::Duration;

use axum::http::HeaderMap;
use axum::http::StatusCode;

/// 본문 한 조각입니다. 보내기 전에 `delay_before`만큼 멈춥니다.
#[derive(Clone, Debug)]
pub(in crate::llm::tests) struct MockBodyPart {
    /// 이 조각을 보내기 전 대기 시간입니다.
    pub delay_before: Duration,
    /// 보낼 바이트입니다.
    pub bytes: Vec<u8>,
}

impl MockBodyPart {
    /// 지연 없이 바로 보낼 조각을 만듭니다.
    pub(in crate::llm::tests) fn immediate(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before: Duration::ZERO,
            bytes: bytes.into(),
        }
    }

    /// 지정한 시간만큼 멈춘 뒤 보낼 조각을 만듭니다.
    pub(in crate::llm::tests) fn after(delay_before: Duration, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before,
            bytes: bytes.into(),
        }
    }
}

/// 한 번 응답할 때 따를 스크립트입니다.
#[derive(Clone, Debug)]
pub(in crate::llm::tests) struct MockResponseScript {
    /// HTTP 상태 코드입니다.
    pub status: StatusCode,
    /// 응답에 붙일 헤더입니다.
    pub response_headers: HeaderMap,
    /// 상태줄·응답 헤더를 보내기 전 대기입니다.
    pub header_delay: Duration,
    /// 본문 조각 목록입니다. 앞에서부터 순서대로 보냅니다.
    pub body_parts: Vec<MockBodyPart>,
    /// true면 헤더를 보내지 않고 끝없이 기다립니다.
    pub hang_before_headers: bool,
}

impl MockResponseScript {
    /// 바로 200과 빈 본문을 돌려주는 기본 스크립트입니다.
    pub(in crate::llm::tests) fn empty_ok() -> Self {
        Self {
            status: StatusCode::OK,
            response_headers: HeaderMap::new(),
            header_delay: Duration::ZERO,
            body_parts: Vec::new(),
            hang_before_headers: false,
        }
    }

    /// 본문 조각만 채운 200 응답 스크립트입니다.
    pub(in crate::llm::tests) fn with_body_parts(body_parts: Vec<MockBodyPart>) -> Self {
        Self {
            body_parts,
            ..Self::empty_ok()
        }
    }

    /// 헤더를 영원히 보내지 않는 스크립트입니다.
    pub(in crate::llm::tests) fn hang_headers() -> Self {
        Self {
            hang_before_headers: true,
            ..Self::empty_ok()
        }
    }
}

/// 모의 서버가 받아 둔 요청 한 건입니다.
#[derive(Clone, Debug)]
pub(in crate::llm::tests) struct RecordedRequest {
    /// HTTP 메서드 문자열입니다.
    pub method: String,
    /// 요청 경로입니다.
    pub path: String,
    /// 요청 헤더입니다.
    pub headers: HeaderMap,
    /// 요청 본문 바이트입니다.
    pub body: Vec<u8>,
}
