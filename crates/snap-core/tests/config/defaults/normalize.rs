//! T2 설정 기본값 시험: 빈 목록 정규화 책임. 생략·빈 목록의 정규화만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn omitted_lists_normalize_to_empty() {
    let text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://example.invalid", "kind": "ollama" }] }"#;

    let config =
        config_from_str(text, Path::new(PATH)).expect("test reads endpoint without models");

    assert!(config.endpoints[0].models.is_empty());
    assert!(config.sampling.is_none());
}

#[test]
fn empty_lists_stay_empty() {
    let text = r#"{ "endpoints": [], "sampling": null }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads empty lists");

    assert!(config.endpoints.is_empty());
    assert!(config.sampling.is_none());
}
