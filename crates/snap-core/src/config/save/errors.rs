//! 설정 저장 실패 값(오류)을 조립하는 자리. `ConfigSaveError`의 칸에 채울 값만 만든다.
//! 선택 규칙은 `select`가 맡고, 오류 값 조립은 이 파일이 맡는다.

use std::path::Path;

use crate::config::save::error::ConfigSaveError;

/// `endpoints` 칸이 배열이 아닐 때 오류. 경로만 담음.
pub(super) fn endpoints_not_array(path: &Path) -> ConfigSaveError {
    ConfigSaveError::EndpointsNotArray {
        path: path.display().to_string(),
    }
}

/// 같은 엔드포인트 안 `models` 칸이 배열이 아닐 때 오류. 엔드포인트 이름만 담음.
pub(super) fn models_not_array(endpoint_name: &str) -> ConfigSaveError {
    ConfigSaveError::ModelsNotArray {
        endpoint: endpoint_name.trim().to_owned(),
    }
}

/// 같은 이름 엔드포인트를 찾지 못했을 때 오류. 이름만 담음.
pub(super) fn endpoint_not_found(name: &str) -> ConfigSaveError {
    ConfigSaveError::EndpointNotFound {
        endpoint: name.trim().to_owned(),
    }
}

/// 같은 엔드포인트 안 같은 이름 모델을 찾지 못했을 때 오류. 모델 이름만 담음.
pub(super) fn model_not_found(name: &str) -> ConfigSaveError {
    ConfigSaveError::ModelNotFound {
        model: name.trim().to_owned(),
    }
}

/// `api_keys` 칸이 객체가 아닐 때 오류. 경로만 담음.
pub(super) fn api_keys_not_object(path: &Path) -> ConfigSaveError {
    ConfigSaveError::ApiKeysNotObject {
        path: path.display().to_string(),
    }
}
