//! 자격 파일(`credentials.jsonc`) 텍스트 해석: JSONC로 해석해 `api_keys` 아래 엔드포인트 이름별 키만 읽습니다.

use std::collections::BTreeMap;

use jsonc_parser::JsonValue;

use crate::jsonc::{self, JsoncError, WrongType, pick_object, take_optional};
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
    /// 자격 파일이 최상위 객체가 아닙니다.
    #[error("자격 파일 해석 실패: 최상위가 객체가 아닙니다")]
    RootNotObject,
    /// `api_keys` 자체가 객체가 아닙니다. 어떤 값도 담지 않습니다.
    #[error("자격 파일 해석 실패: `api_keys`가 객체가 아닙니다")]
    ApiKeysNotObject,
    /// `api_keys` 아래 항목의 값이 문자열이 아닙니다. 엔드포인트 이름만 담습니다.
    #[error("자격 파일 해석 실패: 엔드포인트 `{endpoint}`의 키 값이 문자열이 아닙니다")]
    KeyNotString { endpoint: String },
}

impl From<JsoncError> for CredentialsError {
    fn from(error: JsoncError) -> Self {
        match error {
            JsoncError::Syntax { line } => Self::JsoncSyntax { line },
            JsoncError::RootNotObject => Self::RootNotObject,
        }
    }
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
    /// - [`CredentialsError::RootNotObject`]: 최상위가 객체가 아닌 경우.
    /// - [`CredentialsError::ApiKeysNotObject`]: `api_keys`가 객체가 아닌 경우.
    /// - [`CredentialsError::KeyNotString`]: `api_keys` 아래 값이 문자열이 아닌 경우
    ///   (엔드포인트 이름만 담고 값은 담지 않는다).
    pub fn parse(text: &str) -> Result<Self, CredentialsError> {
        // 최상위 자체가 객체가 아닌 경우(문자열·숫자·배열 등)는
        // `From<JsoncError>`가 [`CredentialsError::RootNotObject`]로 바꿔 거부한다.
        // `api_keys`는 `take`로 꺼내 복제 없이 소비한다(남는 최상위 항목은 버린다).
        let mut root = jsonc::parse_root_object(text)?;
        let Some(api_keys_object) = take_optional(&mut root, "api_keys", pick_object)
            .map_err(|WrongType| CredentialsError::ApiKeysNotObject)?
        else {
            return Ok(Self::default());
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
mod tests;
