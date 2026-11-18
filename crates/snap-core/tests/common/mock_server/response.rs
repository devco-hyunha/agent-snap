//! 스크립트 상태줄·응답 헤더로 응답 조립을 시작하는 만들기
//!
//! `record`·`chat_handler`·`models_handler`가 같은 조립을 다시 적지 않게 한곳에 둠

use axum::http::response::Builder;
use axum::response::Response;

use super::script::MockResponseScript;

/// 스크립트의 상태줄과 응답 헤더를 붙여 응답 조립을 시작함
pub(super) fn response_builder(script: &MockResponseScript) -> Builder {
    let mut builder = Response::builder().status(script.status);
    for (name, value) in script.response_headers.iter() {
        builder = builder.header(name, value);
    }
    builder
}
