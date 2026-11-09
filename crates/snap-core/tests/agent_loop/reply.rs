//! 03-2 루프 왕복 시험: AC 1(도구 왕복→stop), AC 4(미지원 감지), AC 1 규칙 4(거부 정지).

use snap_core::agent::{
    AgentLoop, FallbackPoint, LoopConfig, LoopEnd, LoopError, LoopOutcome, LoopStartError,
};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    base_config, cancel_token, content_stream, empty_stream, registry_with_approval,
    registry_with_fixed, registry_with_fixed_twice, test_working_dir, tool_call_stream,
    tool_calls_stream,
};

/// AC 1: 모의 서버가 도구 호출→stop으로 답하고, 등록한 도구가 결과로 왕복하는지 봅니다.
#[tokio::test]
async fn tool_call_round_trip_ends_with_stop() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(tool_call_stream("call_a", "echo", "{\"x\":1}")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;

    let config = LoopConfig::new(
        registry_with_fixed("echo", "ok"),
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        test_working_dir(),
    );
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;

    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.final_text, "최종 답");
    assert_eq!(result.turns, 2);
    assert_eq!(result.tool_calls.len(), 1);
    let record = &result.tool_calls[0];
    assert_eq!(record.call_id, "call_a");
    assert_eq!(record.tool_name, "echo");
    assert_eq!(record.result, "ok");
    assert!(record.executed_at_unix_secs > 0);

    let requests = server.recorded_requests().await;
    assert_eq!(requests.len(), 2, "왕복이 두 번 있어야 합니다");
    let second: serde_json::Value =
        serde_json::from_slice(&requests[1].body).expect("둘째 요청 본문 JSON");
    let messages = second["messages"].as_array().expect("messages 배열");
    let assistant = messages
        .iter()
        .find(|m| m["role"] == "assistant")
        .expect("assistant 메시지");
    assert_eq!(assistant["tool_calls"][0]["id"], "call_a");
    assert_eq!(assistant["tool_calls"][0]["function"]["name"], "echo");
    let tool = messages
        .iter()
        .find(|m| m["role"] == "tool")
        .expect("tool 메시지");
    assert_eq!(tool["tool_call_id"], "call_a");
    assert_eq!(tool["content"], "ok");
}

/// AC 4: 내용도 도구 호출도 없는 응답이면 이유가 있는 오류로 끝나고, 폴백 지점을 남깁니다.
#[tokio::test]
async fn empty_answer_fails_with_unsupported_reason() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(empty_stream())).await;
    let config = LoopConfig::new(
        registry_with_fixed("echo", "ok"),
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        test_working_dir(),
    );
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    let LoopOutcome::Failed(LoopError::Unsupported { fallback, point }) = outcome else {
        panic!("미지원 오류로 끝나야 합니다: {outcome:?}");
    };
    // 상태 단정: 표기 글자 상수의 상수 값을 폴백 지점 타입으로 봅니다(할 일 4, T2).
    assert_eq!(
        format!("{fallback}"),
        snap_core::agent::FallbackConnector::FALLBACK_CONNECTOR
    );
    assert_eq!(point, FallbackPoint::NoToolCallText);
}

/// AC 1 규칙 4: 승인이 필요한 도구는 즉시 거부로 알고 멈추며, 같은 호출을 되풀이하지 않습니다.
#[tokio::test]
async fn approval_request_stops_loop_after_one_rejection() {
    let server = MockServer::spawn_queue(vec![MockResponseScript::with_body_parts(
        tool_calls_stream(&[
            ("call_b", "run", "{\"cmd\":\"ls\"}"),
            ("call_d", "write", "{\"path\":\"a.txt\"}"),
        ]),
    )])
    .await;
    let config = LoopConfig::new(
        registry_with_approval("run"),
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        test_working_dir(),
    );
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    let LoopOutcome::Finished(LoopEnd::ApprovalRejected {
        stopped_record,
        partial,
    }) = outcome
    else {
        panic!("거부로 끝나야 합니다: {outcome:?}");
    };
    assert!(stopped_record.result.contains("승인 거부"));
    // 거부 기록은 마지막 기록이 아니라 거부된 호출의 기록입니다. 한 턴에 다른 호출이 뒤에 있어도 멈춤 대상은 앞의 거부입니다.
    assert_eq!(stopped_record.call_id, "call_b");
    assert_eq!(partial.tool_calls.len(), 2);
    assert_eq!(
        server.recorded_requests().await.len(),
        1,
        "거부 뒤에는 왕복을 보내지 않습니다"
    );
}

/// 등록 규칙: 같은 이름이 두 개면 첫 요청을 만들기 전에 막습니다. 뒤의 도구는 `find`가 앞의 도구를
/// 찾아 조용히 가려지므로, 요청을 보내는 것 자체가 결과가 어긋난 왕복입니다.
#[tokio::test]
async fn duplicate_tool_name_stops_before_first_request() {
    let server = MockServer::spawn_queue(vec![MockResponseScript::with_body_parts(
        content_stream("최종 답"),
    )])
    .await;
    let config = LoopConfig::new(
        registry_with_fixed_twice("echo", "ok"),
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        test_working_dir(),
    );
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    let LoopOutcome::Failed(LoopError::Start(LoopStartError::DuplicateToolName { name })) = outcome
    else {
        panic!("중복 등록 오류로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(name, "echo");
    assert_eq!(
        server.recorded_requests().await.len(),
        0,
        "중등록이면 요청을 보내지 않아야 합니다"
    );
}
