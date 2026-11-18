//! 스트림 도중 취소와 재시도 대기 중 취소, 서버 연결 정리 통합 테스트.

use std::time::{Duration, Instant};

use axum::http::StatusCode;

use super::{client_at_with, retry_status_http, test_http};
use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use crate::common::support::{content_events, delta_line, sample_request};
use snap_core::llm::{CancellationToken, LlmClient, TransportError};

// 취소 시 스트림이 끝나는 지점까지 취소 오류 하나만 나는지 확인하는 시험.
#[tokio::test]
async fn cancel_mid_stream_reports_cancelled_quickly() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(delta_line("첫").into_bytes()),
        MockBodyPart::after(Duration::from_secs(30), b"never".to_vec()),
    ]))
    .await;
    let cancel = CancellationToken::new();
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), cancel.clone())
        .await
        .expect("열려야 합니다");
    let first = stream.next_event().await.expect("이벤트").expect("내용");
    assert_eq!(content_events(&[first]), "첫");

    cancel.cancel();
    let started = Instant::now();
    let error = stream
        .next_event()
        .await
        .expect("취소 오류")
        .expect_err("취소");
    assert!(started.elapsed() < Duration::from_millis(500));
    assert!(matches!(error, TransportError::Cancelled));
    assert!(stream.next_event().await.is_none());
    assert!(server.wait_until_idle(Duration::from_secs(3)).await);
}

// 429가 준 2초짜리 `Retry-After` 대기를 다 채우기 전에 취소하는 시험.
// 대기는 tokio::select!로 취소와 경합해 취소 시점에 바로 끝남
#[tokio::test]
async fn cancel_during_retry_wait_ends_immediately() {
    let server = MockServer::spawn(MockResponseScript::status_with_retry_after(
        StatusCode::TOO_MANY_REQUESTS,
        "2",
    ))
    .await;
    let cancel = CancellationToken::new();

    // 취소는 250밀리초 뒤로 미룸. 그 사이에 첫 요청 헤더가 와 대기가 시작됨
    let started = Instant::now();
    let later = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(250)).await;
        later.cancel();
    });

    let error = client_at_with(server.base_url(), None, retry_status_http(3))
        .stream_chat(sample_request(None), cancel)
        .await
        .expect_err("재시도 대기는 취소로 끝나야 합니다");
    let elapsed = started.elapsed();

    assert!(
        matches!(error, TransportError::Cancelled),
        "취소 오류여야 합니다: {error:?}"
    );
    assert!(
        elapsed < Duration::from_millis(500),
        "취소가 재시도 대기를 방치하고 끝났습니다: {elapsed:?}"
    );

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        1,
        "취소로 끝나서 재시도 요청이 나면 안 됩니다"
    );
}
