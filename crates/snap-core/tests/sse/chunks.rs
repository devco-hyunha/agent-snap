//! 조각 경계 불변 단위 테스트(AC 1).

use crate::common::support::{decode_all, decode_byte_by_byte, decode_chunks, mixed_stream_bytes};
use snap_core::llm::StreamEvent;

#[test]
fn mixed_stream_byte_by_byte_matches_whole() {
    let bytes = mixed_stream_bytes();
    let (whole_events, whole_assembled) = decode_all(&bytes).unwrap();
    let (byte_events, byte_assembled) = decode_byte_by_byte(&bytes).unwrap();
    assert_eq!(whole_events, byte_events);
    assert_eq!(whole_assembled, byte_assembled);
}

#[test]
fn mixed_stream_arbitrary_splits_match_whole() {
    let bytes = mixed_stream_bytes();
    let (whole_events, whole_assembled) = decode_all(&bytes).unwrap();
    let mid = bytes.len() / 3;
    let splits: [&[u8]; 3] = [&bytes[..mid], &bytes[mid..mid * 2], &bytes[mid * 2..]];
    let (split_events, split_assembled) = decode_chunks(&splits).unwrap();
    assert_eq!(whole_events, split_events);
    assert_eq!(whole_assembled, split_assembled);
}

#[test]
fn multibyte_and_crlf_split_midway_match_whole() {
    // "안"의 UTF-8은 EC 95 88입니다. 문자 중간과 CRLF 중간에서 자릅니다.
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"data: {\"choices\":[{\"delta\":{\"content\":\"");
    bytes.extend_from_slice("안".as_bytes());
    bytes.extend_from_slice(b"\"}}]}\r\n\r\n");
    let (whole_events, whole_assembled) = decode_all(&bytes).unwrap();
    assert!(
        whole_events
            .iter()
            .any(|e| matches!(e, StreamEvent::Content(t) if t == "안"))
    );

    // 멀티바이트 한가운데와 CRLF 한가운데로 자른다.
    let content_start = bytes
        .windows(3)
        .position(|w| w == "안".as_bytes())
        .expect("content 바이트");
    let cr_pos = bytes.iter().rposition(|&b| b == b'\r').expect("CR");
    let chunks: [&[u8]; 4] = [
        &bytes[..content_start + 1],
        &bytes[content_start + 1..cr_pos],
        &bytes[cr_pos..cr_pos + 1],
        &bytes[cr_pos + 1..],
    ];
    let (split_events, split_assembled) = decode_chunks(&chunks).unwrap();
    assert_eq!(whole_events, split_events);
    assert_eq!(whole_assembled, split_assembled);
}
