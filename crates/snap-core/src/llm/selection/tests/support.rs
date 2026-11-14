//! 선택 값 골격 지원부(시험 트리의 준비 값 한곳)입니다(02-7 AC 1).
//!
//! 시험 본문이 서로 다른 선택 키 확인에서 같은 설정 골격을 쓰므로, 설정을 만들고 키 흔적을
//! 만드는 일만 이 파일에 둡니다(dev-principles 2절 SRP·3절 DRY).
//! 시험 본문은 같은 폴더의 `keys.rs`·`sampling.rs`·`client_build.rs`·`reasoning_extra.rs`가 맡습니다.

use std::collections::BTreeMap;

use crate::config::{
    AgentSettings, Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig,
    Reasoning, Sampling, ToolCallForm,
};
use crate::secrets::SecretValue;

/// 키 값이 오류에 드러나지 않는지 확인하는 시험에서 쓰는 키 흔적 값입니다.
pub(super) const KEY_MARK: &str = "key-ac1-unit-mark";

/// 모델 값과 루트 기본값이 섞인 설정을 직접 만듭니다. 엔드포인트 두 개의 이름은 `pc1`·`pc2`입니다.
pub(super) fn config_with_two_endpoints() -> Config {
    let model_first = ModelConfig {
        name: "fast".into(),
        id: "id-first".into(),
        sampling: Sampling {
            temperature: Some(0.7),
            top_p: None,
            max_tokens: None,
        },
        reasoning: Some(Reasoning::Low),
        extra: BTreeMap::from([("keep_alive".into(), ExtraValue::Number("0".into()))]),
        // 선택 경로 시험이라 맥락 창 값은 기본값을 그대로 둔다(생략할 때의 값과 같은 65536).
        context_window: 65536,
        // 호출 형식 선언도 선택 경로 시험에서는 기본 선언을 그대로 쓴다(생략할 때의 값과 같음).
        tool_call_form: ToolCallForm::default(),
    };
    let model_second = ModelConfig {
        name: "fast".into(),
        id: "id-second".into(),
        sampling: Sampling {
            temperature: Some(0.3),
            top_p: None,
            max_tokens: None,
        },
        reasoning: None,
        extra: BTreeMap::new(),
        context_window: 65536,
        tool_call_form: ToolCallForm::default(),
    };
    Config {
        target_language: "ko".into(),
        endpoints: vec![
            EndpointConfig {
                name: "pc1".into(),
                base_url: SecretValue::new("https://pc1.example.invalid/v1"),
                kind: EndpointKind::OpenAi,
                models: vec![model_first],
            },
            EndpointConfig {
                name: "pc2".into(),
                base_url: SecretValue::new("https://pc2.example.invalid/v1"),
                kind: EndpointKind::OpenAi,
                models: vec![model_second],
            },
        ],
        sampling: Sampling {
            temperature: Some(0.5),
            top_p: Some(0.9),
            max_tokens: Some(64),
        },
        http: HttpSettings::default(),
        agent: AgentSettings::default(),
    }
}
