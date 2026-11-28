//! 상용 난수원 산술 계산 자리. 다음 상태·허용 개수·나초 시드만 담음.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::consts::rng::MIX_MULTIPLIER;
use crate::consts::rng::MIX_SHIFT_FIRST;
use crate::consts::rng::MIX_SHIFT_LAST;
use crate::consts::rng::MIX_SHIFT_SECOND;
use crate::consts::rng::SEED_GAMMA;

/// 상태를 산술(승수·가감·시프트)로 섞어 다음 상태로 옮김.
pub(super) fn next_state(state: u64) -> u64 {
    let mixed = state.wrapping_add(SEED_GAMMA).wrapping_mul(MIX_MULTIPLIER);
    mixed ^ (mixed >> MIX_SHIFT_FIRST) ^ (mixed >> MIX_SHIFT_SECOND) ^ (mixed >> MIX_SHIFT_LAST)
}

/// 상한 `width`개 구간을 균등하게 나누는 값의 개수.
///
/// `u128` 범위를 `width`로 나눈 나머지를 빼서 버림 구간을 없앰.
pub(super) fn accepted_count(width: u64) -> u128 {
    let range = u128::from(u64::MAX) + 1;
    range - (range % u128::from(width))
}

/// 1970-01-01부터의 나초. 그 이전 시각은 0으로 봄.
pub(super) fn epoch_nanos(now: SystemTime) -> u64 {
    let Ok(since_epoch) = now.duration_since(UNIX_EPOCH) else {
        return 0;
    };
    u64::try_from(since_epoch.as_nanos()).unwrap_or(u64::MAX)
}
