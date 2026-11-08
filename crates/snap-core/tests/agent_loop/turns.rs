//! 03-2 왕복 수·잘림·JSON 복구 시험: AC 2(상한 초과), AC 3(인자 JSON), AC 5(4,000자 잘림).

use snap_core::agent::{
    AgentLoop, LoopEnd, LoopOutcome, TRUNCATION_MARK, recover_arguments, truncate_tool_result,
};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    cancel_token, loop_config, registry_with_fixed, tool_call_stream, tool_calls_stream,
};

/// 도구 호출 여러 개를 한 번 알리는 모의 서버 스크립트로 감쌉니다.
fn tool_calls_script(calls: &[(&str, &str, &str)]) -> MockResponseScript {
    MockResponseScript::with_body_parts(tool_calls_stream(calls))
}

/// AC 2: 왕복 상한을 채우면 상한 초과로 끝나고, 부분 결과에 지금까지 기록이 남습니다.
#[tokio::test]
async fn turn_limit_ends_loop_with_partial_result() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(tool_call_stream(
        "call_a",
        "echo",
        "{\"x\":1}",
    )))
    .await;
    let config = loop_config(registry_with_fixed("echo", "ok"), 2, None);
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    let LoopOutcome::Finished(LoopEnd::TurnLimit { partial }) = outcome else {
        panic!("상한 초과로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(partial.turns, 2);
    assert_eq!(partial.tool_calls.len(), 2);
    assert_eq!(
        server.recorded_requests().await.len(),
        2,
        "상한만큼만 왕복을 보냅니다"
    );
}

/// AC 3: 한 턴의 도구 호출 세 갈래를 확인합니다.
/// - 잘린 인자는 복구를 거쳐 도구가 실행되고,
/// - 복구 안 되는 인자는 `오류: 인자 JSON 형식 오류` 한 줄,
/// - 등록에 없는 이름은 `도구 없음` 한 줄을 돌려줍니다.
#[tokio::test]
async fn broken_arguments_and_missing_tool_return_one_line_errors() {
    let server = MockServer::spawn_queue(vec![tool_calls_script(&[
        ("call_a", "echo", "{\"x\":1"),
        ("call_b", "echo", "{\"x\":}"),
        ("call_c", "nope", "{\"x\":1}"),
    ])]);
    let server = server.await;
    let config = loop_config(registry_with_fixed("echo", "ok"), 1, None);
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    let LoopOutcome::Finished(LoopEnd::TurnLimit { partial }) = outcome else {
        panic!("상한으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(
        partial.tool_calls[0].result, "ok",
        "복구된 인자는 도구 실행을 통과해야 합니다"
    );
    assert_eq!(
        partial.tool_calls[1].result, "오류: 인자 JSON 형식 오류",
        "복구 안 되는 인자는 한 줄 오류입니다"
    );
    assert_eq!(partial.tool_calls[2].result, "도구 없음: nope");
}

/// AC 3 연장: 잘린 인자 JSON은 닫힘 기호를 채워 복구합니다(단위 확인).
#[test]
fn repaired_arguments_use_the_repaired_json() {
    let repaired = recover_arguments("{\"a\":[1,2").expect("닫힘 기호를 채워 복구해야 합니다");
    assert_eq!(repaired, "{\"a\":[1,2]}");
    assert!(recover_arguments("{\"a\":}").is_err());
    assert_eq!(
        recover_arguments("").expect("빈 인자는 빈 객체로 봅니다"),
        "{}"
    );
}

/// AC 5: 상한을 넘는 도구 결과는 앞만 남기고 `…[truncated]`를 붙입니다(단위 확인).
#[test]
fn tool_result_truncates_to_cap_with_mark() {
    let long = "가".repeat(4_001);
    let cut = truncate_tool_result(&long, 4_000);
    assert!(cut.ends_with(TRUNCATION_MARK));
    assert_eq!(cut.chars().count(), 4_000);
    assert!(cut.starts_with("가"));

    let short = "짧은 결과";
    assert_eq!(truncate_tool_result(short, 4_000), short);
}
