//! 이벤트 타입 자리. 줄 처리가 만드는 단위만 담는 파일. 델타 조립은 `delta/`가 담당.

/// `[DONE]` 표시. data 페이로드가 이 값이면 스트림을 정상 종료로 처리.
pub const DONE_MARKER: &str = "[DONE]";

/// 이벤트 종류. content·생각·도구 호출·사용량·끝을 가르는 구분 타입.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    /// content 조각.
    Content,

    /// 생각 조각.
    Reasoning,

    /// 도구 호출 델타.
    ToolCalls,

    /// 사용량.
    Usage,

    /// 끝.
    End,
}

/// 줄 처리가 만들어 내는 이벤트 단위.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SseEvent {
    /// `data` 줄을 개행으로 잇고 빈 줄에서 내보낸 이벤트.
    Data {
        /// 여러 `data` 줄을 개행으로 잇고 콜론 뒤 첫 칸만 뺀 내용.
        payload: String,
    },

    /// 스트림 정상 종료.
    End,
}
