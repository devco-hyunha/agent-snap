//! JSONC 텍스트를 [`Config`]로 읽는 최상위 조립 골격입니다. 칸 읽기는 주제 파일(`model`·
//! `sampling`·`http`·`agent`·`target_language`)이 맡고, 칸 단위 검증 규칙은 `validate` 모듈, 오류 생성은
//! `error` 모듈이 맡습니다.
//!
//! 오류에는 이름과 칸 이름만 담고 입력 값은 담지 않습니다. 거부하는 키 칸은 값을 읽지 않고
//! 칸이 있는지만 봅니다.

use jsonc_parser::{JsonArray, JsonObject, JsonValue};

use super::super::error::{ConfigError, EndpointIssue};
use super::agent::read_agent;
use super::http::read_http;
use super::model::read_endpoints;
use super::sampling::read_root_sampling;
use super::target_language::read_target_language;
use super::types::Config;
use crate::jsonc::{self, WrongType, pick_array, take_optional};

impl Config {
    /// 설정 파일 텍스트를 JSONC로 읽어 검증합니다.
    ///
    /// 주석(`//`, `/* */`)과 마지막 쉼표를 허용하고, 모르는 칸은 무시합니다. 빠진 값은
    /// 기본값이 됩니다. 단, `sampling`·`http` 안의 모르는 항목과 거부 칸(엔드포인트의
    /// `api_key`·`api_key_env`, 최상위 `models`, 모델의 `endpoint`, `extra`의 예약 키)은
    /// 예외로 거부합니다.
    ///
    /// # 오류
    /// 문법 오류는 줄 번호만, 그 밖의 거부는 이름만 담은 [`ConfigError`]로 돌려줍니다.
    pub fn parse(text: &str) -> Result<Self, ConfigError> {
        let mut root = jsonc::parse_root_object(text)?;
        if root.get("models").is_some() {
            return Err(EndpointIssue::LegacyTopLevelModels.into());
        }
        Ok(Self {
            target_language: read_target_language(&mut root)?,
            endpoints: read_endpoints(&mut root)?,
            sampling: read_root_sampling(&mut root)?,
            http: read_http(&mut root)?,
            agent: read_agent(&mut root)?,
        })
    }
}

/// 객체의 배열 칸을 꺼내 항목마다 `read_item`으로 읽고, trim한 이름이 겹치면 거부합니다.
///
/// 칸이 없으면 빈 목록입니다. `wrong_type`은 칸이 배열이 아닐 때, `duplicate_error`는 이름이
/// 겹칠 때 오류를 만듭니다. 엔드포인트·모델 목록 읽기가 같은 규칙을 한 자리에서 씁니다.
pub(super) fn read_unique_named<'a, T>(
    owner: &mut JsonObject<'a>,
    field: &str,
    wrong_type: impl FnOnce() -> ConfigError,
    mut read_item: impl FnMut(usize, JsonValue<'a>) -> Result<T, ConfigError>,
    name_of: fn(&T) -> &str,
    duplicate_error: impl Fn(String) -> ConfigError,
) -> Result<Vec<T>, ConfigError> {
    let items = take_optional(owner, field, pick_array)
        .map_err(|WrongType| wrong_type())?
        .map(JsonArray::take_inner)
        .unwrap_or_default();
    let mut seen_names = std::collections::HashSet::new();
    let mut read_items = Vec::with_capacity(items.len());
    for (index, item) in items.into_iter().enumerate() {
        let read = read_item(index, item)?;
        if !seen_names.insert(name_of(&read).to_owned()) {
            return Err(duplicate_error(name_of(&read).to_owned()));
        }
        read_items.push(read);
    }
    Ok(read_items)
}
