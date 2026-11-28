//! 모델 수정 연산이 바꿀 칸만 담는 타입 자리. `None`은 그 칸을 그대로 둔다는 뜻.

/// 모델 수정 연산이 바꿀 칸의 묶음. `None`이면 그 칸은 건드리지 않는다.
#[derive(Debug, Default)]
pub struct ModelChanges {
    /// 서버에 보낼 모델 식별 값.
    pub id: Option<String>,
    /// 맥락 토큰 한도. 값 검증은 저장 직전 `check_config`가 맡는다.
    pub context_window: Option<u64>,
    /// 생각 강도 값. 값 검증은 저장 직전 `check_config`가 맡는다.
    pub reasoning: Option<String>,
}
