//! `glob` 도구 시험: 패턴 글자, 숨김 기본 제외, 비밀 거르기, 경계.
//!
//! 패턴 매처는 파일명·상대 경로 문자열만 봅니다(구분자는 `/`).

use crate::tools::glob::GlobTool;
use crate::tools::tool::Tool;

use super::support::TempFolder;

/// 도구를 돌려 결과 텍스트를 돌려줍니다. 오류도 한 줄 문자열로 봅니다.
async fn glob_out(folder: &TempFolder, args: &str) -> String {
    let ctx = super::support::context_of(folder);
    GlobTool.call(args, &ctx).await.text().to_owned()
}

/// `*`는 구분자를 넘지 않습니다: 상위 파일만 맞습니다.
#[tokio::test]
async fn star_does_not_cross_folders() {
    let folder = TempFolder::new("glob-star");
    folder.write_file("a.txt", "x");
    folder.write_file("sub/b.txt", "x");
    let text = glob_out(&folder, r#"{"pattern":"*.txt"}"#).await;
    assert!(text.contains("a.txt"), "{text}");
    assert!(
        !text.contains("sub/b.txt"),
        "*는 구분자를 넘지 않습니다: {text}"
    );
}

/// `**`는 폴더 사이까지 건넙니다.
#[tokio::test]
async fn double_star_crosses_folders() {
    let folder = TempFolder::new("glob-double");
    folder.write_file("a.txt", "x");
    folder.write_file("sub/deep/b.txt", "x");
    let text = glob_out(&folder, r#"{"pattern":"**/*.txt"}"#).await;
    assert!(text.contains("a.txt"), "{text}");
    assert!(text.contains("sub/deep/b.txt"), "{text}");
}

/// `?`는 한 글자만 맞습니다.
#[tokio::test]
async fn question_mark_matches_one_char() {
    let folder = TempFolder::new("glob-question");
    folder.write_file("ab.txt", "x");
    folder.write_file("a.txt", "x");
    folder.write_file("abc.txt", "x");
    let text = glob_out(&folder, r#"{"pattern":"a?.txt"}"#).await;
    assert!(text.contains("ab.txt"), "{text}");
    assert!(
        !text.contains("a.txt") && !text.contains("abc.txt"),
        "{text}"
    );
}

/// 숨김은 기본 제외, opt 포함. 비밀 이름은 어떤 opt에도 나오지 않습니다.
#[tokio::test]
async fn hidden_and_secret_entries() {
    let folder = TempFolder::new("glob-hidden");
    folder.write_file(".hidden.txt", "x");
    folder.write_file("shown.txt", "x");
    folder.write_file("agent-snap.jsonc", "{}");
    let text = glob_out(&folder, r#"{"pattern":"**","hidden":true}"#).await;
    assert!(text.contains("shown.txt"), "{text}");
    assert!(text.contains(".hidden.txt"), "opt 포함: {text}");
    assert!(!text.contains("agent-snap.jsonc"), "{text}");
}

/// 작업 폴더 밖 경로는 거부합니다(AC 2).
#[tokio::test]
async fn outside_path_is_refused() {
    let folder = TempFolder::new("glob-outside");
    let text = glob_out(&folder, r#"{"pattern":"*","path":"../escape.txt"}"#).await;
    assert!(text.starts_with("오류: "), "{text}");
    assert!(text.contains("경계 밖"), "{text}");
    assert_eq!(text.lines().count(), 1, "{text}");
}

/// 맞는 자리가 없으면 빈 결과입니다.
#[tokio::test]
async fn no_match_is_empty() {
    let folder = TempFolder::new("glob-none");
    folder.write_file("a.txt", "x");
    let text = glob_out(&folder, r#"{"pattern":"*.zzz"}"#).await;
    assert!(text.is_empty(), "빈 결과여야 합니다: {text}");
}

/// 등록 규칙대로 이름·설명·필수 인자·읽기 전용 표식을 한 자리에서 확인합니다.
#[test]
fn tool_shape_is_ready_for_registration() {
    assert_eq!(GlobTool.name(), "glob");
    assert!(!GlobTool.description().is_empty());
    let schema = GlobTool.parameters_schema();
    assert_eq!(schema["required"], serde_json::json!(["pattern"]));
    assert!(!GlobTool.executes_writes());
}

/// 패턴 종류가 틀리면 한 줄 오류입니다.
#[tokio::test]
async fn wrong_pattern_type_is_one_line_error() {
    let folder = TempFolder::new("glob-args");
    let text = glob_out(&folder, r#"{"pattern":5}"#).await;
    assert!(text.starts_with("오류: "), "{text}");
    assert_eq!(text.lines().count(), 1, "{text}");
}
