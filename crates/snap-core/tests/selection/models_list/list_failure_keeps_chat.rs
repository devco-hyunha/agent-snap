//! 선택 키 클라이언트에서 모델 목록 조회가 실패해도 채팅 요청은 막히지 않음 확인
//!
//! 목록 라우트가 SSE 본문을 받아 형식 오류로 실패한 상태에서도, 같은 선택 키 채팅 전송은 설정 id로 성공

use snap_core::config::Config;
use snap_core::llm::{CancellationToken, LlmClient, TransportError};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{body_parts_sse, content_events, drain, success_stream_body};

use super::support::{FAST_SELECTION_KEY, KEEPING_ID, ac3_credentials, ac3_http_config_text};

/// 목록 조회가 형식 오류로 실패한 서버라도 선택 키 채팅 요청은 설정 id로 성공
#[tokio::test]
async fn list_failure_does_not_block_selected_chat() {
    // 본문은 SSE 줄 하나. 목록 라우트는 형식 오류, 채팅 라우트는 같은 본문을 SSE로 읽음
    let body = success_stream_body(&["chat-ok"]);
    let server =
        MockServer::spawn(MockResponseScript::with_body_parts(body_parts_sse(&body))).await;

    let config = Config::parse(&ac3_http_config_text(server.base_url())).expect("AC 3 설정");
    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");

    // 먼저 목록 조회가 형식 오류로 실패함 확인
    let list_error = client
        .list_models(CancellationToken::new())
        .await
        .expect_err("목록 JSON이 아닌 본문은 형식 오류");
    assert!(
        matches!(list_error, TransportError::ModelsListFormat(_)),
        "목록 조회는 형식 오류여야 합니다: {list_error:?}"
    );

    // 같은 클라이언트의 채팅 전송 성공. 설정의 손쓴 id가 본문 모델 값으로 들어감
    let request = crate::common::support::sample_request(None);
    let mut stream = client
        .stream_chat(request, CancellationToken::new())
        .await
        .expect("채팅 전송은 목록 실패와 무관해야 합니다");
    let events = drain(&mut stream).await;
    assert_eq!(content_events(&events), "chat-ok");

    // 요청 기록은 목록 1건, 채팅 1건
    let records = server.recorded_requests().await;
    assert_eq!(
        records.len(),
        2,
        "목록 요청과 채팅 요청 각 한 번씩이어야 합니다"
    );
    let chat_record = records
        .iter()
        .find(|record| record.method == "POST")
        .expect("채팅 요청 기록");
    let body_json: serde_json::Value =
        serde_json::from_slice(&chat_record.body).expect("채팅 요청 본문 JSON");
    assert_eq!(
        body_json["model"].as_str(),
        Some(KEEPING_ID),
        "손쓴 id로 요청해야 합니다"
    );
}
