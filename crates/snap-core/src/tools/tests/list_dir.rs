//! `list_dir` 도구 시험: 경계·비밀 거르기, 숨김 기본 제외, 재귀 opt, 줄 상한.
//!
//! 비밀 이름 정본은 `path_guard`(D17), 줄 상한 정본은 `display`(spec AC 5)를 그대로 씁니다.

use crate::agent::TRUNCATION_MARK;
use crate::tools::list_dir::ListDirTool;
use crate::tools::tool::Tool;

use super::support::TempFolder;

/// 도구를 돌려 결과 텍스트를 돌려줍니다. 오류도 한 줄 문자열로 봅니다.
async fn list_out(folder: &TempFolder, args: &str) -> String {
    let ctx = super::support::context_of(folder);
    ListDirTool.call(args, &ctx).await.text().to_owned()
}

/// 기본은 비재귀: 하위 폴더는 이름만, 그 안 파일은 나오지 않습니다.
#[tokio::test]
async fn default_listing_is_non_recursive_and_relative() {
    let folder = TempFolder::new("list-basic");
    folder.write_file("a.txt", "x");
    folder.write_file("sub/inner.txt", "x");
    let text = list_out(&folder, "{}").await;
    assert!(text.contains("a.txt"), "{text}");
    assert!(text.contains("sub"), "{text}");
    assert!(
        !text.contains("inner.txt"),
        "기본은 재귀가 아닙니다: {text}"
    );
}

/// `recursive` opt가 하위 폴더 파일을 담습니다.
#[tokio::test]
async fn recursive_opt_includes_nested_files() {
    let folder = TempFolder::new("list-recursive");
    folder.write_file("sub/inner.txt", "x");
    let text = list_out(&folder, r#"{"recursive":true}"#).await;
    assert!(text.contains("sub/inner.txt"), "{text}");
}

/// 숨김 자리는 기본 제외, opt로 포함됩니다.
#[tokio::test]
async fn hidden_entries_excluded_by_default_and_included_opt() {
    let folder = TempFolder::new("list-hidden");
    folder.write_file(".hidden.txt", "x");
    folder.write_file("shown.txt", "x");
    let text = list_out(&folder, "{}").await;
    assert!(text.contains("shown.txt"), "{text}");
    assert!(!text.contains(".hidden.txt"), "숨김 기본 제외: {text}");
    let text = list_out(&folder, r#"{"hidden":true}"#).await;
    assert!(text.contains(".hidden.txt"), "opt 포함: {text}");
}

/// 비밀 파일 이름은 자리와 상관없이 늘 거립니다(AC 4).
#[tokio::test]
async fn secret_files_are_never_listed() {
    let folder = TempFolder::new("list-secret");
    folder.write_file("agent-snap.jsonc", "{}");
    folder.write_file("credentials.jsonc", "{}");
    folder.write_file("nested/credentials.jsonc", "{}");
    let text = list_out(&folder, r#"{"recursive":true}"#).await;
    for secret in ["agent-snap.jsonc", "credentials.jsonc"] {
        assert!(!text.contains(secret), "{secret}이 새지 않았습니다: {text}");
    }
}

/// 작업 폴더 밖 경로는 거부합니다(AC 2).
#[tokio::test]
async fn outside_path_is_refused() {
    let folder = TempFolder::new("list-outside");
    let text = list_out(&folder, r#"{"path":"../escape.txt"}"#).await;
    assert!(text.starts_with("오류: "), "{text}");
    assert!(text.contains("경계 밖"), "{text}");
    assert_eq!(text.lines().count(), 1, "{text}");
}

/// 자리가 폴더가 아니면 이유를 알립니다.
#[tokio::test]
async fn file_path_is_refused_as_folder() {
    let folder = TempFolder::new("list-file");
    folder.write_file("a.txt", "x");
    let text = list_out(&folder, r#"{"path":"a.txt"}"#).await;
    assert!(text.starts_with("오류: "), "{text}");
    assert!(text.contains("폴더가"), "{text}");
}

/// 등록 규칙대로 이름·설명·필수 인자·읽기 전용 표식을 한 자리에서 확인합니다.
#[test]
fn tool_shape_is_ready_for_registration() {
    assert_eq!(ListDirTool.name(), "list_dir");
    assert!(!ListDirTool.description().is_empty());
    let schema = ListDirTool.parameters_schema();
    assert_eq!(schema["required"], serde_json::json!([]));
    assert!(!ListDirTool.executes_writes());
}

/// 200줄을 넘으면 잘림 표식을 붙입니다(AC 5).
#[tokio::test]
async fn listing_over_200_lines_truncates_with_mark() {
    let folder = TempFolder::new("list-cap");
    for index in 0..201 {
        folder.write_file(&format!("f{index:03}.txt"), "x");
    }
    let text = list_out(&folder, r#"{"recursive":true}"#).await;
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 201, "200줄+표식: {text}");
    assert_eq!(lines.last(), Some(&TRUNCATION_MARK), "{text}");
}
