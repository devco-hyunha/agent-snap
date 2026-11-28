//! 02-8 T2 설정 거부 시험: 모델 `context_window` 값 책임. 1 이상 정수 밖 거부만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

/// 1 이상 정수 밖 값. 거부 후 오류에 입력 값이 안 나오지 않는지 보는 재료.
const BAD_VALUES: [&str; 6] = ["0", "-2", "1.5", "\"10\"", "{}", "[1]"];

/// 정상 값 시험 재료.
const VALID_TEXT: &str = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
  "models": [{ "name": "small", "id": "small-id", "context_window": 32768 }] }] }"#;

#[test]
fn rejects_bad_context_window_values() {
    for value in BAD_VALUES {
        let text = format!(
            r#"{{ "endpoints": [{{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
              "models": [{{ "name": "small", "id": "small-id", "context_window": {value} }}] }}] }}"#
        );

        let err =
            config_from_str(&text, Path::new(PATH)).expect_err("test expects bad context window");
        let message = err.to_string();

        assert!(
            message.contains("small") && message.contains("context_window"),
            "message names model and option in {text}: {message}"
        );
        assert!(
            !message.contains(value.trim_matches('"')),
            "message hides the input value {value}: {message}"
        );
    }
}

#[test]
fn rejection_debug_shows_only_model_name() {
    let bad_text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
      "models": [{ "name": "small", "id": "small-id", "context_window": 0 }] }] }"#;
    let err = config_from_str(bad_text, Path::new(PATH)).expect_err("test expects rejection");
    let debug = format!("{err:?}");

    assert!(
        debug.contains("small") && debug.contains("ContextWindowRejected"),
        "debug names model and kind: {debug}"
    );
    assert!(!debug.contains('0'), "debug hides the input value: {debug}");
}

#[test]
fn accepts_context_window_of_one() {
    let text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
      "models": [{ "name": "small", "id": "small-id", "context_window": 1 }] }] }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test accepts 1");
    assert_eq!(config.endpoints[0].models[0].context_window, 1);
}

#[test]
fn accepts_normal_context_window() {
    let config = config_from_str(VALID_TEXT, Path::new(PATH)).expect("test accepts normal value");
    assert_eq!(config.endpoints[0].models[0].context_window, 32768);
}

#[test]
fn locale_texts_share_the_option_name() {
    let ko = snap_core::locale::ko::validate::CONFIG_CONTEXT_WINDOW_REJECTED;
    let en = snap_core::locale::en::validate::CONFIG_CONTEXT_WINDOW_REJECTED;

    assert!(
        ko.contains("context_window") && en.contains("context_window"),
        "both locales name the option: ko={ko}, en={en}"
    );
    assert!(
        !ko.is_empty() && !en.is_empty() && ko != en,
        "ko and en are distinct non-empty texts for the same option: ko={ko}, en={en}"
    );
}
