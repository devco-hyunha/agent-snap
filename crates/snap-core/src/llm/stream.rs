//! 바이트 조각을 받아 하나의 응답으로 합치는 스트림 상태 타입 자리.

use serde_json::Value;

use crate::llm::ChunkErrorKind;
use crate::llm::Delta;
use crate::llm::DeltaState;
use crate::llm::SseError;
use crate::llm::SseEvent;
use crate::llm::lines::LineState;
use crate::llm::split_error::SplitError;
use crate::llm::summary::redact_summary;
use crate::llm::summary::stream_error_text;

/// 바이트 조각을 받아 하나의 응답으로 합치는 스트림 상태.
#[derive(Debug, Default)]
pub struct StreamState {
    lines: LineState,
    delta: DeltaState,
    // 형식 오류에 밝히는 이벤트 번호. data 이벤트가 늘 때마다 1 증가.
    event_index: usize,
    // 오류로 스트림이 끝났는지. 이후 조각은 읽지 않음.
    is_failed: bool,
}

impl StreamState {
    /// 빈 스트림 상태.
    pub fn new() -> Self {
        Self::default()
    }

    /// 바이트 조각을 넣고 합침. 오류가 나면 스트림이 끝난다.
    pub fn push(&mut self, bytes: &[u8]) -> Result<(), SseError> {
        if self.is_failed {
            return Ok(());
        }
        let events = self.lines.push(bytes).map_err(|e| self.split_error(&e))?;
        self.consume(&events)
    }

    /// 스트림 끝. 빈 줄 없이 끝난 마지막 이벤트까지 합침.
    pub fn finish(&mut self) -> Result<(), SseError> {
        if self.is_failed {
            return Ok(());
        }
        let events = self.lines.finish().map_err(|e| self.split_error(&e))?;
        self.consume(&events)
    }

    /// 끝까지 합친 하나의 응답.
    pub fn delta(&self) -> Delta {
        self.delta.current()
    }

    /// 잘린 UTF-8 문자열을 형식 오류로 옮김. 이벤트 번호와 종류만 담음.
    fn split_error(&self, split: &SplitError) -> SseError {
        let kind = ChunkErrorKind::from_split_error(split);
        SseError::format(self.event_index + 1, kind)
    }

    /// 이벤트 목록을 델타에 합침. `error` 청크와 JSON 실패를 먼저 본다.
    fn consume(&mut self, events: &[SseEvent]) -> Result<(), SseError> {
        for event in events {
            let SseEvent::Data { payload } = event else {
                continue;
            };
            self.event_index += 1;
            let value: Value = match serde_json::from_str(payload) {
                Ok(value) => value,
                Err(error) => {
                    let kind = ChunkErrorKind::from_json_error(&error);
                    return Err(self.fail(SseError::format(self.event_index, kind)));
                }
            };
            let summary = match stream_error_text(&value) {
                Some(raw) => raw,
                None => {
                    self.delta.apply_value(&value);
                    continue;
                }
            };
            return Err(self.fail(SseError::stream(redact_summary(&summary))));
        }
        Ok(())
    }

    /// 오류를 남기고 스트림을 끝난 상태로 만듦.
    fn fail(&mut self, error: SseError) -> SseError {
        self.is_failed = true;
        error
    }
}
