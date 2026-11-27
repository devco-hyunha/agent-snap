//! 선택 키 문자열 해석 자리. 설정 읽기 없이 형식만 봄.

use crate::consts::limits::NAME_SEPARATOR;
use crate::llm::client::selection::error::SelectionError;
use crate::llm::client::selection::error::SelectionFormatProblem;
use crate::llm::client::selection::key::SelectionKey;

/// `엔드포인트이름/모델이름` 문자열을 선택 키로 해석함.
///
/// 구분자가 없거나 앞·뒤가 trim 후 비면 형식 오류로 거부함. 구분자가 둘 이상이면
/// 앞쪽 구분자만 쓰고 나머지를 모델 이름의 일부로 본다.
pub fn parse_selection_key(key: &str) -> Result<SelectionKey, SelectionError> {
    let Some((raw_endpoint, raw_model)) = key.split_once(NAME_SEPARATOR) else {
        return Err(SelectionError::format(
            SelectionFormatProblem::MissingSeparator,
        ));
    };
    let endpoint = raw_endpoint.trim();
    let model = raw_model.trim();
    if endpoint.is_empty() || model.is_empty() {
        return Err(SelectionError::format(SelectionFormatProblem::EmptyPart));
    }
    Ok(SelectionKey {
        endpoint: endpoint.to_owned(),
        model: model.to_owned(),
    })
}
