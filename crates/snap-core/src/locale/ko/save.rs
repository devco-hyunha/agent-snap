//! 한국어 설정 저장 문구.
/// 저장 경로에 폴더 칸이 없을 때 문구. 경로만 붙임.
pub const SAVE_NO_FOLDER: &str = "저장 경로에 폴더 이름이 없습니다";

/// 설정 폴더를 만들지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_CREATE_FOLDER_FAILED: &str = "설정 폴더를 만들 수 없습니다";

/// 임시 파일을 만들지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_CREATE_TEMP_FAILED: &str = "임시 파일을 만들 수 없습니다";

/// 임시 파일에 쓰지 못했을 때 문구. 폴더 경로만 붙임.
pub const SAVE_WRITE_TEMP_FAILED: &str = "임시 파일에 쓸 수 없습니다";

/// 원본 교체를 하지 못했을 때 문구. 원본 경로만 붙임. 원본은 그대로 남는다.
pub const SAVE_REPLACE_FAILED: &str = "설정 파일을 교체할 수 없습니다. 원본은 그대로입니다";

/// 설정 파일을 읽지 못했을 때 저장 문구. 경로만 붙임.
pub const CONFIG_SAVE_READ_FAILED: &str = "설정 파일을 읽을 수 없어 저장하지 않습니다";

/// `endpoints` 칸이 배열이 아닐 때 문구. 경로만 붙임.
pub const CONFIG_SAVE_ENDPOINTS_NOT_ARRAY: &str = "endpoints 칸이 배열이 아닙니다";

/// 같은 이름 엔드포인트를 찾지 못했을 때 문구. 이름만 붙임.
pub const CONFIG_SAVE_ENDPOINT_NOT_FOUND: &str = "해당 이름의 엔드포인트가 없습니다";

/// 같은 엔드포인트 안 `models` 칸이 배열이 아닐 때 문구. 엔드포인트 이름만 붙임.
pub const CONFIG_SAVE_MODELS_NOT_ARRAY: &str = "해당 엔드포인트의 models 칸이 배열이 아닙니다";

/// 같은 엔드포인트 안 같은 이름 모델을 찾지 못했을 때 문구. 모델 이름만 붙임.
pub const CONFIG_SAVE_MODEL_NOT_FOUND: &str = "해당 이름의 모델이 없습니다";

/// 자격 파일의 `api_keys` 칸이 객체가 아닐 때 문구. 경로만 붙임.
pub const CONFIG_SAVE_API_KEYS_NOT_OBJECT: &str = "api_keys 칸이 객체가 아닙니다";
