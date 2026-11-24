//! T3 검증 시험: 모르는 샘플링 항목 책임. 이름 없는 항목 거부만 봄.
use crate::material::reject_message;

#[test]
fn rejects_unknown_sampling_item() {
    let text = r#"{ "sampling": { "topk": 5 } }"#;

    let message = reject_message(text);

    assert!(
        message.contains("topk"),
        "message names the item: {message}"
    );
}
