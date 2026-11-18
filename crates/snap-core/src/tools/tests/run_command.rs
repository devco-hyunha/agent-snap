//! `run_command` 도구 시험
//!
//! 셸을 쓰는 시험은 이 OS에서 확인이 쉬운 명령만 쓰고, 확인이 어려운 항목은 본문에 표기로 남김
//! 시간 상한은 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 봄

use std::sync::Arc;
use std::time::Duration;

use crate::config::AgentSettings;
use crate::tools::approver::{ApprovalFuture, ApprovalReply, ApproverBox, CommandApprover};
use crate::tools::run_command::RunCommandTool;
use crate::tools::run_command_settings::RunCommandSettings;
use crate::tools::tool::{Tool, ToolContext, ToolOutcome};

use super::support::TempFolder;

/// 응답을 즉시 돌려주는 가짜 승인자. 도구 단위는 승인자를 부르지 않으므로 표식으로만 사용
struct ApprovedApprover;

impl CommandApprover for ApprovedApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        Box::pin(async move { ApprovalReply::Approved })
    }
}

fn command_args(command: &str) -> String {
    serde_json::json!({ "command": command }).to_string()
}

/// 상한을 짧게 지정한 설정(실행 상한 초, 승인 대기 초)
fn run_settings(run_seconds: u64, approval_seconds: u64) -> RunCommandSettings {
    RunCommandSettings::from_settings(&AgentSettings {
        run_command_seconds: Duration::from_secs(run_seconds),
        approval_seconds: Duration::from_secs(approval_seconds),
    })
}

/// 승인자·시간 상한을 채운 작업 폴더 기준 실행 판 생성
fn run_context(folder: &TempFolder, settings: RunCommandSettings) -> ToolContext {
    let mut ctx = super::support::context_of(folder);
    ctx.run_command = settings;
    ctx.approver = Some(boxed_approved());
    ctx
}

fn boxed_approved() -> ApproverBox {
    Arc::new(ApprovedApprover)
}

/// 승인 표식이 없는 호출은 셸에 넘기지 않고 승인 요청만 반환(미실행)
#[tokio::test]
async fn call_without_approval_returns_approval_required() {
    let folder = TempFolder::new("run-noapproval");
    let ctx = super::support::context_of(&folder);
    let outcome = RunCommandTool
        .call(&command_args("echo snap-core-run"), &ctx)
        .await;
    assert!(
        matches!(outcome, ToolOutcome::ApprovalRequired { .. }),
        "승인 전에는 승인 요청만: {outcome:?}"
    );
    assert_eq!(RunCommandTool.name(), "run_command");
    assert!(RunCommandTool.executes_writes());
}

/// 승인 표식이 채워진 뒤에만 셸 명령이 실행되고, 표준 출력과 종료 코드가 결과에 남음
#[tokio::test]
async fn approved_command_runs_and_reports_output() {
    let folder = TempFolder::new("run-ok");
    let ctx = run_context(&folder, run_settings(20, 5));
    ctx.mark_approved();
    let outcome = RunCommandTool
        .call(&command_args("echo snap-core-run"), &ctx)
        .await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("승인된 명령은 실행되어야 합니다: {outcome:?}");
    };
    assert!(text.contains("종료 코드: 0"), "{text}");
    assert!(text.contains("snap-core-run"), "표준 출력: {text}");
}

/// 실행 상한(짧게 주입)을 넘기면 미완료로 알리고, 그 상한 안에서 복귀(넉넉한 상한만 확인)
#[tokio::test]
async fn run_deadline_reports_incomplete() {
    let folder = TempFolder::new("run-deadline");
    let ctx = run_context(&folder, run_settings(1, 5));
    ctx.mark_approved();
    let started = std::time::Instant::now();
    let outcome = RunCommandTool
        .call(&command_args(&long_sleep_command("20")), &ctx)
        .await;
    let ToolOutcome::Error(text) = outcome else {
        panic!("미완료 오류여야 합니다: {outcome:?}");
    };
    assert!(text.contains("미완료"), "{text}");
    assert!(
        started.elapsed() < Duration::from_secs(12),
        "실행 상한(1초) 안에 미완료로 돌아와야 합니다: {:?}",
        started.elapsed()
    );
}

/// cancel 토큰이 취소되면 실행을 중지하고 결과에 중단을 남김
#[tokio::test]
async fn cancel_token_stops_running_command() {
    let folder = TempFolder::new("run-cancel");
    let ctx = run_context(&folder, run_settings(20, 5));
    ctx.mark_approved();
    ctx.cancel_token().cancel();
    let outcome = RunCommandTool
        .call(&command_args(&long_sleep_command("20")), &ctx)
        .await;
    let ToolOutcome::Error(text) = outcome else {
        panic!("중단 오류여야 합니다: {outcome:?}");
    };
    assert!(text.contains("중단"), "{text}");
}

/// 승인 표식은 그 호출에서만 쓰임. 같은 판으로 두 번째 호출을 하면 다시 승인 요청만 반환
#[tokio::test]
async fn approval_flag_is_consumed_by_that_call() {
    let folder = TempFolder::new("run-flag-once");
    let ctx = run_context(&folder, run_settings(20, 5));
    ctx.mark_approved();
    let ToolOutcome::Done(first) = RunCommandTool
        .call(&command_args("echo snap-first"), &ctx)
        .await
    else {
        panic!("승인된 첫 호출은 실행되어야 합니다");
    };
    assert!(first.contains("snap-first"), "{first}");
    // 표식을 소비했으므로 같은 판에서의 둘째 호출은 셸에 넘기지 않고 승인 요청만 돌려줌
    let outcome = RunCommandTool
        .call(&command_args("echo snap-second"), &ctx)
        .await;
    assert!(
        matches!(outcome, ToolOutcome::ApprovalRequired { .. }),
        "둘째 호출은 승인자를 다시 타야 합니다: {outcome:?}"
    );
}

/// 승인자가 채워진 상태에서도 명령 문자열의 비밀 이름은 보조 검사로 거부
///
/// 셸이 명령을 나누는 문자(`;`·`|`·탭·줄바꿈)로 이어진 조각도 이름 판정 대상
#[tokio::test]
async fn secret_path_in_command_is_rejected_even_after_approval() {
    let folder = TempFolder::new("run-secret");
    let ctx = run_context(&folder, run_settings(20, 5));
    ctx.mark_approved();
    // 승인 표식은 호출마다 소비되므로 시험은 호출 전에 매번 채움
    for target in [
        "credentials.jsonc",
        "./sub/agent-snap.jsonc",
        ".env.local",
        "nested/dir/credentials.jsonc",
        "a;credentials.jsonc",
        "a|credentials.jsonc",
        "a\tcredentials.jsonc",
        "a\ncredentials.jsonc",
    ] {
        ctx.mark_approved();
        let outcome = RunCommandTool
            .call(&command_args(&format!("type {target}")), &ctx)
            .await;
        let ToolOutcome::Error(text) = outcome else {
            panic!("거부되어야 합니다({target}): {outcome:?}");
        };
        assert!(text.contains("비밀 파일"), "{target}: {text}");
    }
}

/// `command` 인자가 없거나 인자 JSON이 깨지면 한 줄 오류만 반환
#[tokio::test]
async fn missing_command_arg_is_one_line_error() {
    let folder = TempFolder::new("run-args");
    let ctx = super::support::context_of(&folder);
    for args in [r#"{ "path": "a.txt" }"#, "{\"command\":"] {
        let ToolOutcome::Error(text) = RunCommandTool.call(args, &ctx).await else {
            panic!("거부되어야 합니다: {args}");
        };
        assert!(text.starts_with("오류: "), "오류 한 줄 규격: {text}");
        assert_eq!(text.lines().count(), 1, "한 줄만: {text}");
    }
}

/// 승인된 명령은 작업 폴더를 기준으로 실행됨
///
/// 확인은 `pwd`·`cd` 출력이 아닌 파일 존재로 봄. 경계 밖은 승인자가 막음
#[tokio::test]
async fn approved_command_runs_inside_working_folder() {
    let folder = TempFolder::new("run-workdir");
    folder.write_file("marker.txt", "marker");
    let ctx = run_context(&folder, run_settings(20, 5));
    ctx.mark_approved();
    let outcome = RunCommandTool
        .call(&command_args(read_marker_command()), &ctx)
        .await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("실행되어야 합니다: {outcome:?}");
    };
    assert!(text.contains("marker"), "작업 폴더 안 파일 읽기: {text}");
}

#[cfg(windows)]
fn read_marker_command() -> &'static str {
    "type marker.txt"
}

#[cfg(not(windows))]
fn read_marker_command() -> &'static str {
    "cat marker.txt"
}

/// 셸이 두 운영체제에서 같은 의미로 기다리게 하는 긴 대기 명령
///
/// Windows는 `ping`, 그 밖의 운영체제는 `sleep`을 사용
#[cfg(windows)]
fn long_sleep_command(secs: &str) -> String {
    // Windows는 `ping`이 항목마다 약 1초를 기다림. 표준 입력이 없는 시험 환경에서도 그대로 대기
    format!("ping -n {secs} -w 1000 127.0.0.1 >nul")
}

#[cfg(not(windows))]
fn long_sleep_command(secs: &str) -> String {
    format!("sleep {secs}")
}
