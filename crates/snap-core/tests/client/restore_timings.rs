//! T8 시각 기록 시험: 복원 수신자가 남기는 토큰 시각 순서와 값만 봄.

use std::time::Instant;

use snap_core::llm::RestoreReceiver;
use snap_core::llm::StreamReceiver;

use crate::stream_payload::data_line;
use crate::stream_payload::mixed_stream;
use crate::stream_payload::payload_with;

#[test]
fn timings_follow_token_order() {
    let stream = mixed_stream();
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream.as_bytes())
        .expect("test accepts stream");
    let outcome = receiver.finish().expect("test finishes stream");

    let start = outcome.timings.request_start;
    let first = outcome.timings.first_token.expect("token time is recorded");
    let last = outcome.timings.last_token.expect("token time is recorded");
    assert!(
        start <= first && first <= last,
        "timings keep request start, first token, last token order"
    );
}

#[test]
fn accounting_only_stream_has_no_token_time() {
    let stream = data_line(&payload_with(
        serde_json::json!({ "finish_reason": "stop" }),
    ));
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream.as_bytes())
        .expect("test accepts stream");
    let outcome = receiver.finish().expect("test finishes stream");

    assert!(
        outcome.timings.first_token.is_none() && outcome.timings.last_token.is_none(),
        "no token time when content never grows"
    );
}
