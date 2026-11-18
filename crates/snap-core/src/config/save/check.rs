//! 설정 저장 검증: 검증 규칙(이름·주소·모델 id·`extra` 예약 키)으로 쓰기 가능 여부를 확인

use std::collections::BTreeMap;

use super::super::error::{ConfigError, EndpointIssue, ModelIssue, ValidationError};
use super::super::read::types::{EndpointKind, ExtraValue, Reasoning, Sampling, ToolCallForm};
use super::super::read::validate::{self, is_valid_base_url};
use super::names::{NameScope, checked_save_name};

/// 엔드포인트 추가·수정 정보. 키 값은 여기에 넣지 않고 자격 파일(`credentials.jsonc`)에만 씀
#[derive(Debug)]
pub struct EndpointSave {
    /// 엔드포인트 이름. trim한 이 이름으로 기존 항목을 찾아 고침
    pub name: String,
    /// 서버 기본 주소
    pub base_url: String,
    /// 서버 종류
    pub kind: EndpointKind,
    /// 이 엔드포인트 아래에 쓸 모델 목록
    pub models: Vec<ModelSave>,
}

impl EndpointSave {
    /// 설정 파일에 쓸 수 있는지 검증 규칙으로 확인
    ///
    /// 엔드포인트 이름·주소, 모델 이름·id, 모델 `extra`의 예약 키를 봄. `extra` 예약 키 판단은 읽기가 쓰는 규칙 목록(`validate::RESERVED_EXTRA_KEYS`)을 그대로 씀
    ///
    /// # 오류
    /// 이름 규칙 위반은 [`EndpointIssue::EmptyEndpointName`]·[`EndpointIssue::EndpointNameHasSlash`]
    /// 또는 [`ModelIssue::EmptyModelName`]·[`ModelIssue::ModelNameHasSlash`], 모델 id 규칙
    /// 위반은 [`ModelIssue::EmptyModelId`], 주소 규칙 위반은 [`EndpointIssue::InvalidBaseUrl`],
    /// `extra` 예약 키는 [`ModelIssue::ExtraReservedKey`]. 입력 값은 오류에 미포함
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.check().map(|_| ())
    }

    /// 검증하며 쓰기 단계에 넘길 값(trim한 이름과 원본 정보)을 만듦
    ///
    /// 규칙을 통과하지 못한 값은 여기까지 오지 않아, 검증을 거부한 값이 파일 쓰기로 흐르는 경로가 없음
    pub(crate) fn check(&self) -> Result<CheckedEndpoint<'_>, ConfigError> {
        let name = checked_save_name(&self.name, NameScope::Endpoint)?;
        if !is_valid_base_url(&self.base_url) {
            return Err(EndpointIssue::InvalidBaseUrl {
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

/// 설정 파일에 쓰는 모델 하나
///
/// `None` 항목과 `None`으로 채운 샘플링 항목은 파일에 쓰지 않음. 기본값을 굳이 파일에 쓰지 않는 규칙.
#[derive(Debug, Default, Clone)]
pub struct ModelSave {
    /// 모델 이름. 같은 엔드포인트의 기존 모델과 trim한 이 이름으로 비교
    pub name: String,
    /// 서버에 보낼 모델 id
    pub id: String,
    /// 생각 옵션. `None`이면 파일에 쓰지 않음
    pub reasoning: Option<Reasoning>,
    /// 요청 본문에 합칠 추가 값. 비어 있으면 파일에 쓰지 않음
    pub extra: BTreeMap<String, ExtraValue>,
    /// 맥락 창 토큰 수. `None`이면 파일에 쓰지 않음(읽을 때 65536)
    pub context_window: Option<u64>,
    /// 호출 형식 선언. `None`이면 파일에 쓰지 않음(읽을 때 `openai`)
    pub tool_call_form: Option<ToolCallForm>,
    /// 샘플링 값. `None`으로 채운 항목은 파일에 쓰지 않음
    pub sampling: Sampling,
}

/// 검증한 엔드포인트 한 항목. `name`은 trim한 값이라 쓰기 단계가 그대로 씀
// `check`(pub(crate))가 돌려주는 타입이라 타입 가시성도 같은 크기로 맞춤. 가시성이
// 타입보다 좁으면 공개 함수가 더 좁은 타입을 반환하는 셈이 되어 경고를 받음
pub(crate) struct CheckedEndpoint<'a> {
    /// trim한 엔드포인트 이름
    pub(crate) name: String,
    /// 원본 저장 정보(주소·종류·모델 목록)
    pub(crate) save: &'a EndpointSave,
    /// 검증한 모델 목록
    pub(crate) models: Vec<CheckedModel<'a>>,
}

/// 검증한 모델 한 항목. `name`과 `id`는 trim한 값이라 쓰기 단계가 그대로 씀
pub(crate) struct CheckedModel<'a> {
    /// trim한 모델 이름
    pub(crate) name: String,
    /// trim한 모델 id
    pub(crate) id: String,
    /// 원본 모델 정보(옵션·샘플링·extra)
    pub(crate) save: &'a ModelSave,
}

impl<'a> CheckedModel<'a> {
    /// 모델 이름·id 규칙과 `extra` 예약 키를 검증 규칙으로 확인
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
            return Err(ModelIssue::ExtraReservedKey {
                model: name.clone(),
                key: key.to_owned(),
            }
            .into());
        }
        Ok(Self { name, id, save })
    }

    /// 모델 `extra`에 예약 키가 있으면 그 키를 찾음. 읽기 규칙 목록 재사용
    fn extra_reserved_key(extra: &BTreeMap<String, ExtraValue>) -> Option<&'static str> {
        validate::RESERVED_EXTRA_KEYS
            .into_iter()
            .find(|key| extra.contains_key(*key))
    }
}
