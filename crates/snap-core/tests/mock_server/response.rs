//! 응답 조립 자리. 스크립트 규격을 axum 응답으로 바꾸는 일만 담당.
use axum::body::Body;
use axum::http::Response;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::body::ChunkBody;
use crate::script::MockScript;

/// 응답 스크립트를 axum 응답으로 바꿈. 헤더 지연은 핸들러가 먼저 처리한다.
pub fn build_response(script: &MockScript) -> axum::response::Response {
    let Ok(status) = StatusCode::from_u16(script.status) else {
        return Response::new(Body::empty()).into_response();
    };
    let body = Body::new(ChunkBody::new(script.body.clone(), script.stop));
    let mut builder = Response::builder()
        .status(status)
        .header(axum::http::header::CONTENT_TYPE, "text/event-stream");
    for (name, value) in &script.headers {
        builder = builder.header(name, value);
    }
    builder
        .body(body)
        .map(IntoResponse::into_response)
        .unwrap_or_else(|_| Response::new(Body::empty()).into_response())
}
