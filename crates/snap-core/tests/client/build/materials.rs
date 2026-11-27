//! 02-7 T2 조립 시험 공통 재료. 설정 한 채·선택 키·자격 키·조립 창구만 담음.
//! 같은 시험 묶음의 스트림·래퍼·폴백 시험이 재사용하는 재료 파일.

use snap_core::config::Config;
use snap_core::config::CredentialsError;
use snap_core::config::EndpointConfig;
use snap_core::config::HttpSettings;
use snap_core::config::ModelConfig;
use snap_core::config::SecretString;
use snap_core::config::resolve_api_key;
use snap_core::llm::ClientBundle;
use snap_core::llm::assemble_client;
use snap_core::llm::find_target;
use snap_core::llm::parse_selection_key;

use crate::server::MockServer;
use crate::stream_open::base_url;

/// 시험용 선택 키. `local/qwen`은 설정의 엔드포인트 이름과 모델 이름.
pub(crate) const KEY: &str = "local/qwen";

/// 시험용 모델 이름.
pub(crate) const MODEL_NAME: &str = "qwen";

/// 시험용 모델 식별 값. 서버에 보내는 값.
pub(crate) const MODEL_ID: &str = "qwen-id";

/// 조립 시험용 시간 값. 연결은 넉넉히, 헤더 대기와 유휴는 짧게 둔다.
pub(crate) fn http_settings() -> HttpSettings {
    HttpSettings {
        retries: 1,
        backoff_min: 0.0,
        backoff_max: 1.0,
        retry_after_max: 60.0,
        connect_seconds: 5.0,
        header_wait_seconds: 2.0,
        stream_idle_seconds: 1.0,
    }
}

/// 모의 서버 주소를 쓰는 설정 한 채. `local` 엔드포인트와 `qwen` 모델 하나, `kind`은 인수로 지정한 값.
/// 조립 시험의 정본 재료이고, 폴백 시험이 목록 밖 값을 넣는 데도 함께 쓴다.
pub(crate) fn config_with_kind(server: &MockServer, kind: &str) -> Config {
    Config {
        language: "ko".to_owned(),
        endpoints: vec![EndpointConfig {
            name: "local".to_owned(),
            base_url: SecretString::new(base_url(server)),
            kind: kind.to_owned(),
            models: vec![ModelConfig {
                name: MODEL_NAME.to_owned(),
                id: MODEL_ID.to_owned(),
                sampling: None,
                reasoning: None,
                extra: None,
            }],
        }],
        sampling: None,
        http: http_settings(),
    }
}

/// 자격 파일 표 한 채. 엔드포인트 이름과 키 값.
pub(crate) fn credentials(key: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut keys = serde_json::Map::new();
    keys.insert(
        "local".to_owned(),
        serde_json::Value::String(key.to_owned()),
    );
    keys
}

/// 조립 창구. 선택 키 해석 → 설정 조회 → 자격 키 정하기 → 클라이언트 조립.
#[allow(clippy::expect_used)]
pub(crate) fn bundle_for(server: &MockServer, key: &str) -> ClientBundle {
    bundle_for_kind(server, key, "openai")
}

/// `kind` 값을 지정한 설정으로 조립하는 창구. 폴백 시험이 쓰는 재료.
#[allow(clippy::expect_used)]
pub(crate) fn bundle_for_kind(server: &MockServer, key: &str, kind: &str) -> ClientBundle {
    let config = config_with_kind(server, kind);
    let parsed = parse_selection_key(KEY).expect("test selection key parses");
    let target = find_target(&config, &parsed).expect("test target is in config");
    let keys: Result<serde_json::Map<String, serde_json::Value>, CredentialsError> =
        Ok(credentials(key));
    let api_key = resolve_api_key(keys, "local").expect("test credentials map resolves");
    assemble_client(&target, api_key, &config.http)
}
