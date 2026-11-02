//! SSE 줄 처리 단위 테스트(AC 2의 주석·줄 끝·data 결합·BOM·[DONE]·D12).

use crate::llm::{SseItem, SseParser};

#[test]
fn comments_do_not_emit_events() {
    let mut parser = SseParser::new();
    let items = parser
        .push(b": keep-alive\n: OPENROUTER PROCESSING\n\n")
        .unwrap();
    assert!(items.is_empty());
    let tail = parser.finish().unwrap();
    assert!(tail.is_empty());
}

#[test]
fn crlf_lf_cr_and_multiple_data_lines() {
    let mut parser = SseParser::new();
    let mut items = parser
        .push(b"data: {\"a\":1}\r\ndata: {\"b\":2}\n\ndata: lone\r\r\n")
        .unwrap();
    items.extend(parser.finish().unwrap());
    assert_eq!(
        items,
        vec![
            SseItem::Data {
                index: 1,
                data: "{\"a\":1}\n{\"b\":2}".to_string(),
            },
            SseItem::Data {
                index: 2,
                data: "lone".to_string(),
            },
        ]
    );
}

#[test]
fn leading_bom_is_stripped_once() {
    let mut parser = SseParser::new();
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"data: hi\n\n");
    let items = parser.push(&bytes).unwrap();
    assert_eq!(
        items,
        vec![SseItem::Data {
            index: 1,
            data: "hi".to_string()
        }]
    );
}

#[test]
fn done_marker_emits_done() {
    let mut parser = SseParser::new();
    let items = parser.push(b"data: [DONE]\n\n").unwrap();
    assert_eq!(items, vec![SseItem::Done { index: 1 }]);
}

#[test]
fn finish_without_blank_line_dispatches_last_event() {
    let mut parser = SseParser::new();
    parser.push(b"data: {\"x\":1}").unwrap();
    let items = parser.finish().unwrap();
    assert_eq!(
        items,
        vec![SseItem::Data {
            index: 1,
            data: "{\"x\":1}".to_string()
        }]
    );
}
