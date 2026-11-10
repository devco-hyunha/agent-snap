//! `write_file` 도구 시험(spec AC 2와 AC 4, D49).
//!
//! 새로 쓰기·덮어쓰기 표시, 경계 밖 거부, 비밀 파일과 `.git` 쓰기를 막는 결과를 확인한다.

use crate::tools::tool::{Tool, ToolOutcome};
use crate::tools::write_file::WriteFileTool;

use super::support::TempFolder;

async fn write_out(folder: &TempFolder, args_json: &str) -> ToolOutcome {
    let ctx = super::support::context_of(folder);
    WriteFileTool.call(args_json, &ctx).await
}

fn write_args(path: &str, content: &str) -> String {
    serde_json::json!({ "path": path, "content": content }).to_string()
}

/// AC 4: 없는 파일을 쓰면 새로 쓰기 표시와 함께 경로를 보여줍니다.
#[tokio::test]
async fn new_file_is_written_with_new_marker() {
    let folder = TempFolder::new("write-new");
    let ToolOutcome::Done(text) = write_out(&folder, &write_args("notes/a.txt", "내용")).await
    else {
        panic!("쓰기가 성공해야 합니다");
    };
    assert!(text.contains("새로 쓰기"), "{text}");
    assert!(text.contains("notes/a.txt"), "경로 표시: {text}");
    assert_eq!(
        std::fs::read_to_string(folder.path().join("notes/a.txt")).expect("쓴 파일"),
        "내용"
    );
}

/// AC 4: 기존 파일을 쓰면 덮어쓰기 표시를 결과에 담습니다.
#[tokio::test]
async fn overwrite_marks_existing_file() {
    let folder = TempFolder::new("write-over");
    folder.write_file("keep.txt", "옛날");
    let ToolOutcome::Done(text) = write_out(&folder, &write_args("keep.txt", "새 내용")).await
    else {
        panic!("쓰기가 성공해야 합니다");
    };
    assert!(text.contains("덮어쓰기"), "{text}");
    assert_eq!(
        std::fs::read_to_string(folder.path().join("keep.txt")).expect("쓴 파일"),
        "새 내용"
    );
}

/// AC 4: 쓰기는 승인 요청을 만들지 않고 표시로만 알립니다(D49).
#[tokio::test]
async fn write_never_asks_for_approval() {
    let folder = TempFolder::new("write-flag");
    let outcome = write_out(&folder, &write_args("ok.txt", "내용")).await;
    assert!(
        !matches!(outcome, ToolOutcome::ApprovalRequired { .. }),
        "쓰기는 승인 대상이 아닙니다(D49)"
    );
    // 03-8 승인 구분 기준에는 쓰기 도구로 표기됩니다.
    assert!(WriteFileTool.executes_writes());
}

/// AC 2: 작업 폴더 밖 경로는 거부하고 파일을 만들지 않습니다.
#[tokio::test]
async fn out_of_boundary_is_rejected() {
    let folder = TempFolder::new("write-guard");
    let ToolOutcome::Error(text) = write_out(&folder, &write_args("../out.txt", "내용")).await
    else {
        panic!("거부되어야 합니다");
    };
    assert!(text.starts_with("오류: "), "오류 한 줄 규격: {text}");
    assert!(text.contains("경계 밖"), "{text}");
    let outside = folder.path().parent().expect("부모 폴더").join("out.txt");
    assert!(
        !outside.exists(),
        "거부된 경로에 파일을 만들지 않아야 합니다"
    );
}

/// AC 4: 비밀 파일 쓰기는 거부하고 기존 내용을 건드리지 않습니다.
#[tokio::test]
async fn secret_file_write_is_blocked() {
    let folder = TempFolder::new("write-secret");
    folder.write_file("agent-snap.jsonc", "{}");
    let ToolOutcome::Error(text) = write_out(
        &folder,
        &write_args("agent-snap.jsonc", "{\"endpoints\":[]}"),
    )
    .await
    else {
        panic!("거부되어야 합니다");
    };
    assert!(text.contains("비밀 파일"), "{text}");
    assert_eq!(
        std::fs::read_to_string(folder.path().join("agent-snap.jsonc")).expect("원본"),
        "{}",
        "비밀 파일 내용은 그대로여야 합니다"
    );
}

/// AC 4: 거부 오류에 비밀 파일 안 값(키 이름과 키 값)을 넣지 않습니다.
#[tokio::test]
async fn secret_write_error_does_not_expose_values() {
    let folder = TempFolder::new("write-secret-text");
    folder.write_file(
        "credentials.jsonc",
        "{\"api_keys\":{\"local\":\"secret-key\"}}",
    );
    let ToolOutcome::Error(text) =
        write_out(&folder, &write_args("credentials.jsonc", "내용")).await
    else {
        panic!("거부되어야 합니다");
    };
    assert!(
        !text.contains("api_keys"),
        "키 이름이 노출되지 않습니다: {text}"
    );
    assert!(
        !text.contains("secret-key"),
        "키 값이 노출되지 않습니다: {text}"
    );
}

/// AC 2: `.git` 아래 쓰기는 거부합니다(D49).
#[tokio::test]
async fn git_write_is_blocked() {
    let folder = TempFolder::new("write-git");
    let ToolOutcome::Error(text) = write_out(&folder, &write_args(".git/config", "내용")).await
    else {
        panic!("거부되어야 합니다");
    };
    assert!(text.contains(".git"), "{text}");
}

/// AC 4: `content` 인자가 없으면 이유 한 줄만 돌려줍니다.
#[tokio::test]
async fn missing_content_arg_is_one_line_error() {
    let folder = TempFolder::new("write-args");
    let ToolOutcome::Error(text) = write_out(&folder, r#"{ "path": "a.txt" }"#).await else {
        panic!("거부되어야 합니다");
    };
    assert!(text.starts_with("오류: "), "{text}");
    assert_eq!(text.lines().count(), 1, "한 줄만: {text}");
}

/// AC 4: 인자 JSON이 깨지면 이유 한 줄만 돌려줍니다.
#[tokio::test]
async fn broken_args_is_one_line_error() {
    let folder = TempFolder::new("write-broken");
    let ToolOutcome::Error(text) = write_out(&folder, "{\"path\":").await else {
        panic!("거부되어야 합니다");
    };
    assert!(text.starts_with("오류: "), "{text}");
    assert_eq!(text.lines().count(), 1, "한 줄만: {text}");
}

/// AC 4: 등록 규칙대로 이름과 설명과 필수 인자 목록을 한 자리에서 확인합니다.
#[test]
fn tool_shape_is_ready_for_registration() {
    assert_eq!(WriteFileTool.name(), "write_file");
    assert!(!WriteFileTool.description().is_empty());
    let schema = WriteFileTool.parameters_schema();
    assert_eq!(schema["required"], serde_json::json!(["path", "content"]));
}
