//! 설정 기본값 자리.
/// snap 설정 폴더 이름.
pub const CONFIG_DIR_NAME: &str = "snap";

/// 자격 파일 이름.
pub const CREDENTIALS_FILE_NAME: &str = "credentials.jsonc";

/// 자격 파일 최상위 키 이름.
pub const API_KEYS_KEY: &str = "api_keys";

/// 언어 기본값.
pub const DEFAULT_LANGUAGE: &str = "ko";

/// 한국어 언어 값.
pub const LANGUAGE_KO: &str = "ko";

/// 영어 언어 값.
pub const LANGUAGE_EN: &str = "en";

/// 언어 허용값 목록.
pub const ALLOWED_LANGUAGES: [&str; 2] = [LANGUAGE_KO, LANGUAGE_EN];

/// snap 설정 파일 이름.
pub const SNAP_FILE_NAME: &str = "snap.jsonc";

/// 재시도 횟수 기본값.
pub const DEFAULT_RETRIES: u64 = 3;

/// 재시도 대기 최소 초 기본값.
pub const DEFAULT_BACKOFF_MIN_SECONDS: f64 = 1.0;

/// 재시도 대기 최대 초 기본값.
pub const DEFAULT_BACKOFF_MAX_SECONDS: f64 = 30.0;

/// 재시도 권유 최대 초 기본값.
pub const DEFAULT_RETRY_AFTER_MAX_SECONDS: f64 = 60.0;

/// 연결 시도 최대 초 기본값.
pub const DEFAULT_CONNECT_SECONDS: f64 = 10.0;

/// 응답 헤더 대기 최대 초 기본값.
pub const DEFAULT_HEADER_WAIT_SECONDS: f64 = 300.0;

/// 응답 스트림 대기 최대 초 기본값.
pub const DEFAULT_STREAM_IDLE_SECONDS: f64 = 120.0;

/// Ollama 자동 발견 기본 주소.
pub const DEFAULT_OLLAMA_BASE_URL: &str = "http://127.0.0.1:11434/v1";

/// LM Studio 자동 발견 기본 주소.
pub const DEFAULT_LMSTUDIO_BASE_URL: &str = "http://127.0.0.1:1234/v1";

/// 모델 맥락 토큰 한도 기본값.
pub const DEFAULT_CONTEXT_WINDOW: u64 = 65536;
