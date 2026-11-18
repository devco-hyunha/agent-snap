//! 복원한 이벤트를 순서대로 꺼내는 스트림 손잡이.

use std::fmt;

use crate::llm::client::error::TransportError;
use crate::llm::client::frames::{ChatDriver, ChatFrame};
use crate::llm::client::outcome::StreamOutcome;
use crate::llm::codec::assembly::ChatAssembly;
use crate::llm::codec::delta::{AssembledResponse, StreamEvent};

/// 복원한 [`StreamEvent`]를 순서대로 꺼내는 스트림 손잡이.
///
/// 안쪽 HTTP 응답·디코더는 전송만 만짐. 공개면에는 이벤트만 보임. 내보내며 조립기로도 누적.
pub struct ChatStream {
    driver: Option<Box<dyn ChatDriver>>,
    assembly: ChatAssembly,
    outcome: Option<StreamOutcome>,
    finished: bool,
}

impl ChatStream {
    /// 전송이 만든 드라이버로 스트림을 만듦.
    pub(crate) fn new(driver: Box<dyn ChatDriver>) -> Self {
        Self {
            driver: Some(driver),
            assembly: ChatAssembly::new(),
            outcome: None,
            finished: false,
        }
    }

    /// 다음 이벤트를 반환. 더 없으면 `None`. 오류면 그 자리에서 스트림이 끝남.
    pub async fn next_event(&mut self) -> Option<Result<StreamEvent, TransportError>> {
        let driver = self.driver.as_mut()?;
        match driver.next_frame().await {
            ChatFrame::Event(Ok(event)) => {
                self.assembly.apply_event(&event);
                Some(Ok(event))
            }
            ChatFrame::Event(Err(error)) => {
                self.close(None);
                Some(Err(error))
            }
            ChatFrame::End(outcome) => {
                self.close(outcome);
                None
            }
        }
    }

    /// 스트림을 끝낸 뒤 사용량·시각만 남김(전송 손잡이는 버림).
    fn close(&mut self, outcome: Option<StreamOutcome>) {
        self.driver = None;
        self.outcome = outcome;
        self.finished = true;
    }

    /// 스트림이 끝난 뒤 사용량·시각. 끝나기 전이면 `None`.
    pub fn outcome(&self) -> Option<&StreamOutcome> {
        if self.finished {
            self.outcome.as_ref()
        } else {
            None
        }
    }

    /// 지금까지 꺼낸 이벤트를 누적해 합친 응답.
    pub fn assembled(&self) -> &AssembledResponse {
        self.assembly.assembled()
    }
}

impl fmt::Debug for ChatStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChatStream")
            .field("has_driver", &self.driver.is_some())
            .field("outcome", &self.outcome)
            .field("finished", &self.finished)
            .finish()
    }
}
