//! 한도·상한 자리. 검증이 쓰는 값만 담음.

/// 허용 `http` 키. 이 목록 밖은 모르는 키로 거부.
pub const ALLOWED_HTTP_KEYS: [&str; 7] = [
    "retries",
    "backoff_min",
    "backoff_max",
    "retry_after_max",
    "connect_seconds",
    "header_wait_seconds",
    "stream_idle_seconds",
];

/// `extra`에 둘 수 없는 예약 키. 요청 본문이 정하는 칸.
pub const RESERVED_EXTRA_KEYS: [&str; 4] = ["model", "messages", "stream", "tools"];

/// 허용 주소 형식 접두. 대소문자는 가리지 않음.
pub const ALLOWED_URL_SCHEMES: [&str; 2] = ["http://", "https://"];

/// 이름 구분자. 선택 키 `엔드포인트이름/모델이름`의 구분자라 이름에 못 씀.
pub const NAME_SEPARATOR: char = '/';

/// 허용 샘플링 항목. 이 목록 밖은 모르는 항목으로 거부.
pub const SAMPLING_ITEMS: [&str; 3] = ["temperature", "top_p", "max_tokens"];

/// 온도 최소.
pub const TEMPERATURE_MIN: f64 = 0.0;

/// 온도 최대.
pub const TEMPERATURE_MAX: f64 = 2.0;

/// 상위 확률 합계 최소(초과). 0은 허용하지 않음.
pub const TOP_P_MIN_EXCLUSIVE: f64 = 0.0;

/// 상위 확률 합계 최대.
pub const TOP_P_MAX: f64 = 1.0;

/// 생성 토큰 수 최소.
pub const MAX_TOKENS_MIN: u64 = 1;

/// 재시도 횟수 최소. 0은 재시도 안 함.
pub const RETRIES_MIN: u64 = 0;

/// 시간 값 하한. 0 초과.
pub const SECONDS_MIN_EXCLUSIVE: f64 = 0.0;

/// 모델 맥락 토큰 한도 최소. 1 이상 정수만 허용.
pub const CONTEXT_WINDOW_MIN: u64 = 1;
