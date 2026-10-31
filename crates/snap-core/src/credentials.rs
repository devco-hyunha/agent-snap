//! 자격 파일(`credentials.jsonc`) 텍스트 해석: JSONC로 해석해 `api_keys` 아래 엔드포인트 이름별 키만 읽습니다.

use std::collections::BTreeMap;

use jsonc_parser::{JsonValue, ParseOptions};

use crate::secrets::SecretValue;

/// 자격 파일 해석에 실패했을 때 나오는 오류입니다.
///
/// 모든 오류는 키 값(원문)을 담지 않습니다. Display와 Debug에 값이 드러나지 않음을
/// 단위 테스트로 확인합니다(명세 AC 5).
#[derive(Debug, thiserror::Error)]
pub enum CredentialsError {
    /// JSONC 문법 오류입니다. 줄 번호만 담고 줄 내용과 값은 담지 않습니다.
    #[error("자격 파일 해석 실패: JSONC 문법 오류 (줄 {line}번)")]
    JsoncSyntax { line: usize },
    /// `api_keys` 자체가 객체가 아닙니다. 어떤 값도 담지 않습니다.
    #[error("자격 파일 해석 실패: `api_keys`가 객체가 아닙니다")]
    ApiKeysNotObject,
    /// `api_keys` 아래 항목의 값이 문자열이 아닙니다. 엔드포인트 이름만 담습니다.
    #[error("자격 파일 해석 실패: 엔드포인트 `{endpoint}`의 키 값이 문자열이 아닙니다")]
    KeyNotString { endpoint: String },
}

/// 자격 파일에서 읽은 엔드포인트 이름별 비밀 값입니다.
///
/// 해석한 결과를 담고, 이름으로 키를 찾습니다. 이름은 적힌 그대로 비교하며 대소문자와
/// `-`·`_`를 서로 다른 글자로 봅니다(DECISIONS D10).
#[derive(Debug, Default)]
pub struct Credentials {
    keys: BTreeMap<String, SecretValue>,
}

impl Credentials {
    /// 자격 파일 텍스트를 JSONC로 해석합니다.
    ///
    /// 주석(`//`, `/* */`)과 마지막 쉼표를 허용하고, 최상위 객체 아래 `api_keys`의
    /// `"<엔드포인트 이름>": "<키>"`만 읽습니다. `api_keys` 밖의 최상위 항목은 읽지 않고
    /// 무시합니다. `api_keys`가 없거나 비어 있으면 키가 하나도 없는 상태입니다(오류 아님).
    ///
    /// # 오류
    /// - [`CredentialsError::JsoncSyntax`]: JSONC 문법 오류(줄 번호만 담는다).
    /// - [`CredentialsError::ApiKeysNotObject`]: `api_keys`가 객체가 아닌 경우.
    /// - [`CredentialsError::KeyNotString`]: `api_keys` 아래 값이 문자열이 아닌 경우
    ///   (엔드포인트 이름만 담고 값은 담지 않는다).
    pub fn parse(text: &str) -> Result<Self, CredentialsError> {
        let root = jsonc_parser::parse_to_value(text, &ParseOptions::default()).map_err(|e| {
            CredentialsError::JsoncSyntax {
                line: e.line_display(),
            }
        })?;
        // 최상위 자체가 객체가 아닌 경우(문자열·숫자·배열 등)는 `api_keys`를 찾을 수 없으므로
        // [`CredentialsError::ApiKeysNotObject`]로 거부한다. 최상위 객체에서 `api_keys`를
        // `take`로 꺼내 엔트리 자체를 복제하지 않고 소비한다(남는 최상위 항목은 버린다).
        let Some(JsonValue::Object(mut root_object)) = root else {
            return Err(CredentialsError::ApiKeysNotObject);
        };
        let Some(api_keys) = root_object.take("api_keys") else {
            return Ok(Self::default());
        };
        let JsonValue::Object(api_keys_object) = api_keys else {
            return Err(CredentialsError::ApiKeysNotObject);
        };
        let mut keys: BTreeMap<String, SecretValue> = BTreeMap::new();
        for (name, value) in api_keys_object.take_inner() {
            let JsonValue::String(key) = value else {
                return Err(CredentialsError::KeyNotString {
                    endpoint: name.into_owned(),
                });
            };
            keys.insert(name.into_owned(), SecretValue::new(key));
        }
        Ok(Self { keys })
    }

    /// 자격 파일이 없는 상태를 만듭니다. 어느 이름을 찾아도 '없음'이고 빈 목록입니다(AC 2).
    pub fn absent() -> Self {
        Self::default()
    }

    /// 엔드포인트 이름으로 비밀 값을 찾습니다.
    ///
    /// 이름이 적혀 있고 값이 빈 문자열이 아니면 그 값을 돌려주고, 이름이 없거나 값이 빈
    /// 문자열이면 `None`('없음', 오류 아님)을 돌려줍니다(DECISIONS D14).
    pub fn find_key(&self, endpoint_name: &str) -> Option<&SecretValue> {
        self.keys
            .get(endpoint_name)
            .filter(|secret| !secret.reveal().is_empty())
    }

    /// 읽은 키가 하나도 있는지 알려줍니다.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

#[cfg(test)]
mod tests {
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
        assert!(
            matches!(&error, CredentialsError::KeyNotString { endpoint } if endpoint == "bad-ep")
        );
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
}
