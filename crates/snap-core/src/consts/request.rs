//! 요청 본문·헤더 키 이름 자리. 본문 조립과 헤더 조립이 쓰는 키 이름만 담음.

/// 요청 본문 모델 식별 키.
pub const BODY_MODEL_KEY: &str = "model";

/// 요청 본문 메시지 목록 키.
pub const BODY_MESSAGES_KEY: &str = "messages";

/// 요청 본문 메시지 역할 키.
pub const BODY_ROLE_KEY: &str = "role";

/// 요청 본문 메시지 본문 키.
pub const BODY_CONTENT_KEY: &str = "content";

/// 요청 본문 도구 목록 키.
pub const BODY_TOOLS_KEY: &str = "tools";

/// 요청 본문 온도 키.
pub const BODY_TEMPERATURE_KEY: &str = "temperature";

/// 요청 본문 상위 확률 합계 키.
pub const BODY_TOP_P_KEY: &str = "top_p";

/// 요청 본문 생성 최대 토큰 수 키.
pub const BODY_MAX_TOKENS_KEY: &str = "max_tokens";

/// 요청 본문 스트리밍 표시 키. 항상 참으로 넣음.
pub const BODY_STREAM_KEY: &str = "stream";

/// 생각 강도를 한 필드로 넣을 때 키.
pub const REASONING_EFFORT_KEY: &str = "reasoning_effort";

/// OpenRouter가 쓰는 생각 객체 키.
pub const REASONING_OBJECT_KEY: &str = "reasoning";

/// OpenRouter 생각 객체 안의 강도 키.
pub const REASONING_EFFORT_INNER_KEY: &str = "effort";

/// `off` 대신 본문에 담는 강도 값.
pub const REASONING_VALUE_OFF: &str = "none";

/// 본문에 담는 강도 값. 설정 값과 같은 이름.
pub const REASONING_VALUE_LOW: &str = "low";

/// 본문에 담는 강도 값. 설정 값과 같은 이름.
pub const REASONING_VALUE_MEDIUM: &str = "medium";

/// 본문에 담는 강도 값. 설정 값과 같은 이름.
pub const REASONING_VALUE_HIGH: &str = "high";

/// 인증 헤더 이름.
pub const AUTH_HEADER: &str = "authorization";

/// 인증 헤더 값의 형식 접두.
pub const AUTH_SCHEME: &str = "Bearer";

/// 채팅 completions 경로 조각.
pub const CHAT_COMPLETIONS_PATH: &str = "chat/completions";
