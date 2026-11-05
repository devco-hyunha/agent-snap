#![allow(dead_code, unreachable_pub)]

//! 모의 서버가 쓰는 스크립트 규격과 요청 기록의 타입입니다.
//!
//! 서버 수명(`super::server`)·본문 스트림 구현(`super::handler`)과 분리했습니다.
//! 테스트가 응답 형식을 조립할 때 이 타입들을 먼저 만납니다.

use std::time::Duration;
use std::time::Instant;

use axum::http::HeaderMap;
use axum::http::HeaderName;
use axum::http::StatusCode;

/// 본문 한 조각입니다. 보내기 전에 `delay_before`만큼 멈춥니다.
#[derive(Clone, Debug)]
pub struct MockBodyPart {
    /// 이 조각을 보내기 전 대기 시간입니다.
    pub delay_before: Duration,
    /// 보낼 바이트입니다.
    pub bytes: Vec<u8>,
    /// true면 이 조각 자리에서 본문을 마무리 없이 끊는 표식입니다. `bytes`는 보내지 않습니다.
    pub disconnect: bool,
}

impl MockBodyPart {
    /// 지연 없이 바로 보낼 조각을 만듭니다.
    pub fn immediate(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before: Duration::ZERO,
            bytes: bytes.into(),
            disconnect: false,
        }
    }

    /// 지정한 시간만큼 멈춘 뒤 보낼 조각을 만듭니다.
    pub fn after(delay_before: Duration, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before,
            bytes: bytes.into(),
            disconnect: false,
        }
    }

    /// 앞 조각까지 보낸 자리에서 본문을 마무리 없이 끊는 표식 조각입니다.
    ///
    /// 스트림 시작 후 끊김 시험(02-6 AC 5)이 씁니다. 정상 종료가 아니라 끊김이어야 하므로 씁니다.
    pub fn disconnect() -> Self {
        Self {
            delay_before: Duration::ZERO,
            bytes: Vec::new(),
            disconnect: true,
        }
    }
}

/// 한 번 응답할 때 따를 스크립트입니다.
#[derive(Clone, Debug)]
pub struct MockResponseScript {
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
    pub fn empty_ok() -> Self {
        Self {
            status: StatusCode::OK,
            response_headers: HeaderMap::new(),
            header_delay: Duration::ZERO,
            body_parts: Vec::new(),
            hang_before_headers: false,
        }
    }

    /// 본문 조각만 채운 200 응답 스크립트입니다.
    pub fn with_body_parts(body_parts: Vec<MockBodyPart>) -> Self {
        Self {
            body_parts,
            ..Self::empty_ok()
        }
    }

    /// 헤더를 영원히 보내지 않는 스크립트입니다.
    pub fn hang_headers() -> Self {
        Self {
            hang_before_headers: true,
            ..Self::empty_ok()
        }
    }

    /// 지정한 상태 코드와 `Retry-After` 헤더를 붙이고 빈 본문으로 바로 응답하는 스크립트입니다.
    ///
    /// 상태 재시도 통합 시험(429·503)이 헤더 있는 응답을 조립할 때 씁니다.
    pub fn status_with_retry_after(status: StatusCode, retry_after: &str) -> Self {
        let headers = HeaderMap::from_iter([(
            HeaderName::from_static("retry-after"),
            retry_after.parse().expect("Retry-After 헤더 값"),
        )]);
        Self {
            status,
            response_headers: headers,
            ..Self::empty_ok()
        }
    }
}

/// 모의 서버가 받아 둔 요청 한 건입니다.
#[derive(Clone, Debug)]
pub struct RecordedRequest {
    /// HTTP 메서드 문자열입니다.
    pub method: String,
    /// 요청 경로입니다.
    pub path: String,
    /// 요청 헤더입니다.
    pub headers: HeaderMap,
    /// 요청 본문 바이트입니다.
    pub body: Vec<u8>,
    /// 요청이 도착한 단조 시각입니다. 반복 요청 사이의 간격 확인에 씁니다.
    pub received_at: Instant,
}
