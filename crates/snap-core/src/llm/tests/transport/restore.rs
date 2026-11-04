//! 응답 스트림 복원·디코더 계약 통합 테스트(AC 1, D28).

use std::time::Duration;

use super::super::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use super::super::support::sample_request;
use super::super::support::{body_parts_sse, content_events, data_line, mixed_stream_bytes};
use super::{client_at_with, drain, test_http};
use crate::llm::{CancellationToken, LlmClient, StreamEvent};

// AC 1: 02-3 AC 1과 같은 형식을 모의 서버가 조각으로 나눠 보내면 같은 결과로 복원된다.
#[tokio::test]
async fn streams_restored_events_from_mock_sse() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(body_parts_sse(
        &mixed_stream_bytes(),
    )))
    .await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");

    let events = drain(&mut stream).await;
    assert_eq!(content_events(&events), "안녕");
    assert!(events.contains(&StreamEvent::Done));
    let outcome = stream.outcome().expect("정상 종료 사용량·시각");
    assert_eq!(
        outcome.usage.as_ref().expect("사용량").total_tokens,
        Some(8)
    );
    // 서버가 연결을 닫은 뒤 종료하므로 연결이 남지 않습니다.
    assert!(server.wait_until_idle(Duration::from_secs(3)).await);
}

// AC 1(D28): 스트림이 한 번 실패한 뒤 성공으로 보일 `Ok`를 내지 않는다.
#[tokio::test]
async fn transport_emits_no_success_after_first_decoder_error() {
    let bytes = mixed_stream_bytes();
    let mut script = MockResponseScript::with_body_parts(Vec::new());
    // 앞의 정상 조각, 그 뒤 형식 오류를 만드는 조각, 마지막으로 정상 데이터를 둡니다.
    let mut parts = body_parts_sse(&bytes[..bytes.len() - 4]);
    parts.push(MockBodyPart::immediate(b"data: {broken\n\n".to_vec()));
    parts.push(MockBodyPart::immediate(
        data_line(r#"{"choices":[{"delta":{"content":"뒤"}}]}"#)
            .as_bytes()
            .to_vec(),
    ));
    parts.push(MockBodyPart::immediate(b"data: [DONE]\n\n".to_vec()));
    script.body_parts = parts;

    let server = MockServer::spawn(script).await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");

    let mut saw_error = false;
    while let Some(event) = stream.next_event().await {
        if event.is_err() {
            saw_error = true;
            break;
        }
    }
    assert!(saw_error, "첫 실패를 맞아야 합니다");
    assert!(
        stream.next_event().await.is_none(),
        "오류 뒤에 이벤트가 더 있으면 안 됩니다"
    );
    assert!(stream.outcome().is_none(), "오류 뒤 정상 종료 없음");
}
