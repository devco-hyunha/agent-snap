//! 헤더 전 연결 오류 세부 구분·오류 가리기 통합 테스트(AC 2·6).

use super::super::common::support::sample_request;
use super::{client_at_with, connect_check_http, test_http};
use snap_core::config::HttpSettings;
use snap_core::llm::{
    CancellationToken, LlmClient, PreHeaderConnectKind, RetryFailureInput, TransportError,
};
// 분류 코드가 크레이트 안에서 내보내는 같은 이름의 세부 타입을 시험 안쪽에서 가리키는 이름입니다.
use crate::llm::PreHeaderConnectKind as 내부_연결_세부;

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
    // 연결 세부 확인이라 재시도는 켜지 않는다(연결 재시도는 02-4 판단 시험이 별도로 본다).
    let http = HttpSettings {
        retries: 0,
        ..test_http()
    };
    let error = client_at_with("http://agent-snap-does-not-exist.invalid/", None, http)
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
            내부_연결_세부::DnsFailure,
            내부_연결_세부::ConnectionRefused,
            내부_연결_세부::ConnectTimeout,
            내부_연결_세부::ResetBeforeHeaders,
            내부_연결_세부::Tls,
            내부_연결_세부::Proxy,
        ]
    );
    assert_eq!(kinds.len(), 6);
    for kind in kinds {
        // 분류 코드가 준 값이 표에서 쓰는 「헤더 전 연결」 세부와 같은 값으로 옮아감 여부를 본다.
        // 내부 세부는 공개_세부와 같은 값이므로 공개_세부로 옮겨 확인한다.
        let 공개_세부 = match kind {
            내부_연결_세부::DnsFailure => PreHeaderConnectKind::DnsFailure,
            내부_연결_세부::ConnectionRefused => PreHeaderConnectKind::ConnectionRefused,
            내부_연결_세부::ConnectTimeout => PreHeaderConnectKind::ConnectTimeout,
            내부_연결_세부::ResetBeforeHeaders => PreHeaderConnectKind::ResetBeforeHeaders,
            내부_연결_세부::Tls => PreHeaderConnectKind::Tls,
            내부_연결_세부::Proxy => PreHeaderConnectKind::Proxy,
        };
        assert!(matches!(
            RetryFailureInput::PreHeaderConnect(공개_세부),
            RetryFailureInput::PreHeaderConnect(_)
        ));
    }
}
