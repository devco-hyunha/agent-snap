//! 선택 키로 만드는 클라이언트 준비 테스트(02-7 AC 1): 샘플링 우선·키 정하기·선택 오류를 확인합니다.

use std::collections::BTreeMap;

use super::SelectError;
use crate::config::{
    Config, EndpointConfig, EndpointKind, ExtraValue, HttpSettings, ModelConfig, Reasoning,
    Sampling,
};
use crate::credentials::Credentials;
use crate::secrets::SecretValue;

const KEY_MARK: &str = "key-ac1-unit-mark";

/// 모델 값과 루트 기본값이 섞인 설정을 직접 만듭니다. 엔드포인트 두 개의 이름은 `pc1`·`pc2`입니다.
fn config_with_two_endpoints() -> Config {
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
    }
}

/// AC 1: 선택 키가 다른 같은 별칭은 서로 다른 엔드포인트와 서버용 id로 준비된다.
#[test]
fn select_model_splits_same_alias_by_selection_key() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let first = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");
    let second = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(first.model_id, "id-first");
    assert_eq!(second.model_id, "id-second");
    assert_eq!(first.base_url.reveal(), "https://pc1.example.invalid/v1");
    assert_eq!(second.base_url.reveal(), "https://pc2.example.invalid/v1");
}

/// AC 1: 샘플링은 모델별 값이 루트 기본값보다 앞서고, 모델에 없는 항목만 루트를 따라 간다.
#[test]
fn select_model_merges_sampling_with_model_priority() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let selected = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");

    assert_eq!(selected.sampling.temperature, Some(0.7));
    assert_eq!(selected.sampling.top_p, Some(0.9));
    assert_eq!(selected.sampling.max_tokens, Some(64));
}

/// AC 1: 키는 02-2의 엔드포인트 이름 찾기(자격 파일 `api_keys`)로 정하고, 없는 엔드포인트는 없음이다.
#[test]
fn select_model_finds_key_by_endpoint_name() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::parse(&format!(r#"{{"api_keys":{{"pc1":"{KEY_MARK}"}}}}"#))
        .expect("자격 파일");

    let first = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");
    let second = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(first.api_key.expect("pc1 키").reveal(), KEY_MARK);
    assert!(second.api_key.is_none());
}

/// AC 1: `reasoning`과 `extra`는 설정 값 그대로 준비 값에 담긴다.
#[test]
fn select_model_keeps_reasoning_and_extra() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let selected = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");

    assert_eq!(selected.reasoning, Some(Reasoning::Low));
    assert_eq!(
        selected.extra.get("keep_alive"),
        Some(&ExtraValue::Number("0".into()))
    );
    assert!(
        config
            .select_model("pc2/fast", &credentials)
            .expect("pc2/fast")
            .reasoning
            .is_none()
    );
}

/// AC 2 경로: 없는 선택 키를 고를 때 선택 오류가 그대로 올라오고, 오류 문자열에 주소가 드러나지 않는다.
#[test]
fn select_model_selection_error_hides_hosts() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let error = match config.select_model("pc9/fast", &credentials) {
        Ok(_) => panic!("설정에 없는 키는 거부해야 합니다"),
        Err(error) => error,
    };

    assert!(matches!(error, SelectError::Selection(_)));
    let text = format!("{error} {error:?}");
    assert!(text.contains("pc9/fast"), "입력 키가 없습니다: {text}");
    assert!(
        !text.contains("pc1.example.invalid") && !text.contains("pc2.example.invalid"),
        "오류에 주소가 있습니다: {text}"
    );
    assert!(!text.contains(KEY_MARK), "오류에 키 값이 있습니다: {text}");
}

/// AC 1: 조립된 클라이언트는 준비 값의 엔드포인트 주소와 모델 값을 그대로 갖는다.
#[test]
fn client_for_carries_selected_values() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let client = config
        .client_for("pc2/fast", &credentials)
        .expect("pc2/fast");
    let selected = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(client.model_id, selected.model_id);
    assert_eq!(client.sampling, selected.sampling);
    assert_eq!(client.reasoning, selected.reasoning);
    assert_eq!(client.extra, selected.extra);
}
