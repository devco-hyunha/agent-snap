//! ripgrep 연동 시험: JSON 분석 결과의 비밀 거르기와 경로 표시.
//!
//! `rg --json` 모의 줄을 직접 만들어 거르기를 확인합니다. 이 시험은 ripgrep이
//! PATH에 없어도 돌아가므로, 비밀 자리·내용이 새는지를 이 환경과 무관하게 봅니다.

use std::path::PathBuf;

use crate::tools::ripgrep::parse_ripgrep_json;

/// `rg --json` match 줄 한 줄을 만들어 줍니다(rg가 내보내는 모양과 같은 형태).
fn json_match_line(path: &str, text: &str, line_no: usize) -> String {
    let value = serde_json::json!({
        "type": "match",
        "data": {
            "path": {"text": path},
            "lines": {"text": text},
            "line_number": line_no,
            "absolute_offset": 0,
            "submatches": [{"match": {"text": "secret-value"}, "start": 0, "end": 12}]
        }
    });
    value.to_string()
}

/// content 형식의 표시는 `rg --json`의 자리와 작업 폴더 기준을 맞춘 결과입니다.
#[test]
fn content_lines_use_relative_display() {
    let working_dir = PathBuf::from("/work");
    let normal_line = json_match_line("/work/keep.txt", "secret-value", 2);
    let lines = parse_ripgrep_json(&normal_line, &working_dir, "content").expect("분석 성공");
    assert_eq!(lines, vec!["keep.txt:2:secret-value".to_owned()]);
}

/// 비밀 자리 줄은 content·files·count 형식 전부에서 사라지는지 확인합니다(AC 4).
#[test]
fn secret_paths_are_dropped_from_parsed_events() {
    let working_dir = PathBuf::from("/work");
    let secret_line = json_match_line("/work/credentials.jsonc", "{\"k\":\"secret-value\"}", 1);
    let normal_line = json_match_line("/work/keep.txt", "secret-value", 2);
    let stdout = format!("{secret_line}\n{normal_line}\n");
    let lines = parse_ripgrep_json(&stdout, &working_dir, "content").expect("분석 성공");
    assert_eq!(lines, vec!["keep.txt:2:secret-value".to_owned()]);
    let files = parse_ripgrep_json(&stdout, &working_dir, "files").expect("분석 성공");
    assert_eq!(files, vec!["keep.txt".to_owned()]);
    let counts = parse_ripgrep_json(&stdout, &working_dir, "count").expect("분석 성공");
    assert_eq!(counts, vec!["keep.txt:1".to_owned()]);
}

/// Windows 표시(`\` 구분)도 비밀 이름이면 거르는지 봅니다.
/// `\` 구분 표기는 Windows에서만 의미 있으므로 이 시험은 Windows 전용입니다.
#[cfg(windows)]
#[test]
fn secret_paths_with_native_separator_are_dropped() {
    let working_dir = PathBuf::from("C:\\work");
    let line = json_match_line("C:\\work\\credentials.jsonc", "{\"k\":\"secret-value\"}", 1);
    let lines = parse_ripgrep_json(&line, &working_dir, "content").expect("분석 성공");
    assert!(
        lines.is_empty(),
        "비밀 자리는 content에 남지 않습니다: {lines:?}"
    );
}
