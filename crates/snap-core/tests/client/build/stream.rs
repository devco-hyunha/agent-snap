//! 02-7 T2 조립 성공 시험. 조립된 클라이언트가 같은 요청 본문으로 스트림을 여는지만 봄.

use snap_core::llm::LlmClient;
use snap_core::llm::StreamReceiver;
use tokio_util::sync::CancellationToken;

use crate::build::materials::MODEL_ID;
use crate::build::materials::bundle_for;
use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;
use crate::stream_payload::FIRST_CHUNK;

/// 조립 결과의 모델 id를 요청에 넣어 열면, 서버는 같은 본문을 받고 스트림이 열린다(spec AC 1).
#[tokio::test]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn assembled_client_opens_stream_with_same_body() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok().with_body(vec![
        StreamStep::chunk(FIRST_CHUNK),
        StreamStep::Stop(StreamStop::End),
    ]));

    let bundle = bundle_for(&server, "sk-buildvalue123");
    assert_eq!(bundle.model_id, MODEL_ID, "bundle carries config model id");

    let mut request = request_with_user_message();
    request.model_id = bundle.model_id.clone();

    let mut receiver = bundle
        .client
        .open(&request, CancellationToken::new())
        .await
        .expect("assembled client opens stream");
    let outcome = receiver.finish().expect("assembled stream finishes");

    assert_eq!(
        outcome.delta.content, "가",
        "assembled stream restores body"
    );

    let recorded = server
        .requests()
        .snapshot()
        .into_iter()
        .next()
        .expect("mock server records one request");
    let body: serde_json::Value =
        serde_json::from_str(&recorded.body_text()).expect("recorded body is json");
    assert_eq!(
        body.get("model"),
        Some(&serde_json::json!(MODEL_ID)),
        "request body carries the bundle model id"
    );
    assert_eq!(
        body.get("stream"),
        Some(&serde_json::json!(true)),
        "request body is streaming"
    );

    server.stop();
}
