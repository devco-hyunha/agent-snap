//! 02-8 T1 설정 기본값 시험: 맥락 한도 정규화 책임. `context_window` 생략만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn omitted_context_window_defaults_to_65536() {
    let text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama",
      "models": [{ "name": "small", "id": "small-id" }] }] }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads model without window");

    assert_eq!(config.endpoints[0].models[0].context_window, 65536);
}
