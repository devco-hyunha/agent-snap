//! 선택 키 클라이언트에서 모델 목록 조회가 실패해도 채팅 요청은 막히지 않음을 보는 시험입니다(02-7 AC 3, spec "조회 실패와 손쓴 id 요청은 서로 막지 않는다").
//!
//! 목록 라우트가 SSE 본문(목록 JSON이 아닌 자리)을 받아 형식 오류로 실패한 상태에서도,
//! 같은 선택 키로 만든 클라이언트의 채팅 전송은 설정에 적힌 손쓴 id(`KEEPING_ID`)로 성공한다.
//! 조회가 설정을 건드리지 않는 읽기임을 시험으로 고정한다(리뷰 1차 권고).
//! 모의 서버는 한 스크립트를 두 라우트에 같이 쓰므로, 목록 라우트의 본문이 목록 JSON이
//! 아닌 자리에서 형식 오류가 나고 채팅 라우트는 같은 본문을 SSE로 성공시킨다.

use snap_core::config::Config;
use snap_core::llm::{CancellationToken, LlmClient, TransportError};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{body_parts_sse, content_events, drain, success_stream_body};

use super::support::{FAST_SELECTION_KEY, KEEPING_ID, ac3_credentials, ac3_http_config_text};

/// AC 3: 목록 조회가 형식 오류로 실패한 서버라도 선택 키 채팅 요청은 설정의 id로 성공한다.
#[tokio::test]
async fn list_failure_does_not_block_selected_chat() {
    // 본문은 SSE 줄 하나. 목록 라우트는 목록 JSON이 아니라 형식 오류가 나고,
    // 채팅 라우트는 같은 본문을 SSE로 읽는다(스크립트 하나를 두 라우트가 같이 쓴다).
    let body = success_stream_body(&["chat-ok"]);
    let server =
        MockServer::spawn(MockResponseScript::with_body_parts(body_parts_sse(&body))).await;

    let config = Config::parse(&ac3_http_config_text(server.base_url())).expect("AC 3 설정");
    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");

    // 먼저 목록 조회가 형식 오류로 실패함을 확인한다.
    let list_error = client
        .list_models(CancellationToken::new())
        .await
        .expect_err("목록 JSON이 아닌 본문은 형식 오류");
    assert!(
        matches!(list_error, TransportError::ModelsListFormat(_)),
        "목록 조회는 형식 오류여야 합니다: {list_error:?}"
    );

    // 그 다음 같은 클라이언트의 채팅 전송은 성공한다. 설정의 손쓴 id가 본문 모델 값으로 들어간다.
    let request = crate::common::support::sample_request(None);
    let mut stream = client
        .stream_chat(request, CancellationToken::new())
        .await
        .expect("채팅 전송은 목록 실패와 무관해야 합니다");
    let events = drain(&mut stream).await;
    assert_eq!(content_events(&events), "chat-ok");

    // 요청 기록은 목록 요청 1건과 채팅 요청 1건, 합계 2건이다.
    // 채팅 요청 본문의 모델 값이 설정 id와 같다(선택 키 조립이 조회 실패로 섞이지 않음).
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
