//! 02-3 T1 SSE 줄 처리 시험. 형식별 입력(주석·data 여러 줄·줄 끝 3종·BOM·`[DONE]`·빈 줄 없는 종료)만 봄.
//!
//! 조각 경계에서 잘린 멀티바이트 문자 복원은 `boundaries.rs`가 본다.
use snap_core::llm::LineState;
use snap_core::llm::SseEvent;

#[test]
fn comment_lines_make_no_event() {
    let mut state = LineState::new();
    let bytes = b": keep-alive\n: OPENROUTER PROCESSING\ndata: a\n\n";

    let events = state.push(bytes).unwrap();

    assert_eq!(events, vec![data_event("a")], "comment lines are skipped");
}

#[test]
fn multiple_data_lines_join_with_newline() {
    let mut state = LineState::new();
    let bytes = b"data: first\ndata: second\n\n";

    let events = state.push(bytes).unwrap();

    assert_eq!(
        events,
        vec![data_event("first\nsecond")],
        "data lines join with newline at blank line"
    );
}

#[test]
fn three_line_endings_split_the_same() {
    let inputs = [
        b"data: a\r\ndata: b\r\n\r\n".to_vec(),
        b"data: a\ndata: b\n\n".to_vec(),
        b"data: a\rdata: b\r\r".to_vec(),
    ];

    for bytes in inputs {
        let mut state = LineState::new();
        let mut events = state.push(&bytes).unwrap();
        events.extend(state.finish().unwrap());
        assert_eq!(
            events,
            vec![data_event("a\nb")],
            "line ending works for {bytes:?}"
        );
    }
}

#[test]
fn lone_cr_at_end_of_chunk_splits_after_next_lf() {
    let mut state = LineState::new();

    let first = state.push(b"data: a\r").unwrap();
    let second = state.push(b"\n\n").unwrap();

    assert!(first.is_empty(), "lone CR waits for the next chunk");
    assert_eq!(second, vec![data_event("a")]);
}

#[test]
fn stream_end_with_lone_cr_drops_the_line_end() {
    let mut state = LineState::new();

    let pushed = state.push(b"data: a\r").unwrap();
    let finished = state.finish().unwrap();

    assert!(pushed.is_empty(), "lone CR at chunk end waits");
    assert_eq!(
        finished,
        vec![data_event("a")],
        "lone CR at stream end is a line end, not payload"
    );
}

#[test]
fn stream_end_with_lone_cr_ends_done_marker() {
    let mut state = LineState::new();

    let pushed = state.push(b"data: [DONE]\r").unwrap();
    let finished = state.finish().unwrap();

    assert!(pushed.is_empty(), "lone CR at chunk end waits");
    assert_eq!(
        finished,
        vec![SseEvent::End],
        "[DONE] with lone CR ends the stream"
    );
}

#[test]
fn leading_bom_is_dropped_once() {
    let mut state = LineState::new();
    let bytes = "\u{feff}data: a\n\n".as_bytes().to_vec();

    let events = state.push(&bytes).unwrap();

    assert_eq!(events, vec![data_event("a")], "leading BOM is dropped");
}

#[test]
fn bom_mid_stream_is_not_dropped() {
    let mut state = LineState::new();
    let bytes = "data: \u{feff}a\n\n".as_bytes().to_vec();

    let events = state.push(&bytes).unwrap();

    assert_eq!(
        events,
        vec![data_event("\u{feff}a")],
        "only the leading BOM is dropped"
    );
}

#[test]
fn done_marker_ends_stream() {
    let mut state = LineState::new();
    let bytes = b"data: a\n\ndata: [DONE]\n\n";

    let events = state.push(bytes).unwrap();

    assert_eq!(
        events,
        vec![data_event("a"), SseEvent::End],
        "[DONE] ends the stream"
    );
}

#[test]
fn stream_end_without_blank_line_flushes_last_event() {
    let mut state = LineState::new();

    let pushed = state.push(b"data: tail").unwrap();
    let finished = state.finish().unwrap();

    assert!(pushed.is_empty(), "no blank line means no event yet");
    assert_eq!(
        finished,
        vec![data_event("tail")],
        "last event is flushed without blank line"
    );
}

#[test]
fn data_payload_drops_only_first_space_after_colon() {
    let mut state = LineState::new();
    let bytes = b"data:   spaced  \n\n";

    let events = state.push(bytes).unwrap();

    assert_eq!(events, vec![data_event("  spaced  ")]);
}

#[test]
fn blank_line_without_data_makes_no_event() {
    let mut state = LineState::new();

    let events = state.push(b"\n\n\n").unwrap();

    assert!(events.is_empty(), "blank line without data is empty");
}

/// data 이벤트 값.
fn data_event(payload: &str) -> SseEvent {
    SseEvent::Data {
        payload: payload.to_owned(),
    }
}
