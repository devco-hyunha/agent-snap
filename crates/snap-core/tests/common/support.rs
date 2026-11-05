#![allow(dead_code, unreachable_pub)]

//! SSE·델타·전송·요청 테스트 공통 도우미입니다.

use std::time::Duration;

use super::mock_server::MockBodyPart;
use snap_core::config::{EndpointKind, ExtraValue, HttpSettings, Reasoning, Sampling};
#[allow(unused_imports)]
use snap_core::llm::{
    AssembledResponse, CancellationToken, ChatMessage, ChatRequest, ChatStream, LlmClient,
    OpenAiCompatClient, StreamDecoder, StreamEvent, StreamParseError, ToolDefinition,
};
use snap_core::secrets::SecretValue;

pub fn decode_all(bytes: &[u8]) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
    let mut decoder = StreamDecoder::new();
    let mut events = decoder.push(bytes)?;
    let (tail, assembled) = decoder.finish()?;
    events.extend(tail);
    Ok((events, assembled))
}

pub fn decode_chunks(
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

pub fn decode_byte_by_byte(
    bytes: &[u8],
) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
    let chunks: Vec<&[u8]> = bytes.chunks(1).collect();
    decode_chunks(&chunks)
}

pub fn content_events(events: &[StreamEvent]) -> String {
    let mut out = String::new();
    for event in events {
        if let StreamEvent::Content(text) = event {
            out.push_str(text);
        }
    }
    out
}

pub fn reasoning_events(events: &[StreamEvent]) -> String {
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
pub fn client_at(
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

/// OpenAI 호환 기준으로 여는 짧은 이름의 도우미입니다.
pub fn client_at_with(
    base_url: &str,
    api_key: Option<&str>,
    http: HttpSettings,
) -> OpenAiCompatClient {
    client_at(base_url, EndpointKind::OpenAi, api_key, http)
}

pub fn data_line(json: &str) -> String {
    format!("data: {json}\n\n")
}

/// 내용 전달 한 줄을 조립합니다. 내용 값만 자리에 끼웁니다.
pub fn delta_line(content: &str) -> String {
    data_line(&format!(
        r#"{{"choices":[{{"delta":{{"content":"{content}"}}}}]}}"#
    ))
}

/// 정상 스트림의 마지막 줄입니다. 내용 줄 다음에 보냅니다.
pub fn done_line() -> String {
    String::from("data: [DONE]\n\n")
}

pub fn mixed_stream_bytes() -> Vec<u8> {
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
pub fn test_http() -> HttpSettings {
    HttpSettings {
        connect_seconds: Duration::from_secs(5),
        header_wait_seconds: Duration::from_secs(2),
        stream_idle_seconds: Duration::from_secs(1),
        ..HttpSettings::default()
    }
}

/// 헤더 전 연결 세부를 보는 시험용 시간 값입니다.
///
/// Windows는 닫힌 포트에 대한 연결을 몇 초 다시 시도합니다. 헤더 대기(2초)가 먼저
/// 끝나면 시간 초과로 보여 연결 세부를 못 봅니다. 헤더 대기를 넉넉히 둬 연결 실패가 먼저 오게 합니다.
pub fn connect_check_http() -> HttpSettings {
    HttpSettings {
        header_wait_seconds: Duration::from_secs(30),
        ..test_http()
    }
}

/// 상태 재시도 통합 시험용 시간 값: 연결·헤더 대기는 `test_http`를 잇고, 백오프와 `Retry-After` 상한만 짧게 씁니다.
///
/// 시험 값: 백오프 10~100밀리초, `Retry-After` 상한 2초, 헤더 대기 10초.
/// 헤더 대기가 재시도 대기(백오프 10~100밀리초, `Retry-After`가 준 초)보다 길어야
/// 재시도 도중 시간 초과의 신호로 잡히지 않습니다.
/// `retries`는 매개변수로 받아 시험이 요청 수(AC 2)를 확인하는 기준으로 씁니다.
pub fn retry_status_http(retries: u32) -> HttpSettings {
    HttpSettings {
        retries,
        backoff_min: Duration::from_millis(10),
        backoff_max: Duration::from_millis(100),
        retry_after_max: Duration::from_secs(2),
        header_wait_seconds: Duration::from_secs(10),
        ..test_http()
    }
}

/// 본문·헤더·전송 확인에 쓰는 하나의 요청입니다.
///
/// 생각 옵션을 `None`으로 넣으면 도구·생각 항목이 없는 단순 요청입니다.
/// 생각 옵션을 넣으면 그 옵션에 맞춰 본문에 생각 항목이 붙고, 본문 검사와 필요한 `echo` 도구가 함께 들어갑니다.
pub fn sample_request(reasoning: Option<Reasoning>) -> ChatRequest {
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
pub fn body_parts_sse(bytes: &[u8]) -> Vec<MockBodyPart> {
    bytes
        .chunks(37)
        .map(|chunk| MockBodyPart::immediate(chunk.to_vec()))
        .collect()
}

/// 지정한 내용 줄 뒤에 `data: [DONE]`을 붙여 성공 스트림 본문을 조립합니다.
pub fn success_stream_body(contents: &[&str]) -> Vec<u8> {
    let mut out = Vec::new();
    for content in contents {
        out.extend_from_slice(delta_line(content).as_bytes());
    }
    out.extend_from_slice(done_line().as_bytes());
    out
}

/// 스트림을 끝까지 읽고 이벤트만 모읍니다.
pub async fn drain(stream: &mut ChatStream) -> Vec<StreamEvent> {
    let mut events = Vec::new();
    while let Some(event) = stream.next_event().await {
        match event {
            Ok(event) => events.push(event),
            Err(error) => panic!("스트림 오류: {error}"),
        }
    }
    events
}
