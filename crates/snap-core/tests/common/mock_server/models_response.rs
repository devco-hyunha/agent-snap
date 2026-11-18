#![allow(dead_code, unreachable_pub)]

//! `GET /models` 목록 응답을 스크립트에서 조립하는 만들기들
//!
//! 본문 조각 이어 붙이기·목록 본문 고르기·목록 응답 조립이 이 파일의 책임

use axum::body::{Body, Bytes};
use axum::http::StatusCode;
use axum::response::Response;

use super::response::response_builder;
use super::script::{MockBodyPart, MockResponseScript};

/// 스크립트 본문 조각을 이어 붙여 응답 바이트로 만듦
///
/// 끊김 표식은 목록에서 쓰지 않음
pub(super) fn join_body_parts(parts: &[MockBodyPart]) -> Vec<u8> {
    parts
        .iter()
        .filter(|part| !part.disconnect)
        .flat_map(|part| part.bytes.iter().copied())
        .collect()
}

/// 목록 본문 조각을 정함. 스크립트 본문 조각 목록이 비면 빈 본문
pub(super) fn list_body_parts(script: &MockResponseScript) -> &[MockBodyPart] {
    &script.body_parts
}

/// 스크립트 상태줄·헤더에 목록 본문(JSON 한 덩어리)을 붙여 응답을 만듦
///
/// 본문이 스트림으로 이어지지 않는 단일 응답. 요청 기록의 상태도 그 응답 상태
pub(super) fn list_response(script: &MockResponseScript, body: Vec<u8>) -> Response {
    response_builder(script)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .expect("모의 목록 응답 조립")
}
