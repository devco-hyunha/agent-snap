//! T1 선택 키 시험 공통 재료. 설정 한 채를 만들고 오류 문구만 꺼냄.

use snap_core::config::Config;
use snap_core::config::EndpointConfig;
use snap_core::config::HttpSettings;
use snap_core::config::ModelConfig;
use snap_core::config::SecretString;
use snap_core::llm::find_target;
use snap_core::llm::parse_selection_key;

/// 시험용 엔드포인트 주소. 오류 문구에 안 나오는지를 확인하는 재료.
pub const HOST: &str = "secret-host.invalid";

/// 시험용 키 원문. 오류 문구에 안 나오는지를 확인하는 재료.
pub const KEY: &str = "local/absent";

/// `local` 엔드포인트와 `qwen`·`other` 모델을 둔 설정 한 채.
pub fn config() -> Config {
    Config {
        language: String::new(),
        endpoints: vec![EndpointConfig {
            name: "local".to_owned(),
            base_url: SecretString::new(format!("https://{HOST}/v1")),
            kind: "ollama".to_owned(),
            models: vec![
                ModelConfig {
                    name: "qwen".to_owned(),
                    id: "qwen-id".to_owned(),
                    sampling: None,
                    reasoning: None,
                    extra: None,
                },
                ModelConfig {
                    name: "other".to_owned(),
                    id: "other-id".to_owned(),
                    sampling: None,
                    reasoning: None,
                    extra: None,
                },
            ],
        }],
        sampling: None,
        http: HttpSettings::default(),
    }
}

/// 성공한 경우에만 대상 표시(엔드포인트 이름 / 모델 이름)를 꺼냄. 실패하면 시험을 세움.
pub fn target_text(config: &Config, key: &str) -> String {
    let parsed =
        parse_selection_key(key).unwrap_or_else(|err| panic!("selection key should parse: {err}"));
    let target =
        find_target(config, &parsed).unwrap_or_else(|err| panic!("target should be found: {err}"));
    format!(
        "{} / {} / {}",
        target.endpoint.name.trim(),
        target.model.name.trim(),
        target.model.id
    )
}

/// 거부된 경우에만 오류 문구 두 개(Display·Debug)를 꺼냄. 성공하면 시험을 세움.
pub fn reject_texts(key: &str) -> (String, String) {
    let parsed = parse_selection_key(key);
    let err = match parsed {
        Ok(key) => {
            let config = config();
            find_target(&config, &key)
                .err()
                .unwrap_or_else(|| panic!("test expects rejection, got ok: {key}"))
        }
        Err(err) => err,
    };
    (err.to_string(), format!("{err:?}"))
}
