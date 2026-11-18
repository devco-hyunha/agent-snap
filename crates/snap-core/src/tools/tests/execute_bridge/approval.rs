//! execute 브리지 승인 게이트 시험.

use crate::llm::ToolCall;
use crate::tools::approver::ApprovalReply;

use super::super::support::TempWorkdir;
use super::shared::execute_text;
use crate::agent::executor::run_tool_calls;
use crate::agent::record::registry::ToolRegistry;

/// 승인자 없는 상태(즉시 거부)에서는 쓰기 호출이 진행하지 않고 거부로 기록됨.
#[tokio::test]
async fn write_call_without_approver_is_rejected_and_logged() {
    let workdir = TempWorkdir::new("bridge-write-rejected");
    let text = super::shared::run_execute(
        &workdir,
        "await run_tool('write_file', JSON.stringify({path: 'x.txt', content: 'x'}));",
    )
    .await;
    assert!(
        text.split('\n')
            .next()
            .unwrap_or_default()
            .contains("승인 거부"),
        "거부 요약: {text}"
    );
    assert!(
        text.contains("[execute][1/1] write_file("),
        "기록 줄: {text}"
    );
    assert!(!workdir.path().join("x.txt").exists(), "쓰기 없어야 함");
}

/// 승인 창구를 지난 쓰기 호출만 실행되고, 기록은 호출마다 남음.
#[tokio::test]
async fn approved_write_call_runs_through_gate() {
    let workdir = TempWorkdir::new("bridge-write-approved");
    let mut ctx = workdir.context();
    ctx.approver = Some(super::super::approver::approved_approver());
    let text = execute_text(
        &ctx,
        "await run_tool('write_file', JSON.stringify({path: 'made.txt', content: '만들림'})); 'ok';",
    )
    .await;
    assert!(text.contains("ok"), "요약: {text}");
    assert!(
        workdir.path().join("made.txt").exists(),
        "승인된 쓰기는 실행되어야 합니다"
    );
}

/// `for` 반복 쓰기가 승인 1회로 여러 쓰기가 되지 않고, 쓰기마다 승인 창구를 탐.
#[tokio::test]
async fn repeated_write_calls_each_gate_the_same_place() {
    let workdir = TempWorkdir::new("bridge-write-repeat");
    let mut ctx = workdir.context();
    let (approver, counted) =
        super::super::approver::counting_approver_with_reply(&[ApprovalReply::Approved]);
    ctx.approver = Some(approver);
    let script = "const files = ['a.txt','b.txt','c.txt']; for (const f of files) { await run_tool('write_file', JSON.stringify({path: f, content: f})); } 'ok';";
    let text = execute_text(&ctx, script).await;
    assert!(text.contains("ok"), "요약: {text}");
    assert_eq!(
        counted.load(std::sync::atomic::Ordering::Relaxed),
        3,
        "쓰기 3건은 승인 창구 3번을 타야 합니다"
    );
    assert!(
        text.contains("[execute][3/3] write_file("),
        "기록 3번: {text}"
    );
    assert!(
        workdir.path().join("c.txt").exists(),
        "세 번째 쓰기까지 실행되어야 합니다"
    );
}

/// 승인 2개 기준(전체 확인 1승인 + 안의 쓰기 1승인)을 승인자 호출 횟수로 확인.
///
/// `execute`가 승인 요청을 내면 승인자를 부르고, 승인 뒤에 다시 불린 호출 안에서 안의 쓰기가 같은 승인자를 한 번 더 탐 → 합쳐 호출 2번.
#[tokio::test]
async fn full_execute_call_counts_two_approvals() {
    let workdir = TempWorkdir::new("bridge-ac5-two-approvals");
    let mut ctx = workdir.context();
    let (approver, counted) = super::super::approver::counting_approver_with_reply(&[
        ApprovalReply::Approved,
        ApprovalReply::Approved,
    ]);
    ctx.approver = Some(approver);
    let script =
        "await run_tool('write_file', JSON.stringify({path: 'ac5.txt', content: 'ac5'})); 'ok';";
    let calls = [ToolCall {
        id: "call-ac5-1".to_owned(),
        name: "execute".to_owned(),
        arguments: serde_json::json!({"script": script}).to_string(),
    }];
    let run = run_tool_calls(&ToolRegistry::with_agent_tools(), &ctx, &calls).await;
    assert_eq!(
        counted.load(std::sync::atomic::Ordering::Relaxed),
        2,
        "전체 확인 1개 + 안의 쓰기 1개가 승인자 호출이어야 합니다"
    );
    let record = run
        .records
        .iter()
        .find(|record| record.tool_name == "execute")
        .expect("execute 기록이 있어야 합니다");
    assert!(record.result.contains("ok"), "요약: {}", record.result);
    assert!(
        workdir.path().join("ac5.txt").exists(),
        "승인 2개를 지난 쓰기가 실행되어야 합니다"
    );
    assert!(
        run.rejection.is_none(),
        "승인이 통과하면 거부 기록이 없어야 합니다"
    );
}
