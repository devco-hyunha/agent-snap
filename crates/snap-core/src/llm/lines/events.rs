//! 완성 줄을 SSE 이벤트로 바꾸는 상태 타입 자리.

use crate::llm::event::DONE_MARKER;
use crate::llm::event::SseEvent;
use crate::llm::lines::reassemble::Lines;
use crate::llm::lines::split::data_payload;
use crate::llm::split_error::SplitError;

const BOM_CHAR: char = '\u{feff}';

/// 바이트 조각을 SSE 이벤트로 바꾸는 줄 처리 상태.
#[derive(Debug, Default)]
pub struct LineState {
    lines: Lines,
    // 맨 앞 BOM을 처리했는지. 저장소 전체에서 한 번만 적용.
    bom_handled: bool,
    // 여러 `data` 줄 누적 칸.
    data_lines: Vec<String>,
    // `[DONE]`을 받고 스트림이 정상이 됐는지.
    is_done: bool,
}

impl LineState {
    /// 빈 줄 처리 상태.
    pub fn new() -> Self {
        Self::default()
    }

    /// 바이트 조각을 넣고 나온 이벤트 목록.
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<SseEvent>, SplitError> {
        if self.is_done {
            // `[DONE]` 이후 조각은 읽지 않음.
            return Ok(Vec::new());
        }
        let lines = self.lines.push(bytes)?;
        Ok(self.consume_lines(&lines))
    }

    /// 스트림 끝 처리. 빈 줄 없이 끝난 마지막 이벤트를 내보냄(결정 9번).
    pub fn finish(&mut self) -> Result<Vec<SseEvent>, SplitError> {
        if self.is_done {
            return Ok(Vec::new());
        }
        let mut events = Vec::new();
        if let Some(tail) = self.lines.finish()? {
            let tail_lines = vec![tail];
            events.extend(self.consume_lines(&tail_lines));
        }
        events.extend(self.flush_event());
        Ok(events)
    }

    /// 완성 줄 목록을 이벤트로 바꿈.
    fn consume_lines(&mut self, lines: &[String]) -> Vec<SseEvent> {
        let mut events = Vec::new();
        for line in lines {
            events.extend(self.consume_line(line));
        }
        events
    }

    /// 줄 하나 처리. 주석·빈 줄·`data` 줄 결과를 반환.
    fn consume_line(&mut self, line: &str) -> Vec<SseEvent> {
        let line = match self.bom_handled {
            true => line,
            false => {
                self.bom_handled = true;
                line.strip_prefix(BOM_CHAR).unwrap_or(line)
            }
        };
        if line.is_empty() {
            return self.flush_event();
        }
        if line.starts_with(':') {
            return Vec::new();
        }
        let Some(payload) = data_payload(line) else {
            return Vec::new();
        };
        if payload == DONE_MARKER {
            self.is_done = true;
            let mut events = self.flush_event();
            events.push(SseEvent::End);
            return events;
        }
        self.data_lines.push(payload.to_owned());
        Vec::new()
    }

    /// 빈 줄에서 누적된 `data`를 이벤트로 내보냄. 누적 칸은 항상 비움.
    fn flush_event(&mut self) -> Vec<SseEvent> {
        let data_lines = std::mem::take(&mut self.data_lines);
        if data_lines.is_empty() {
            return Vec::new();
        }
        vec![SseEvent::Data {
            payload: data_lines.join("\n"),
        }]
    }
}
