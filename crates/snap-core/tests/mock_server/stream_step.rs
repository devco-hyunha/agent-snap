//! 본문 스트림 규칙 처리 결과 타입 자리. 필드만 담는 enum만 둠.
use super::script::StreamStop;

/// 규칙 처리 결과.
pub(crate) enum Step {
    /// 보낼 조각.
    Bytes(Vec<u8>),

    /// 규칙 종료. 연결 끊기 방식.
    Finished(StreamStop),
}
