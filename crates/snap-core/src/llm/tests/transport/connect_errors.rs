//! 헤더 전 연결 오류 세부 구분·오류 가리기 통합 테스트(AC 2·6).

use super::super::support::sample_request;
use super::{client_at_with, connect_check_http, test_http};
use crate::llm::{
    CancellationToken, LlmClient, PreHeaderConnectKind, RetryFailureInput, TransportError,
};

// AC 6: 연결이 거부되면 헤더 전 연결 세부 `ConnectionRefused`로 구분된다.
#[tokio::test]
async fn connection_refused_is_pre_header_connect_kind() {
    // 닫힌 포트에 연결해 거부를 만듭니다.
    let closed = tokio::net::TcpListener::bind(std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
        .await
        .expect("빈 포트 확인용 소켓");
    let addr = closed.local_addr().expect("확인용 주소");
    drop(closed);

    let error = client_at_with(
        &format!("http://{addr}"),
        Some("sk-refused-key-1234567890abcd"),
        connect_check_http(),
    )
    .stream_chat(sample_request(None), CancellationToken::new())
    .await
    .expect_err("연결 거부");
    assert!(
        matches!(
            &error,
            TransportError::ConnectFailed {
                kind: PreHeaderConnectKind::ConnectionRefused,
                connect_seconds: 5,
            }
        ),
        "연결 거부 세부여야 합니다: {error:?}"
    );
    // AC 6: Display·Debug에 키와 base_url 호스트가 없다.
    let key = "sk-refused-key-1234567890abcd";
    assert!(!format!("{error}").contains("127.0.0.1"));
    assert!(!format!("{error:?}").contains("127.0.0.1"));
    assert!(!format!("{error}").contains(key));
    assert!(!format!("{error:?}").contains(key));
}

// AC 6: DNS 조회 실패는 헤더 전 연결 세부 `DnsFailure`로 구분된다.
#[tokio::test]
async fn dns_failure_is_pre_header_connect_kind() {
    let error = client_at_with(
        "http://agent-snap-does-not-exist.invalid/",
        None,
        test_http(),
    )
    .stream_chat(sample_request(None), CancellationToken::new())
    .await
    .expect_err("DNS 실패");
    assert!(
        matches!(
            &error,
            TransportError::ConnectFailed {
                kind: PreHeaderConnectKind::DnsFailure,
                ..
            }
        ),
        "DNS 실패 세부여야 합니다: {error:?}"
    );
    // AC 6: Display·Debug에 base_url 호스트가 없다.
    assert!(!format!("{error}").contains("agent-snap-does-not-exist"));
    assert!(!format!("{error:?}").contains("agent-snap-does-not-exist"));
}

// AC 6: 분류가 내보내는 세부 6종이 02-4 표의 「헤더 전 연결」 후보와 같은 값으로 채워진다.
#[test]
fn connect_kinds_from_classification_match_retry_input_names() {
    let kinds = crate::llm::transport::connect_kinds_by_row();
    assert_eq!(
        kinds,
        [
            PreHeaderConnectKind::DnsFailure,
            PreHeaderConnectKind::ConnectionRefused,
            PreHeaderConnectKind::ConnectTimeout,
            PreHeaderConnectKind::ResetBeforeHeaders,
            PreHeaderConnectKind::Tls,
            PreHeaderConnectKind::Proxy,
        ]
    );
    assert_eq!(kinds.len(), 6);
    for kind in kinds {
        assert!(matches!(
            RetryFailureInput::PreHeaderConnect(kind),
            RetryFailureInput::PreHeaderConnect(_)
        ));
    }
}
