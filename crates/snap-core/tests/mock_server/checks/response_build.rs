//! 응답 조립 확인. 상태 코드와 이벤트 스트림 타입을 axum 응답으로 바꾼 결과를 본다.
use axum::http::header::CONTENT_TYPE;

use crate::response::build_response;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;

#[test]
fn response_keeps_script_status_and_event_stream_type() {
    let script = MockScript::with_status(201).with_body(vec![StreamStep::chunk("data: a\n\n")]);

    let response = build_response(&script);

    assert_eq!(response.status(), 201);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default(),
        "text/event-stream"
    );
}

#[test]
fn response_with_empty_body_keeps_ok_status() {
    let script = MockScript::ok().with_stop(StreamStop::End);

    let response = build_response(&script);

    assert_eq!(response.status(), 200);
}

#[test]
fn response_carries_script_extra_headers() {
    let script = MockScript::with_status(429).with_header("retry-after", "12");

    let response = build_response(&script);

    assert_eq!(
        response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default(),
        "12"
    );
}
