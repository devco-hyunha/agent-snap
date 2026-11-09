//! `edit_file` 도구 시험(spec AC 3).
//!
//! 일치가 하나인 교체, 일치가 없는 거부, 여러 곳에서 맞는 거부, 파일을 고치지 않고 두는
//! 확인을 본다.

use crate::tools::edit_file::EditFileTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::support::TempFolder;

async fn edit_out(folder: &TempFolder, args_json: &str) -> ToolOutcome {
    let ctx = super::support::context_of(folder);
    EditFileTool.call(args_json, &ctx).await
}

fn edit_args(path: &str, find: &str, replace: &str) -> String {
    serde_json::json!({
        "path": path,
        "find": find,
        "replace": replace
    })
    .to_string()
}

/// 한 줄 오류 결과인지 확인하고 문장을 받아 온다(spec AC 3의 짧은 오류).
fn assert_one_line_error(outcome: ToolOutcome, expected: &str) {
    let ToolOutcome::Error(text) = outcome else {
        panic!("거부되어야 합니다");
    };
    assert!(text.starts_with("오류: "), "오류 한 줄 규격: {text}");
    assert!(text.contains(expected), "{expected}이(가) 없음: {text}");
    assert_eq!(text.lines().count(), 1, "오류는 한 줄만: {text}");
}

/// AC 3: 일치가 하나만 있으면 그 자리만 바꿉니다.
#[tokio::test]
async fn single_match_is_replaced() {
    let folder = TempFolder::new("edit-one");
    folder.write_file("code.rs", "fn a() {}\n");
    let ToolOutcome::Done(text) =
        edit_out(&folder, &edit_args("code.rs", "fn a()", "fn b()")).await
    else {
        panic!("교체가 성공해야 합니다");
    };
    assert!(text.contains("교체 성공"), "{text}");
    let saved = std::fs::read_to_string(folder.path().join("code.rs")).expect("파일");
    assert!(saved.contains("fn b()"));
    assert!(!saved.contains("fn a()"));
}

/// AC 3: 일치가 둘 이상이면 교체하지 않고 이유만 알리고 파일을 그대로 둣니다.
#[tokio::test]
async fn multiple_match_is_rejected_and_file_untouched() {
    let folder = TempFolder::new("edit-multi");
    folder.write_file("dup.rs", "same\nsame\n");
    let original = std::fs::read_to_string(folder.path().join("dup.rs")).expect("파일");
    assert_one_line_error(
        edit_out(&folder, &edit_args("dup.rs", "same", "other")).await,
        "다중 일치",
    );
    let after = std::fs::read_to_string(folder.path().join("dup.rs")).expect("파일");
    assert_eq!(after, original, "다중 일치면 파일을 고치지 않습니다");
}

/// AC 3: 일치가 없으면 이유만 알립니다.
#[tokio::test]
async fn no_match_is_rejected() {
    let folder = TempFolder::new("edit-none");
    folder.write_file("none.rs", "text");
    assert_one_line_error(
        edit_out(&folder, &edit_args("none.rs", "없는문자열", "other")).await,
        "일치 없음",
    );
}

/// AC 3: 바꿀 문자열이 찾을 문자열과 같아도 일치가 하나면 성공이고 파일은 그대로입니다.
#[tokio::test]
async fn same_find_and_replace_keeps_file() {
    let folder = TempFolder::new("edit-same");
    folder.write_file("same.rs", "고유");
    let outcome = edit_out(&folder, &edit_args("same.rs", "고유", "고유")).await;
    assert!(matches!(outcome, ToolOutcome::Done(_)));
    let saved = std::fs::read_to_string(folder.path().join("same.rs")).expect("파일");
    assert_eq!(saved, "고유", "바꿀 문자열이 같으면 파일은 그대로입니다");
}

/// AC 2: 작업 폴더 밖 경로는 거부합니다.
#[tokio::test]
async fn out_of_boundary_is_rejected() {
    let folder = TempFolder::new("edit-guard");
    assert_one_line_error(
        edit_out(&folder, &edit_args("../out.rs", "a", "b")).await,
        "경계 밖",
    );
}

/// AC 4: 비밀 파일 편집은 거부하고 파일을 고치지 않습니다.
#[tokio::test]
async fn secret_file_is_blocked_and_untouched() {
    let folder = TempFolder::new("edit-secret");
    folder.write_file("agent-snap.jsonc", "{}");
    assert_one_line_error(
        edit_out(&folder, &edit_args("agent-snap.jsonc", "{}", "[]")).await,
        "비밀 파일",
    );
    let saved = std::fs::read_to_string(folder.path().join("agent-snap.jsonc")).expect("원본");
    assert_eq!(saved, "{}", "비밀 파일은 고치지 않습니다");
}

/// AC 3: `.git` 아래 편집은 거부합니다(D49).
#[tokio::test]
async fn git_edit_is_blocked() {
    let folder = TempFolder::new("edit-git");
    folder.write_file(".git/config", "a");
    assert_one_line_error(
        edit_out(&folder, &edit_args(".git/config", "a", "b")).await,
        ".git",
    );
}

/// AC 3: `find` 인자가 없으면 이유 한 줄만 돌려줍니다.
#[tokio::test]
async fn missing_find_arg_is_one_line_error() {
    let folder = TempFolder::new("edit-args");
    assert_one_line_error(
        edit_out(&folder, r#"{ "path": "a.rs" }"#).await,
        "find는 문자열이어야 합니다",
    );
}

/// AC 3: 등록 규칙대로 이름과 필수 인자 목록을 한 자리에서 확인합니다.
#[test]
fn tool_shape_is_ready_for_registration() {
    assert_eq!(EditFileTool.name(), "edit_file");
    assert!(EditFileTool.executes_writes());
    let schema = EditFileTool.parameters_schema();
    assert_eq!(
        schema["required"],
        serde_json::json!(["path", "find", "replace"])
    );
}
