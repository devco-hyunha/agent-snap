//! 02-6 AC 5 통합 시험: 스트림 시작 후 끊김 — 요청 1회로 스트림 오류, 재시도 없음.
//!
//! 모의 서버가 200과 본문 일부(SSE 내용 한 줄)를 보내고 본문을 마무리 없이 연결을 끊는다.
//! 받은 내용은 끊김 지점까지 결과에 남고 스트림은 끝난다.

use super::super::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use super::super::common::support::{content_events, delta_line, sample_request};
use super::{client_at_with, retry_status_http, set_unit_sample};
use snap_core::llm::{CancellationToken, LlmClient, TransportError};

// AC 5: 스트림 시작 후 끊김 — 요청 1회로 스트림 오류(재시도 없음), 받은 내용은 끊김 지점까지 유지.
// 재시도 3회 값으로 켜도 스트림 시작 후 끊김은 재시도 밖이므로 첫 요청 하나로 끝나야 합니다.
#[tokio::test]
async fn mid_stream_disconnect_keeps_received_content_and_ends() {
    // 본문은 내용 한 줄까지 보내고 [DONE] 없이 끊습니다. 끊김 표식이 마지막 조각입니다.
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(delta_line("끊김전").as_bytes().to_vec()),
        MockBodyPart::disconnect(),
    ]))
    .await;
    let client = set_unit_sample(
        client_at_with(server.base_url(), None, retry_status_http(3)),
        1.0,
    );
    let mut stream = client
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");

    // 끊김 지점까지 받은 내용이 이벤트로 유지됩니다.
    let first = stream.next_event().await.expect("이벤트").expect("내용");
    assert_eq!(content_events(&[first]), "끊김전");

    // 끊김 지점에서 스트림 오류로 끝납니다. 새 오류 종류가 아니라 02-5에 있던 본문 읽기 끊김입니다.
    let error = stream
        .next_event()
        .await
        .expect("스트림 오류")
        .expect_err("끊김은 오류여야 합니다");
    assert!(
        matches!(error, TransportError::BodyRead),
        "본문 읽기 끊김 오류여야 합니다: {error:?}"
    );
    // 오류 직후 스트림이 열려 있지 않고 끝난 자리입니다.
    assert!(stream.next_event().await.is_none());

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        1,
        "스트림 시작 후 끊김은 재시도하지 않고 요청 1회로 끝나야 합니다"
    );
}
