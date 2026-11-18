//! 선택 값 골격 지원부. 시험 트리의 준비 값 한곳.
//!
//! 설정을 만들고 키 흔적을 만드는 일만 둠. 시험 본문은 같은 폴더의 다른 파일이 맡음.

use std::collections::BTreeMap;

use crate::config::{
    AgentSettings, Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig,
    Reasoning, Sampling, ToolCallForm,
};
use crate::secrets::SecretValue;

/// 키 값이 오류에 드러나지 않는지 확인하는 시험에서 쓰는 키 흔적 값.
pub(super) const KEY_MARK: &str = "key-ac1-unit-mark";

/// 모델 값과 루트 기본값이 섞인 설정을 직접 만듦. 엔드포인트 이름은 `pc1`·`pc2`.
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
        // 선택 경로 시험이라 맥락 창 값은 기본값을 그대로 둠.
        context_window: 65536,
        // 호출 형식 선언도 선택 경로 시험에서는 기본 선언을 그대로 씀.
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
