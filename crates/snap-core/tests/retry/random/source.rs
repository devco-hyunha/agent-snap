//! 상용 난수원 경계 시험. 같은 시드 재현과 상한 초과 금지만 봄.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use snap_core::llm::RandomSource;
use snap_core::llm::SystemRandomSource;

/// 기준 시각. 1970-01-01 00:00:00 UTC.
const NOW: SystemTime = UNIX_EPOCH;

/// 뽑는 값 개수.
const DRAWS: usize = 200;

/// 작은 상한.
const SMALL_MAX: u64 = 9;

/// 같은 시각에서 만든 난수원은 같은 값 순서로 뽑음.
#[test]
fn same_seed_repeats_the_same_sequence() {
    let mut first = SystemRandomSource::with_time(NOW);
    let mut second = SystemRandomSource::with_time(NOW);

    for max in [7_u64, 1_000, 1_000_000] {
        for _ in 0..DRAWS {
            assert_eq!(
                first.next_inclusive(max),
                second.next_inclusive(max),
                "same seed gives the same pick"
            );
        }
    }
}

/// 상한이 0이면 항상 0.
#[test]
fn zero_max_returns_zero() {
    let mut rng = SystemRandomSource::with_time(NOW);
    for _ in 0..DRAWS {
        assert_eq!(rng.next_inclusive(0), 0, "zero max returns zero");
    }
}

/// 상한이 `u64`의 최대로 가장 넓 때를 빼고 모두 `max` 이하.
#[test]
fn picks_never_exceed_max() {
    let mut rng = SystemRandomSource::with_time(NOW);
    for max in [1_u64, 2, 9, 255, 1_000_000, u64::MAX - 1] {
        for _ in 0..DRAWS {
            let pick = rng.next_inclusive(max);
            assert!(pick <= max, "pick {pick} over max {max}");
        }
    }
}

/// 작은 상한을 여러 번 뽑으면 값이 골라짐.
#[test]
fn small_max_draws_more_than_one_value() {
    let mut rng = SystemRandomSource::with_time(NOW);
    let mut seen: Vec<u64> = Vec::new();
    for _ in 0..DRAWS {
        let pick = rng.next_inclusive(SMALL_MAX);
        assert!(pick <= SMALL_MAX, "pick {pick} over {SMALL_MAX}");
        if !seen.contains(&pick) {
            seen.push(pick);
        }
    }
    assert!(
        seen.len() > 1,
        "small max draws more than one value: {seen:?}"
    );
}
