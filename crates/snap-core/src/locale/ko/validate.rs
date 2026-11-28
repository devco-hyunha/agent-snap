//! 한국어 설정 검증 문구.
/// 언어 허용값 안내까지 담은 문구. 항목 이름만 밝힘.
pub const CONFIG_LANGUAGE_REJECTED: &str = "language 값이 올바르지 않습니다. 허용값은 ko, en입니다";

/// 엔드포인트의 자격 키 칸을 거부할 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_ENDPOINT_KEY_REJECTED: &str =
    "엔드포인트에 키 칸이 있습니다. 키는 credentials.jsonc에 두세요";

/// 최상위의 옛 `models` 칸을 거부할 때 문구.
pub const CONFIG_LEGACY_MODELS_REJECTED: &str = "모델은 endpoints[].models에 두세요";

/// 모델의 옛 `endpoint` 칸을 거부할 때 문구. 모델 이름만 붙임.
pub const CONFIG_MODEL_ENDPOINT_REJECTED: &str = "모델에 endpoint 칸은 없습니다";

/// 엔드포인트 이름 중복을 거부할 때 문구. 이름만 붙임.
pub const CONFIG_ENDPOINT_NAME_DUPLICATE: &str = "엔드포인트 이름이 중복됩니다";

/// 같은 엔드포인트 안 모델 이름 중복을 거부할 때 문구. 이름만 붙임.
pub const CONFIG_MODEL_NAME_DUPLICATE: &str = "모델 이름이 중복됩니다";

/// trim 후 빈 엔드포인트 이름을 거부할 때 문구. 자리 번호만 붙임.
pub const CONFIG_ENDPOINT_NAME_EMPTY: &str = "엔드포인트 이름이 비어 있습니다";

/// 엔드포인트 이름 칸이 아예 없을 때 문구. 자리 번호만 붙임.
pub const CONFIG_ENDPOINT_NAME_MISSING: &str = "엔드포인트 이름 칸이 없습니다";

/// trim 후 빈 모델 이름을 거부할 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_MODEL_NAME_EMPTY: &str = "모델 이름이 비어 있습니다";

/// 모델 이름 칸이 아예 없을 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_MODEL_NAME_MISSING: &str = "모델 이름 칸이 없습니다";

/// trim 후 빈 모델 `id`를 거부할 때 문구. 이름만 붙임.
pub const CONFIG_MODEL_ID_EMPTY: &str = "모델 id가 비어 있습니다";

/// 이름에 `/`가 있을 때 거부 문구. 이름만 붙임.
pub const CONFIG_NAME_SEPARATOR: &str = "이름에 /를 쓸 수 없습니다";

/// `kind` 값 거부 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_KIND_REJECTED: &str = "kind 값이 허용값 밖입니다";

/// `base_url` 형식 오류 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_BASE_URL_REJECTED: &str = "base_url 형식이 올바르지 않습니다";

/// `reasoning` 값 거부 문구. 모델 이름만 붙임.
pub const CONFIG_REASONING_REJECTED: &str = "reasoning 값이 허용값 밖입니다";

/// `context_window` 값 거부 문구. 모델 이름만 붙임.
pub const CONFIG_CONTEXT_WINDOW_REJECTED: &str = "context_window 값은 1 이상 정수만 됩니다";

/// `extra` 예약 키 거부 문구. 모델 이름과 키 이름만 붙임.
pub const CONFIG_EXTRA_RESERVED_KEY: &str = "extra에 예약 키를 쓸 수 없습니다";

/// 샘플링 항목 거부 문구. 대상과 항목 이름만 붙임.
pub const CONFIG_SAMPLING_REJECTED: &str = "sampling 항목이 허용 범위 밖이거나 모르는 항목입니다";

/// `http` 항목 거부 문구. 항목 이름만 붙임.
pub const CONFIG_HTTP_REJECTED: &str = "http 항목이 허용 범위 밖이거나 모르는 항목입니다";

/// `backoff_min`과 `backoff_max` 관계 거부 문구. 항목 이름만 붙임.
pub const CONFIG_BACKOFF_RANGE: &str = "backoff_min이 backoff_max보다 큽니다";

/// 오류 문구에 쓰는 샘플링 대상 표시. 모델별 값이 없을 때·블록 이름과 같은 값.
pub const SAMPLING_SCOPE: &str = "sampling";

/// 오류 문구에 쓰는 `http` 블록 이름. 블록 자체가 객체가 아닐 때 씀.
pub const HTTP_BLOCK: &str = "http";
