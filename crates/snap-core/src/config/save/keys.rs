//! 설정 저장 연산이 쓰는 `snap.jsonc` 칸 이름 상수 자리. 저장 연산의 한곳 정본.

/// 최상위 엔드포인트 목록 칸 이름.
pub(super) const ENDPOINTS_KEY: &str = "endpoints";

/// 엔드포인트 안 모델 목록 칸 이름.
pub(super) const MODELS_KEY: &str = "models";

/// 엔드포인트·모델 이름 칸 이름.
pub(super) const NAME_KEY: &str = "name";

/// 모델 식별 칸 이름.
pub(super) const ID_KEY: &str = "id";

/// 엔드포인트 주소 칸 이름.
pub(super) const BASE_URL_KEY: &str = "base_url";

/// 엔드포인트 서버 형식 칸 이름.
pub(super) const KIND_KEY: &str = "kind";

/// 모델 맥락 토큰 한도 칸 이름.
pub(super) const CONTEXT_WINDOW_KEY: &str = "context_window";

/// 모델 생각 강도 칸 이름.
pub(super) const REASONING_KEY: &str = "reasoning";
