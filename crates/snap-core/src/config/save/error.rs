//! 설정 저장 실패 상태 자리. 경로와 엔드포인트 이름만 담고 키·주소·파일 내용은 미포함.

use thiserror::Error;

use crate::config::credentials::CredentialsError;
use crate::config::validate::ValidationError;
use crate::jsonc::JsoncError;
use crate::jsonc::SaveError;
use crate::locale::save::CONFIG_SAVE_API_KEYS_NOT_OBJECT;
use crate::locale::save::CONFIG_SAVE_ENDPOINT_NOT_FOUND;
use crate::locale::save::CONFIG_SAVE_ENDPOINTS_NOT_ARRAY;
use crate::locale::save::CONFIG_SAVE_MODEL_NOT_FOUND;
use crate::locale::save::CONFIG_SAVE_MODELS_NOT_ARRAY;
use crate::locale::save::CONFIG_SAVE_READ_FAILED;

/// 설정 저장 실패 상태. 검증 거부는 기존 검증 오류를 그대로 씀.
#[derive(Debug, Error)]
pub enum ConfigSaveError {
    /// 설정 파일을 읽지 못한 자리. 경로만 밝힘.
    #[error("{CONFIG_SAVE_READ_FAILED}: {path}")]
    Read {
        /// 읽지 못한 설정 파일 경로.
        path: String,
    },

    /// 편집 결과 해석이 문법에 걸린 자리. 줄 번호만 밝힘.
    #[error(transparent)]
    Jsonc(
        /// JSONC 해석이 낸 오류. 값·줄 내용은 미포함.
        #[from]
        Box<JsoncError>,
    ),

    /// 저장 직전 설정 검증에 걸린 자리. 검증 오류가 이름만 담음.
    #[error(transparent)]
    Validation(
        /// `check_config`가 낸 오류. 키·주소는 미포함.
        #[from]
        Box<ValidationError>,
    ),

    /// 저장 직전 자격 구조 검사에 걸린 자리. 자격 검사가 경로만 담음.
    #[error(transparent)]
    Credentials(
        /// `api_keys_from_value`가 낸 오류. 키 값은 미포함.
        #[from]
        Box<CredentialsError>,
    ),

    /// 원자 저장이 실패한 자리. 경로만 밝힘.
    #[error(transparent)]
    Save(
        /// 저장 골격이 낸 오류. 경로만 담음.
        #[from]
        Box<SaveError>,
    ),

    /// `endpoints` 칸이 배열이 아니라 편집할 수 없는 자리. 경로만 밝힘.
    #[error("{CONFIG_SAVE_ENDPOINTS_NOT_ARRAY}: {path}")]
    EndpointsNotArray {
        /// 설정 파일 경로.
        path: String,
    },

    /// 같은 이름 엔드포인트를 찾지 못한 자리. 이름만 밝힘.
    #[error("{CONFIG_SAVE_ENDPOINT_NOT_FOUND}: {endpoint}")]
    EndpointNotFound {
        /// 찾지 못한 엔드포인트 이름.
        endpoint: String,
    },

    /// 같은 엔드포인트 안 `models` 칸이 배열이 아니라 편집할 수 없는 자리.
    /// 엔드포인트 이름만 밝힘.
    #[error("{CONFIG_SAVE_MODELS_NOT_ARRAY}: {endpoint}")]
    ModelsNotArray {
        /// `models` 칸을 가진 엔드포인트 이름.
        endpoint: String,
    },

    /// 같은 엔드포인트 안 같은 이름 모델을 찾지 못한 자리. 모델 이름만 밝힘.
    #[error("{CONFIG_SAVE_MODEL_NOT_FOUND}: {model}")]
    ModelNotFound {
        /// 찾지 못한 모델 이름.
        model: String,
    },

    /// 자격 파일의 `api_keys` 칸이 객체가 아니라 편집할 수 없는 자리. 경로만 밝힘.
    #[error("{CONFIG_SAVE_API_KEYS_NOT_OBJECT}: {path}")]
    ApiKeysNotObject {
        /// 자격 파일 경로.
        path: String,
    },
}
