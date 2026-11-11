use snap_core::agent::{AgentLoop, FallbackPoint, LoopEnd, LoopOutcome, ToolCallFormat};
use snap_core::config::ToolCallForm;

use axum::http::StatusCode;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{cancel_token, content_stream, form_config, registry_with_fixed};

use super::tools_of;

/// AC 1: `auto` 선언에서 미지원 400을 받으면 같은 요청을 2안 조립으로 다시 보내고, 2안 도구 호출이 이어진다.
#[tokio::test]
async fn auto_switches_on_unsupported_status_and_runs_text_call() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript {
            status: StatusCode::BAD_REQUEST,
            ..MockResponseScript::empty_ok()
        },
        MockResponseScript::with_body_parts(content_stream(
            "```json\n{\"name\":\"echo\",\"arguments\":{\"x\":1}}\n```",
        )),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&form_config(
        registry_with_fixed("echo", "ok"),
        ToolCallForm::Auto,
    ))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    // 4x 전환 기록이 2안 형식 칸과 함께 결과 자리에 남습니다(AC 5).
    assert_eq!(
        result.fallback_point,
        Some(FallbackPoint::UnsupportedStatus { status: 400 })
    );
    let executed = result
        .tool_calls
        .iter()
        .find(|record| record.tool_name == "echo")
        .expect("echo 기록");
    assert_eq!(executed.format, ToolCallFormat::Text);
    assert_eq!(executed.result, "ok");

    let requests = server.recorded_requests().await;
    let first: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("첫째 요청 본문 JSON");
    assert!(
        !tools_of(&first).is_empty(),
        "첫째 요청에는 1안 tools가 있습니다"
    );
    let second: serde_json::Value =
        serde_json::from_slice(&requests[1].body).expect("둘째 요청 본문 JSON");
    assert!(
        tools_of(&second).is_empty(),
        "전환한 둘째 요청은 tools가 비어야 합니다"
    );
    let system = second["messages"].as_array().expect("messages")[0]["content"]
        .as_str()
        .expect("system content");
    assert!(
        system.contains("echo"),
        "2안 지시 블록이 없습니다: {system}"
    );
}

/// AC 1: `auto` 선언의 텍스트 1회는 1안으로 계속 보내고(전환 후보만 기록), 2회 연속에서 전환이 확정된다.
#[tokio::test]
async fn auto_text_streak_switches_after_second_turn() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(content_stream("호출 없는 첫 답")),
        MockResponseScript::with_body_parts(content_stream("호출 없는 둘째 답")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&form_config(
        registry_with_fixed("echo", "ok"),
        ToolCallForm::Auto,
    ))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.fallback_point, Some(FallbackPoint::TextStreak));
    let requests = server.recorded_requests().await;
    let second: serde_json::Value =
        serde_json::from_slice(&requests[1].body).expect("둘째 요청 본문 JSON");
    assert!(
        !tools_of(&second).is_empty(),
        "텍스트 1회는 전환 후보일 뿐이므로 둘째 요청은 1안 tools를 지킵니다"
    );
    let second_messages = second["messages"].as_array().expect("messages 배열");
    let assistant = second_messages
        .iter()
        .find(|message| message["role"] == "assistant")
        .expect("텍스트 1회 뒤에 assistant 줄을 이어쓰기에 붙여야 합니다");
    assert_eq!(assistant["content"], "호출 없는 첫 답");
    assert!(
        assistant.get("tool_calls").is_none(),
        "텍스트 1회 왕복은 도구 호출 줄을 붙이지 않습니다"
    );
    let third: serde_json::Value =
        serde_json::from_slice(&requests[2].body).expect("셋째 요청 본문 JSON");
    assert!(
        tools_of(&third).is_empty(),
        "2회 연속에서 전환했으므로 셋째 요청은 tools가 비어야 합니다"
    );
}
