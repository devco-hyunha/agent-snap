//! 선택 키 조회 결과 타입 자리. 필드만 담음.

use crate::config::types::EndpointConfig;
use crate::config::types::ModelConfig;

/// 선택 키가 가리키는 엔드포인트와 모델 한 짝.
#[derive(Debug)]
pub struct SelectionTarget<'a> {
    /// 설정에서 찾은 엔드포인트.
    pub endpoint: &'a EndpointConfig,

    /// 그 엔드포인트 아래에서 찾은 모델.
    pub model: &'a ModelConfig,
}
