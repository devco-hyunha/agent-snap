//! T4 시험: 모의 서버로 확인하는 요청 전송 책임. `kind`별 본문과 무키 요청만 봄.
use snap_core::config::SecretString;
use snap_core::llm::ChatRequest;
use snap_core::llm::EndpointKind;
use snap_core::llm::ReasoningEffort;
use snap_core::llm::auth_header;
use snap_core::llm::build_body;
use snap_core::llm::completions_url;

use crate::fixtures::RawClient;
use crate::request::KEY;
use crate::request::request_with_reasoning;
use crate::request::sampling_with_values;
use crate::request::user_message;
use crate::script::MockScript;
use crate::server::MockServer;
use crate::stream_open::base_url;
use serde_json::json;

/// 모의 서버 시험용 요청 골격. 강도·extra·샘플링·메시지를 채움.
fn request() -> ChatRequest {
    let mut request = request_with_reasoning(Some(ReasoningEffort::High));
    request.messages = vec![user_message()];
    request.sampling = sampling_with_values();
    request.extra = Some(extra());
    request
}

/// 시험용 extra. 서버가 받는 그대로 본문에 들어가야 하는 키.
fn extra() -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert("vendor".to_owned(), json!("x"));
    map
}

/// 모의 서버에 본문·헤더를 보내고 받은 요청 하나를 꺼냄.
#[allow(clippy::expect_used)]
async fn send_and_record(
    server: &MockServer,
    kind: EndpointKind,
    key: Option<&SecretString>,
) -> crate::types::RecordedRequest {
    let body = build_body(&request(), kind);
    let headers = auth_header(key).into_iter().collect::<Vec<_>>();
    let headers: Vec<(&str, &str)> = headers
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();
    let body_text = body.to_string();

    let path = completions_url(&base_url(server))
        .split_once("://")
        .map(|(_, rest)| {
            let slash = rest.find('/').unwrap_or(0);
            rest[slash..].to_owned()
        })
        .unwrap_or_default();

    let mut client = RawClient::connect(server.address())
        .await
        .expect("test connects to mock server");
    let _head = client
        .send("POST", &path, &headers, &body_text)
        .await
        .expect("test sends request");

    server
        .requests()
        .snapshot()
        .into_iter()
        .next()
        .expect("mock server records one request")
}

#[tokio::test]
async fn mock_server_receives_chat_completions_path() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let recorded = send_and_record(&server, EndpointKind::Openai, None).await;

    assert_eq!(recorded.method, "POST");
    assert_eq!(recorded.path, "/v1/chat/completions");

    server.stop();
}

#[tokio::test]
async fn mock_server_receives_reasoning_effort_for_shared_kinds() {
    let kinds = [
        EndpointKind::Openai,
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::Llamacpp,
    ];
    for kind in kinds {
        #[allow(clippy::unwrap_used)]
        let server = MockServer::start().await.unwrap();
        server.enqueue(MockScript::ok());

        let recorded = send_and_record(&server, kind, None).await;
        let body: serde_json::Value =
            serde_json::from_str(&recorded.body_text()).expect("recorded body is json");

        assert_eq!(
            body.get("reasoning_effort"),
            Some(&json!("high")),
            "kind {kind} sends flat reasoning_effort"
        );
        assert!(
            body.get("reasoning").is_none(),
            "kind {kind} sends no nested reasoning"
        );

        server.stop();
    }
}

#[tokio::test]
async fn mock_server_receives_nested_reasoning_for_openrouter() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let recorded = send_and_record(&server, EndpointKind::Openrouter, None).await;
    let body: serde_json::Value =
        serde_json::from_str(&recorded.body_text()).expect("recorded body is json");

    assert_eq!(
        body.get("reasoning"),
        Some(&json!({"effort": "high"})),
        "openrouter nests effort"
    );

    server.stop();
}

#[tokio::test]
async fn mock_server_receives_no_reasoning_for_lmstudio() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let recorded = send_and_record(&server, EndpointKind::Lmstudio, None).await;
    let body: serde_json::Value =
        serde_json::from_str(&recorded.body_text()).expect("recorded body is json");

    assert!(
        body.get("reasoning_effort").is_none() && body.get("reasoning").is_none(),
        "lmstudio sends no common thinking field"
    );

    server.stop();
}

#[tokio::test]
async fn mock_server_receives_extra_and_stream_and_sampling() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let recorded = send_and_record(&server, EndpointKind::Openai, None).await;
    let body: serde_json::Value =
        serde_json::from_str(&recorded.body_text()).expect("recorded body is json");

    assert_eq!(body.get("vendor"), Some(&json!("x")), "extra merges as is");
    assert_eq!(body.get("stream"), Some(&json!(true)));
    assert_eq!(body.get("temperature"), Some(&json!(0.5)));
    assert_eq!(body.get("max_tokens"), Some(&json!(64)));
    assert_eq!(body.get("model"), Some(&json!("model-id")));

    server.stop();
}

#[tokio::test]
async fn mock_server_receives_bearer_header_only_for_key() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let key = SecretString::new(KEY.to_owned());
    let recorded = send_and_record(&server, EndpointKind::Openai, Some(&key)).await;

    assert_eq!(
        recorded.header("authorization"),
        Some("Bearer sk-testvalue123"),
        "key goes to authorization header only"
    );
    assert!(
        !recorded.body_text().contains(KEY),
        "body carries no key value"
    );
    assert!(!recorded.path.contains(KEY), "path carries no key value");

    server.stop();
}

#[tokio::test]
async fn mock_server_accepts_request_without_authorization_header() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    let recorded = send_and_record(&server, EndpointKind::Ollama, None).await;

    assert!(
        recorded.header("authorization").is_none(),
        "keyless endpoint sends no authorization header"
    );

    server.stop();
}
