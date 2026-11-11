use snap_core::agent::{AgentLoop, LoopEnd, LoopOutcome, ToolCallFormat};
use snap_core::config::ToolCallForm;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    cancel_token, content_stream, form_config, registry_with_fixed, registry_with_list_dir,
};

/// AC 4: 스키마에 안 맞는 인자는 도구 실행 전에 짧은 오류로 돌아가고, 2안 형식 기록 칸에 남는다.
#[tokio::test]
async fn schema_mismatch_returns_short_notice_before_running_tool() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(content_stream(
            "{\"name\":\"list_dir\",\"arguments\":{\"path\":9}}",
        )),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&form_config(registry_with_list_dir(), ToolCallForm::Text))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    let record = &result.tool_calls[0];
    assert_eq!(record.tool_name, "list_dir");
    assert!(
        record.result.contains("인자 형식이 어긋났어"),
        "짧은 오류가 결과에 있어야 합니다: {}",
        record.result
    );
    assert!(record.result.contains("필요한 것"));
    assert_eq!(record.format, ToolCallFormat::Text);
}

/// AC 4: 등록에 없는 도구 이름은 도구 실행 전에 "등록되지 않은 도구야(사용 가능: …)" 안내로 돌아간다.
#[tokio::test]
async fn unregistered_tool_name_returns_notice_before_running_tool() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(content_stream(
            "{\"name\":\"no_such_tool\",\"arguments\":{}}",
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
        ToolCallForm::Text,
    ))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    let record = &result.tool_calls[0];
    assert_eq!(record.tool_name, "no_such_tool");
    assert_eq!(record.format, ToolCallFormat::Text);
    assert!(
        record
            .result
            .contains("등록되지 않은 도구야(사용 가능: echo)"),
        "미등록 안내에 사용 가능 목록이 있어야 합니다: {}",
        record.result
    );
    assert!(
        record.result.contains("`no_such_tool`는 쓸 수 없어"),
        "문제된 이름이 안내에 있어야 합니다: {}",
        record.result
    );
    let requests = server.recorded_requests().await;
    let second: serde_json::Value =
        serde_json::from_slice(&requests[1].body).expect("둘째 요청 본문 JSON");
    let messages = second["messages"].as_array().expect("messages 배열");
    let result_line = messages
        .iter()
        .filter(|message| message["role"] == "user")
        .filter_map(|message| message["content"].as_str())
        .find(|content| content.starts_with("도구 결과 [no_such_tool]"))
        .expect("미등록 안내가 도구 결과 줄로 이어쓰기에 붙어야 합니다");
    assert!(
        result_line.contains("등록되지 않은 도구야"),
        "결과 줄에 안내가 있어야 합니다: {result_line}"
    );
}

/// AC 3: JSON을 찾지 못한 왕복에는 짧은 오류를 한 번만 넘기고, 그 뒤 왕복에서 답으로 끝난다.
#[tokio::test]
async fn missing_json_notices_once_and_then_stops() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream(
        "JSON 없는 답",
    )))
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&form_config(
        registry_with_fixed("echo", "ok"),
        ToolCallForm::Text,
    ))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.tool_calls.len(), 1, "안내 기록은 한 번만 남습니다");
    assert!(result.tool_calls[0].result.contains("JSON을 찾지 못했어"));
    assert_eq!(server.recorded_requests().await.len(), 2);
}
