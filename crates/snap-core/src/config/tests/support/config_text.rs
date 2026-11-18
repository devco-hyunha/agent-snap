//! 설정 텍스트 조립·해석 도움: JSONC 문자열을 만들고 해석해 설정 객체·오류로 반환. 가짜 호스트는 `checks.rs`의 상수를 그대로 사용.

use crate::config::{Config, ConfigEnv, ConfigError, HostOs};

use super::checks::HOST;

/// 모든 환경 정보가 비어 있는 설정 폴더 계산 입력.
pub(crate) fn empty_env(os: HostOs) -> ConfigEnv {
    ConfigEnv {
        os,
        config_dir_override: None,
        appdata: None,
        xdg_config_home: None,
        home: None,
    }
}

/// 정상 `base_url` 값 조립.
pub(crate) fn base_url() -> String {
    format!("https://{HOST}/v1")
}

/// `kind`를 정해 만든 엔드포인트 객체 하나.
pub(crate) fn endpoint_object_with_kind(
    name: &str,
    base_url: &str,
    kind: &str,
    tail: &str,
) -> String {
    format!(r#"{{"name":"{name}","base_url":"{base_url}","kind":"{kind}"{tail}}}"#)
}

/// `kind`가 `openai`인 엔드포인트 객체 하나.
pub(crate) fn endpoint_object(name: &str, base_url: &str, tail: &str) -> String {
    endpoint_object_with_kind(name, base_url, "openai", tail)
}

/// 모델 객체 하나.
pub(crate) fn model_object(name: &str, id: &str, tail: &str) -> String {
    format!(r#"{{"name":"{name}","id":"{id}"{tail}}}"#)
}

/// 모델 객체들을 `models` 배열 칸으로 담은 문자열.
pub(crate) fn models_tail(models: &[String]) -> String {
    format!(r#","models":[{}]"#, models.join(","))
}

/// 엔드포인트 객체들(쉼표로 이은 것)과 루트 칸으로 설정 텍스트 조립.
fn config_with_endpoints_and_root_tail(endpoints: &[String], root_tail: &str) -> String {
    format!(r#"{{"endpoints":[{}]{root_tail}}}"#, endpoints.join(","))
}

pub(crate) fn config_of(endpoints: &[String]) -> String {
    config_with_endpoints_and_root_tail(endpoints, "")
}

/// 엔드포인트 `main` 하나에 모델 객체들을 담은 설정 텍스트.
pub(crate) fn config_with_models(models: &[String]) -> String {
    config_of(&[endpoint_object("main", &base_url(), &models_tail(models))])
}

/// 엔드포인트 `main` 하나의 객체 조립.
fn main_endpoint_with_fast_model(endpoint_tail: &str) -> String {
    let models = models_tail(&[model_object("fast", "id-1", "")]);
    endpoint_object("main", &base_url(), &format!("{models}{endpoint_tail}"))
}

pub(crate) fn config_with_endpoint_tail(endpoint_tail: &str) -> String {
    config_of(&[main_endpoint_with_fast_model(endpoint_tail)])
}

/// 엔드포인트 `main`의 `kind` 칸 없는 설정 텍스트.
pub(crate) fn config_without_kind() -> String {
    config_of(&[format!(r#"{{"name":"main","base_url":"{}"}}"#, base_url())])
}

/// 엔드포인트 `main` 아래 모델 `fast`에 `id` 칸 없는 설정 텍스트.
pub(crate) fn config_without_model_id() -> String {
    config_with_models(&[r#"{"name":"fast"}"#.to_owned()])
}

pub(crate) fn config_with_model_tail(model_tail: &str) -> String {
    config_with_models(&[model_object("fast", "id-1", model_tail)])
}

/// 정상 엔드포인트에 루트 칸을 더한 설정 텍스트.
pub(crate) fn config_with_root_tail(root_tail: &str) -> String {
    config_with_endpoints_and_root_tail(&[main_endpoint_with_fast_model("")], root_tail)
}

/// 정상 설정을 읽어 설정 객체로 반환.
pub(crate) fn parse_ok(text: &str) -> Config {
    Config::parse(text).unwrap_or_else(|error| panic!("정상 설정이어야 합니다: {error}"))
}

/// 거부되어야 하는 설정을 읽어 설정 오류로 반환.
pub(crate) fn parse_err(text: &str) -> ConfigError {
    match Config::parse(text) {
        Ok(config) => panic!("거부되어야 합니다: {config:?}"),
        Err(error) => error,
    }
}
