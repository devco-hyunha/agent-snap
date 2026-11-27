//! 02-7 T2 보정 시험: 조립 창구의 `kind` 폴백 책임. 허용 목록 밖 `kind`로 조립하면 OpenAI 형식 본문이 되는지 봄.

use snap_core::llm::LlmClient;
use snap_core::llm::ReasoningEffort;
use snap_core::llm::StreamReceiver;
use tokio_util::sync::CancellationToken;

use crate::build::materials::MODEL_ID;
use crate::build::materials::bundle_for_kind;
use crate::request::request_with_reasoning;
use crate::request::user_message;
use crate::request_body::field;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_payload::FIRST_CHUNK;

/// 허용 목록 밖 `kind` 값을 가진 설정으로 강도를 채운 요청을 조립하면, 조립 창구가 OpenAI 형식 본문으로 스트림을 연다.
/// OpenAI 형식은 강도를 `reasoning_effort` 키 한 칸으로 열고, OpenRouter 형식은 `reasoning` 객체로 열므로 두 키로 형식을 가린다.
#[tokio::test]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn outside_kind_assembles_as_openai_body() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK),
        StreamStep::Stop(StreamStop::End),
    ]));

    let bundle = bundle_for_kind(&server, "sk-buildvalue123", "unknown_kind");
    assert_eq!(bundle.model_id, MODEL_ID, "fallback bundle keeps model id");

    let mut request = request_with_reasoning(Some(ReasoningEffort::High));
    request.model_id = bundle.model_id.clone();
    request.messages = vec![user_message()];

    let mut receiver = bundle
        .client
        .open(&request, CancellationToken::new())
        .await
        .expect("fallback client opens stream");
    let outcome = receiver.finish().expect("fallback stream finishes");

    assert_eq!(outcome.delta.content, "가", "fallback stream restores body");

    let recorded = server
        .requests()
        .snapshot()
        .into_iter()
        .next()
        .expect("mock server records one request");
    let body: serde_json::Value =
        serde_json::from_str(&recorded.body_text()).expect("recorded body is json");
    assert_eq!(
        field(&body, "model"),
        Some(&serde_json::json!(MODEL_ID)),
        "fallback body carries the bundle model id"
    );
    assert_eq!(
        field(&body, "stream"),
        Some(&serde_json::json!(true)),
        "fallback body is streaming"
    );
    assert_eq!(
        field(&body, "reasoning_effort"),
        Some(&serde_json::json!("high")),
        "fallback body opens the OpenAI form effort key"
    );
    assert!(
        field(&body, "reasoning").is_none(),
        "fallback body does not open the OpenRouter form reasoning object"
    );

    server.stop();
}
