//! 요청 본문·인증 헤더 통합 테스트입니다(AC 2 요청부).

use serde_json::Value;

use super::mock_server::{MockResponseScript, MockServer};
use super::support::{client_at, sample_request, test_http};
use crate::config::{EndpointKind, Reasoning};
use crate::llm::{CancellationToken, ChatRequest, LlmClient};

async fn send_and_body(
    kind: EndpointKind,
    api_key: Option<&str>,
    request: ChatRequest,
) -> (Value, Option<String>) {
    let server = MockServer::spawn(MockResponseScript::empty_ok()).await;
    let client = client_at(server.base_url(), kind, api_key, test_http());

    let stream = client
        .stream_chat(request, CancellationToken::new())
        .await
        .expect("요청이 성공해야 합니다");
    assert!(stream.outcome().is_none());

    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 1);
    let req = &recorded[0];
    assert_eq!(req.method, "POST");
    assert_eq!(req.path, "/chat/completions");

    let auth = req
        .headers
        .get("authorization")
        .map(|value| value.to_str().expect("Authorization 문자열").to_owned());

    let body: Value = serde_json::from_slice(&req.body).expect("본문 JSON");
    (body, auth)
}

fn assert_common_body(body: &Value) {
    assert_eq!(body["model"], "demo-model");
    assert_eq!(body["stream"], true);
    assert_eq!(body["messages"][0]["role"], "user");
    assert_eq!(body["messages"][0]["content"], "hello");
    assert_eq!(body["temperature"], 0.2);
    assert_eq!(body["top_p"], 0.9);
    assert_eq!(body["max_tokens"], 64);
    assert_eq!(body["tools"][0]["type"], "function");
    assert_eq!(body["tools"][0]["function"]["name"], "echo");
    assert_eq!(body["keep_alive"], 0);
    assert_eq!(body["options"]["num_ctx"], 8192);
    assert!(body.get("tool_choice").is_none());
    assert!(body.get("n").is_none());
    assert!(body.get("stream_options").is_none());
    assert!(body.get("context_window").is_none());
}

#[tokio::test]
async fn openai_reasoning_effort_and_bearer() {
    let (body, auth) = send_and_body(
        EndpointKind::OpenAi,
        Some("sk-test-key"),
        sample_request(Some(Reasoning::High)),
    )
    .await;
    assert_common_body(&body);
    assert_eq!(body["reasoning_effort"], "high");
    assert!(body.get("reasoning").is_none());
    assert_eq!(auth.as_deref(), Some("Bearer sk-test-key"));
}

#[tokio::test]
async fn ollama_vllm_llamacpp_use_reasoning_effort() {
    for kind in [
        EndpointKind::Ollama,
        EndpointKind::Vllm,
        EndpointKind::LlamaCpp,
    ] {
        let (body, auth) = send_and_body(kind, None, sample_request(Some(Reasoning::Off))).await;
        assert_eq!(body["reasoning_effort"], "none");
        assert!(body.get("reasoning").is_none());
        assert!(auth.is_none());
    }
}

#[tokio::test]
async fn openrouter_uses_nested_reasoning_effort() {
    let (body, auth) = send_and_body(
        EndpointKind::OpenRouter,
        Some("or-key"),
        sample_request(Some(Reasoning::Medium)),
    )
    .await;
    assert_common_body(&body);
    assert_eq!(body["reasoning"]["effort"], "medium");
    assert!(body.get("reasoning_effort").is_none());
    assert_eq!(auth.as_deref(), Some("Bearer or-key"));
}

#[tokio::test]
async fn lmstudio_omits_common_reasoning_field() {
    let (body, auth) = send_and_body(
        EndpointKind::LmStudio,
        None,
        sample_request(Some(Reasoning::Low)),
    )
    .await;
    assert_common_body(&body);
    assert!(body.get("reasoning_effort").is_none());
    assert!(body.get("reasoning").is_none());
    assert_eq!(body["keep_alive"], 0);
    assert!(auth.is_none());
}

#[tokio::test]
async fn no_reasoning_omits_related_fields() {
    let (body, _) = send_and_body(EndpointKind::OpenAi, None, sample_request(None)).await;
    assert!(body.get("reasoning_effort").is_none());
    assert!(body.get("reasoning").is_none());
}
