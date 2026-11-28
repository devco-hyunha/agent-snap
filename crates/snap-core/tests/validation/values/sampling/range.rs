//! T3 검증 시험: 샘플링 범위 책임. 설정·모델 값 상한 거부만 봄.
use crate::material::endpoints;
use crate::material::reject_message;

const MODEL: &str = "small";

#[test]
fn rejects_sampling_out_of_range() {
    let cases = [
        (r#"{ "sampling": { "temperature": 2.5 } }"#, "temperature"),
        (r#"{ "sampling": { "temperature": -1 } }"#, "temperature"),
        (r#"{ "sampling": { "top_p": 0 } }"#, "top_p"),
        (r#"{ "sampling": { "top_p": 1.2 } }"#, "top_p"),
        (r#"{ "sampling": { "max_tokens": 0 } }"#, "max_tokens"),
    ];
    for (text, item) in cases {
        let message = reject_message(text);

        assert!(
            message.contains(item),
            "message names {item} in {text}: {message}"
        );
    }
}

#[test]
fn rejects_model_sampling_out_of_range() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "https://secret-host.invalid", "kind": "ollama",
             "models": [{ "name": "small", "id": "id-a", "sampling": { "temperature": 9 } }] }]"#,
    );

    let message = reject_message(&text);

    assert!(
        message.contains(MODEL) && message.contains("temperature"),
        "message names model and item: {message}"
    );
    assert!(
        !message.contains("9"),
        "message hides input value: {message}"
    );
}
