//! `decide_retry`의 지수 대기(Full Jitter) 시험. spec AC 3의 범위·0 하한을 난수 고정으로 확인함.
//!
//! 헤더가 없거나 해석할 수 없으면 헤더 없는 것으로 보고 지수 대기를 탄다.

use std::time::Duration;

use snap_core::llm::FailureInput;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RetryDecision;
use snap_core::llm::backoff_cap;
use snap_core::llm::decide_retry;

use crate::fixtures::FixedSource;
use crate::fixtures::MaxSource;
use crate::fixtures::NOW;
use crate::fixtures::SETTINGS;
use crate::fixtures::ZeroSource;

/// 헤더가 없는 연결 오류는 지수 대기 상한 안에서 뽑은 값만큼 대기.
#[test]
fn connect_error_without_header_uses_jitter_wait() {
    let mut rng = FixedSource { value: 500_000_000 };
    let decision = decide_retry(
        FailureInput::PreHeaderConnect(PreHeaderConnectKind::Refused),
        None,
        1,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::from_millis(500)
        }
    );
}

/// 뽑은 값이 0이면 대기 0초.
#[test]
fn zero_pick_is_wait_zero() {
    let mut rng = ZeroSource;
    let decision = decide_retry(
        FailureInput::PreHeaderConnect(PreHeaderConnectKind::Dns),
        None,
        3,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: Duration::ZERO
        }
    );
}

/// 뽑은 값이 상한과 같으면 대기 상한과 같음.
#[test]
fn cap_pick_is_cap_wait() {
    let mut rng = MaxSource;
    let decision = decide_retry(
        FailureInput::PreHeaderConnect(PreHeaderConnectKind::Tls),
        None,
        2,
        &SETTINGS,
        NOW,
        &mut rng,
    );
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: backoff_cap(2, &SETTINGS)
        }
    );
}

/// 헤더가 없는 재시도 후보 상태 코드는 지수 대기를 씀.
#[test]
fn status_without_header_uses_jitter_wait() {
    let mut rng = MaxSource;
    let decision = decide_retry(FailureInput::Status(503), None, 4, &SETTINGS, NOW, &mut rng);
    assert_eq!(
        decision,
        RetryDecision::Wait {
            wait: backoff_cap(4, &SETTINGS)
        }
    );
}

/// 해석 불가 헤더는 헤더 없는 것으로 보고 지수 대기를 탄다.
#[test]
fn unparseable_header_falls_back_to_backoff() {
    let headers = [Some("-1"), Some("abc"), Some("3.5"), None];
    for header in headers {
        let mut rng = FixedSource { value: 500_000_000 };
        let decision = decide_retry(
            FailureInput::Status(429),
            header,
            1,
            &SETTINGS,
            NOW,
            &mut rng,
        );
        let expected = RetryDecision::Wait {
            wait: Duration::from_millis(500),
        };
        assert_eq!(decision, expected, "헤더 {header:?}");
    }
}

/// 지수 대기는 시도마다 상한을 넘지 않음. 시도 4는 최대 재시도 3회의 마지막 시도.
#[test]
fn jitter_wait_stays_within_cap() {
    for attempt in 1_u64..=4 {
        let mut rng = MaxSource;
        let decision = decide_retry(
            FailureInput::PreHeaderConnect(PreHeaderConnectKind::Reset),
            None,
            attempt,
            &SETTINGS,
            NOW,
            &mut rng,
        );
        let RetryDecision::Wait { wait } = decision else {
            panic!("대기 판단이어야 함: 시도 {attempt}");
        };
        assert!(wait <= backoff_cap(attempt, &SETTINGS), "시도 {attempt}");
    }
}
