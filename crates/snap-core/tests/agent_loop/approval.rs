//! 03-6 T1 승인 격자 통합 시험: 승인 표식이 호출 단위로만 열리는지 확인한다(spec AC 1·D22 5.3절).
//!
//! 승인자가 한 번 승인한 뒤에 따르는 호출이 승인자를 다시 타는지 본다.
//! 표식이 실행 판에 남는 코드라면 두 번째 호출이 승인자 없이 실행되고 세 번째 왕복까지 간다.

use snap_core::agent::{AgentLoop, LoopEnd, LoopOutcome, ToolRegistry};
use snap_core::llm::CancellationToken;
use snap_core::tools::approver::ApprovalReply;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    LoopTempWorkdir, content_stream, counting_approver, loop_config_with_approver,
    tool_calls_stream,
};

/// `run_command` 인자 JSON 문자열을 만드는 한 자리입니다.
/// 루프가 도구 호출 델타에서 읽는 인자 형식 그대로를 씁니다.
fn command_args(command: &str) -> String {
    serde_json::json!({"command": command}).to_string()
}

/// 승인자가 승인한 호출만 실행하고, 뒤따르는 호출은 다시 승인자를 타는지 확인한다.
///
/// 첫 왕복: `run_command`(echo first) → 승인자 첫 응답 = 승인 → 실행.
/// 둘째 왕복: `run_command`(echo second) → 승인자 둘째 응답 = 거부 → 루프가 승인 거부로 멈춤.
/// 표식이 실행 판에 남는 코드라면 둘째 호출이 승인자 없이 실행되고 세 번째 왕복까지 간다.
#[tokio::test]
async fn approval_flag_applies_only_to_that_call() {
    let first_args = command_args("echo first");
    let second_args = command_args("echo second");
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(tool_calls_stream(&[(
            "call_first",
            "run_command",
            &first_args,
        )])),
        MockResponseScript::with_body_parts(tool_calls_stream(&[(
            "call_second",
            "run_command",
            &second_args,
        )])),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;

    let workdir = LoopTempWorkdir::new("approval-grid");
    let (approver, calls) = counting_approver(&[ApprovalReply::Approved, ApprovalReply::Rejected]);
    let config = loop_config_with_approver(
        ToolRegistry::with_agent_tools(),
        workdir.path().to_path_buf(),
        approver,
    );
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        CancellationToken::new(),
    )
    .run(&config)
    .await;

    let LoopOutcome::Finished(LoopEnd::ApprovalRejected {
        stopped_record,
        partial,
    }) = outcome
    else {
        panic!("둘째 호출이 승인자를 타서 거부로 끝나야 합니다: {outcome:?}");
    };
    // 승인자가 두 번 불려야 호출마다 확인한 것이다. 한 번이면 표식이 실행 판에 남은 것이다.
    assert_eq!(
        calls.load(std::sync::atomic::Ordering::Relaxed),
        2,
        "승인자는 호출마다 불려야 합니다"
    );
    assert_eq!(stopped_record.call_id, "call_second");
    assert!(
        stopped_record.result.contains("승인 거부"),
        "거부 문구: {}",
        stopped_record.result
    );
    // 첫 승인으로 실행된 호출만 셸에 넘어가고, 둘째 호출은 거부로 기록에만 남는다.
    assert_eq!(
        partial.tool_calls.len(),
        2,
        "호출 두 개가 기록되어야 합니다"
    );
    assert_eq!(partial.tool_calls[0].call_id, "call_first");
    assert!(
        partial.tool_calls[0].result.contains("first"),
        "첫 명령은 실행되어야 합니다: {}",
        partial.tool_calls[0].result
    );
    assert!(
        !partial.tool_calls[0].result.contains("second"),
        "첫 기록에 둘째 명령 결과가 섞여서는 안 됩니다"
    );
    assert_eq!(partial.tool_calls[1].call_id, "call_second");
    assert!(
        partial.tool_calls[1].result.contains("승인 거부"),
        "둘째 명령은 실행되지 않고 거부로 기록되어야 합니다: {}",
        partial.tool_calls[1].result
    );
    // 승인자가 거절한 호출은 셸에 넘어가지 않으므로 셸 실행 결과 표기가 없다.
    // 거부 문구는 승인 요약(두 번째 명령)을 뒤에 붙이므로, 셸 결과 표기로 가린다.
    assert!(
        !partial.tool_calls[1].result.contains("종료 코드"),
        "거부된 명령은 셸에 넘어가서는 안 됩니다: {}",
        partial.tool_calls[1].result
    );
}
