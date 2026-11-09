//! 경로 검사 1차 시험(spec AC 2·AC 4).
//!
//! 경계(바깥·`..`·절대경로)와 비밀 파일 이름 차단을 확인한다.

use crate::tools::path_guard::{check_tool_path, check_write_path};

use super::support::TempFolder;

/// 거부에 이유 문장이 들어있는지 함께 본다.
fn assert_rejected(working_dir: &std::path::Path, path: &str, expected: &str) {
    let error = check_tool_path(working_dir, path).expect_err("거부되어야 한다");
    let reason = error.to_string();
    assert!(reason.contains(expected), "거부 이유: {reason}");
}

#[test]
fn work_folder_inside_is_allowed() {
    let folder = TempFolder::new("guard-ok");
    let inside = folder.write_file("src/main.rs", "fn main() {}");
    let target = check_tool_path(folder.path(), "src/main.rs").expect("작업 폴더 안이므로 통과");
    assert_eq!(target, inside);
}

#[test]
fn outside_parent_path_is_rejected() {
    let folder = TempFolder::new("guard-parent");
    assert_rejected(folder.path(), "../outside.txt", "경계 밖");
}

#[test]
fn outside_absolute_path_is_rejected() {
    let folder = TempFolder::new("guard-abs");
    let outside = folder
        .path()
        .parent()
        .expect("부모 폴더")
        .join("outside.txt");
    assert_rejected(
        folder.path(),
        outside.to_str().expect("경로 문자열"),
        "경계 밖",
    );
}

#[test]
fn nested_parent_escape_is_rejected() {
    let folder = TempFolder::new("guard-deep");
    assert_rejected(folder.path(), "a/b/../../../outside.txt", "경계 밖");
}

#[test]
fn empty_path_is_rejected() {
    let folder = TempFolder::new("guard-empty");
    assert_rejected(folder.path(), "  ", "경계 밖");
}

#[test]
fn work_folder_itself_is_rejected() {
    let folder = TempFolder::new("guard-self");
    let as_text = folder.path().to_str().expect("경로 문자열");
    assert_rejected(folder.path(), as_text, "경계 밖");
}

#[test]
fn secret_files_are_blocked() {
    let folder = TempFolder::new("guard-secret");
    for name in ["agent-snap.jsonc", "credentials.jsonc"] {
        folder.write_file(name, "{}");
        assert_rejected(folder.path(), name, "비밀 파일");
    }
}

#[test]
fn secret_files_in_subfolders_are_blocked() {
    let folder = TempFolder::new("guard-secret-nested");
    folder.write_file("nested/credentials.jsonc", "{}");
    assert_rejected(folder.path(), "nested/credentials.jsonc", "비밀 파일");
}

#[cfg(windows)]
#[test]
fn secret_file_name_is_case_insensitive_on_windows() {
    let folder = TempFolder::new("guard-secret-case");
    folder.write_file("Agent-Snap.JSONC", "{}");
    assert_rejected(folder.path(), "Agent-Snap.JSONC", "비밀 파일");
}

#[cfg(not(windows))]
#[test]
fn secret_file_name_is_case_sensitive_on_unix() {
    let folder = TempFolder::new("guard-secret-case");
    folder.write_file("Agent-Snap.JSONC", "{}");
    assert!(check_tool_path(folder.path(), "Agent-Snap.JSONC").is_ok());
}

#[test]
fn similar_names_are_not_secret() {
    let folder = TempFolder::new("guard-similar");
    folder.write_file("agent-snap.json", "{}");
    assert!(check_tool_path(folder.path(), "agent-snap.json").is_ok());
}

#[test]
fn dot_path_is_normalized() {
    let folder = TempFolder::new("guard-dot");
    let inside = folder.write_file("keep.txt", "text");
    let target = check_tool_path(folder.path(), "./keep.txt").expect("정규화한 경로");
    assert_eq!(target, inside);
}

#[test]
fn git_write_is_blocked_by_write_check() {
    let folder = TempFolder::new("guard-git");
    // 쓰기 검사는 `.git` 아래를 막고, 읽기 검사는 막지 않는다(D49).
    assert!(check_tool_path(folder.path(), ".git/config").is_ok());
    assert!(check_write_path(folder.path(), ".git/config").is_err());
}
