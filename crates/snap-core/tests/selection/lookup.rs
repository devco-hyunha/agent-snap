//! T1 선택 키 시험: 대상 조회 성공 책임. trim한 이름 비교만 봄.

use crate::material::config;
use crate::material::target_text;

#[test]
fn finds_endpoint_and_model() {
    let text = target_text(&config(), "local/qwen");
    assert_eq!(text, "local / qwen / qwen-id");
}

#[test]
fn finds_model_under_trimmed_names() {
    let text = target_text(&config(), " local / qwen ");
    assert_eq!(text, "local / qwen / qwen-id");
}

#[test]
fn finds_second_model_of_same_endpoint() {
    let text = target_text(&config(), "local/other");
    assert_eq!(text, "local / other / other-id");
}
