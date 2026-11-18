//! 저장이 이름을 확인하는 자리: 규칙에 맞지 않으면 설정 오류로 바꿈.
//!
//! 검증 규칙(`validate::checked_name`)이 걸러낸 이름 거부를 설정 오류로 바꾸는 일만 맡음.
//! 설정 쪽과 자격 쪽이 같은 매핑을 각자 적지 않게 한 자리로 모임.
//! trim한 이름만 담음(입력 원문은 미포함).

use super::super::error::{EndpointIssue, ModelIssue, ValidationError};
use super::super::read::validate::{self, NameRejection};

/// 확인하려는 이름이 무엇인지 나타내는 정보.
///
/// 모델 이름은 오류 문구에 엔드포인트 이름을 앞에 밝혀야 하므로, trim한 엔드포인트
/// 이름을 함께 가짐.
#[derive(Debug)]
pub(super) enum NameScope {
    /// 엔드포인트 이름.
    Endpoint,
    /// 모델 이름. `endpoint`는 trim한 엔드포인트 이름.
    Model { endpoint: String },
}

/// 이름을 검증 규칙으로 확인하고, 규칙에 맞지 않으면 검증 오류를 그대로 돌려줌.
///
/// 성공하면 trim한 이름을 돌려주므로 쓰기 단계가 그대로 씀. 설정 오류로 바꾸는 일은
/// `ConfigError`가 정한 `From`을 쓰는 호출부의 `?` 한 곳에만 적음.
pub(super) fn checked_save_name(raw: &str, scope: NameScope) -> Result<String, ValidationError> {
    validate::checked_name(raw).map_err(|rejection| name_error(rejection, scope))
}

/// 이름 거부를 검증 오류로 바꿈.
fn name_error(rejection: NameRejection, scope: NameScope) -> ValidationError {
    match (rejection, scope) {
        (NameRejection::Empty, NameScope::Endpoint) => EndpointIssue::EmptyEndpointName.into(),
        (NameRejection::HasSlash { name }, NameScope::Endpoint) => {
            EndpointIssue::EndpointNameHasSlash { endpoint: name }.into()
        }
        (NameRejection::Empty, NameScope::Model { endpoint }) => ModelIssue::EmptyModelName {
            endpoint: endpoint.clone(),
        }
        .into(),
        (NameRejection::HasSlash { name }, NameScope::Model { endpoint }) => {
            ModelIssue::ModelNameHasSlash {
                endpoint: endpoint.clone(),
                model: name,
            }
            .into()
        }
    }
}
