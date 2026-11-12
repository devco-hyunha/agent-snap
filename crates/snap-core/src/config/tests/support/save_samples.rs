//! 설정 저장 견본 조립: 저장 정보·모델 저장·예약 키 목록과 설정 파일 텍스트 견본을 한 곳에 모습니다.
//!
//! 설정 저장을 보는 시험과 저장 경로 가리기를 보는 시험, 이름 바꾸기·삭제 시험이 같은 조립과
//! 같은 텍스트를 다시 적지 않게 하려는 자리입니다(중복 금지·3절 DRY). 본문은 `save/config.rs`와
//! `save/rename.rs`에서 그대로 옮겨 왔습니다.

use std::collections::BTreeMap;

use super::config_text::base_url;
use crate::config::{
    EndpointKind, EndpointSave, ExtraValue, ModelSave, Reasoning, Sampling, ToolCallForm,
};

/// 이름과 주소를 고른 저장 정보(모델 `fast` 하나, 모델 옵션 전부).
pub(crate) fn full_save(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::LlamaCpp,
        models: vec![full_model_save()],
    }
}

/// `reasoning`만 준 저장 정보(AC 4).
pub(crate) fn reasoning_only_save() -> EndpointSave {
    EndpointSave {
        name: "main".to_owned(),
        base_url: base_url(),
        kind: EndpointKind::Ollama,
        models: vec![ModelSave {
            name: "fast".to_owned(),
            id: "id-1".to_owned(),
            reasoning: Some(Reasoning::Low),
            ..Default::default()
        }],
    }
}

/// `name`과 `base_url`만 고른 저장 정보(모델 없음).
pub(crate) fn save_named(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::OpenAi,
        models: Vec::new(),
    }
}

/// 모델 옵션을 다 준 저장 정보(trim한 이름 ` fast `를 씁니다).
pub(crate) fn full_model_save() -> ModelSave {
    ModelSave {
        name: " fast ".to_owned(),
        id: "id-1".to_owned(),
        reasoning: Some(Reasoning::Low),
        extra: full_extra(),
        context_window: Some(32768),
        // D50 결정 7: 호출 형식 선언도 옵션을 다 준 값에서 함께 채웁니다.
        tool_call_form: Some(ToolCallForm::Auto),
        sampling: Sampling {
            temperature: Some(0.2),
            top_p: None,
            max_tokens: Some(1024),
        },
    }
}

/// 이름과 id만 고르고 나머지는 옵션 다 준 값에서 채웁니다.
pub(crate) fn model_save(name: &str, id: &str) -> ModelSave {
    ModelSave {
        name: name.to_owned(),
        id: id.to_owned(),
        ..full_model_save()
    }
}

/// `extra`에 예약 키 `key`를 넣은 모델 저장 정보입니다.
pub(crate) fn model_with_extra_key(name: &str, key: &str) -> ModelSave {
    let mut extra = full_extra();
    extra.insert(key.to_owned(), ExtraValue::Number("1".to_owned()));
    ModelSave {
        extra,
        ..model_save(name, "id-1")
    }
}

/// 02-2 읽기 규칙이 거부하는 `extra` 예약 키 목록입니다(구현 상수를 쓰면 순환 검증이 됩니다).
pub(crate) fn validate_reserved_keys() -> [&'static str; 4] {
    ["model", "messages", "stream", "tools"]
}

/// 벤치마크 결론(작업 1, D40)에 맞는 `extra`: 생각 길이·`textVerbosity`는 새 키가 아니라 `extra`로 흡수합니다.
pub(crate) fn full_extra() -> BTreeMap<String, ExtraValue> {
    let mut extra = BTreeMap::new();
    extra.insert("keep_alive".to_owned(), ExtraValue::Number("0".to_owned()));
    extra.insert(
        "textVerbosity".to_owned(),
        ExtraValue::String("high".to_owned()),
    );
    extra.insert(
        "thinking".to_owned(),
        ExtraValue::Object(
            [(
                "budgetTokens".to_owned(),
                ExtraValue::Number("2048".to_owned()),
            )]
            .into_iter()
            .collect(),
        ),
    );
    extra
}

/// AC 1에 쓰는 파일: 주석 3종, 손대지 않은 엔드포인트 항목, 손대지 않은 칸이 있습니다.
pub(crate) const ANNOTATED_FILE: &str = r#"// 사용자 주석입니다.
{
  /* 블록 주석 */
  "target_language": "ko",
  "endpoints": [
    // 배열 안 주석
    {"name": "local", "base_url": "http://local.invalid/v1", "kind": "ollama"} // 항목 뒤 주석
  ],
  // http 위 주석
  "http": { "retries": 5 },
  "unknown_root_key": { "keep_alive": 1 }
}"#;

/// AC 1 원문 비교에 쓰는 항목들입니다.
pub(crate) const LOCAL_ENDPOINT_TEXT: &str =
    r#"{"name": "local", "base_url": "http://local.invalid/v1", "kind": "ollama"}"#;
pub(crate) const ROOT_EXTRA_TEXT: &str = "\"unknown_root_key\": { \"keep_alive\": 1 }";
pub(crate) const HTTP_TEXT: &str = "\"retries\": 5";

/// 저장 전에 `main` 엔드포인트와 `fast` 모델이 있는 파일 텍스트입니다(수정 검증용).
pub(crate) const EXISTING_MAIN_FILE: &str = r#"{
  "endpoints": [
    {
      "name": "main",
      "base_url": "http://old.invalid/v1", // 주소 설명 주석
      "kind": "openai",
      "notes": " 그대로 두는 칸",
      "models": [
        // 모델 안 주석
        {"name": "fast", "id": "first-id", "reasoning": "high"}
      ]
    }
  ]
}"#;

/// 빈 엔드포인트 목록만 있는 설정 파일 텍스트입니다.
pub(crate) const EMPTY_ENDPOINTS_FILE: &str = "{\"endpoints\":[]}";
/// 내용을 읽을 수 없는 설정 파일 텍스트입니다.
pub(crate) const BROKEN_TEXT: &str = "{\"endpoints\":[}";

/// 안전한 쓰기·이름 바꾸기·삭제 시험이 쓰는 `main` 엔드포인트가 있는 설정 파일 텍스트입니다.
pub(crate) const CONFIG_TEXT: &str = r#"{
  // 사용자 주석입니다.
  "endpoints": [{"name": "main", "base_url": "http://local.invalid/v1", "kind": "ollama"}]
}"#;
/// 안전한 쓰기·이름 바꾸기·삭제 시험이 쓰는 `main` 키가 있는 자격 파일 텍스트입니다.
pub(crate) const CREDENTIALS_TEXT: &str = r#"{
  // 사용자 주석입니다.
  "api_keys": { "main": "main-key-111" }
}"#;
/// 이름 바꾸기가 02-2 검증에서 거부되는 `main2` 항목이 이미 있는 설정 파일 텍스트입니다.
pub(crate) const TWO_ENDPOINTS_TEXT: &str = r#"{
  "endpoints": [
    {"name": "main", "base_url": "http://local.invalid/v1", "kind": "ollama"},
    {"name": "main2", "base_url": "http://local.invalid/v1", "kind": "ollama"}
  ]
}"#;
