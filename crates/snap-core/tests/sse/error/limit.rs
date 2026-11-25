//! 02-3 T4 요약 길이 상한 시험. 상한을 지키는 동작과 상한 정점 보존만 봄.

use snap_core::llm::SUMMARY_MAX_CHARS;
use snap_core::llm::redact_summary;

#[test]
fn summary_is_cut_at_length_limit() {
    let long = format!("{}{}", "a".repeat(400), " tail");
    let summary = redact_summary(&long);

    let shown = summary.expose();
    assert!(
        shown.len() <= SUMMARY_MAX_CHARS,
        "summary respects the limit: {} chars",
        shown.len()
    );
}

#[test]
fn summary_never_exceeds_length_limit() {
    let long = "ab ".repeat(200);
    let summary = redact_summary(&long);
    let shown = summary.expose();

    assert!(
        shown.len() <= SUMMARY_MAX_CHARS,
        "summary must stop at 120 chars, got {}",
        shown.len()
    );
}

#[test]
fn summary_keeps_text_at_exact_limit() {
    let exact = "ab".repeat(60);
    let summary = redact_summary(&exact);
    let shown = summary.expose();

    assert_eq!(
        shown.len(),
        SUMMARY_MAX_CHARS,
        "text at the limit stays at the limit"
    );
}
