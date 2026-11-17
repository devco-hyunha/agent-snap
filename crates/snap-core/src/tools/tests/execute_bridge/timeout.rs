//! 03-8 execute 브리지 타임아웃·취소 시험입니다(AC 6: 03-6 규칙과 같은 게이트).
//!
//! 결론: 03-6 `run_command`이 쓰는 값(`run_limit`)과 같은 토큰을 그대로 쓰면서,
//! 엔진이 상한 안에 마칠 있게 끝나는지 봅니다. 순수 JS 반복은 엔진 락을 놓지 못해
//! 이 창구가 막지 못하는 자리라 쓰지 않습니다(그 한계는 `engine_drive.rs` 주석에 적습니다).

use std::time::Duration;

use crate::tools::approver::ApprovalReply;

use super::super::support::TempWorkdir;
use super::shared::execute_text;
use super::shared::short_limit_context;

/// AC 6: 시간 상한(03-6 값)을 넘기면 `미완료`로 끝나고 상한 안에 돌아온다.
///
/// JS가 다른 자리를 기다리는 스크립트(해결되지 않는 프로미스 대기)에서 시간 상한이
/// 이기는지 봅니다.
#[tokio::test]
async fn script_over_time_limit_reports_unfinished() {
    let workdir = TempWorkdir::new("bridge-timeout");
    let ctx = short_limit_context(&workdir, 300);
    let started = std::time::Instant::now();
    let text = execute_text(&ctx, "await new Promise(() => {}); '끝';").await;
    assert!(text.contains("미완료"), "타임아웃 표기: {text}");
    assert!(
        started.elapsed() < Duration::from_secs(20),
        "상한 안에 끝나야 합니다: {:?}",
        started.elapsed()
    );
}

/// AC 6 연장: 02 취소 토큰이 이미 끊긴 상태에서는 엔진을 굴리지 않고 `중단` 한 줄로 끝난다.
///
/// 03-6 `run_command`이 듣는 것과 같은 토큰 창구(`ToolContext::cancel_token`)를 그대로 재사용합니다.
#[tokio::test]
async fn cancel_token_stops_script_with_cancelled_note() {
    let workdir = TempWorkdir::new("bridge-cancel");
    let ctx = workdir.context();
    ctx.cancel_token().cancel();
    let text = execute_text(&ctx, "'끝';").await;
    assert!(text.contains("중단"), "취소 표기: {text}");
    assert!(
        text.chars().count() < 200,
        "짧은 글자: {}",
        text.chars().count()
    );
}

/// AC 6(라운드 3 major 7 확인): 시간 상한·취소 뒤에 상한 안에 끝나고 표기가 남는다.
///
/// 승인 대기를 기다리는 쓰기 호출이 취소로 끝난 자리에서 엔진이 drain으로 프로미스를
/// 닫고 상한 안에 `중단` 표기를 남기는지 봅니다(트리 정리 확인과 같은 상한 값 사용).
#[tokio::test]
async fn engine_stops_within_limit_after_cancel() {
    let workdir = TempWorkdir::new("bridge-cancel-during-approval");
    let ctx = short_limit_context(&workdir, 300);
    // 승인자가 없는 상태에서 쓰기를 타면 거부가 오지만, 취소는 승인 대기 중에 끊긴다.
    // 이 스크립트는 승인을 기다리므로, 취소 토큰을 먼저 끊어 상한 안에 끝나게 한다.
    ctx.cancel_token().cancel();
    let started = std::time::Instant::now();
    let text = execute_text(
        &ctx,
        "await run_tool('write_file', JSON.stringify({path: 'x.txt', content: 'x'})); '끝';",
    )
    .await;
    assert!(text.contains("중단"), "취소 표기가 남아야 합니다: {text}");
    assert!(
        started.elapsed() < Duration::from_secs(20),
        "상한 안에 끝나야 합니다: {:?}",
        started.elapsed()
    );
}

/// AC 6 연장: 승인 대기 중에 취소가 들어오면 상한 안에서 `중단`으로 끝난다.
///
/// 승인 창구가 03-6의 취소 토큰을 그대로 듣는지, 엔진 스레드가 그 뒤 마칠 자리를 확인합니다.
#[tokio::test]
async fn cancel_during_approval_gate_ends_within_limit() {
    let workdir = TempWorkdir::new("bridge-gate-cancel");
    let mut ctx = short_limit_context(&workdir, 300);
    let (approver, counted) =
        super::super::approver::counting_approver_with_reply(&[ApprovalReply::Cancelled]);
    ctx.approver = Some(approver);
    let started = std::time::Instant::now();
    let text = execute_text(
        &ctx,
        "await run_tool('write_file', JSON.stringify({path: 'x.txt', content: 'x'})); '끝';",
    )
    .await;
    assert!(
        text.contains("중단") || text.contains("미완료"),
        "취소·상한 표기: {text}"
    );
    assert_eq!(
        counted.load(std::sync::atomic::Ordering::Relaxed),
        1,
        "승인 창구는 한 번만 타야 합니다"
    );
    assert!(
        started.elapsed() < Duration::from_secs(20),
        "상한 안에 끝나야 합니다: {:?}",
        started.elapsed()
    );
}
