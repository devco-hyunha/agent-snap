//! 02-6 재시도 고리 시험: 판단 반복 결과가 재시도·포기를 고르고, 마지막 오류에 포기 이유가 남음을 본다.
//!
//! 상한 이하 `Retry-After` 헤더는 대기로 옮기고, 상한 초과는 헤더 초를 밝힌 포기이고,
//! 후보가 아닌 4xx와 상한을 넘긴 `Retry-After`는 대기로 옮기지 않고 포기한다.

use std::time::{Duration, SystemTime};

use crate::llm::client::{TimeoutSegment, TransportError};
use crate::llm::retry::retry_loop::{LoopOutcome, judge_once};
use crate::llm::retry::{
    NotCandidateDetail, PreHeaderConnectKind, RetryGiveUpReason, RetrySettings,
};

fn settings() -> RetrySettings {
    RetrySettings {
        retries: 3,
        backoff_min: Duration::from_secs(1),
        backoff_max: Duration::from_secs(30),
        retry_after_max: Duration::from_secs(60),
    }
}

fn server_status(status: u16) -> TransportError {
    TransportError::ServerStatus {
        status,
        body_summary: String::new(),
        give_up: None,
    }
}

/// AC 1 줄거리: 상한 이하 2초짜리 `Retry-After` 헤더가 붙은 429는 2초 대기 권고가 된다.
#[test]
fn judge_once_recommends_retry_after_within_cap() {
    let outcome = judge_once(
        server_status(429),
        Some("2"),
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::Retry { wait } => assert_eq!(wait, Duration::from_secs(2)),
        other => panic!("대기 권고여야 합니다: {other:?}"),
    }
}

/// AC 2 앞부분: 헤더 없는 503은 첫 시도에서 백오프 대기 권고이고, 마지막 오류의 포기 이유는 없다.
#[test]
fn judge_once_backs_off_for_plain_503() {
    let outcome = judge_once(
        server_status(503),
        None,
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::Retry { wait } => assert_eq!(wait, Duration::from_secs(1)),
        other => panic!("대기 권고여야 합니다: {other:?}"),
    }
}

/// AC 3 앞부분: 상한(60초)을 넘는 90초 헤더는 헤더 초를 밝힌 포기이다.
#[test]
fn judge_once_gives_up_when_retry_after_exceeds_cap() {
    let outcome = judge_once(
        server_status(429),
        Some("90"),
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::GiveUp(TransportError::ServerStatus {
            status: 429,
            give_up,
            ..
        }) => {
            assert_eq!(
                give_up,
                Some(RetryGiveUpReason::RetryAfterExceedsCap {
                    wait: Duration::from_secs(90),
                })
            );
        }
        other => panic!("포기 봉지여야 합니다: {other:?}"),
    }
}

/// AC 3 앞부분: 후보가 아닌 400은 마지막 오류로 끝나고, 포기가 대상 아님이 남는다.
#[test]
fn judge_once_gives_up_on_non_candidate_4xx() {
    let outcome = judge_once(
        server_status(400),
        None,
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::GiveUp(TransportError::ServerStatus {
            status: 400,
            give_up,
            ..
        }) => {
            assert_eq!(
                give_up,
                Some(RetryGiveUpReason::NotCandidate(
                    NotCandidateDetail::HttpStatus { status: 400 },
                ))
            );
        }
        other => panic!("포기 봉지여야 합니다: {other:?}"),
    }
}

/// AC 2 끝부분: 마지막 실패의 시도 번호가 재시도 값 3을 넘으면 시도 횟수 넘김 포기이다.
#[test]
fn judge_once_gives_up_after_attempt_limit() {
    let outcome = judge_once(
        server_status(503),
        None,
        6,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::GiveUp(TransportError::ServerStatus { give_up, .. }) => {
            assert_eq!(
                give_up,
                Some(RetryGiveUpReason::AttemptsExceeded { attempt: 6 })
            );
        }
        other => panic!("포기 봉지여야 합니다: {other:?}"),
    }
}

/// AC 4 줄거리: 헤더 대기 초과는 재시도 대상이 아니라 대상 아님 포기로 넘긴다.
#[test]
fn judge_once_gives_up_header_wait_timeout() {
    let outcome = judge_once(
        TransportError::Timeout {
            segment: TimeoutSegment::HeaderWait,
            seconds: 30,
        },
        None,
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    assert!(matches!(
        outcome,
        LoopOutcome::GiveUp(TransportError::Timeout {
            segment: TimeoutSegment::HeaderWait,
            ..
        })
    ));
}

/// 헤더 전 연결 실패는 02-4 후보라 재시도 권고로 나온다.
#[test]
fn judge_once_retries_pre_header_connect_failure() {
    let outcome = judge_once(
        TransportError::ConnectFailed {
            kind: PreHeaderConnectKind::ConnectionRefused,
            connect_seconds: 5,
        },
        None,
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    match outcome {
        LoopOutcome::Retry { wait } => assert_eq!(wait, Duration::from_secs(1)),
        other => panic!("연결 후보는 재시도 권고여야 합니다: {other:?}"),
    }
}

/// AC 4 앞부분: 취소로 끝난 스트림의 오류는 판단 없이 마지막 오류로만 넘긴다.
#[test]
fn judge_once_passes_cancelled_to_caller() {
    let outcome = judge_once(
        TransportError::Cancelled,
        None,
        1,
        settings(),
        SystemTime::now(),
        Some(1.0),
    );
    assert!(matches!(
        outcome,
        LoopOutcome::GiveUp(TransportError::Cancelled)
    ));
}
