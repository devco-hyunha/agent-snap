//! 사용량·원시 시각 반환 통합 테스트(AC 5).

use super::{client_at_with, drain, test_http};
use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use crate::common::support::{
    data_line, delta_line, done_line, sample_request, success_stream_body,
};
use snap_core::llm::CancellationToken;
use snap_core::llm::LlmClient;

// AC 5: 서버가 사용량을 주면 그 값이고, 시각은 요청 시작·첫 토큰·마지막 토큰 순서다.
#[tokio::test]
async fn outcome_carries_usage_and_raw_timings() {
    // 내용 줄 두 줄 뒤 사용량 줄과 `[DONE]`을 붙인다.
    let mut bytes = Vec::new();
    bytes.extend_from_slice(delta_line("가").as_bytes());
    bytes.extend_from_slice(delta_line("김").as_bytes());
    bytes.extend_from_slice(
        data_line(
            r#"{"choices":[{"delta":{}}],"usage":{"prompt_tokens":2,"completion_tokens":7,"total_tokens":9}}"#,
        )
        .as_bytes(),
    );
    bytes.extend_from_slice(done_line().as_bytes());

    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(bytes),
    ]))
    .await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");
    drain(&mut stream).await;

    let outcome = stream.outcome().expect("종료 후 사용량·시각");
    let usage = outcome.usage.as_ref().expect("사용량 값");
    assert_eq!(usage.completion_tokens, Some(7));
    assert_eq!(usage.total_tokens, Some(9));
    let first = outcome.timing.first_token.expect("첫 토큰 시각");
    let last = outcome.timing.last_token.expect("마지막 토큰 시각");
    assert!(outcome.timing.request_started <= first);
    assert!(first <= last);
}

// AC 5: 서버가 사용량을 안 주면 "없음"이고, 시각 순서는 그대로 남는다.
#[tokio::test]
async fn outcome_usage_none_when_server_omits_it() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(success_stream_body(&["없음"])),
    ]))
    .await;
    let mut stream = client_at_with(server.base_url(), None, test_http())
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");
    drain(&mut stream).await;
    let outcome = stream.outcome().expect("종료 후 시각");
    assert!(outcome.usage.is_none(), "없음이어야 합니다");
    assert!(outcome.timing.first_token.is_some());
    assert!(outcome.timing.last_token.is_some());
}
