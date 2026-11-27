//! T5 스트림 상태 계약 시험: 한 번 실패한 뒤 재호출이 성공으로 오해되지 않음만 봄.

use std::time::Instant;

use snap_core::llm::RestoreReceiver;
use snap_core::llm::StreamReceiver;
use snap_core::llm::TransportError;

use crate::stream_payload::data_line;
use crate::stream_payload::delta_payload;
use crate::stream_payload::error_payload;

/// content가 하나 들어온 뒤 서버 오류 청크로 실패한 스트림 원문.
fn stream_that_fails() -> String {
    let content = delta_payload(serde_json::json!({ "content": "ab" }));
    format!(
        "{}{}",
        data_line(&content),
        data_line(&error_payload("서버 오류"))
    )
}

#[test]
fn failed_stream_finish_keeps_failure() {
    let mut receiver = RestoreReceiver::new(Instant::now());
    let first = receiver.push(stream_that_fails().as_bytes());

    assert!(first.is_err(), "first push reports failure");
    let finished = receiver.finish();

    assert!(finished.is_err(), "finish after failure is not success");
}

#[test]
fn failed_stream_rejects_late_push() {
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream_that_fails().as_bytes())
        .expect_err("test fails at error chunk");

    let late = receiver.push(b"data: {\"choices\":[{\"delta\":{\"content\":\"x\"}}]}\n\n");

    let error = late.expect_err("late push is not success");
    assert!(
        matches!(error, TransportError::Stream { .. }),
        "late push keeps stream failure"
    );
}

#[test]
fn failed_stream_repeats_same_failure_on_repush() {
    let mut receiver = RestoreReceiver::new(Instant::now());
    let first = receiver
        .push(stream_that_fails().as_bytes())
        .expect_err("test fails at error chunk");

    let second = receiver.push(b"").expect_err("repush keeps failure");

    assert_eq!(
        first.to_string(),
        second.to_string(),
        "repush reports the first failure, not a new success"
    );
}

#[test]
fn failed_stream_rejects_finish_after_repush() {
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream_that_fails().as_bytes())
        .expect_err("test fails at error chunk");
    let _ = receiver
        .push(b"")
        .expect_err("test keeps failure on repush");

    let finished = receiver.finish();

    assert!(finished.is_err(), "finish after repush is not success");
}

#[test]
fn closed_receiver_rejects_push_after_finish() {
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(data_line(&delta_payload(serde_json::json!({ "content": "ab" }))).as_bytes())
        .expect("test accepts content");
    receiver.finish().expect("test finishes stream");

    let late = receiver.push(b"data: [DONE]\n\n");

    let error = late.expect_err("closed receiver rejects late push");
    assert!(
        matches!(error, TransportError::Closed),
        "closed receiver reports stream end, not cancel: {error}"
    );
}
