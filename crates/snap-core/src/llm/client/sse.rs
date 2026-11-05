//! SSE 줄 처리: 바이트 조각을 data 페이로드와 `[DONE]`으로 바꿉니다.
//!
//! 스트림 입력 라인 디코더라 클라이언트 쪽 책임이고, 쓰는 곳은 `codec::decode`의
//! `StreamDecoder` 하나뿐입니다. 이 디코더를 통과한 이벤트와 합친 응답을
//! `client::stream`·`client::outcome`이 받아 쓰므로 줄 처리 정본은 여기만 씁니다(작업 08 2단계).

use crate::llm::error::{FormatErrorKind, StreamParseError};

/// SSE 파서가 내보내는 한 항목입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SseItem {
    /// 한 이벤트의 `data` 필드를 개행으로 이은 본문입니다.
    Data {
        /// 1부터 세는 data 이벤트 번호입니다.
        index: usize,
        /// data 본문입니다.
        data: String,
    },
    /// data 본문이 `[DONE]`인 정상 종료 표식입니다.
    Done {
        /// 1부터 세는 data 이벤트 번호입니다.
        index: usize,
    },
}

/// 바이트 조각을 받아 SSE data 이벤트를 내보내는 파서입니다.
///
/// 줄 끝 CRLF·LF·CR, `:` 주석, 여러 `data` 결합, 맨 앞 BOM, 조각 경계·멀티바이트 잘림을
/// 처리합니다. 빈 줄 없이 끝난 마지막 이벤트는 [`SseParser::finish`]에서 내보냅니다(D12).
#[derive(Debug, Default)]
pub struct SseParser {
    buffer: Vec<u8>,
    bom_checked: bool,
    /// 현재 이벤트에 쌓인 data 줄(아직 빈 줄로 내보내지 않음).
    data_lines: Vec<String>,
    /// 지금까지 내보낸 data 이벤트 수입니다.
    emitted_events: usize,
    finished: bool,
}

impl SseParser {
    /// 빈 파서를 만듭니다.
    pub fn new() -> Self {
        Self::default()
    }

    /// 다음에 내보낼 data 이벤트의 번호(1부터)를 돌려줍니다.
    pub fn next_event_index(&self) -> usize {
        self.emitted_events + 1
    }

    /// 바이트 조각을 넣고, 완성된 SSE 항목을 돌려줍니다.
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<SseItem>, StreamParseError> {
        if self.finished {
            return Ok(Vec::new());
        }
        self.buffer.extend_from_slice(bytes);
        self.strip_bom_if_needed();
        self.drain_complete_lines(false)
    }

    /// 남은 버퍼를 마지막 이벤트로 처리합니다(D12). 이후 `push`는 빈 결과를 냅니다.
    pub fn finish(&mut self) -> Result<Vec<SseItem>, StreamParseError> {
        if self.finished {
            return Ok(Vec::new());
        }
        self.finished = true;
        self.strip_bom_if_needed();
        let mut items = self.drain_complete_lines(true)?;
        if !self.buffer.is_empty() {
            let line_bytes = std::mem::take(&mut self.buffer);
            let line = self.decode_line(&line_bytes)?;
            self.consume_line(&line);
        }
        if let Some(item) = self.dispatch_pending() {
            items.push(item);
        }
        Ok(items)
    }

    fn strip_bom_if_needed(&mut self) {
        if self.bom_checked {
            return;
        }
        if self.buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
            self.buffer.drain(..3);
            self.bom_checked = true;
            return;
        }
        // BOM이 아니면 확정. 1~2바이트만 와 UTF-8 BOM 접두일 수 있으면 대기.
        if self.buffer.len() >= 3 || !could_be_bom_prefix(&self.buffer) {
            self.bom_checked = true;
        }
    }

    fn drain_complete_lines(&mut self, at_end: bool) -> Result<Vec<SseItem>, StreamParseError> {
        let mut items = Vec::new();
        while let Some((line_end, skip)) = find_line_end(&self.buffer, at_end) {
            let line_bytes = self.buffer[..line_end].to_vec();
            self.buffer.drain(..line_end + skip);
            let line = self.decode_line(&line_bytes)?;
            if line.is_empty() {
                if let Some(item) = self.dispatch_pending() {
                    items.push(item);
                }
            } else {
                self.consume_line(&line);
            }
        }
        Ok(items)
    }

    fn decode_line(&self, bytes: &[u8]) -> Result<String, StreamParseError> {
        match std::str::from_utf8(bytes) {
            Ok(text) => Ok(text.to_string()),
            Err(_) => Err(StreamParseError::Format {
                event_index: self.next_event_index(),
                kind: FormatErrorKind::InvalidUtf8,
            }),
        }
    }

    fn consume_line(&mut self, line: &str) {
        if line.starts_with(':') {
            return;
        }
        let (field, value) = split_field(line);
        if field != "data" {
            return;
        }
        self.data_lines.push(value.to_string());
    }

    fn dispatch_pending(&mut self) -> Option<SseItem> {
        if self.data_lines.is_empty() {
            return None;
        }
        let mut data = self.data_lines.join("\n");
        self.data_lines.clear();
        if data.ends_with('\n') {
            data.pop();
        }
        if data.is_empty() {
            return None;
        }
        self.emitted_events += 1;
        let index = self.emitted_events;
        if data == "[DONE]" {
            Some(SseItem::Done { index })
        } else {
            Some(SseItem::Data { index, data })
        }
    }
}

fn could_be_bom_prefix(bytes: &[u8]) -> bool {
    matches!(bytes, [] | [0xEF] | [0xEF, 0xBB])
}

/// `(줄 바이트 길이, 줄 끝 바이트 수)`. `at_end`이면 버퍼 끝의 단독 CR도 줄 끝으로 본다.
fn find_line_end(buffer: &[u8], at_end: bool) -> Option<(usize, usize)> {
    let mut i = 0;
    while i < buffer.len() {
        match buffer[i] {
            b'\n' => return Some((i, 1)),
            b'\r' => {
                if i + 1 < buffer.len() {
                    if buffer[i + 1] == b'\n' {
                        return Some((i, 2));
                    }
                    return Some((i, 1));
                }
                if at_end {
                    return Some((i, 1));
                }
                return None;
            }
            _ => i += 1,
        }
    }
    None
}

fn split_field(line: &str) -> (&str, &str) {
    match line.split_once(':') {
        None => (line, ""),
        Some((name, rest)) => {
            let value = rest.strip_prefix(' ').unwrap_or(rest);
            (name, value)
        }
    }
}
