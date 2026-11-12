//! 설정 텍스트 조립과 해석 도움: JSONC 문자열을 만들고 해석해서 설정 객체·오류로 돌려줍니다.
//!
//! 가리기 검사용 가짜 호스트는 가리기 확인 파일(`checks.rs`)의 한 곳을 그대로 씁니다.

use crate::config::{Config, ConfigEnv, ConfigError, HostOs};

use super::checks::HOST;

/// 모든 환경 정보가 비어 있는 설정 폴더 계산 입력입니다.
pub(crate) fn empty_env(os: HostOs) -> ConfigEnv {
    ConfigEnv {
        os,
        config_dir_override: None,
        appdata: None,
        xdg_config_home: None,
        home: None,
    }
}

/// 정상 `base_url` 값입니다.
pub(crate) fn base_url() -> String {
    format!("https://{HOST}/v1")
}

/// `kind`를 정해 만든 엔드포인트 객체 하나입니다. `tail`은 `kind` 뒤에 그대로 붙는 칸들입니다
/// (`,"x":1` 형태).
pub(crate) fn endpoint_object_with_kind(
    name: &str,
    base_url: &str,
    kind: &str,
    tail: &str,
) -> String {
    format!(r#"{{"name":"{name}","base_url":"{base_url}","kind":"{kind}"{tail}}}"#)
}

/// `kind`가 `openai`인 엔드포인트 객체 하나입니다. `tail`은 `kind` 뒤에 그대로 붙는 칸들입니다.
pub(crate) fn endpoint_object(name: &str, base_url: &str, tail: &str) -> String {
    endpoint_object_with_kind(name, base_url, "openai", tail)
}

/// 모델 객체 하나입니다. `tail`은 `id` 뒤에 그대로 붙는 칸들입니다.
pub(crate) fn model_object(name: &str, id: &str, tail: &str) -> String {
    format!(r#"{{"name":"{name}","id":"{id}"{tail}}}"#)
}

/// 모델 객체들을 `models` 칸으로 담은 엔드포인트 `tail`입니다(`,"models":[…]` 형태).
pub(crate) fn models_tail(models: &[String]) -> String {
    format!(r#","models":[{}]"#, models.join(","))
}

/// 엔드포인트 객체들(쉼표로 이은 것)을 `endpoints`에 담고 루트 칸 `root_tail`(`,"x":1` 형태)을
/// 더한 설정 텍스트입니다.
fn config_with_endpoints_and_root_tail(endpoints: &[String], root_tail: &str) -> String {
    format!(r#"{{"endpoints":[{}]{root_tail}}}"#, endpoints.join(","))
}

/// 엔드포인트 객체들(쉼표로 이은 것)을 `endpoints`에 담은 설정 텍스트입니다.
pub(crate) fn config_of(endpoints: &[String]) -> String {
    config_with_endpoints_and_root_tail(endpoints, "")
}

/// 엔드포인트 `main` 하나에 모델 객체들을 담은 설정입니다.
pub(crate) fn config_with_models(models: &[String]) -> String {
    config_of(&[endpoint_object("main", &base_url(), &models_tail(models))])
}

/// 모델 하나 `fast`가 든 엔드포인트 `main` 객체입니다. `endpoint_tail`은 엔드포인트 객체에
/// 더하는 칸입니다.
fn main_endpoint_with_fast_model(endpoint_tail: &str) -> String {
    let models = models_tail(&[model_object("fast", "id-1", "")]);
    endpoint_object("main", &base_url(), &format!("{models}{endpoint_tail}"))
}

/// 모델 하나 `fast`가 든 엔드포인트 `main` 하나의 설정입니다. `endpoint_tail`은 엔드포인트
/// 객체에 더하는 칸입니다.
pub(crate) fn config_with_endpoint_tail(endpoint_tail: &str) -> String {
    config_of(&[main_endpoint_with_fast_model(endpoint_tail)])
}

/// 엔드포인트 `main`의 `kind` 칸이 없는 설정입니다.
pub(crate) fn config_without_kind() -> String {
    config_of(&[format!(r#"{{"name":"main","base_url":"{}"}}"#, base_url())])
}

/// 엔드포인트 `main` 아래 모델 `fast`에 `id` 칸이 없는 설정입니다.
pub(crate) fn config_without_model_id() -> String {
    config_with_models(&[r#"{"name":"fast"}"#.to_owned()])
}

/// 엔드포인트 `main` 아래 모델 `fast` 하나가 있고, 모델 객체에 `model_tail` 칸을 더한 설정입니다.
pub(crate) fn config_with_model_tail(model_tail: &str) -> String {
    config_with_models(&[model_object("fast", "id-1", model_tail)])
}

/// 정상 엔드포인트에 루트 칸(`root_tail`, `,"x":1` 형태)을 더한 설정입니다.
pub(crate) fn config_with_root_tail(root_tail: &str) -> String {
    config_with_endpoints_and_root_tail(&[main_endpoint_with_fast_model("")], root_tail)
}

/// 정상 설정을 읽습니다.
pub(crate) fn parse_ok(text: &str) -> Config {
    Config::parse(text).unwrap_or_else(|error| panic!("정상 설정이어야 합니다: {error}"))
}

/// 거부되어야 하는 설정을 읽어 오류를 돌려줍니다.
pub(crate) fn parse_err(text: &str) -> ConfigError {
    match Config::parse(text) {
        Ok(config) => panic!("거부되어야 합니다: {config:?}"),
        Err(error) => error,
    }
}
