//! 복원한 이벤트를 순서대로 꺼내는 스트림 손잡이입니다.

use std::fmt;

use crate::llm::client::error::TransportError;
use crate::llm::client::frames::{ChatDriver, ChatFrame};
use crate::llm::client::outcome::StreamOutcome;
use crate::llm::codec::delta::StreamEvent;

/// 복원한 [`StreamEvent`]를 순서대로 꺼내는 스트림 손잡이입니다.
///
/// 안쪽 HTTP 응답·디코더는 전송 구현만 만집니다. 공개면에는 이벤트만 보입니다.
pub struct ChatStream {
    driver: Option<Box<dyn ChatDriver>>,
    outcome: Option<StreamOutcome>,
    finished: bool,
}

impl ChatStream {
    /// 전송이 만들어 낸 드라이버로 스트림을 만듭니다.
    pub(crate) fn new(driver: Box<dyn ChatDriver>) -> Self {
        Self {
            driver: Some(driver),
            outcome: None,
            finished: false,
        }
    }

    /// 다음 이벤트를 돌려줍니다. 더 없으면 `None`입니다.
    ///
    /// 오류가 나면 그 자리에서 스트림이 끝납니다.
    pub async fn next_event(&mut self) -> Option<Result<StreamEvent, TransportError>> {
        let driver = self.driver.as_mut()?;
        match driver.next_frame().await {
            ChatFrame::Event(Ok(event)) => Some(Ok(event)),
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

    /// 스트림을 끝낸 뒤 사용량·시각만 남깁니다(전송 손잡이는 버립니다).
    fn close(&mut self, outcome: Option<StreamOutcome>) {
        self.driver = None;
        self.outcome = outcome;
        self.finished = true;
    }

    /// 스트림이 끝난 뒤 사용량·시각입니다. 끝나기 전이면 `None`입니다.
    pub fn outcome(&self) -> Option<&StreamOutcome> {
        if self.finished {
            self.outcome.as_ref()
        } else {
            None
        }
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
