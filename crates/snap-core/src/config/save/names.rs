//! 저장이 이름을 확인하는 자리: 규칙에 맞지 않으면 설정 오류로 바꿉니다.
//!
//! 이 파일의 책임은 하나입니다. 02-2 검증 규칙(`validate::checked_name`)이 걸러낸 이름
//! 거부를 설정 오류로 바꾸는 일. 설정 쪽(엔드포인트 이름·모델 이름)과 자격 쪽(엔드포인트
//! 이름)이 같은 매핑을 각자 적지 않게 한 자리로 모였습니다(dev-principles 3절 DRY).
//! 담는 값과 오류 문구는 02-2가 정한 그대로이고, trim한 이름만 담습니다(입력 원문은 담지
//! 않습니다).

use super::super::error::{EndpointIssue, ModelIssue, ValidationError};
use super::super::read::validate::{self, NameRejection};

/// 확인하려는 이름이 무엇인지 나타내는 정보입니다.
///
/// 모델 이름은 오류 문구에 엔드포인트 이름을 앞에 밝혀야 하므로, trim한 엔드포인트
/// 이름을 함께 가집니다.
#[derive(Debug)]
pub(super) enum NameScope {
    /// 엔드포인트 이름입니다.
    Endpoint,
    /// 모델 이름입니다. `endpoint`는 trim한 엔드포인트 이름입니다.
    Model { endpoint: String },
}

/// 이름을 02-2 규칙으로 확인하고, 규칙에 맞지 않으면 02-2 검증 오류를 그대로 돌려줍니다.
///
/// 성공하면 trim한 이름을 돌려주므로 쓰기 단계가 그대로 씁니다. 설정 오류로 바꾸는 일은
/// `ConfigError`가 정한 `From`을 쓰는 호출부의 `?` 한 곳에만 적힙니다 (리뷰 지적 N2:
/// 검증 오류를 설정 오류로 바꾸는 변환을 한 번 더 적지 않기).
pub(super) fn checked_save_name(raw: &str, scope: NameScope) -> Result<String, ValidationError> {
    validate::checked_name(raw).map_err(|rejection| name_error(rejection, scope))
}

/// 이름 거부를 02-2 검증 오류로 바꿉니다.
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
