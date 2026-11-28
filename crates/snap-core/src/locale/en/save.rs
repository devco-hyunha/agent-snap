//! 영어 설정 저장 문구.
/// 저장 경로에 폴더 칸이 없을 때 문구. 경로만 붙임.
pub const SAVE_NO_FOLDER: &str = "Save path has no folder name";

/// 설정 폴더를 만들지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_CREATE_FOLDER_FAILED: &str = "Failed to create config folder";

/// 임시 파일을 만들지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_CREATE_TEMP_FAILED: &str = "Failed to create temporary file";

/// 임시 파일에 쓰지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_WRITE_TEMP_FAILED: &str = "Failed to write temporary file";

/// 원본 교체를 하지 못했을 때 문구. 원본 경로만 붙임. 원본은 그대로 남는다.
pub const SAVE_REPLACE_FAILED: &str = "Failed to replace config file. Original file is unchanged";

/// 설정 파일을 읽지 못했을 때 저장 문구. 경로만 붙임.
pub const CONFIG_SAVE_READ_FAILED: &str = "Cannot read config file. Save skipped";

/// `endpoints` 칸이 배열이 아닐 때 문구. 경로만 붙임.
pub const CONFIG_SAVE_ENDPOINTS_NOT_ARRAY: &str = "endpoints field is not an array";

/// 같은 이름 엔드포인트를 찾지 못했을 때 문구. 이름만 붙임.
pub const CONFIG_SAVE_ENDPOINT_NOT_FOUND: &str = "No endpoint with that name";

/// 같은 엔드포인트 안 `models` 칸이 배열이 아닐 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_SAVE_MODELS_NOT_ARRAY: &str = "The models field of that endpoint is not an array";

/// 같은 엔드포인트 안 같은 이름 모델을 찾지 못했을 때 문구. 모델 이름만 붙임.
pub const CONFIG_SAVE_MODEL_NOT_FOUND: &str = "No model with that name";

/// 자격 파일의 `api_keys` 칸이 객체가 아닐 때 문구. 경로만 붙임.
pub const CONFIG_SAVE_API_KEYS_NOT_OBJECT: &str = "api_keys field is not an object";
