//! SSE·델타 테스트 공통 도우미입니다.

use crate::llm::{AssembledResponse, StreamDecoder, StreamEvent, StreamParseError};

pub(super) fn decode_all(
    bytes: &[u8],
) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
    let mut decoder = StreamDecoder::new();
    let mut events = decoder.push(bytes)?;
    let (tail, assembled) = decoder.finish()?;
    events.extend(tail);
    Ok((events, assembled))
}

pub(super) fn decode_chunks(
    chunks: &[&[u8]],
) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
    let mut decoder = StreamDecoder::new();
    let mut events = Vec::new();
    for chunk in chunks {
        events.extend(decoder.push(chunk)?);
    }
    let (tail, assembled) = decoder.finish()?;
    events.extend(tail);
    Ok((events, assembled))
}

pub(super) fn decode_byte_by_byte(
    bytes: &[u8],
) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
    let chunks: Vec<&[u8]> = bytes.chunks(1).collect();
    decode_chunks(&chunks)
}

pub(super) fn content_events(events: &[StreamEvent]) -> String {
    let mut out = String::new();
    for event in events {
        if let StreamEvent::Content(text) = event {
            out.push_str(text);
        }
    }
    out
}

pub(super) fn reasoning_events(events: &[StreamEvent]) -> String {
    let mut out = String::new();
    for event in events {
        if let StreamEvent::Reasoning(text) = event {
            out.push_str(text);
        }
    }
    out
}

pub(super) fn data_line(json: &str) -> String {
    format!("data: {json}\n\n")
}

pub(super) fn mixed_stream_bytes() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    out.extend_from_slice(b": keep-alive\r\n");
    out.extend_from_slice(b": OPENROUTER PROCESSING\n");
    out.extend_from_slice(
        data_line(r#"{"choices":[{"delta":{"content":"안","reasoning_content":"생"}}]}"#)
            .as_bytes(),
    );
    out.extend_from_slice(
        data_line(r#"{"choices":[{"delta":{"content":"녕","reasoning":"각"}}]}"#).as_bytes(),
    );
    out.extend_from_slice(
        data_line(
            r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"call_a","function":{"name":"alpha","arguments":"{\"x\":"}}]}}]}"#,
        )
        .as_bytes(),
    );
    out.extend_from_slice(
        data_line(
            r#"{"choices":[{"delta":{"tool_calls":[{"index":1,"id":"call_b","function":{"name":"beta","arguments":"{\"y\":1"}}]}}]}"#,
        )
        .as_bytes(),
    );
    out.extend_from_slice(
        data_line(
            r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":"1}"}},{"index":1,"function":{"arguments":"}"}}],"reasoning_details":[{"type":"reasoning.text"}]}}]}"#,
        )
        .as_bytes(),
    );
    out.extend_from_slice(
        data_line(
            r#"{"choices":[{"delta":{}}],"usage":{"prompt_tokens":3,"completion_tokens":5,"total_tokens":8}}"#,
        )
        .as_bytes(),
    );
    out.extend_from_slice(b"data: [DONE]\r\n\r\n");
    out
}
