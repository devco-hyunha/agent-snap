//! 영어 설정 문구.
/// Debug 출력에 쓰는 가렸음 표시.
pub const SECRET_REDACTED: &str = "redacted";

/// 자격 파일이 없을 때 문구. 경로만 붙임.
pub const CREDENTIALS_FILE_NOT_FOUND: &str = "Credentials file not found";

/// 자격 파일을 읽지 못했을 때 문구. 경로만 붙임.
pub const CREDENTIALS_READ_FAILED: &str = "Failed to read credentials file";

/// 자격 파일을 해석하지 못했을 때 문구. 경로만 붙임.
pub const CREDENTIALS_PARSE_FAILED: &str = "Failed to parse credentials file format";

/// api_keys 구조가 기준과 다를 때 문구. 경로만 붙임.
pub const CREDENTIALS_TYPE_FAILED: &str = "Invalid api_keys format in credentials file";

/// 설정 파일을 읽지 못했을 때 문구. 경로만 붙임.
pub const CONFIG_READ_FAILED: &str = "Failed to read config file";

/// 설정 파일을 해석하지 못했을 때 문구. 경로와 줄 번호만 붙임.
pub const CONFIG_PARSE_FAILED: &str = "Failed to parse config file format";

/// 설정 구조가 기준과 다를 때 문구. 경로만 붙임.
pub const CONFIG_INVALID_SHAPE: &str = "Invalid config file format";

/// 언어 허용값 안내까지 담은 문구. 항목 이름만 밝힘.
pub const CONFIG_LANGUAGE_REJECTED: &str = "Invalid language value. Allowed values are ko, en";

/// 엔드포인트의 자격 키 칸을 거부할 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_ENDPOINT_KEY_REJECTED: &str =
    "Endpoint has a key field. Keep keys in credentials.jsonc";

/// 최상위의 옛 `models` 칸을 거부할 때 문구.
pub const CONFIG_LEGACY_MODELS_REJECTED: &str = "Put models in endpoints[].models";

/// 모델의 옛 `endpoint` 칸을 거부할 때 문구. 모델 이름만 붙임.
pub const CONFIG_MODEL_ENDPOINT_REJECTED: &str = "Model has no endpoint field";

/// 엔드포인트 이름 중복을 거부할 때 문구. 이름만 붙임.
pub const CONFIG_ENDPOINT_NAME_DUPLICATE: &str = "Duplicate endpoint name";

/// 같은 엔드포인트 안 모델 이름 중복을 거부할 때 문구. 이름만 붙임.
pub const CONFIG_MODEL_NAME_DUPLICATE: &str = "Duplicate model name";

/// trim 후 빈 엔드포인트 이름을 거부할 때 문구. 자리 번호만 붙임.
pub const CONFIG_ENDPOINT_NAME_EMPTY: &str = "Empty endpoint name";

/// 엔드포인트 이름 칸이 아예 없을 때 문구. 자리 번호만 붙임.
pub const CONFIG_ENDPOINT_NAME_MISSING: &str = "Missing endpoint name field";

/// trim 후 빈 모델 이름을 거부할 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_MODEL_NAME_EMPTY: &str = "Empty model name";

/// 모델 이름 칸이 아예 없을 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_MODEL_NAME_MISSING: &str = "Missing model name field";

/// trim 후 빈 모델 `id`를 거부할 때 문구. 이름만 붙임.
pub const CONFIG_MODEL_ID_EMPTY: &str = "Empty model id";

/// 이름에 `/`가 있을 때 거부 문구. 이름만 붙임.
pub const CONFIG_NAME_SEPARATOR: &str = "Name must not contain /";

/// `kind` 값 거부 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_KIND_REJECTED: &str = "kind value is outside the allowed list";

/// `base_url` 형식 오류 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_BASE_URL_REJECTED: &str = "Invalid base_url format";

/// `reasoning` 값 거부 문구. 모델 이름만 붙임.
pub const CONFIG_REASONING_REJECTED: &str = "reasoning value is outside the allowed list";

/// `extra` 예약 키 거부 문구. 모델 이름과 키 이름만 붙임.
pub const CONFIG_EXTRA_RESERVED_KEY: &str = "extra must not contain a reserved key";

/// 샘플링 항목 거부 문구. 대상과 항목 이름만 붙임.
pub const CONFIG_SAMPLING_REJECTED: &str = "sampling item is out of range or unknown";

/// `http` 항목 거부 문구. 항목 이름만 붙임.
pub const CONFIG_HTTP_REJECTED: &str = "http item is out of range or unknown";

/// `backoff_min`과 `backoff_max` 관계 거부 문구. 항목 이름만 붙임.
pub const CONFIG_BACKOFF_RANGE: &str = "backoff_min is greater than backoff_max";

/// 오류 문구에 쓰는 샘플링 대상 표시. 모델별 값이 없을 때·블록 이름과 같은 값.
pub const SAMPLING_SCOPE: &str = "sampling";

/// 오류 문구에 쓰는 `http` 블록 이름. 블록 자체가 객체가 아닐 때 씀.
pub const HTTP_BLOCK: &str = "http";
