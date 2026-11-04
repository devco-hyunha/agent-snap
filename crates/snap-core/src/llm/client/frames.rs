//! 전송이 만들어 낸 스트림 프레임과 드라이버 타입입니다.

use std::pin::Pin;

use crate::llm::client::error::TransportError;
use crate::llm::client::outcome::StreamOutcome;
use crate::llm::delta::StreamEvent;

/// 전송이 만들어 낸 스트림의 한 장입니다.
///
/// 이벤트 한 개, 또는 마지막에 오는 종료(사용량·시각 포함)만 담습니다.
pub(crate) enum ChatFrame {
    /// 복원한 이벤트, 그 자리에서 난 오류입니다.
    Event(Result<StreamEvent, TransportError>),
    /// 스트림이 끝났습니다. 정상 종료일 때만 사용량·시각을 담습니다.
    End(Option<StreamOutcome>),
}

/// 다음 프레임을 만드는 전송 쪽의 손잡이입니다.
///
/// 전송 구현만 이 트레이트를 구현하고, `ChatStream`은 이것만 믿고 이벤트를 꺼냅니다.
pub(crate) trait ChatDriver: Send {
    /// 서버에서 다음 프레임을 하나 받아 옵니다.
    fn next_frame(&mut self) -> Pin<Box<dyn std::future::Future<Output = ChatFrame> + Send + '_>>;
}
