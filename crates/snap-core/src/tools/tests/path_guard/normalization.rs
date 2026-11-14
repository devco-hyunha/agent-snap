//! 경로 정규화 시험(`path_norm`의 `.` 지움, 03-3 T1).
//!
//! 상대 경로에 섞인 `.`를 지운 정규화 결과를 `check_tool_path`가 돌려주는지를 본다.

use crate::tools::path_guard::check_tool_path;

use super::super::support::TempFolder;

#[test]
fn dot_path_is_normalized() {
    let folder = TempFolder::new("guard-dot");
    let inside = folder.write_file("keep.txt", "text");
    let target = check_tool_path(folder.path(), "./keep.txt").expect("정규화한 경로");
    assert_eq!(target, inside);
}
