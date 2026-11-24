//! 설정 검증 오류. 이름만 밝히고 값·주소·키는 미포함.

use thiserror::Error;

use crate::locale::config::CONFIG_BACKOFF_RANGE;
use crate::locale::config::CONFIG_BASE_URL_REJECTED;
use crate::locale::config::CONFIG_ENDPOINT_KEY_REJECTED;
use crate::locale::config::CONFIG_ENDPOINT_NAME_DUPLICATE;
use crate::locale::config::CONFIG_ENDPOINT_NAME_EMPTY;
use crate::locale::config::CONFIG_ENDPOINT_NAME_MISSING;
use crate::locale::config::CONFIG_EXTRA_RESERVED_KEY;
use crate::locale::config::CONFIG_HTTP_REJECTED;
use crate::locale::config::CONFIG_KIND_REJECTED;
use crate::locale::config::CONFIG_LANGUAGE_REJECTED;
use crate::locale::config::CONFIG_LEGACY_MODELS_REJECTED;
use crate::locale::config::CONFIG_MODEL_ENDPOINT_REJECTED;
use crate::locale::config::CONFIG_MODEL_ID_EMPTY;
use crate::locale::config::CONFIG_MODEL_NAME_DUPLICATE;
use crate::locale::config::CONFIG_MODEL_NAME_EMPTY;
use crate::locale::config::CONFIG_MODEL_NAME_MISSING;
use crate::locale::config::CONFIG_NAME_SEPARATOR;
use crate::locale::config::CONFIG_REASONING_REJECTED;
use crate::locale::config::CONFIG_SAMPLING_REJECTED;

/// 설정 검증 실패 상태. 엔드포인트·모델·항목 이름만 담음.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// 엔드포인트의 자격 키 칸. 값은 읽지도 출력하지 않음.
    #[error("{CONFIG_ENDPOINT_KEY_REJECTED}: {endpoint}")]
    EndpointKeyField {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// 최상위의 옛 `models` 칸.
    #[error("{CONFIG_LEGACY_MODELS_REJECTED}")]
    LegacyModels,

    /// 모델의 옛 `endpoint` 칸.
    #[error("{CONFIG_MODEL_ENDPOINT_REJECTED}: {endpoint} / {model}")]
    ModelEndpointField {
        /// 엔드포인트 이름.
        endpoint: String,
        /// 모델 이름.
        model: String,
    },

    /// 엔드포인트 이름 중복(trim 후).
    #[error("{CONFIG_ENDPOINT_NAME_DUPLICATE}: {endpoint}")]
    DuplicateEndpointName {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// 같은 엔드포인트 안 모델 이름 중복(trim 후).
    #[error("{CONFIG_MODEL_NAME_DUPLICATE}: {endpoint} / {model}")]
    DuplicateModelName {
        /// 엔드포인트 이름.
        endpoint: String,
        /// 모델 이름.
        model: String,
    },

    /// trim 후 빈 엔드포인트 이름. 자리 번호만 밝힘.
    #[error("{CONFIG_ENDPOINT_NAME_EMPTY}: #{index}")]
    EmptyEndpointName {
        /// 엔드포인트 목록 안 자리 번호(0부터 시작).
        index: usize,
    },

    /// 엔드포인트 `name` 칸이 아예 없음. 자리 번호만 밝힘.
    #[error("{CONFIG_ENDPOINT_NAME_MISSING}: #{index}")]
    EndpointNameMissing {
        /// 엔드포인트 목록 안 자리 번호(0부터 시작).
        index: usize,
    },

    /// trim 후 빈 모델 이름.
    #[error("{CONFIG_MODEL_NAME_EMPTY}: {endpoint}")]
    EmptyModelName {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// 모델 `name` 칸이 아예 없음. 엔드포인트 이름만 밝힘.
    #[error("{CONFIG_MODEL_NAME_MISSING}: {endpoint}")]
    ModelNameMissing {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// trim 후 빈 모델 `id`.
    #[error("{CONFIG_MODEL_ID_EMPTY}: {endpoint} / {model}")]
    EmptyModelId {
        /// 엔드포인트 이름.
        endpoint: String,
        /// 모델 이름.
        model: String,
    },

    /// 이름에 `/`가 있음.
    #[error("{CONFIG_NAME_SEPARATOR}: {name}")]
    NameHasSeparator {
        /// 엔드포인트 이름 또는 모델 이름.
        name: String,
    },

    /// `kind` 값이 목록 밖. 입력 값은 미포함.
    #[error("{CONFIG_KIND_REJECTED}: {endpoint}")]
    KindRejected {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// `base_url` 형식 오류. 입력 값은 미포함.
    #[error("{CONFIG_BASE_URL_REJECTED}: {endpoint}")]
    BaseUrlRejected {
        /// 엔드포인트 이름.
        endpoint: String,
    },

    /// `reasoning` 값이 목록 밖. 입력 값은 미포함.
    #[error("{CONFIG_REASONING_REJECTED}: {model}")]
    ReasoningRejected {
        /// 모델 이름.
        model: String,
    },

    /// `extra`의 예약 키. `extra` 값은 미포함.
    #[error("{CONFIG_EXTRA_RESERVED_KEY}: {model} / {key}")]
    ExtraReservedKey {
        /// 모델 이름.
        model: String,
        /// 예약 키 이름.
        key: String,
    },

    /// 샘플링 항목 이름과 범위 오류. 입력 값은 미포함.
    #[error("{CONFIG_SAMPLING_REJECTED}: {scope} / {item}")]
    SamplingRejected {
        /// 대상 표시. 모델 이름, 없으면 기본값 표시.
        scope: String,
        /// 항목 이름.
        item: String,
    },

    /// `http` 항목 이름 오류. 입력 값은 미포함.
    #[error("{CONFIG_HTTP_REJECTED}: {item}")]
    HttpRejected {
        /// 항목 이름.
        item: String,
    },

    /// `language` 값이 허용값 밖. 입력 값은 미포함.
    #[error("{CONFIG_LANGUAGE_REJECTED}")]
    Language,

    /// `backoff_min`이 `backoff_max`보다 큼. 입력 값은 미포함.
    #[error("{CONFIG_BACKOFF_RANGE}: backoff_min, backoff_max")]
    BackoffRange,
}
