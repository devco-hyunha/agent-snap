//! execute 브리지 타임아웃·취소 시험.
//!
//! `run_command`가 쓰는 값(`run_limit`)·같은 토큰을 그대로 쓰면서 엔진이 상한 안에 마칠 있게 끝남 확인. 순수 JS 반복은 엔진 락을 놓지 못해 이 창구가 막지 못하는 자리라 쓰지 않음.

use std::time::Duration;

use crate::tools::approver::ApprovalReply;

use super::super::support::TempWorkdir;
use super::shared::execute_text;
use super::shared::short_limit_context;

/// 시간 상한(`run_command` 값)을 넘기면 `미완료`로 끝나고 상한 안에 돌아옴.
///
/// 해결되지 않는 프로미스를 기다리는 스크립트에서 시간 상한이 이기는지 확인.
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

/// 취소 토큰이 이미 끊긴 상태에서는 엔진을 굴리지 않고 `중단` 한 줄로 끝남.
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

/// 시간 상한·취소 뒤에 상한 안에 끝나고 `중단` 표기가 남음.
#[tokio::test]
async fn engine_stops_within_limit_after_cancel() {
    let workdir = TempWorkdir::new("bridge-cancel-during-approval");
    let ctx = short_limit_context(&workdir, 300);
    // 쓰기는 승인을 기다리므로 취소 토큰을 먼저 끊어 상한 안에 끝나게 함.
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

/// 승인 대기 중에 취소가 들어오면 상한 안에서 `중단`으로 끝남.
///
/// 승인 창구가 취소 토큰을 그대로 듣는지, 엔진 스레드가 그 뒤 마칠 자리 확인.
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
