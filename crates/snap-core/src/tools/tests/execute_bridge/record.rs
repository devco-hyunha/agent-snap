//! execute 브리지 기록·실패 기록 시험.
//!
//! 기록 줄 형식과 짧은 오류 글자만 확인. 길이 책임은 `length.rs`가 봄.

use super::super::support::TempWorkdir;
use super::shared::run_execute;

/// 호출 기록이 `[execute][i/total]` 형식으로 호출마다 남고 결과에 기록 칸으로 붙음.
#[tokio::test]
async fn every_call_leaves_a_record_line_in_the_result() {
    let workdir = TempWorkdir::new("bridge-record-lines");
    workdir.write("notes.txt", "hello bridge\n");
    let text = run_execute(
        &workdir,
        "const a = await read_file({path: 'notes.txt'}); const b = await list_dir({path: '.'}); `${a.trim()}|${b.split('\\n').length}`;",
    )
    .await;
    assert!(
        text.contains("기록: [execute][1/1] read_file(")
            && text.contains("[execute][2/2] list_dir("),
        "기록 줄 2개: {text}"
    );
    assert!(text.lines().count() == 3, "요약 줄과 기록 줄 2개만: {text}");
}

/// 런타임 오류는 짧은 오류 글자로만 돌아오고 스크립트 원문이 결과에 통째로 오지 않음.
#[tokio::test]
async fn runtime_error_returns_short_text_without_script_source() {
    let workdir = TempWorkdir::new("bridge-runtime-error");
    let text = run_execute(&workdir, "const broken = not_a_function(); broken;").await;
    assert!(text.starts_with("오류:"), "오류 글자: {text}");
    assert!(!text.contains("not_a_function"), "원문 제외: {text}");
    assert!(
        text.chars().count() < 200,
        "짧은 글자: {}",
        text.chars().count()
    );
}

/// 구문 오류도 짧은 오류 글자로 돌아옴.
#[tokio::test]
async fn syntax_error_returns_short_text() {
    let workdir = TempWorkdir::new("bridge-syntax-error");
    let text = run_execute(&workdir, "const =;").await;
    assert!(text.starts_with("오류:"), "오류 글자: {text}");
    assert!(
        text.chars().count() < 200,
        "짧은 글자: {}",
        text.chars().count()
    );
}
