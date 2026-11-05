//! 바이트 조각을 SSE·델타로 해석해 스트림 이벤트와 합친 응답을 만듭니다.

use crate::llm::codec::delta::{AssembledResponse, DeltaAssembler, StreamEvent};
use crate::llm::error::{FormatErrorKind, StreamParseError};
use crate::llm::{SseItem, SseParser};

/// 네트워크 없이 바이트 조각을 이벤트와 합친 응답으로 바꾸는 디코더입니다.
#[derive(Debug, Default)]
pub struct StreamDecoder {
    sse: SseParser,
    delta: DeltaAssembler,
    failed: bool,
    /// `finish`에서 나온 data인지 구분해 잘린 JSON을 TruncatedEvent로 보고합니다.
    finishing: bool,
}

impl StreamDecoder {
    /// 빈 디코더를 만듭니다.
    pub fn new() -> Self {
        Self::default()
    }

    /// 지금까지 합친 응답의 참조를 돌려줍니다.
    pub fn assembled(&self) -> &AssembledResponse {
        self.delta.assembled()
    }

    /// 바이트 조각을 넣고 새로 나온 스트림 이벤트를 돌려줍니다.
    ///
    /// 한 번 `Err`를 낸 뒤에는 빈 `Ok`만 돌려줍니다. 성공을 만들어 주지 않는다는 계약이고(D28),
    /// 전송 쪽은 첫 `Err` 이후로 이 디코더를 다시 쓰지 않습니다.
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<StreamEvent>, StreamParseError> {
        if self.failed {
            return Ok(Vec::new());
        }
        let items = match self.sse.push(bytes) {
            Ok(items) => items,
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        };
        self.apply_sse_items(items)
    }

    /// 스트림 끝을 알리고 남은 이벤트를 처리한 뒤 합친 응답을 돌려줍니다(D12).
    pub fn finish(mut self) -> Result<(Vec<StreamEvent>, AssembledResponse), StreamParseError> {
        if self.failed {
            return Ok((Vec::new(), self.delta.into_assembled()));
        }
        self.finishing = true;
        let items = match self.sse.finish() {
            Ok(items) => items,
            Err(error) => {
                self.failed = true;
                return Err(error);
            }
        };
        let events = self.apply_sse_items(items)?;
        Ok((events, self.delta.into_assembled()))
    }

    fn apply_sse_items(
        &mut self,
        items: Vec<SseItem>,
    ) -> Result<Vec<StreamEvent>, StreamParseError> {
        let mut events = Vec::new();
        for item in items {
            let result = match item {
                SseItem::Done { index } => self.delta.push_data("[DONE]", index),
                SseItem::Data { index, data } => self.delta.push_data(&data, index),
            };
            match result {
                Ok(more) => events.extend(more),
                Err(error) => {
                    self.failed = true;
                    return Err(map_finish_format(error, self.finishing));
                }
            }
        }
        Ok(events)
    }
}

fn map_finish_format(error: StreamParseError, finishing: bool) -> StreamParseError {
    match error {
        StreamParseError::Format {
            event_index,
            kind: FormatErrorKind::InvalidJson,
        } if finishing => StreamParseError::Format {
            event_index,
            kind: FormatErrorKind::TruncatedEvent,
        },
        other => other,
    }
}
