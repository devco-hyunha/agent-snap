//! 설정 저장이 검증하는 자리: 02-2 규칙으로 쓰기 가능 여부를 확인합니다.
//!
//! 이 파일의 책임은 하나입니다. `EndpointSave`·`ModelSave`가 설정 파일에 쓸 수 있는
//! 값인지 02-2 검증 규칙(이름·주소·모델 id·`extra` 예약 키)으로 확인하고, 통과한 값만
//! 쓰기 단계가 쓰는 검증한 항목(`CheckedEndpoint`·`CheckedModel`)로 만드는 것. 저장이
//! 파일을 고치는 편집은 `save_config`가, 자격 쪽 편집은 `save_credentials`가 맡습니다.

use std::collections::BTreeMap;

use super::super::error::{ConfigError, ValidationError};
use super::super::read::types::{EndpointKind, ExtraValue, Reasoning, Sampling, ToolCallForm};
use super::super::read::validate::{self, is_valid_base_url};
use super::names::{NameScope, checked_save_name};

/// 엔드포인트 하나를 추가·수정할 때 주는 정보입니다.
///
/// 키 값은 여기에 넣지 않습니다. 키는 자격 파일(`credentials.jsonc`)에만 씁니다(D10).
#[derive(Debug)]
pub struct EndpointSave {
    /// 엔드포인트 이름입니다. trim한 이 이름으로 기존 항목을 찾아 고칩니다.
    pub name: String,
    /// 서버 기본 주소입니다.
    pub base_url: String,
    /// 서버 종류입니다.
    pub kind: EndpointKind,
    /// 이 엔드포인트 아래에 쓸 모델 목록입니다.
    pub models: Vec<ModelSave>,
}

impl EndpointSave {
    /// 이 정보로 설정 파일에 쓸 수 있는지 02-2 검증 규칙으로 확인합니다.
    ///
    /// 엔드포인트 이름·주소, 모델 이름·id, 모델 `extra`의 예약 키를 봅니다. `extra` 예약 키
    /// 판단은 02-2 읽기가 쓰는 규칙 목록(`validate::RESERVED_EXTRA_KEYS`)을 그대로 씁니다.
    ///
    /// # 오류
    /// 이름 규칙 위반은 [`ValidationError::EmptyEndpointName`]·[`ValidationError::EndpointNameHasSlash`]
    /// 또는 [`ValidationError::EmptyModelName`]·[`ValidationError::ModelNameHasSlash`], 모델 id 규칙
    /// 위반은 [`ValidationError::EmptyModelId`], 주소 규칙 위반은 [`ValidationError::InvalidBaseUrl`],
    /// `extra` 예약 키는 [`ValidationError::ExtraReservedKey`]입니다. 입력 값은 오류에 담지 않습니다.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.check().map(|_| ())
    }

    /// 검증하며, 쓰기 단계에 넘길 값(trim한 이름과 원본 정보)을 만듭니다.
    ///
    /// 규칙을 통과하지 못한 값은 여기까지 오지 않아, 검증을 거부한 값이 파일 쓰기로 흐르는 경로가
    /// 없습니다.
    pub(crate) fn check(&self) -> Result<CheckedEndpoint<'_>, ConfigError> {
        let name = checked_save_name(&self.name, NameScope::Endpoint)?;
        if !is_valid_base_url(&self.base_url) {
            return Err(ValidationError::InvalidBaseUrl {
                endpoint: name.clone(),
            }
            .into());
        }
        Ok(CheckedEndpoint {
            name: name.clone(),
            save: self,
            models: self
                .models
                .iter()
                .map(|model| CheckedModel::check(&name, model))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// 설정 파일에 쓰는 모델 하나입니다.
///
/// `None`인 항목과 `None`으로 채운 샘플링 항목은 파일에 쓰지 않습니다. 예를 들어 `reasoning`만
/// 준 저장이면 파일에는 `reasoning` 칸만 생깁니다(D14·D29: 기본값을 파일에 굳이 안 씀).
#[derive(Debug, Default, Clone)]
pub struct ModelSave {
    /// 모델 이름입니다. 같은 엔드포인트의 기존 모델과 trim한 이 이름으로 비교합니다.
    pub name: String,
    /// 서버에 보낼 모델 id입니다.
    pub id: String,
    /// 생각 옵션입니다. `None`이면 파일에 쓰지 않습니다.
    pub reasoning: Option<Reasoning>,
    /// 요청 본문에 합칠 추가 값입니다. 비어 있으면 파일에 쓰지 않습니다.
    pub extra: BTreeMap<String, ExtraValue>,
    /// 맥락 창 토큰 수입니다. `None`이면 파일에 쓰지 않습니다(읽을 때 65536, D29).
    pub context_window: Option<u64>,
    /// 호출 형식 선언입니다. `None`이면 파일에 쓰지 않습니다(읽을 때 `openai`, D50 결정 7).
    pub tool_call_form: Option<ToolCallForm>,
    /// 샘플링 값입니다. `None`으로 채운 항목은 파일에 쓰지 않습니다.
    pub sampling: Sampling,
}

/// 검증한 엔드포인트 한 항목입니다. `name`은 trim한 값이라 쓰기 단계가 그대로 씁니다.
// `check`(pub(crate))가 돌려주는 타입이라 타입 가시성도 같은 크기로 맞춥니다. 가시성이
// 타입보다 좁으면 공개 함수가 더 좁은 타입을 반환하는 셈이 되어 경고를 받습니다.
pub(crate) struct CheckedEndpoint<'a> {
    /// trim한 엔드포인트 이름입니다.
    pub(crate) name: String,
    /// 원본 저장 정보입니다(주소·종류·모델 목록).
    pub(crate) save: &'a EndpointSave,
    /// 검증한 모델 목록입니다.
    pub(crate) models: Vec<CheckedModel<'a>>,
}

/// 검증한 모델 한 항목입니다. `name`과 `id`는 trim한 값이라 쓰기 단계가 그대로 씁니다.
pub(crate) struct CheckedModel<'a> {
    /// trim한 모델 이름입니다.
    pub(crate) name: String,
    /// trim한 모델 id입니다.
    pub(crate) id: String,
    /// 원본 모델 정보입니다(옵션·샘플링·extra).
    pub(crate) save: &'a ModelSave,
}

impl<'a> CheckedModel<'a> {
    /// 모델 이름·id 규칙과 `extra` 예약 키를 02-2 규칙으로 확인합니다.
    fn check(endpoint: &str, save: &'a ModelSave) -> Result<Self, ConfigError> {
        let name = checked_save_name(
            &save.name,
            NameScope::Model {
                endpoint: endpoint.to_owned(),
            },
        )?;
        let id = validate::trimmed_non_empty(&save.id)
            .ok_or_else(|| ValidationError::empty_model_id(endpoint, &name))?;
        if let Some(key) = Self::extra_reserved_key(&save.extra) {
            return Err(ValidationError::ExtraReservedKey {
                model: name.clone(),
                key: key.to_owned(),
            }
            .into());
        }
        Ok(Self { name, id, save })
    }

    /// 모델 `extra`에 예약 키가 있으면 그 키를 찾습니다. 02-2 읽기 규칙 목록을 재사용합니다(D15).
    fn extra_reserved_key(extra: &BTreeMap<String, ExtraValue>) -> Option<&'static str> {
        validate::RESERVED_EXTRA_KEYS
            .into_iter()
            .find(|key| extra.contains_key(*key))
    }
}
