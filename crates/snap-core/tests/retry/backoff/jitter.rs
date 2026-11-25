//! Full Jitter 대기 시험. spec AC 3의 범위·0 하한을 난수를 고정해 확인함.

use std::time::Duration;

use snap_core::llm::RandomSource;
use snap_core::llm::backoff_cap;
use snap_core::llm::full_jitter_wait;

use crate::fixtures::FixedSource;
use crate::fixtures::MaxSource;
use crate::fixtures::SETTINGS;
use crate::fixtures::ZeroSource;

/// 공급원이 받은 상한을 나열하는 공급원. 범위 확인용.
struct RangeSource {
    bounds: Vec<u64>,
}

impl RandomSource for RangeSource {
    fn next_inclusive(&mut self, max: u64) -> u64 {
        self.bounds.push(max);
        max
    }
}

/// 시도 1의 상한(1초) 안에서 지정한 나노초를 그대로 대기.
#[test]
fn fixed_pick_is_that_wait() {
    let mut rng = FixedSource { value: 500_000_000 };
    let wait = full_jitter_wait(1, &SETTINGS, &mut rng);
    assert_eq!(wait, Duration::from_millis(500));
}

/// 0을 뽑으면 대기 0초.
#[test]
fn zero_pick_is_zero_wait() {
    let mut rng = ZeroSource;
    let wait = full_jitter_wait(3, &SETTINGS, &mut rng);
    assert_eq!(wait, Duration::ZERO);
}

/// 상한과 같은 값을 뽑으면 대기 상한과 같음.
#[test]
fn cap_pick_is_cap_wait() {
    let mut rng = MaxSource;
    let wait = full_jitter_wait(2, &SETTINGS, &mut rng);
    assert_eq!(wait, backoff_cap(2, &SETTINGS));
}

/// 시도마다 공급원에 넘어가는 상한이 백오프 상한과 같음.
#[test]
fn range_upper_bound_is_backoff_cap() {
    let mut rng = RangeSource { bounds: Vec::new() };
    for attempt in 1_u64..=8 {
        full_jitter_wait(attempt, &SETTINGS, &mut rng);
    }
    let expected: Vec<u64> = (1_u64..=8)
        .map(|attempt| {
            backoff_cap(attempt, &SETTINGS)
                .as_nanos()
                .try_into()
                .unwrap()
        })
        .collect();
    assert_eq!(rng.bounds, expected);
}
