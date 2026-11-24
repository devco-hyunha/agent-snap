//! T2 설정 읽기 시험: 중첩 구조 책임. 엔드포인트와 모델의 계층만 봄.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn reads_nested_endpoints_and_models() {
    let text = r#"{
  // 언어 지정
  "language": "en",
  "endpoints": [
    {
      "name": "primary",
      "base_url": "https://example.invalid/base",
      "kind": "openai",
      "models": [
        { "name": "small", "id": "small-id", "reasoning": "low" }
      ]
    }
  ]
}"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads commented config");

    assert_eq!(config.language, "en");
    assert_eq!(config.endpoints.len(), 1);
    let endpoint = &config.endpoints[0];
    assert_eq!(endpoint.name, "primary");
    assert_eq!(endpoint.kind, "openai");
    assert_eq!(endpoint.models.len(), 1);
    let model = &endpoint.models[0];
    assert_eq!(model.name, "small");
    assert_eq!(model.id, "small-id");
    assert_eq!(model.reasoning.as_deref(), Some("low"));
    assert!(model.sampling.is_none());
    assert!(model.extra.is_none());
}
