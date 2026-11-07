//! 선택 키 규칙(02-7 작업 1): `엔드포인트이름/모델이름`을 나누고 설정에서 엔드포인트와 그 아래 모델을 찾는다.

use super::types::{Config, EndpointConfig, ModelConfig};

/// 선택 키가 설정에서 모델을 찾는 데 쓰지 못하는 상태입니다.
///
/// Display와 Debug에는 입력 키만 담고 자격 키와 base_url 호스트는 담지 않습니다(02-7 spec.md 오류 표).
/// 값이 없는 항목은 `#[error]` 문자열에만 담기고, 표시할 비밀 값이 없어 derive `Debug`을 그대로 씁니다.
#[derive(thiserror::Error, Debug)]
pub enum SelectionError {
    /// 구분자(슬래시)가 없는 선택 키입니다. 모델 이름만 넘긴 입력을 함께 잡습니다(선택 키 필수).
    #[error("구분자 `/`가 없는 선택 키: {input}")]
    NoSeparator {
        /// 설정에 넘긴 입력 문자열입니다.
        input: String,
    },
    /// 구분자가 둘 있거나 이름이 빈 선택 키입니다.
    #[error("형식이 틀린 선택 키: {input}")]
    Malformed {
        /// 설정에 넘긴 입력 문자열입니다.
        input: String,
    },
    /// 설정의 엔드포인트와 그 아래 모델에서 찾지 못한 선택 키입니다.
    #[error("설정에 없는 선택 키: {key}")]
    UnknownKey {
        /// 설정에 넘긴 선택 키입니다.
        key: String,
    },
    /// 고를 모델이 없는 상태입니다. `endpoints`/`models`가 비어 있음을 안내만 담습니다.
    #[error("고를 모델이 없습니다: 설정의 `endpoints`/`models`가 비어 있습니다")]
    EmptyList,
}

impl Config {
    /// 선택 키(`엔드포인트이름/모델이름`)로 엔드포인트와 그 아래 모델을 찾습니다.
    ///
    /// 구분자는 키의 첫 슬래시 하나입니다(02-2 설정 이름은 `/`를 담지 않음, D20).
    /// 찾은 값은 설정 그대로이고, 샘플링 잇기와 키 정하기는 `llm/selection.rs`가 더합니다.
    ///
    /// # 오류
    /// - [`SelectionError::NoSeparator`]: 구분자가 없는 키(모델 이름만 넘긴 입력).
    /// - [`SelectionError::Malformed`]: 구분자가 둘이거나 이름이 빈 키.
    /// - [`SelectionError::UnknownKey`]: 설정의 엔드포인트와 모델에서 찾지 못한 키.
    /// - [`SelectionError::EmptyList`]: 고를 모델이 없음(`endpoints`가 빈 설정).
    pub fn select(
        &self,
        selection_key: &str,
    ) -> Result<(&EndpointConfig, &ModelConfig), SelectionError> {
        let (endpoint_name, model_name) = split_selection_key(selection_key)?;
        if self.endpoints.is_empty() {
            return Err(SelectionError::EmptyList);
        }
        self.models()
            .find(|(endpoint, model)| endpoint.name == endpoint_name && model.name == model_name)
            .ok_or_else(|| SelectionError::UnknownKey {
                key: selection_key.to_owned(),
            })
    }
}

/// 선택 키를 첫 슬래시 하나로 나눕니다. 구분자 없이는 '구분자 없음', 빈 이름과 둘째 구분자는 '형식 오류'입니다.
fn split_selection_key(selection_key: &str) -> Result<(&str, &str), SelectionError> {
    let Some((endpoint_name, model_name)) = selection_key.split_once('/') else {
        return Err(SelectionError::NoSeparator {
            input: selection_key.to_owned(),
        });
    };
    if endpoint_name.is_empty() || model_name.is_empty() || model_name.contains('/') {
        return Err(SelectionError::Malformed {
            input: selection_key.to_owned(),
        });
    }
    Ok((endpoint_name, model_name))
}
