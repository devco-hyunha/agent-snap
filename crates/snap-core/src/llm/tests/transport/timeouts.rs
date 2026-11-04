//! 헤더 대기·스트림 유휴 시간 초과 통합 테스트(AC 3).

use std::time::{Duration, Instant};

use super::super::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use super::super::support::{content_events, data_line, sample_request};
use super::{client_at_with, drain, test_http};
use crate::llm::{CancellationToken, LlmClient, TimeoutSegment, TransportError};

// AC 3: 헤더가 유휴 설정보다 늦게 와도 헤더 대기 안에 오면 성공한다.
#[tokio::test]
async fn header_later_than_idle_setting_succeeds() {
    let server = MockServer::spawn(MockResponseScript {
        header_delay: Duration::from_millis(400),
        body_parts: vec![
            MockBodyPart::immediate(
                data_line(r#"{"choices":[{"delta":{"content":"ok"}}]}"#)
                    .as_bytes()
                    .to_vec(),
            ),
            MockBodyPart::immediate(b"data: [DONE]\n\n".to_vec()),
        ],
        ..MockResponseScript::empty_ok()
    })
    .await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("헤더 대기는 유휴보다 길어야 합니다");
    let events = drain(&mut stream).await;
    assert_eq!(content_events(&events), "ok");
}

// AC 3: 헤더가 끝내 오지 않으면 헤더 대기 시간 초과로 끝난다.
#[tokio::test]
async fn header_needing_timeout_is_header_wait_segment() {
    let server = MockServer::spawn(MockResponseScript::hang_headers()).await;
    let started = Instant::now();
    let error = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect_err("헤더 미도착은 오류");
    let text = format!("{error}");
    assert!(
        matches!(
            &error,
            TransportError::Timeout {
                segment: TimeoutSegment::HeaderWait,
                seconds: 2,
            }
        ),
        "헤더 대기 초과여야 합니다: {error:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(6));
    assert!(!text.contains("127.0.0.1"));
}

// AC 3: 헤더 이후 본문이 멈추면 유휴 시간 초과로 끝난다.
#[tokio::test]
async fn body_stall_is_stream_idle_segment() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(
            data_line(r#"{"choices":[{"delta":{"content":"첫"}}]}"#)
                .as_bytes()
                .to_vec(),
        ),
        MockBodyPart::after(Duration::from_secs(5), b"last".to_vec()),
    ]))
    .await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");
    let first = stream.next_event().await.expect("이벤트");
    assert_eq!(content_events(&[first.expect("내용")]), "첫");

    let started = Instant::now();
    let error = stream
        .next_event()
        .await
        .expect("초과")
        .expect_err("멈춤은 오류");
    assert!(
        matches!(
            &error,
            TransportError::Timeout {
                segment: TimeoutSegment::StreamIdle,
                seconds: 1,
            }
        ),
        "유휴 초과여야 합니다: {error:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(4));
    assert!(stream.outcome().is_none());
}
