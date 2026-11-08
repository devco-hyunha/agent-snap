//! 03-2 프롬프트 앞머리·`extra` 합쳐 보내기 시험: AC 6(프롬프트 앞머리), AC 7(`extra` 합치기).

use snap_core::agent::{
    AgentLoop, LoopEnd, LoopOutcome, ToolCallProfile, ToolRegistry, assemble_messages,
};
use snap_core::config::{ExtraValue, Sampling};
use snap_core::llm::ChatRequest;

use std::collections::BTreeMap;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{cancel_token, content_stream, registry_with_fixed};

/// AC 6: 프롬프트 앞머리가 messages 맨자리에 오고, AGENTS.md 합치기 자리가 그 뒤에 붙는지 봅니다.
#[test]
fn system_prompt_is_first_message() {
    let messages = assemble_messages("규범 앞머리", "", "과제 한 장");
    assert_eq!(messages[0].role, "system");
    assert!(messages[0].content.starts_with("규범 앞머리"));
    assert_eq!(messages[1].role, "user");
    assert_eq!(messages[1].content, "과제 한 장");
}

/// AC 6 연장: 03-5가 채울 AGENTS.md 합치기 자리를 앞머리 뒤에 붙여 넣습니다.
#[test]
fn agents_md_joins_after_prompt_head() {
    let messages = assemble_messages("규범 앞머리", "규칙 파일 요약", "과제 한 장");
    assert!(
        messages[0].content.contains("규범 앞머리"),
        "앞머리가 먼저 남아야 합니다"
    );
    assert!(
        messages[0].content.contains("규칙 파일 요약"),
        "AGENTS.md 합치기 자리가 따라야 합니다"
    );
}

/// AC 7: `tool_call` 프로파일 온도가 요청 본문 `temperature`에 실리고 모델 `extra`도 함께 보냅니다.
#[tokio::test]
async fn profile_temperature_and_extra_reach_request_body() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream("답"))).await;
    let config = snap_tool_call_config(registry_with_fixed("echo", "ok"));
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    assert!(matches!(
        outcome,
        LoopOutcome::Finished(LoopEnd::Stop { .. })
    ));

    let requests = server.recorded_requests().await;
    let body: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("요청 본문 JSON");
    assert_eq!(body["temperature"], 0.1, "프로파일 온도가 실려야 합니다");
    assert_eq!(body["keep_alive"], 0, "모델 extra가 그대로 합쳐져야 합니다");
    assert_eq!(body["num_ctx"], 8192);
}

fn snap_tool_call_config(registry: ToolRegistry) -> snap_core::agent::LoopConfig {
    let base = ChatRequest {
        model: "loop-test-model".into(),
        messages: Vec::new(),
        tools: Vec::new(),
        sampling: Sampling::default(),
        reasoning: None,
        extra: BTreeMap::from([
            ("keep_alive".to_owned(), ExtraValue::Number("0".to_owned())),
            ("num_ctx".to_owned(), ExtraValue::Number("8192".to_owned())),
        ]),
    };
    let mut config = snap_core::agent::LoopConfig::new(
        registry,
        base,
        "규범 앞머리",
        "과제 한 장",
        "C:\\workspace\\agent-snap",
    );
    config.max_turns = 4;
    config.profile = Some(ToolCallProfile::new(0.1).expect("0.1은 프로파일 값"));
    config
}
