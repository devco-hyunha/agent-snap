//! `grep` 도구 시험: 3형식, 대소문자 opt, 확장자 필터, 200줄 잘림, 비밀·숨김 거르기, 경계.
//!
//! `PATH`에 ripgrep이 있으면 ripgrep 경로, 없으면 내장 경로로 돌아 시험을 확인합니다.
//! 두 경로가 같은 결과를 내야 하는 기준 시험이고, 개발 규칙(두 OS 동등)에 맞춰 둘 중
//! 어느 환경에서 돌려도 통과해야 합니다.

use crate::agent::TRUNCATION_MARK;
use crate::tools::grep::GrepTool;
use crate::tools::tool::Tool;

use super::support::TempFolder;

/// 도구를 돌려 결과 텍스트를 돌려줍니다. 오류도 한 줄 문자열로 봅니다.
async fn grep_out(folder: &TempFolder, args: &str) -> String {
    let ctx = super::support::context_of(folder);
    GrepTool.call(args, &ctx).await.text().to_owned()
}

/// content 형식은 상대 경로:줄번호:내용을 담아 드립니다.
#[tokio::test]
async fn content_format_lists_matching_lines() {
    let folder = TempFolder::new("grep-content");
    folder.write_file("a.txt", "첫째 줄\n둘째 줄\n첫째 다시\n");
    folder.write_file("sub/b.txt", "첫째 여기\n");
    let text = grep_out(&folder, r#"{"pattern":"첫째"}"#).await;
    assert!(text.contains("a.txt:1:첫째 줄"), "{text}");
    assert!(text.contains("a.txt:3:첫째 다시"), "{text}");
    assert!(text.contains("sub/b.txt:1:첫째 여기"), "{text}");
}

/// 대소문자는 기본 구분, opt로 풉니다.
#[tokio::test]
async fn case_sensitive_default_and_opt() {
    let folder = TempFolder::new("grep-case");
    folder.write_file("c.txt", "Alpha\nalpha\n");
    let text = grep_out(&folder, r#"{"pattern":"alpha"}"#).await;
    assert!(text.contains("c.txt:2:alpha"), "{text}");
    assert!(!text.contains("c.txt:1:"), "{text}");
    let text = grep_out(&folder, r#"{"pattern":"alpha","case_sensitive":false}"#).await;
    assert!(text.contains("c.txt:1:Alpha"), "{text}");
    assert!(text.contains("c.txt:2:alpha"), "{text}");
}

/// files는 파일 하나당 한 줄, count는 `경로:횟수`입니다.
#[tokio::test]
async fn files_and_count_formats() {
    let folder = TempFolder::new("grep-formats");
    folder.write_file("a.txt", "x\nx\n");
    let files = grep_out(&folder, r#"{"pattern":"x","format":"files"}"#).await;
    assert_eq!(files, "a.txt", "{files}");
    let counts = grep_out(&folder, r#"{"pattern":"x","format":"count"}"#).await;
    assert_eq!(counts, "a.txt:2", "{counts}");
}

/// 확장자 필터는 점 유무를 모두 받고 대소문자를 가리지 않습니다.
#[tokio::test]
async fn ext_filter() {
    let folder = TempFolder::new("grep-ext");
    folder.write_file("code.rs", "hit\n");
    folder.write_file("note.txt", "hit\n");
    let text = grep_out(&folder, r#"{"pattern":"hit","ext":".RS"}"#).await;
    assert_eq!(text, "code.rs:1:hit", "{text}");
}

/// 200줄을 넘으면 잘림 표식을 붙입니다(AC 5).
#[tokio::test]
async fn over_200_lines_truncates_with_mark() {
    let folder = TempFolder::new("grep-cap");
    let mut body = String::new();
    for index in 0..205 {
        body.push_str(&format!("hit {index}\n"));
    }
    folder.write_file("many.txt", &body);
    let text = grep_out(&folder, r#"{"pattern":"hit"}"#).await;
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 201, "200줄+표식: {text}");
    assert_eq!(lines.last(), Some(&TRUNCATION_MARK), "{text}");
    assert!(lines[0].starts_with("many.txt:1:hit 0"), "{text}");
}

/// 비밀 파일은 경로도 내용도 그 결과에 없어야 합니다(AC 4·AC 8).
#[tokio::test]
async fn secret_files_are_not_searched() {
    let folder = TempFolder::new("grep-secret");
    folder.write_file(
        "credentials.jsonc",
        "{\"api_keys\":{\"k\":\"secret-value\"}}",
    );
    folder.write_file("agent-snap.jsonc", "{\"endpoints\":[]}");
    // T4(AC 8): `.env` 계열도 숨김 opt를 켠 검색에 결과·경로가 남지 않습니다.
    folder.write_file(".env", "TOPSECRET=1");
    folder.write_file(".env.local", "TOPSECRET_LOCAL=1");
    let text = grep_out(&folder, r#"{"pattern":"secret-value"}"#).await;
    assert!(text.is_empty(), "{text}");
    let text = grep_out(&folder, r#"{"pattern":"api_keys"}"#).await;
    assert!(text.is_empty(), "{text}");
    let text = grep_out(&folder, r#"{"pattern":"endpoints"}"#).await;
    assert!(text.is_empty(), "{text}");
    let text = grep_out(&folder, r#"{"pattern":"TOPSECRET","hidden":true}"#).await;
    assert!(
        text.is_empty(),
        "`.env` 내용과 경로가 새지 않습니다: {text}"
    );
    let text = grep_out(
        &folder,
        r#"{"pattern":"TOPSECRET","format":"files","hidden":true}"#,
    )
    .await;
    assert!(text.is_empty(), "files 형식에도 새지 않습니다: {text}");
}

/// 숨김 자리는 기본 제외, opt로 포함됩니다.
#[tokio::test]
async fn hidden_entries_default_excluded_and_opt_included() {
    let folder = TempFolder::new("grep-hidden");
    folder.write_file(".hidden.txt", "hit\n");
    folder.write_file("shown.txt", "hit\n");
    let text = grep_out(&folder, r#"{"pattern":"hit"}"#).await;
    assert!(!text.contains(".hidden.txt"), "{text}");
    assert!(text.contains("shown.txt"), "{text}");
    let text = grep_out(&folder, r#"{"pattern":"hit","hidden":true}"#).await;
    assert!(text.contains(".hidden.txt"), "opt 포함: {text}");
}

/// 작업 폴더 밖 경로는 거부합니다(AC 2).
#[tokio::test]
async fn outside_path_is_refused() {
    let folder = TempFolder::new("grep-outside");
    let text = grep_out(&folder, r#"{"pattern":"hit","path":"../escape"}"#).await;
    assert!(text.starts_with("오류: "), "{text}");
    assert!(text.contains("경계 밖"), "{text}");
    assert_eq!(text.lines().count(), 1, "{text}");
}

/// 인자 모양이 틀리면 한 줄 오류입니다.
#[tokio::test]
async fn bad_args_are_one_line_errors() {
    let folder = TempFolder::new("grep-args");
    let missing = grep_out(&folder, r#"{"path":"."}"#).await;
    assert!(missing.starts_with("오류: "), "{missing}");
    assert_eq!(missing.lines().count(), 1, "{missing}");
    let format = grep_out(&folder, r#"{"pattern":"x","format":"yaml"}"#).await;
    assert!(format.starts_with("오류: "), "{format}");
    let ext = grep_out(&folder, r#"{"pattern":"x","ext":""}"#).await;
    assert!(ext.starts_with("오류: "), "{ext}");
}

/// 등록 규칙대로 이름·설명·필수 인자·읽기 전용 표식을 한 자리에서 확인합니다.
#[test]
fn tool_shape_is_ready_for_registration() {
    assert_eq!(GrepTool.name(), "grep");
    assert!(!GrepTool.description().is_empty());
    let schema = GrepTool.parameters_schema();
    assert_eq!(schema["required"], serde_json::json!(["pattern"]));
    assert!(!GrepTool.executes_writes());
}

/// 이진 파일은 조용히 건너뜁니다(내장 검색은 텍스트만 봅니다).
#[tokio::test]
async fn binary_files_are_skipped() {
    let folder = TempFolder::new("grep-binary");
    folder.write_bytes("bin.dat", b"hit\xff\xfe");
    let text = grep_out(&folder, r#"{"pattern":"hit"}"#).await;
    assert!(text.is_empty(), "이진은 건너뜁니다: {text}");
}
