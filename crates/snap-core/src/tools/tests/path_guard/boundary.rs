//! 작업 폴더 경계 시험(`path_guard` 검사 순서, spec AC 2·AC 4, 03-3 T1·T3).
//!
//! 바깥·`..`·절대경로·빈 경로·작업 폴더 자체 거부와 `.git` 아래 쓰기 거부를 확인한다.

use crate::tools::path_guard::{check_tool_path, check_write_path};

use super::super::support::TempFolder;

/// 거부에 이유 문장이 들어있는지 함께 본다.
pub(in crate::tools) fn assert_rejected(working_dir: &std::path::Path, path: &str, expected: &str) {
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
fn git_write_is_blocked_by_write_check() {
    let folder = TempFolder::new("guard-git");
    // 쓰기 검사는 `.git` 아래를 막고, 읽기 검사는 막지 않는다(D49).
    assert!(check_tool_path(folder.path(), ".git/config").is_ok());
    assert!(check_write_path(folder.path(), ".git/config").is_err());
}
