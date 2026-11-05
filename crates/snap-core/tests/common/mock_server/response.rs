//! 스크립트 상태줄·응답 헤더로 응답을 조립하는 시작 만들기 하나만 다룹니다.
//!
//! `record`·`chat_handler`·`models_handler` 파일이 같은 조립을 다시 적지 않게
//! 한곳에 뒀다(DRY). 본문 스트림·목록 본문 조립은 각각 자기 파일의 책임이다(SRP).

use axum::http::response::Builder;
use axum::response::Response;

use super::script::MockResponseScript;

/// 스크립트의 상태줄과 응답 헤더를 붙여 응답 조립을 시작합니다. 세 경로가 같은 조립을 다시 적지 않습니다.
pub(super) fn response_builder(script: &MockResponseScript) -> Builder {
    let mut builder = Response::builder().status(script.status);
    for (name, value) in script.response_headers.iter() {
        builder = builder.header(name, value);
    }
    builder
}
