//! 전송이 만든 스트림 프레임과 드라이버 타입.

use std::pin::Pin;

use crate::llm::client::error::TransportError;
use crate::llm::client::outcome::StreamOutcome;
use crate::llm::codec::delta::StreamEvent;

/// 전송이 만든 스트림의 한 장.
///
/// 이벤트 한 개, 또는 마지막 종료(사용량·시각 포함)만 담음.
pub(crate) enum ChatFrame {
    /// 복원한 이벤트 또는 그 자리 오류.
    Event(Result<StreamEvent, TransportError>),
    /// 스트림 종료. 정상일 때만 사용량·시각을 담음.
    End(Option<StreamOutcome>),
}

/// 다음 프레임을 만드는 전송 쪽 손잡이.
///
/// 전송 구현만 이 트레이트를 구현. `ChatStream`은 이것만 보고 이벤트를 꺼냄.
pub(crate) trait ChatDriver: Send {
    /// 서버에서 다음 프레임을 하나 받음.
    fn next_frame(&mut self) -> Pin<Box<dyn std::future::Future<Output = ChatFrame> + Send + '_>>;
}
