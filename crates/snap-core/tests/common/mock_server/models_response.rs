#![allow(dead_code, unreachable_pub)]

//! `GET /models` 목록 응답을 스크립트에서 조립하는 만들기들만 다룹니다.
//!
//! 채팅 본문 스트림 조립(`handler`)과 목록 본문 조립은 바뀌는 이유가 다르므로 파일을
//! 나눴다(SRP, dev-principles 2절). 본문 조각 이어 붙이기·목록 본문 고르기·목록 응답
//! 조립이 이 파일의 책임이다.

use axum::body::{Body, Bytes};
use axum::http::StatusCode;
use axum::response::Response;

use super::response::response_builder;
use super::script::{MockBodyPart, MockResponseScript};

/// 스크립트 본문 조각을 이어 붙여 응답 바이트로 만듭니다.
///
/// 목록 본문은 JSON 한 덩어리 조각 하나로 구성됩니다. 끊김 표식은 목록에서 쓰지 않습니다.
pub(super) fn join_body_parts(parts: &[MockBodyPart]) -> Vec<u8> {
    parts
        .iter()
        .filter(|part| !part.disconnect)
        .flat_map(|part| part.bytes.iter().copied())
        .collect()
}

/// 목록 본문 조각을 정합니다. 스크립트 본문 조각 목록이 비면 빈 본문입니다.
///
/// 빈 본문은 `empty_ok()`이 그 뜻 그대로 아무것도 안 보내는 상태이고, 목록 기본 본문은
/// `models_list()` 스크립트가 자기 시험(`models_list_is_served`)에서 확인합니다.
pub(super) fn list_body_parts(script: &MockResponseScript) -> &[MockBodyPart] {
    &script.body_parts
}

/// 스크립트 상태줄·헤더에 목록 본문(JSON 한 덩어리)을 붙여 응답을 만듭니다.
///
/// 채팅처럼 본문이 스트림으로 이어지지 않는 단일 응답입니다. 02-7 목록 시험은 200 본문과
/// 상태가 바뀐 응답을 함께 쓰며, 요청 기록의 상태도 그 응답 상태입니다.
pub(super) fn list_response(script: &MockResponseScript, body: Vec<u8>) -> Response {
    response_builder(script)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .expect("모의 목록 응답 조립")
}
