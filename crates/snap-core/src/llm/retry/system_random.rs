//! 상용 난수원 자리. `SystemTime` 나초를 시드로 쓰고 새 난수 크레이트 의존은 없음.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::consts::rng::MIX_MULTIPLIER;
use crate::consts::rng::MIX_SHIFT_FIRST;
use crate::consts::rng::MIX_SHIFT_LAST;
use crate::consts::rng::MIX_SHIFT_SECOND;
use crate::consts::rng::SEED_GAMMA;
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

/// 상태를 산술(승수·가감·시프트)로 섞어 다음 상태로 옮김.
fn next_state(state: u64) -> u64 {
    let mixed = state.wrapping_add(SEED_GAMMA).wrapping_mul(MIX_MULTIPLIER);
    mixed ^ (mixed >> MIX_SHIFT_FIRST) ^ (mixed >> MIX_SHIFT_SECOND) ^ (mixed >> MIX_SHIFT_LAST)
}

/// 상한 `width`개 구간을 균등하게 나누는 값의 개수.
///
/// `u128` 범위를 `width`로 나눈 나머지를 빼서 버림 구간을 없앰.
fn accepted_count(width: u64) -> u128 {
    let range = u128::from(u64::MAX) + 1;
    range - (range % u128::from(width))
}

/// 1970-01-01부터의 나초. 그 이전 시각은 0으로 봄.
fn epoch_nanos(now: SystemTime) -> u64 {
    let Ok(since_epoch) = now.duration_since(UNIX_EPOCH) else {
        return 0;
    };
    u64::try_from(since_epoch.as_nanos()).unwrap_or(u64::MAX)
}
