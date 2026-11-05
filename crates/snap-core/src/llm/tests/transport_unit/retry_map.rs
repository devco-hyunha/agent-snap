//! 전송 오류·설정 값을 재시도 판단 입력으로 옮기는 매핑 표 단위 테스트(02-6 spec 매핑 표).

use std::time::Duration;

use crate::config::HttpSettings;
use crate::llm::client::{TimeoutSegment, TransportError};
use crate::llm::retry::retry_map::{retry_input, retry_settings};
use crate::llm::retry::{PreHeaderConnectKind, RetryFailureInput, RetrySettings};

/// 연결 실패 6종 전부가 그대로 헤더 전 연결 후보로 가는지 봅니다.
#[test]
fn connect_failed_maps_to_pre_header_connect() {
    for kind in [
        PreHeaderConnectKind::DnsFailure,
        PreHeaderConnectKind::ConnectionRefused,
        PreHeaderConnectKind::ConnectTimeout,
        PreHeaderConnectKind::ResetBeforeHeaders,
        PreHeaderConnectKind::Tls,
        PreHeaderConnectKind::Proxy,
    ] {
        let error = TransportError::ConnectFailed {
            kind,
            connect_seconds: 10,
        };
        assert_eq!(
            retry_input(&error, false),
            Some(RetryFailureInput::PreHeaderConnect(kind))
        );
    }
}

/// 헤더 대기 초과는 비후보 입력으로 가고, 스트림 유휴 초과는 입력이 없습니다.
#[test]
fn timeout_maps_by_segment() {
    let header_wait = TransportError::Timeout {
        segment: TimeoutSegment::HeaderWait,
        seconds: 300,
    };
    assert_eq!(
        retry_input(&header_wait, false),
        Some(RetryFailureInput::HeaderWaitTimeout)
    );

    let stream_idle = TransportError::Timeout {
        segment: TimeoutSegment::StreamIdle,
        seconds: 120,
    };
    assert_eq!(retry_input(&stream_idle, false), None);
}

/// 서버 상태 응답은 상태 코드와 헤더 유무(전송이 알려 준 값)만 옮깁니다.
#[test]
fn server_status_maps_to_http_status_with_header_flag() {
    let error = TransportError::ServerStatus {
        status: 429,
        body_summary: "가린 본문 요약".to_owned(),
        give_up: None,
    };
    assert_eq!(
        retry_input(&error, true),
        Some(RetryFailureInput::HttpStatus {
            status: 429,
            has_retry_after: true
        })
    );
    assert_eq!(
        retry_input(&error, false),
        Some(RetryFailureInput::HttpStatus {
            status: 429,
            has_retry_after: false
        })
    );
}

/// 취소·스트림 중 실패·클라이언트 제작 실패·본문 읽기 실패는 입력이 없습니다.
#[test]
fn non_retry_failures_have_no_input() {
    let stream_error = TransportError::Streaming(crate::llm::StreamParseError::Format {
        event_index: 1,
        kind: crate::llm::FormatErrorKind::InvalidJson,
    });
    for error in [
        TransportError::ClientBuild,
        TransportError::Cancelled,
        stream_error,
        TransportError::BodyRead,
    ] {
        assert_eq!(retry_input(&error, false), None);
    }
}

/// `http` 설정의 재시도 값 4개만 판단 설정으로 1대1로 옮기는지 봅니다.
#[test]
fn http_settings_maps_to_retry_settings() {
    let http = HttpSettings {
        retries: 2,
        backoff_min: Duration::from_millis(50),
        backoff_max: Duration::from_secs(3),
        retry_after_max: Duration::from_secs(9),
        ..HttpSettings::default()
    };
    assert_eq!(
        retry_settings(&http),
        RetrySettings {
            retries: 2,
            backoff_min: Duration::from_millis(50),
            backoff_max: Duration::from_secs(3),
            retry_after_max: Duration::from_secs(9),
        }
    );
}
