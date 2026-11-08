//! 선택 키 클라이언트 통합 테스트(02-7 AC 1): 모의 서버 두 개로 요청의 행선지와 본문 값을 확인합니다.

use std::collections::BTreeMap;

use snap_core::config::{Config, Sampling};
use snap_core::credentials::Credentials;
use snap_core::llm::{
    CancellationToken, ChatMessage, ChatRequest, ChatStream, LlmClient, StreamEvent,
};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{body_parts_sse, drain, success_stream_body};

const KEY_MARK: &str = "key-ac1-integration";

/// AC 1에서 쓰는 설정 골격입니다. 엔드포인트 이름 `pc1`·`pc2`와 같은 별칭 `fast`를 모의 서버 주소로 잇습니다.
///
/// 같은 별칭 `fast`가 두 엔드포인트에 있고, 모델별 샘플링과 루트 기본 샘플링이 함께 들어가 있습니다.
fn ac1_config_text(pc1_url: &str, pc2_url: &str) -> String {
    format!(
        r#"{{"endpoints":[
            {{"name":"pc1","base_url":"{pc1_url}","kind":"openai","models":[
                {{"name":"fast","id":"id-one","sampling":{{"temperature":0.7,"max_tokens":128}},
                 "reasoning":"low","extra":{{"keep_alive":0}}}}]}},
            {{"name":"pc2","base_url":"{pc2_url}","kind":"openai","models":[
                {{"name":"fast","id":"id-two","sampling":{{"temperature":0.3}},"extra":{{"num_ctx":8192}}}}]}}],
         "sampling":{{"temperature":0.5,"top_p":0.9,"max_tokens":64}}}}"#
    )
}

/// AC 1에서 쓰는 요청 골격입니다. 선택 키가 정한 값에 가리도록 본문 골격만 넘겼습니다.
fn ac1_request() -> ChatRequest {
    ChatRequest {
        model: "caller-model".into(),
        messages: vec![ChatMessage {
            role: "user".into(),
            content: "hello".into(),
            ..Default::default()
        }],
        tools: Vec::new(),
        sampling: Sampling::default(),
        reasoning: None,
        extra: BTreeMap::new(),
    }
}

/// 본문 JSON에서 숫자 값을 꺼냅니다. 본문 필드가 기대 값과 같은지 비교하는 데 쓰입니다.
fn body_number(body: &serde_json::Value, field: &str) -> Option<f64> {
    body.get(field).and_then(serde_json::Value::as_f64)
}

/// 스트림을 끝까지 읽고 내용만 모습니다. 본문이 채워졌는지 확인하는 데 쓰입니다.
async fn read_stream(stream: &mut ChatStream) -> String {
    drain(stream)
        .await
        .into_iter()
        .filter_map(|event| match event {
            StreamEvent::Content(text) => Some(text),
            _ => None,
        })
        .collect()
}

/// AC 1: 선택 키가 가리키는 모의 서버로만 요청이 가고, 본문에 그 모델의 id와 샘플링이 실립니다.
///
/// 같은 별칭 `fast`가 서로 다른 엔드포인트에 있어도 선택 키가 다르면 다른 서버로 갑니다.
#[tokio::test]
async fn selection_key_routes_to_its_endpoint_only() {
    let server_pc1 = MockServer::spawn(MockResponseScript::with_body_parts(body_parts_sse(
        &success_stream_body(&["one"]),
    )))
    .await;
    let server_pc2 = MockServer::spawn(MockResponseScript::with_body_parts(body_parts_sse(
        &success_stream_body(&["two"]),
    )))
    .await;

    let config = Config::parse(&ac1_config_text(
        server_pc1.base_url(),
        server_pc2.base_url(),
    ))
    .expect("AC 1 설정");
    let credentials = Credentials::parse(&format!(r#"{{"api_keys":{{"pc1":"{KEY_MARK}"}}}}"#))
        .expect("자격 파일");

    let first = config
        .client_for("pc1/fast", &credentials)
        .expect("pc1/fast");
    let second = config
        .client_for("pc2/fast", &credentials)
        .expect("pc2/fast");

    let mut first_stream = first
        .stream_chat(ac1_request(), CancellationToken::new())
        .await
        .expect("pc1 요청");
    let mut second_stream = second
        .stream_chat(ac1_request(), CancellationToken::new())
        .await
        .expect("pc2 요청");
    assert_eq!(read_stream(&mut first_stream).await, "one");
    assert_eq!(read_stream(&mut second_stream).await, "two");

    let pc1_records = server_pc1.recorded_requests().await;
    let pc2_records = server_pc2.recorded_requests().await;
    assert_eq!(pc1_records.len(), 1, "pc1 서버가 한 번만 받아야 합니다");
    assert_eq!(pc2_records.len(), 1, "pc2 서버가 한 번만 받아야 합니다");

    let first_body: serde_json::Value =
        serde_json::from_slice(&pc1_records[0].body).expect("pc1 요청 본문");
    assert_eq!(first_body["model"], "id-one");
    assert_eq!(body_number(&first_body, "temperature"), Some(0.7));
    assert_eq!(body_number(&first_body, "top_p"), Some(0.9));
    assert_eq!(body_number(&first_body, "max_tokens"), Some(128.0));
    assert_eq!(first_body["keep_alive"], 0);
    assert_eq!(first_body["reasoning_effort"], "low");
    assert_eq!(pc1_records[0].method, "POST");
    assert_eq!(pc1_records[0].path, "/chat/completions");

    let second_body: serde_json::Value =
        serde_json::from_slice(&pc2_records[0].body).expect("pc2 요청 본문");
    assert_eq!(second_body["model"], "id-two");
    assert_eq!(body_number(&second_body, "temperature"), Some(0.3));
    assert_eq!(body_number(&second_body, "top_p"), Some(0.9));
    assert_eq!(body_number(&second_body, "max_tokens"), Some(64.0));
    assert_eq!(second_body["num_ctx"], 8192);
    assert_eq!(pc2_records[0].method, "POST");
    assert_eq!(pc2_records[0].path, "/chat/completions");

    assert_eq!(
        pc1_records[0]
            .headers
            .get("authorization")
            .expect("pc1에는 키 헤더가 있어야 합니다")
            .to_str()
            .expect("헤더 값"),
        format!("Bearer {KEY_MARK}")
    );
    assert!(
        pc2_records[0].headers.get("authorization").is_none(),
        "pc2에는 키 헤더가 없어야 합니다"
    );
}
