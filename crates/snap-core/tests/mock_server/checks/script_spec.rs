//! 응답 스크립트 규격 확인. 기본값과 채우기 동작을 본다.
use std::time::Duration;

use crate::script::HeaderMode;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;

#[test]
fn default_script_is_status_200_with_immediate_header() {
    let script = MockScript::ok();

    assert_eq!(script.status, 200);
    assert_eq!(script.header, HeaderMode::Immediate);
    assert!(script.body.is_empty());
    assert_eq!(script.stop, StreamStop::End);
}

#[test]
fn header_delay_reports_its_wait() {
    let wait = Duration::from_millis(25);

    assert_eq!(HeaderMode::Immediate.wait(), Duration::ZERO);
    assert_eq!(HeaderMode::Delay(wait).wait(), wait);
}

#[test]
fn builder_fills_status_header_body_stop() {
    let wait = Duration::from_millis(15);
    let body = vec![StreamStep::chunk("data: a\n\n").chunk_after(wait)];

    let script = MockScript::with_status(503)
        .with_header_delay(wait)
        .with_header("retry-after", "7")
        .with_body(body.clone())
        .with_stop(StreamStop::Cut);

    assert_eq!(script.status, 503);
    assert_eq!(script.header, HeaderMode::Delay(wait));
    assert_eq!(
        script.headers,
        vec![("retry-after".to_owned(), "7".to_owned())]
    );
    assert_eq!(script.body, body);
    assert_eq!(script.stop, StreamStop::Cut);
}

#[test]
fn chunk_step_defaults_to_zero_gap() {
    let step = StreamStep::chunk("data: a\n\n");

    assert!(matches!(step, StreamStep::Chunk { gap, .. } if gap.is_zero()));
}

#[test]
fn chunk_after_sets_gap() {
    let wait = Duration::from_millis(7);

    let step = StreamStep::chunk("data: a\n\n").chunk_after(wait);

    assert!(matches!(step, StreamStep::Chunk { gap, .. } if gap == wait));
}
