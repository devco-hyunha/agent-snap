use super::{Credentials, CredentialsError};

/// AC 1: 주석과 마지막 쉼표가 든 자격 파일에서 `api_keys`를 읽고 이름으로 찾는다.
#[test]
fn parse_credentials_with_comments_and_trailing_comma_finds_key_by_name() {
    let text = r#"{
            // 설정 파일과 분리된 자격 파일
            "endpoint": [], /* api_keys 밖 항목은 무시한다 */
            "api_keys": {
                // 메인 서버
                "main-ep": "main-key-111",
                "stage_ep": "stage-key-222",
            }
        }"#;
    let credentials = Credentials::parse(text).expect("주석과 마지막 쉼표를 허용해야 한다");
    let main = credentials
        .find_key("main-ep")
        .expect("main-ep 가 있어야 한다");
    assert_eq!(main.reveal(), "main-key-111");
    let stage = credentials
        .find_key("stage_ep")
        .expect("stage_ep 가 있어야 한다");
    assert_eq!(stage.reveal(), "stage-key-222");
}

/// AC 1: 이름이 없거나 값이 빈 문자열이면 '없음'이다. 대소문자와 `-`·`_`만 다른 이름은 서로 다른 이름이다.
#[test]
fn parse_credentials_names_are_case_and_symbol_sensitive_and_blank_is_none() {
    let text = r#"{ "api_keys": { "Main-Ep": "key-a", "empty_ep": "" } }"#;
    let credentials = Credentials::parse(text).expect("단순 자격 파일");
    assert!(credentials.find_key("Main-Ep").is_some());
    assert!(credentials.find_key("main-ep").is_none());
    assert!(credentials.find_key("Main_Ep").is_none());
    assert!(credentials.find_key("missing-ep").is_none());
    assert!(credentials.find_key("empty_ep").is_none());
}

/// AC 2: 자격 파일이 없으면 키가 하나도 없는 상태이고 어느 이름을 찾아도 '없음'이다.
#[test]
fn absent_credentials_has_no_key_and_is_not_an_error() {
    let credentials = Credentials::absent();
    assert!(credentials.is_empty());
    assert!(credentials.find_key("any-ep").is_none());
}

/// AC 1·AC 2: `api_keys`가 없거나 비어 있으면 오류가 아니라 빈 목록이다.
#[test]
fn missing_or_empty_api_keys_is_empty_list_not_error() {
    let no_api_keys = Credentials::parse(r#"{ "endpoint": [] }"#).expect("api_keys 없음");
    assert!(no_api_keys.is_empty());
    assert!(no_api_keys.find_key("main-ep").is_none());
    let empty_api_keys = Credentials::parse(r#"{ "api_keys": {} }"#).expect("빈 api_keys");
    assert!(empty_api_keys.is_empty());
}

/// AC 3: 값이 문자열이 아닌 항목은 그 엔드포인트 이름만 밝힌 오류로 거부하고 값은 오류에 없다.
#[test]
fn non_string_key_value_is_rejected_with_endpoint_name_only() {
    let text = r#"{ "api_keys": { "good-ep": "good-key-999", "bad-ep": 12345 } }"#;
    let error = Credentials::parse(text).expect_err("문자열이 아닌 값은 거부해야 한다");
    assert!(matches!(&error, CredentialsError::KeyNotString { endpoint } if endpoint == "bad-ep"));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("12345"));
    assert!(!debug.contains("12345"));
    assert!(!display.contains("good-key-999"));
    assert!(!debug.contains("good-key-999"));
}

/// AC 4: JSONC 문법이 틀리면 줄 번호만 담은 오류이고 줄 내용과 값이 없다.
#[test]
fn jsonc_syntax_error_reports_only_line_number() {
    let text = "{ \"api_keys\": { \"ep\": \"secret-value-777\" }\n] } }";
    let error = Credentials::parse(text).expect_err("문법 오류는 거부되어야 한다");
    assert!(matches!(error, CredentialsError::JsoncSyntax { line: 2 }));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("secret-value-777"));
    assert!(!debug.contains("secret-value-777"));
    assert!(!display.contains("api_keys"));
    assert!(!debug.contains("api_keys"));
}

/// AC 3·AC 4의 추가 경로: `api_keys`가 객체가 아니면 값 없이 오류 종류만 담는다.
#[test]
fn api_keys_not_object_carries_no_value() {
    let text = r#"{ "api_keys": "secret-value-777" }"#;
    let error = Credentials::parse(text).expect_err("객체가 아닌 api_keys 는 거부해야 한다");
    assert!(matches!(error, CredentialsError::ApiKeysNotObject));
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("secret-value-777"));
    assert!(!debug.contains("secret-value-777"));
}

/// 최상위가 배열·문자열·숫자 등 비객체이면 `RootNotObject`로 실패한다.
#[test]
fn root_not_object_rejects_array_string_and_number() {
    let cases = ["[]", r#""plain-string""#, "42", "true", "null"];
    for text in cases {
        let error = Credentials::parse(text).expect_err("최상위 비객체는 거부해야 한다");
        assert!(
            matches!(error, CredentialsError::RootNotObject),
            "입력={text:?}, 오류={error:?}"
        );
        let display = error.to_string();
        assert!(
            display.contains("최상위가 객체가 아닙니다"),
            "Display 원인 문구가 맞아야 한다: {display}"
        );
        assert!(!display.contains("api_keys"));
    }
}

/// AC 5: 이 세부 작업의 모든 오류 Display·Debug에 키 값이 없다.
#[test]
fn all_errors_hide_the_key_value_in_display_and_debug() {
    let secret = "super-secret-key-54321";
    let texts = [
        format!("{{ \"api_keys\": {{ \"ep\": {{ \"nested\": \"{secret}\" }} }} }}"),
        format!("{{ \"api_keys\": \"{secret}\" }}"),
        format!("{{ \"api_keys\": {{ \"ep\": \"{secret}\" }} }}}}"),
    ];
    let errors: Vec<CredentialsError> = texts
        .iter()
        .map(|text| Credentials::parse(text).expect_err("해당 텍스트는 거부되어야 한다"))
        .collect();
    for error in errors {
        let display = error.to_string();
        let debug = format!("{error:?}");
        assert!(!display.contains(secret), "Display 에 값이 있다: {display}");
        assert!(!debug.contains(secret), "Debug 에 값이 있다: {debug}");
    }
}
