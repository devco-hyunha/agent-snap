//! 설정 검증 모듈 자리. 이름·거부 칸·값 규칙·trim 정규화를 책임별 파일로 나눔.
mod error;
mod fields;
mod names;
mod normalize;
mod values;

pub use error::ValidationError;
pub use normalize::trim_names;

use serde_json::Value;

use crate::config::validate::fields::check_rejected_fields;
use crate::config::validate::names::check_names;
use crate::config::validate::values::check_values;

/// 해석된 설정 값 전체 확인. 첫 거부만 밝힘.
pub fn check_config(root: &Value) -> Result<(), ValidationError> {
    check_rejected_fields(root)?;
    check_names(root)?;
    check_values(root)?;
    Ok(())
}
