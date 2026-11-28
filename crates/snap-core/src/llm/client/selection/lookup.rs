//! 선택 키로 설정에서 엔드포인트·모델을 찾는 조회 자리.
//!
//! 이름 비교는 02-2 정규화 규칙대로 trim한 값 기준으로 함.

use crate::config::types::Config;
use crate::llm::client::selection::error::SelectionError;
use crate::llm::client::selection::key::SelectionKey;
use crate::llm::client::selection::target::SelectionTarget;

/// 선택 키로 설정에서 엔드포인트와 모델을 찾아 한 짝으로 돌려줌.
///
/// 엔드포인트 이름이 없거나 모델 이름이 그 엔드포인트 `models`에 없으면 대상 없음 오류.
pub fn find_target<'a>(
    config: &'a Config,
    key: &SelectionKey,
) -> Result<SelectionTarget<'a>, SelectionError> {
    let endpoint = config
        .endpoints
        .iter()
        .find(|candidate| candidate.name.trim() == key.endpoint);
    let Some(endpoint) = endpoint else {
        return Err(SelectionError::target_missing(&key.endpoint, &key.model));
    };
    let model = endpoint
        .models
        .iter()
        .find(|candidate| candidate.name.trim() == key.model);
    let Some(model) = model else {
        return Err(SelectionError::target_missing(&key.endpoint, &key.model));
    };
    Ok(SelectionTarget { endpoint, model })
}
