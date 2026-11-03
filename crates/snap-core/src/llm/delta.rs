//! 델타 조립: 스트림 이벤트와 합친 응답을 누적합니다.

use std::collections::BTreeMap;

use crate::llm::chunk::{ParsedChunk, ParsedDelta, ParsedToolCallDelta, parse_data_chunk};
use crate::llm::error::StreamParseError;
use crate::llm::error::redact::summarize_stream_error;

pub use crate::llm::chunk::Usage;

/// 스트림에서 나오는 한 조각 이벤트입니다.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamEvent {
    /// content 델타 조각입니다.
    Content(String),
    /// 생각(`reasoning` / `reasoning_content`) 델타 조각입니다.
    Reasoning(String),
    /// tool_calls 델타입니다. index별로 누적합니다.
    ToolCallDelta {
        /// 병렬 호출 구분 index입니다.
        index: u32,
        /// id 조각(있으면).
        id: Option<String>,
        /// 함수 이름 조각(있으면).
        name: Option<String>,
        /// 인자 JSON 문자열 조각(있으면).
        arguments: Option<String>,
    },
    /// 사용량입니다.
    Usage(Usage),
    /// `[DONE]`으로 정상 종료했습니다.
    Done,
}

/// index별로 합친 도구 호출입니다.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ToolCall {
    /// 도구 호출 id입니다.
    pub id: String,
    /// 함수 이름입니다.
    pub name: String,
    /// 인자 JSON 문자열입니다.
    pub arguments: String,
}

/// 스트림을 끝까지 합친 응답입니다.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssembledResponse {
    /// 이어 붙인 content입니다.
    pub content: String,
    /// 이어 붙인 생각입니다.
    pub reasoning: String,
    /// index별 도구 호출입니다.
    pub tool_calls: BTreeMap<u32, ToolCall>,
    /// 모르는 델타 필드의 (이름, 원문 JSON) 목록입니다. JSON에 나온 삽입 순서를 유지합니다.
    pub unknown_delta_fields: Vec<(String, String)>,
    /// 사용량입니다. 없으면 [`None`](“없음”).
    pub usage: Option<Usage>,
}

/// 청크 data 문자열을 받아 이벤트와 합친 응답을 만드는 조립기입니다.
#[derive(Debug, Default)]
pub struct DeltaAssembler {
    assembled: AssembledResponse,
    done: bool,
    failed: bool,
}

impl DeltaAssembler {
    /// 빈 조립기를 만듭니다.
    pub fn new() -> Self {
        Self::default()
    }

    /// 지금까지 합친 응답의 참조를 돌려줍니다.
    pub fn assembled(&self) -> &AssembledResponse {
        &self.assembled
    }

    /// 합친 응답을 꺼냅니다.
    pub fn into_assembled(self) -> AssembledResponse {
        self.assembled
    }

    /// SSE data 본문 하나를 해석해 이벤트를 돌려줍니다.
    ///
    /// `event_index`는 1부터 세는 data 이벤트 번호입니다(형식 오류용).
    pub fn push_data(
        &mut self,
        data: &str,
        event_index: usize,
    ) -> Result<Vec<StreamEvent>, StreamParseError> {
        if self.failed || self.done {
            return Ok(Vec::new());
        }
        if data == "[DONE]" {
            self.done = true;
            return Ok(vec![StreamEvent::Done]);
        }
        match parse_data_chunk(data, event_index)? {
            ParsedChunk::StreamError { message } => {
                self.failed = true;
                let summary = summarize_stream_error(&message);
                Err(StreamParseError::Stream { summary })
            }
            ParsedChunk::Ok { usage, delta } => {
                let mut events = Vec::new();
                if let Some(usage) = usage {
                    self.assembled.usage = Some(usage.clone());
                    events.push(StreamEvent::Usage(usage));
                }
                if let Some(parsed_delta) = delta {
                    self.apply_parsed_delta(parsed_delta, &mut events);
                }
                Ok(events)
            }
        }
    }

    fn apply_parsed_delta(&mut self, delta: ParsedDelta, events: &mut Vec<StreamEvent>) {
        if let Some(content) = delta.content {
            self.assembled.content.push_str(&content);
            events.push(StreamEvent::Content(content));
        }
        for text in delta.reasonings {
            self.assembled.reasoning.push_str(&text);
            events.push(StreamEvent::Reasoning(text));
        }
        for piece in delta.tool_calls {
            events.push(self.apply_tool_call_delta(piece));
        }
        for (key, raw) in delta.unknown_fields {
            self.assembled.unknown_delta_fields.push((key, raw));
        }
    }

    fn apply_tool_call_delta(&mut self, piece: ParsedToolCallDelta) -> StreamEvent {
        let entry = self.assembled.tool_calls.entry(piece.index).or_default();
        let id = piece.id.inspect(|s| {
            entry.id.push_str(s);
        });
        let name = piece.name.inspect(|s| {
            entry.name.push_str(s);
        });
        let arguments = piece.arguments.inspect(|s| {
            entry.arguments.push_str(s);
        });
        StreamEvent::ToolCallDelta {
            index: piece.index,
            id,
            name,
            arguments,
        }
    }
}
