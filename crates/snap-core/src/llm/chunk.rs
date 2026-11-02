//! 청크 JSON 스키마 해석: choices·delta·usage·error 필드를 읽습니다.

use serde_json::{Map, Value};

use crate::llm::error::{FormatErrorKind, StreamParseError};

/// 토큰 사용량입니다. 청크 `usage` 스키마와 같습니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Usage {
    /// 프롬프트 토큰 수(있으면).
    pub prompt_tokens: Option<u64>,
    /// 완성 토큰 수(있으면).
    pub completion_tokens: Option<u64>,
    /// 전체 토큰 수(있으면).
    pub total_tokens: Option<u64>,
}

/// 청크 JSON에서 읽은 tool_calls 한 조각입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ParsedToolCallDelta {
    pub index: u32,
    pub id: Option<String>,
    pub name: Option<String>,
    pub arguments: Option<String>,
}

/// 청크 JSON의 `delta` 객체에서 읽은 필드입니다.
#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct ParsedDelta {
    pub content: Option<String>,
    /// `reasoning` / `reasoning_content`를 문서 순서대로 담습니다.
    pub reasonings: Vec<String>,
    pub tool_calls: Vec<ParsedToolCallDelta>,
    pub unknown_fields: Vec<(String, String)>,
}

/// data 본문 JSON을 해석한 결과입니다. `[DONE]`은 호출 전에 걸러집니다.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum ParsedChunk {
    /// 최상위 `error` 필드가 있습니다. 요약용 원문 문구만 담습니다(가리기는 호출측).
    StreamError {
        /// 서버 오류 문구(가리기 전).
        message: String,
    },
    /// 정상 청크입니다.
    Ok {
        usage: Option<Usage>,
        delta: Option<ParsedDelta>,
    },
}

/// data 본문 문자열을 청크 스키마로 해석합니다.
pub(super) fn parse_data_chunk(
    data: &str,
    event_index: usize,
) -> Result<ParsedChunk, StreamParseError> {
    let root = parse_object(data, event_index)?;
    if let Some(error_value) = root.get("error") {
        return Ok(ParsedChunk::StreamError {
            message: error_summary_text(error_value),
        });
    }
    let usage = root.get("usage").and_then(parse_usage);
    let delta = root
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|first| first.get("delta"))
        .and_then(Value::as_object)
        .map(parse_delta);
    Ok(ParsedChunk::Ok { usage, delta })
}

fn parse_object(data: &str, event_index: usize) -> Result<Map<String, Value>, StreamParseError> {
    let value: Value = serde_json::from_str(data).map_err(|_| StreamParseError::Format {
        event_index,
        kind: FormatErrorKind::InvalidJson,
    })?;
    match value {
        Value::Object(map) => Ok(map),
        _ => Err(StreamParseError::Format {
            event_index,
            kind: FormatErrorKind::InvalidJson,
        }),
    }
}

fn parse_usage(value: &Value) -> Option<Usage> {
    let obj = value.as_object()?;
    let prompt_tokens = obj.get("prompt_tokens").and_then(Value::as_u64);
    let completion_tokens = obj.get("completion_tokens").and_then(Value::as_u64);
    let total_tokens = obj.get("total_tokens").and_then(Value::as_u64);
    if prompt_tokens.is_none() && completion_tokens.is_none() && total_tokens.is_none() {
        return None;
    }
    Some(Usage {
        prompt_tokens,
        completion_tokens,
        total_tokens,
    })
}

fn parse_delta(delta: &Map<String, Value>) -> ParsedDelta {
    const KNOWN: &[&str] = &[
        "content",
        "reasoning",
        "reasoning_content",
        "tool_calls",
        "role",
    ];
    let mut parsed = ParsedDelta::default();
    if let Some(content) = delta.get("content").and_then(Value::as_str)
        && !content.is_empty()
    {
        parsed.content = Some(content.to_string());
    }
    // serde_json Map은 삽입 순서를 유지하므로 두 생각 필드가 같이 와도 순서가 남는다.
    for (key, value) in delta {
        if key.as_str() != "reasoning" && key.as_str() != "reasoning_content" {
            continue;
        }
        let Some(text) = value.as_str() else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        parsed.reasonings.push(text.to_string());
    }
    if let Some(tool_calls) = delta.get("tool_calls").and_then(Value::as_array) {
        for call in tool_calls {
            if let Some(piece) = parse_tool_call(call) {
                parsed.tool_calls.push(piece);
            }
        }
    }
    for (key, value) in delta {
        if KNOWN.contains(&key.as_str()) {
            continue;
        }
        parsed.unknown_fields.push((key.clone(), value.to_string()));
    }
    parsed
}

fn parse_tool_call(call: &Value) -> Option<ParsedToolCallDelta> {
    let obj = call.as_object()?;
    let index = obj.get("index").and_then(Value::as_u64).unwrap_or(0) as u32;
    let id = obj
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let function = obj.get("function").and_then(Value::as_object);
    let name = function
        .and_then(|f| f.get("name"))
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let arguments = function
        .and_then(|f| f.get("arguments"))
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    Some(ParsedToolCallDelta {
        index,
        id,
        name,
        arguments,
    })
}

fn error_summary_text(value: &Value) -> String {
    if let Some(message) = value
        .as_object()
        .and_then(|o| o.get("message"))
        .and_then(Value::as_str)
    {
        return message.to_string();
    }
    if let Some(text) = value.as_str() {
        return text.to_string();
    }
    // 원문 JSON 전체를 오류에 넣지 않도록 짧은 고정 문구만 쓴다.
    "서버가 오류 청크를 보냈습니다".to_string()
}
