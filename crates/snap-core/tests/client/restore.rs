//! T5 복원 단위 시험: 02-3 형식을 수신자가 같은 결과로 복원하는지 확인만 봄.

use std::time::Instant;

use snap_core::llm::RestoreReceiver;
use snap_core::llm::StreamReceiver;

use crate::stream_payload::mixed_stream;

/// 02-3 AC 1과 같은 형식을 한 번에 넣어 복원한 결과.
#[allow(clippy::expect_used)]
fn restore_once(stream: &str) -> String {
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream.as_bytes())
        .expect("test accepts stream");
    receiver
        .finish()
        .expect("test finishes stream")
        .delta
        .content
}

#[test]
fn mixed_stream_restores_content_like_02_3() {
    let stream = mixed_stream();

    let content = restore_once(&stream);

    assert_eq!(content, "가나다", "receiver restores content in order");
}

#[test]
fn byte_by_byte_pieces_restore_same_as_one_shot() {
    let stream = mixed_stream();
    let one_shot = restore_once(&stream);

    let mut receiver = RestoreReceiver::new(Instant::now());
    for byte in stream.as_bytes() {
        receiver
            .push(std::slice::from_ref(byte))
            .expect("test accepts byte");
    }
    let restored = receiver
        .finish()
        .expect("test finishes byte stream")
        .delta
        .content;

    assert_eq!(
        restored, one_shot,
        "byte boundary keeps same restore result"
    );
}

#[test]
fn restore_keeps_usage_and_reasoning_from_stream() {
    let stream = mixed_stream();
    let mut receiver = RestoreReceiver::new(Instant::now());
    receiver
        .push(stream.as_bytes())
        .expect("test accepts stream");
    let outcome = receiver.finish().expect("test finishes stream");

    assert_eq!(outcome.delta.reasoning, "생각", "reasoning is restored");
    let usage = outcome.delta.usage.expect("usage is read");
    assert_eq!(usage.total_tokens, Some(5), "usage is read at the end");
}
