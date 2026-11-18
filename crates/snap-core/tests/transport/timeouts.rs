//! 헤더 대기·스트림 유휴 시간 초과 통합 테스트.

use std::time::{Duration, Instant};

use super::{client_at_with, drain, test_http};
use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use crate::common::support::{content_events, delta_line, sample_request, success_stream_body};
use snap_core::llm::{CancellationToken, LlmClient, TimeoutSegment, TransportError};

// 헤더 지연을 유휴 설정보다 길게 두고 헤더 대기 안에 받아 성공을 확인하는 시험.
// 설정 값끼리 길이가 겹치지 않는 관계를 보는 제약
#[tokio::test]
async fn header_later_than_idle_setting_succeeds() {
    let server = MockServer::spawn(MockResponseScript {
        header_delay: Duration::from_millis(400),
        body_parts: vec![MockBodyPart::immediate(success_stream_body(&["ok"]))],
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

// 헤더가 끝내 안 오면 헤더 대기 시간 초과로 끝남.
// 오류 문구에 서버 주소가 드러나지 않는 것도 함께 보는 제약
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

// 헤더 이후 본문이 멈추면 스트림 유휴 시간 초과로 끝남을 확인하는 시험.
// 초과 시각과 경과 시간 상한을 함께 재는 제약
#[tokio::test]
async fn body_stall_is_stream_idle_segment() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(delta_line("첫").into_bytes()),
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
