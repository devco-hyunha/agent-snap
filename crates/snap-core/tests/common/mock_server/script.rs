#![allow(dead_code, unreachable_pub)]

//! 모의 서버가 쓰는 응답 스크립트 규격
//!
//! 테스트가 응답 형식을 조립할 때 이 타입을 먼저 만남

use std::time::Duration;

use axum::http::HeaderMap;
use axum::http::HeaderName;
use axum::http::StatusCode;

/// 본문 한 조각. 보내기 전에 `delay_before`만큼 멈춤
#[derive(Clone, Debug)]
pub struct MockBodyPart {
    pub delay_before: Duration,
    pub bytes: Vec<u8>,
    /// true면 이 조각 자리에서 본문을 마무리 없이 끊는 표식. `bytes`는 보내지 않음
    pub disconnect: bool,
}

impl MockBodyPart {
    /// 지연 없이 바로 보낼 조각
    pub fn immediate(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before: Duration::ZERO,
            bytes: bytes.into(),
            disconnect: false,
        }
    }

    /// 지정한 시간만큼 멈춘 뒤 보낼 조각
    pub fn after(delay_before: Duration, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            delay_before,
            bytes: bytes.into(),
            disconnect: false,
        }
    }

    /// 앞 조각까지 보낸 자리에서 본문을 마무리 없이 끊는 표식 조각
    pub fn disconnect() -> Self {
        Self {
            delay_before: Duration::ZERO,
            bytes: Vec::new(),
            disconnect: true,
        }
    }
}

/// 한 번 응답할 때 따를 스크립트
#[derive(Clone, Debug)]
pub struct MockResponseScript {
    pub status: StatusCode,
    pub response_headers: HeaderMap,
    /// 상태줄·응답 헤더를 보내기 전 대기
    pub header_delay: Duration,
    /// 본문 조각 목록. 앞에서부터 순서대로 보냄
    pub body_parts: Vec<MockBodyPart>,
    /// true면 헤더를 보내지 않고 끝없이 대기
    pub hang_before_headers: bool,
}

impl MockResponseScript {
    /// 바로 200과 빈 본문을 돌려주는 기본 스크립트
    pub fn empty_ok() -> Self {
        Self {
            status: StatusCode::OK,
            response_headers: HeaderMap::new(),
            header_delay: Duration::ZERO,
            body_parts: Vec::new(),
            hang_before_headers: false,
        }
    }

    /// 본문 조각만 채운 200 응답 스크립트
    pub fn with_body_parts(body_parts: Vec<MockBodyPart>) -> Self {
        Self {
            body_parts,
            ..Self::empty_ok()
        }
    }

    /// 헤더를 영원히 보내지 않는 스크립트
    pub fn hang_headers() -> Self {
        Self {
            hang_before_headers: true,
            ..Self::empty_ok()
        }
    }

    /// 지정한 상태 코드와 `Retry-After` 헤더를 붙이고 빈 본문으로 바로 응답하는 스크립트
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

    /// `GET /models`에서 돌려주는 모델 목록 응답 스크립트
    pub fn models_list() -> Self {
        let body = serde_json::json!({
            "object": "list",
            "data": [
                {"id": "mock-model-a", "object": "model", "owned_by": "mock"},
                {"id": "mock-model-b", "object": "model", "owned_by": "mock"},
            ],
        });
        Self::with_body_parts(vec![MockBodyPart::immediate(body.to_string().into_bytes())])
    }
}
