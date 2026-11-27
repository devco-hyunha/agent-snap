//! T3 시험: 전송 오류 타입 책임. 담는 값과 Display에 키·호스트가 없는지 봄.
use snap_core::llm::ChunkErrorKind;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RedactedSummary;
use snap_core::llm::SseError;
use snap_core::llm::TransportError;

const HOST: &str = "secret-host.invalid";
const KEY: &str = "sk-secretvalue123";

/// 시험용 가린 요약.
fn summary() -> RedactedSummary {
    RedactedSummary::new("서버이 오류입니다".to_owned())
}

/// 헤더 전 연결 종류 6종.
fn connect_kinds() -> [PreHeaderConnectKind; 6] {
    [
        PreHeaderConnectKind::Dns,
        PreHeaderConnectKind::Refused,
        PreHeaderConnectKind::ConnectTimeout,
        PreHeaderConnectKind::Reset,
        PreHeaderConnectKind::Tls,
        PreHeaderConnectKind::Proxy,
    ]
}

#[test]
fn connect_error_carries_kind_and_seconds() {
    let error = TransportError::connect(PreHeaderConnectKind::Refused, 10.0);

    assert!(
        matches!(&error, TransportError::Connect { kind, connect_seconds } if
            *kind == PreHeaderConnectKind::Refused && *connect_seconds == 10.0),
        "connect error carries kind and seconds"
    );
}

#[test]
fn every_error_text_hides_key_and_host() {
    let errors = all_errors();
    for error in errors {
        let display = error.to_string();
        let debug = format!("{error:?}");
        for secret in [KEY, HOST] {
            assert!(
                !display.contains(secret) && !debug.contains(secret),
                "error text hides secret: {display}"
            );
        }
    }
}

#[test]
fn connect_variants_all_print() {
    for kind in connect_kinds() {
        let error = TransportError::connect(kind, 10.0);
        let display = error.to_string();

        assert!(
            !display.contains("base_url") && !display.contains(HOST),
            "connect display names kind only: {display}"
        );
    }
}

#[test]
fn status_error_carries_code_and_redacted_summary() {
    let error = TransportError::status(429, summary());

    let TransportError::Status { status, summary } = &error else {
        panic!("test expects status error");
    };

    assert_eq!(*status, 429, "status carries code");
    assert_eq!(summary.to_string(), "서버이 오류입니다", "carries summary");
    assert!(
        format!("{error:?}").contains("RedactedSummary"),
        "debug uses redacted wrapper label"
    );
}

#[test]
fn timeout_errors_carry_seconds() {
    let header = TransportError::header_wait(300.0);
    let idle = TransportError::idle(120.0);

    assert!(
        matches!(header, TransportError::HeaderWait { seconds } if seconds == 300.0),
        "header wait carries seconds"
    );
    assert!(
        matches!(idle, TransportError::Idle { seconds } if seconds == 120.0),
        "idle carries seconds"
    );
}

#[test]
fn stream_error_wraps_sse_error() {
    let inner = SseError::format(2, ChunkErrorKind::Syntax);
    let error = TransportError::stream(inner);

    assert!(
        matches!(&error, TransportError::Stream(SseError::Format { index, .. }) if *index == 2),
        "stream error keeps sse error position"
    );
}

#[test]
fn cancelled_error_prints() {
    let error = TransportError::cancelled();

    assert!(!error.to_string().is_empty(), "cancel error has text");
}

/// 시험용 전송 오류 목록.
fn all_errors() -> Vec<TransportError> {
    vec![
        TransportError::connect(PreHeaderConnectKind::Dns, 10.0),
        TransportError::status(503, summary()),
        TransportError::header_wait(300.0),
        TransportError::idle(120.0),
        TransportError::stream(SseError::stream(summary())),
        TransportError::cancelled(),
    ]
}
