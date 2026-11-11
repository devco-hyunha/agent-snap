//! 03-2 루프 시험의 모의 서버 스트림 본문 조각을 만드는 헬퍼입니다.
//!
//! 내용·도구 호출·빈 스트림을 골격으로 채워, 시험마다 본문 문자열을 다시 조립하지 않게 합니다.

#![allow(dead_code, unreachable_pub)]

use super::super::common::mock_server::MockBodyPart;

/// 내용만 있는 스트림 줄을 모의 서버 본문 조각으로 만듭니다.
#[must_use]
pub fn content_stream(content: &str) -> Vec<MockBodyPart> {
    let line = format!(
        "data: {}\n\n",
        serde_json::json!({"choices":[{"delta":{"content":content}}]})
    );
    vec![
        MockBodyPart::immediate(line.into_bytes()),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}

/// 도구 호출 델타가 있는 스트림 줄을 모의 서버 본문 조각으로 만듭니다.
#[must_use]
pub fn tool_call_stream(id: &str, name: &str, arguments: &str) -> Vec<MockBodyPart> {
    let line = format!(
        "data: {}\n\n",
        serde_json::json!({"choices":[{"delta":{"tool_calls":[
            {"index":0,"id":id,"function":{"name":name,"arguments":arguments}}
        ]}}]})
    );
    vec![
        MockBodyPart::immediate(line.into_bytes()),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}

/// 내용도 도구 호출도 없는 빈 스트림입니다(AC 4 미지원 감지용).
#[must_use]
pub fn empty_stream() -> Vec<MockBodyPart> {
    vec![MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec())]
}

/// 도구 호출을 여러 개(병렬 index) 한 번에 알리는 모의 서버 본문 조각입니다(AC 3 여러 갈래 확인용).
#[must_use]
pub fn tool_calls_stream(calls: &[(&str, &str, &str)]) -> Vec<MockBodyPart> {
    let delta_calls: Vec<serde_json::Value> = calls
        .iter()
        .enumerate()
        .map(|(index, (id, name, arguments))| {
            serde_json::json!({"index": index, "id": id, "function": {"name": name, "arguments": arguments}})
        })
        .collect();
    vec![
        MockBodyPart::immediate(
            format!(
                "data: {}\n\n",
                serde_json::json!({"choices":[{"delta":{"tool_calls": delta_calls}}]})
            )
            .into_bytes(),
        ),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}
