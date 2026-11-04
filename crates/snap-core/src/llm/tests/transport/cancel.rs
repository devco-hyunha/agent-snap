//! 스트림 도중 취소·서버 연결 정리 통합 테스트(AC 4).

use std::time::{Duration, Instant};

use super::super::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use super::super::support::{content_events, data_line, sample_request};
use super::{client_at_with, test_http};
use crate::llm::{CancellationToken, LlmClient, TransportError};

// AC 4: 스트림 도중 취소하면 500ms 안에 취소 오류 하나를 내고 스트림이 끝난다.
#[tokio::test]
async fn cancel_mid_stream_reports_cancelled_quickly() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(
            data_line(r#"{"choices":[{"delta":{"content":"첫"}}]}"#)
                .as_bytes()
                .to_vec(),
        ),
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
    // 서버 쪽 연결이 3초 안에 닫혀야 합니다(AC 4).
    assert!(server.wait_until_idle(Duration::from_secs(3)).await);
}
