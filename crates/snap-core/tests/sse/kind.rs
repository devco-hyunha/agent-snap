//! 02-3 T1 SSE 이벤트 종류 구분 시험. 종류 값 다섯 종만 봄.
use snap_core::llm::EventKind;

#[test]
fn five_kinds_cover_stream_units() {
    let pairs = [
        (EventKind::Content, EventKind::Reasoning),
        (EventKind::ToolCalls, EventKind::Usage),
        (EventKind::End, EventKind::Content),
    ];

    for (left, right) in pairs {
        assert_ne!(left, right, "{left:?} and {right:?} are distinct kinds");
    }
}
