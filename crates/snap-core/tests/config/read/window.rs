//! 02-8 T1 설정 읽기 시험: 맥락 한도 값 책임. 모델 `context_window` 값 읽기만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn reads_model_context_window_value() {
    let text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
      "models": [{ "name": "small", "id": "small-id", "context_window": 32768,
                   "sampling": { "max_tokens": 1200 } }] }] }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads context window");

    let model = &config.endpoints[0].models[0];
    assert_eq!(model.context_window, 32768);

    let sampling = model.sampling.expect("test has sampling block");
    assert_eq!(
        sampling.max_tokens,
        Some(1200),
        "max_tokens stays a separate field"
    );
}
