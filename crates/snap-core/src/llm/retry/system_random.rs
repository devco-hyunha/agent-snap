//! 상용 난수원 타입 자리. `SystemTime` 나초 시드 상태 하나만 담음.
//!
//! 산술 계산은 `mixing`이, 상수 값은 `consts::rng`이 맡음.

use std::time::SystemTime;

use crate::consts::rng::SEED_GAMMA;
use crate::llm::retry::mixing::accepted_count;
use crate::llm::retry::mixing::epoch_nanos;
use crate::llm::retry::mixing::next_state;
use crate::llm::retry::random::RandomSource;

/// `SystemTime` 나초를 시드로 쓰는 상용 난수원.
///
/// 같은 시각에서 만든 난수원은 같은 값 순서로 이어짐.
#[derive(Debug, Clone, Copy)]
pub struct SystemRandomSource {
    /// 다음 값을 뽑는 기준 상태.
    state: u64,
}

impl SystemRandomSource {
    /// 시각 하나에서 초기 상태를 세운 난수원 만들기.
    pub fn with_time(now: SystemTime) -> Self {
        Self {
            state: epoch_nanos(now).wrapping_add(SEED_GAMMA),
        }
    }
}

impl Default for SystemRandomSource {
    fn default() -> Self {
        Self::with_time(SystemTime::now())
    }
}

impl RandomSource for SystemRandomSource {
    /// 0 이상 `max` 이하의 균등 무작위 정수를 하나 뽑음.
    ///
    /// 상한이 `u64`의 최대로 가장 넓을 때만 값 그대로 쓰고, 그 밖의 상한은 모두 `max` 이하 구간으로 좁힘.
    ///
    /// 같은 상한을 같은 상태에서 뽑으면 같은 값이 나옴.
    fn next_inclusive(&mut self, max: u64) -> u64 {
        loop {
            self.state = next_state(self.state);
            let Some(width) = max.checked_add(1) else {
                return self.state;
            };
            if u128::from(self.state) < accepted_count(width) {
                return self.state % width;
            }
        }
    }
}
