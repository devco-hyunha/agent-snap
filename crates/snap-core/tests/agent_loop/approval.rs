//! 승인 격자 통합 시험: 승인 표식이 호출 단위로만 열리는지 확인.
//!
//! 한 번 승인한 뒤 따르는 호출이 승인자를 다시 타는지 본다.

use snap_core::agent::{AgentLoop, LoopEnd, LoopOutcome, ToolRegistry};
use snap_core::llm::CancellationToken;
use snap_core::tools::approver::ApprovalReply;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    LoopTempWorkdir, content_stream, counting_approver, loop_config_with_approver,
    tool_calls_stream,
};

/// `run_command` 인자 JSON 문자열.
fn command_args(command: &str) -> String {
    serde_json::json!({"command": command}).to_string()
}

/// 승인한 호출만 실행하고, 뒤따르는 호출은 승인자를 다시 타는지 확인.
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
    // 승인자가 두 번 불려야 호출마다 확인한 것. 한 번이면 표식이 실행 판에 남은 것.
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
    // 승인자가 거절한 호출은 셸에 넘어가지 않으므로 셸 실행 결과 표기가 없음.
    // 거부 문구는 승인 요약을 뒤에 붙이므로, 셸 결과 표기로 가림.
    assert!(
        !partial.tool_calls[1].result.contains("종료 코드"),
        "거부된 명령은 셸에 넘어가서는 안 됩니다: {}",
        partial.tool_calls[1].result
    );
}
