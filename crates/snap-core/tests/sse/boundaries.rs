//! 02-3 T2 조각 경계 불변성 시험. 한 번에 넣은 결과와 나눈 넣기 결과의 동일성만 봄.
use snap_core::llm::LineState;
use snap_core::llm::SseEvent;

/// 형식이 섞인 덩어리. 주석·data 여러 줄·줄 끝 3종·BOM·`[DONE]`·빈 줄 없는 종료·멀티바이트를 한 흐름에 담음.
const MIXED_STREAM: &str = concat!(
    "\u{feff}: keep-alive\r\n",
    ": OPENROUTER PROCESSING\r",
    "data: first\r\ndata: 첫번째\n",
    "\n",
    "data: 🎉이모지\n\n",
    "data: [DONE]\n",
    "data: 꼬리",
);

#[test]
fn one_byte_chunks_match_whole_stream() {
    assert_matches_whole(1);
}

#[test]
fn two_byte_chunks_match_whole_stream() {
    assert_matches_whole(2);
}

#[test]
fn three_byte_chunks_match_whole_stream() {
    assert_matches_whole(3);
}

#[test]
fn every_chunk_size_matches_whole_stream() {
    // 1~3을 명시 시험으로 두고, 그 밖의 경계까지 폭넓게 확인.
    for size in 4..=MIXED_STREAM.len() {
        assert_matches_whole(size);
    }
}

#[test]
fn multibyte_char_across_chunk_boundary_is_restored() {
    let text = "data: 가\n\n";
    // 가 문자의 한가운데(8바이트)에서 잘라 넣음.
    let (head, tail) = text.as_bytes().split_at(8);

    let events = run_stream(&[head, tail]);

    assert_eq!(
        events,
        vec![SseEvent::Data {
            payload: "가".to_owned()
        }],
        "multibyte char split across chunks is restored"
    );
}

/// 덩어리를 한 번에 넣은 결과와 size 칸씩 나눠 넣은 결과가 같은지 확인.
fn assert_matches_whole(size: usize) {
    let whole = run_stream(&[MIXED_STREAM.as_bytes()]);
    let chunks = chunked(MIXED_STREAM.as_bytes(), size);
    let split = run_stream(&chunks);
    assert_eq!(whole, split, "chunk size {size} must not change events");
}

/// 조각 목록을 넣고 스트림을 끝낸 이벤트 목록.
// 시험 헬퍼의 unwrap은 시험 실패로 밝히려는 자리라 허용.
#[allow(clippy::unwrap_used)]
fn run_stream(chunks: &[&[u8]]) -> Vec<SseEvent> {
    let mut state = LineState::new();
    let mut events = Vec::new();
    for chunk in chunks {
        events.extend(state.push(chunk).unwrap());
    }
    events.extend(state.finish().unwrap());
    events
}

/// 바이트를 size 칸씩으로 나눔.
fn chunked(bytes: &[u8], size: usize) -> Vec<&[u8]> {
    bytes.chunks(size).collect()
}
