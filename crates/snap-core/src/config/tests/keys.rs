//! 키 정하기 테스트(명세 AC 3): 자격 파일의 이름별 값이 엔드포인트에 매핑된다.

use super::support::{base_url, config_of, endpoint_object, parse_ok};
use crate::config::Config;
use crate::credentials::Credentials;

fn config_with_endpoint_names(names: &[&str]) -> Config {
    let endpoints: Vec<String> = names
        .iter()
        .map(|name| endpoint_object(name, &base_url(), ""))
        .collect();
    parse_ok(&config_of(&endpoints))
}

/// AC 3: 자격 파일에 trim한 엔드포인트 이름이 있으면 그 비밀 값을 쓴다.
#[test]
fn api_key_is_found_by_trimmed_endpoint_name() {
    let config = config_with_endpoint_names(&["  main  "]);
    let credentials = Credentials::parse(r#"{"api_keys":{"main":"key-a"}}"#).expect("자격 파일");

    let key = config.endpoints[0]
        .api_key(&credentials)
        .expect("키가 있어야 합니다");

    assert_eq!(key.reveal(), "key-a");
}

/// AC 3: 이름이 없거나 빈 문자열이면 '키 없음'이다.
#[test]
fn api_key_is_none_when_name_missing_or_value_empty() {
    let config = config_with_endpoint_names(&["main", "blank"]);
    let credentials =
        Credentials::parse(r#"{"api_keys":{"other":"key-b","blank":""}}"#).expect("자격 파일");

    assert!(config.endpoints[0].api_key(&credentials).is_none());
    assert!(config.endpoints[1].api_key(&credentials).is_none());
}

/// AC 3: 자격 파일이 없으면 '키 없음'이다.
#[test]
fn api_key_is_none_without_credentials_file() {
    let config = config_with_endpoint_names(&["main"]);

    assert!(
        config.endpoints[0]
            .api_key(&Credentials::absent())
            .is_none()
    );
}

/// AC 3: 이름은 적힌 그대로 비교한다. 대소문자와 `-`·`_`가 다르면 다른 이름이다.
#[test]
fn api_key_name_match_is_case_and_symbol_sensitive() {
    let config = config_with_endpoint_names(&["Main-Ep", "main-ep", "main_ep"]);
    let credentials = Credentials::parse(r#"{"api_keys":{"Main-Ep":"key-c"}}"#).expect("자격 파일");

    assert!(config.endpoints[0].api_key(&credentials).is_some());
    assert!(config.endpoints[1].api_key(&credentials).is_none());
    assert!(config.endpoints[2].api_key(&credentials).is_none());
}
