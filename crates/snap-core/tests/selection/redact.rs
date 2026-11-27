//! T1 선택 키 시험: 가리기 책임. 오류 Display·Debug에 키 원문·주소가 없음을 확인.

use crate::material::HOST;
use crate::material::KEY;
use crate::material::reject_texts;

#[test]
fn format_error_hides_key_and_host() {
    let (display, debug) = reject_texts("no-slash-only-name");
    assert!(
        !display.contains(HOST) && !debug.contains(HOST),
        "host is hidden: display={display}, debug={debug}"
    );
    assert!(
        !display.contains("no-slash-only-name") && !debug.contains("no-slash-only-name"),
        "key value is hidden: display={display}, debug={debug}"
    );
}

#[test]
fn target_missing_hides_host() {
    let (display, debug) = reject_texts("local/absent");
    assert!(
        !display.contains(HOST) && !debug.contains(HOST),
        "host is hidden: display={display}, debug={debug}"
    );
}

#[test]
fn target_missing_hides_whole_key_string() {
    let (display, debug) = reject_texts(KEY);
    assert!(
        !display.contains(KEY) && !debug.contains(KEY),
        "whole key string is hidden: display={display}, debug={debug}"
    );
}
