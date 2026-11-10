//! `read_file` 도구 시험(spec AC 1·1a·AC 2·AC 4).
//!
//! 크기·구간 기준은 D49 값(256KB·128KB·50KiB)을 그대로 두고, 잘림 표식과 4,000자 상한은
//! 03-2 정본(`agent/truncate.rs`, `agent/config.rs`)을 재사용합니다.

use crate::agent::{DEFAULT_TOOL_RESULT_MAX_CHARS, TRUNCATION_MARK, truncate_tool_result};
use crate::tools::read_file::ReadFileTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::support::TempFolder;

/// 도구 결과를 한 줄 문자열로 받아 온다.
async fn read_out(folder: &TempFolder, args_json: &str) -> String {
    let ctx = super::support::context_of(folder);
    match ReadFileTool.call(args_json, &ctx).await {
        ToolOutcome::Done(text) => text,
        other => other.text().to_owned(),
    }
}

/// AC 1: 기본 읽기가 앞 128KB만 보여주고 잘림 표식을 붙입니다.
#[tokio::test]
async fn base_read_shows_head_with_truncation_mark() {
    let folder = TempFolder::new("read-head");
    // 128KB와 256KB 사이 내용(한 글자 3바이트 × 45,000 = 135KB)으로 잘림을 확인합니다.
    folder.write_file("big.txt", &format!("{}\n", "가".repeat(45_000)));
    let text = read_out(&folder, r#"{"path":"big.txt"}"#).await;
    assert!(text.starts_with("1: "), "줄 번호가 붙어야 합니다");
    assert!(
        text.ends_with(TRUNCATION_MARK),
        "잘림 표식이 마지막이어야 합니다"
    );
}

/// AC 1: 4,000자 안에 드는 짧은 파일은 표식 없이 줄 번호만 붙습니다.
#[tokio::test]
async fn short_file_has_line_numbers_without_mark() {
    let folder = TempFolder::new("read-short");
    folder.write_file("short.txt", "첫째\n둘째\n");
    let text = read_out(&folder, r#"{"path":"short.txt"}"#).await;
    assert!(text.contains("1: 첫째"));
    assert!(text.contains("2: 둘째"));
    assert!(
        !text.contains(TRUNCATION_MARK),
        "짧으면 표식이 없어야 합니다"
    );
}

/// AC 1a: 256KB 넘는 파일은 기본 읽기를 거부하고 안내를 남깁니다.
#[tokio::test]
async fn base_read_rejects_file_over_256kb_with_hint() {
    let folder = TempFolder::new("read-reject");
    folder.write_file("big.txt", &"x".repeat(256 * 1024 + 10));
    let reason = read_out(&folder, r#"{"path":"big.txt"}"#).await;
    assert!(reason.contains("읽기 한도 초과"), "{reason}");
    assert!(
        reason.contains("offset") && reason.contains("limit") && reason.contains("grep"),
        "구간·검색 안내가 있어야 합니다: {reason}"
    );
}

/// AC 1a: 256KB 넘는 파일도 구간을 지정하면 읽고, 구간 결과는 4,000자 잘림을 적용합니다.
#[tokio::test]
async fn section_read_of_large_file_is_allowed() {
    let folder = TempFolder::new("read-section");
    let mut lines: Vec<String> = Vec::new();
    while lines.join("\n").len() < 260 * 1024 {
        lines.push(format!("{}번째 줄입니다", lines.len() + 1));
    }
    folder.write_file("many.txt", &format!("{}\n", lines.join("\n")));
    let text = read_out(&folder, r#"{"path":"many.txt","offset":2,"limit":300}"#).await;
    assert!(text.starts_with("2: "), "구간은 시작 줄 번호부터 붙입니다");
    assert_eq!(text.lines().count(), 300, "limit만큼만 읽어야 합니다");
    // 구간 결과가 상한보다 길어야 루프 잘림을 그대로 확인합니다.
    assert!(
        text.chars().count() > DEFAULT_TOOL_RESULT_MAX_CHARS,
        "상한보다 긴 결과여야 합니다: {}",
        text.chars().count()
    );
    // 구간 결과도 루프가 자르는 자리(4,000자)에서 표식을 만납니다.
    let sent = truncate_tool_result(&text, DEFAULT_TOOL_RESULT_MAX_CHARS);
    assert!(
        sent.ends_with(TRUNCATION_MARK),
        "4,000자 자리에서 잘림 확인"
    );
}

/// AC 1a: 구간 한 장이 50KiB를 넘으면 읽지 않고 이유를 남깁니다.
#[tokio::test]
async fn section_read_refuses_page_over_50kib() {
    let folder = TempFolder::new("read-page");
    folder.write_file("wide.txt", &"가".repeat(60 * 1024));
    let reason = read_out(&folder, r#"{"path":"wide.txt","offset":1,"limit":100}"#).await;
    assert!(reason.contains("50KiB"), "{reason}");
}

/// AC 1: 바이너리 파일은 읽지 않고 이유를 남깁니다.
#[tokio::test]
async fn binary_file_is_refused() {
    let folder = TempFolder::new("read-binary");
    folder.write_bytes("bin.dat", &[0_u8, 0x41, 0xff, 0xfe]);
    let reason = read_out(&folder, r#"{"path":"bin.dat"}"#).await;
    assert!(reason.contains("바이너리"), "{reason}");
}

/// AC 1: 없는 파일은 이유 한 줄로 알립니다.
#[tokio::test]
async fn missing_file_is_one_line_error() {
    let folder = TempFolder::new("read-missing");
    let reason = read_out(&folder, r#"{"path":"없는.txt"}"#).await;
    assert!(reason.starts_with("오류: "), "오류 한 줄 규격: {reason}");
    assert_eq!(reason.lines().count(), 1, "한 줄만: {reason}");
}

/// AC 1a: `offset`·`limit`은 1부터 받습니다.
#[tokio::test]
async fn offset_below_one_is_refused() {
    let folder = TempFolder::new("read-offset");
    folder.write_file("ok.txt", "text");
    let reason = read_out(&folder, r#"{"path":"ok.txt","offset":0}"#).await;
    assert!(reason.contains("1부터"), "{reason}");
}

/// AC 2·AC 4: 경계 밖·비밀 파일 경로는 도구 오류 한 줄로 거부합니다.
#[tokio::test]
async fn path_guard_refusals_reach_tool_error_line() {
    let folder = TempFolder::new("read-guard");
    folder.write_file("credentials.jsonc", "{\"api_keys\":{}}");
    let ctx = super::support::context_of(&folder);
    for raw in [
        r#"{"path":"../out.txt"}"#,
        r#"{"path":"credentials.jsonc"}"#,
        r#"{"path":".env"}"#,
        r#"{"path":".env.local"}"#,
    ] {
        let outcome = ReadFileTool.call(raw, &ctx).await;
        let line = outcome.text();
        assert!(line.starts_with("오류: "), "오류 한 줄 규격: {line}");
        assert_eq!(line.lines().count(), 1, "오류는 한 줄만: {line}");
    }
    // 비밀 파일 내용은 결과에 남지 않습니다(AC 4).
    let outcome = ReadFileTool
        .call(r#"{"path":"credentials.jsonc"}"#, &ctx)
        .await;
    assert!(
        !outcome.text().contains("api_keys"),
        "비밀 내용이 노출되지 않아야 합니다"
    );
}

/// AC 1a: 4,000자 잘림 정본을 거치면 결과가 표식으로 줄어듭니다.
#[tokio::test]
async fn read_result_through_loop_truncation_has_mark() {
    let folder = TempFolder::new("read-truncate");
    folder.write_file("long.txt", &"나다라".repeat(2_000));
    let ctx = super::support::context_of(&folder);
    let outcome = ReadFileTool
        .call(r#"{"path":"long.txt","offset":1,"limit":800}"#, &ctx)
        .await;
    let sent = truncate_tool_result(outcome.text(), ctx.tool_result_max_chars);
    assert!(sent.ends_with(TRUNCATION_MARK), "4,000자 상한 표식 확인");
}

/// AC 1: 인자 JSON이 깨지면 이유 한 줄만 돌려줍니다.
#[tokio::test]
async fn broken_args_is_one_line_error() {
    let folder = TempFolder::new("read-args");
    folder.write_file("ok.txt", "text");
    let reason = read_out(&folder, "{\"path\":").await;
    assert!(reason.starts_with("오류: "), "{reason}");
    assert_eq!(reason.lines().count(), 1, "한 줄만: {reason}");
}
