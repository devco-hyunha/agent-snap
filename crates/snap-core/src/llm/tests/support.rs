//! SSE·델타·전송·요청 테스트 공통 도우미입니다.

use std::time::Duration;

use super::mock_server::MockBodyPart;
use crate::config::{EndpointKind, ExtraValue, HttpSettings, Reasoning, Sampling};
use crate::llm::{
    AssembledResponse, ChatMessage, ChatRequest, OpenAiCompatClient, StreamDecoder, StreamEvent,
    StreamParseError, ToolDefinition,
};
use crate::secrets::SecretValue;

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

/// 모의 서버 주소로 테스트용 전송 클라이언트를 하나 만듭니다.
///
/// `kind`마다 요청 본문이 달라지므로 본문·헤더를 보는 테스트가 같은 조립 코드를 다시 쓰지 않게 합니다.
/// 키를 주지 않으면 Authorization 헤더를 붙이지 않습니다.
pub(super) fn client_at(
    base_url: &str,
    kind: EndpointKind,
    api_key: Option<&str>,
    http: HttpSettings,
) -> OpenAiCompatClient {
    OpenAiCompatClient::new(
        SecretValue::new(base_url),
        kind,
        api_key.map(SecretValue::new),
        http,
    )
    .expect("클라이언트를 만들어야 합니다")
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

/// 시험용 시간 값: 연결 5초, 헤더 대기 2초, 유휴 1초.
///
/// 연결 시간이 1초처럼 아주 짧으면 Windows가 닫힌 포트에 연결을 다시 시도하느라
/// 라이브러리가 연결 시간 초과로 먼저 알립니다. 연결 세부 확인에 쓸 수 있게 연결만 넉넉히 뒀습니다.
pub(super) fn test_http() -> HttpSettings {
    HttpSettings {
        connect_seconds: Duration::from_secs(5),
        header_wait_seconds: Duration::from_secs(2),
        stream_idle_seconds: Duration::from_secs(1),
        ..HttpSettings::default()
    }
}

/// 본문·헤더·전송 확인에 쓰는 하나의 요청입니다.
///
/// 생각 옵션을 `None`으로 넣으면 도구·생각 항목이 없는 단순 요청입니다.
/// 생각 옵션을 넣으면 그 옵션에 맞춰 본문에 생각 항목이 붙고, 본문 검사와 필요한 `echo` 도구가 함께 들어갑니다.
pub(super) fn sample_request(reasoning: Option<Reasoning>) -> ChatRequest {
    let mut extra = std::collections::BTreeMap::new();
    extra.insert("keep_alive".into(), ExtraValue::Number("0".into()));
    extra.insert(
        "options".into(),
        ExtraValue::Object(std::collections::BTreeMap::from([(
            "num_ctx".into(),
            ExtraValue::Number("8192".into()),
        )])),
    );
    ChatRequest {
        model: "demo-model".into(),
        messages: vec![ChatMessage {
            role: "user".into(),
            content: "hello".into(),
        }],
        tools: if reasoning.is_some() {
            vec![ToolDefinition {
                name: "echo".into(),
                description: "says back".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
            }]
        } else {
            Vec::new()
        },
        sampling: Sampling {
            temperature: Some(0.2),
            top_p: Some(0.9),
            max_tokens: Some(64),
        },
        reasoning,
        extra,
    }
}

/// 바이트를 37개씩 잘라 즉시 보내는 본문 조각으로 만듭니다.
pub(super) fn body_parts_sse(bytes: &[u8]) -> Vec<MockBodyPart> {
    bytes
        .chunks(37)
        .map(|chunk| MockBodyPart::immediate(chunk.to_vec()))
        .collect()
}
