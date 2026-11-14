//! 심볼릭 링크 기준 시험(`link_guard` 판정, spec AC 2: "작업 폴더 안에서 생성된 링크만 허용", 03-3 T3).
//!
//! 링크를 만들 수 없는 환경(Windows: 권한·개발 모드)에서는 링크 시험이 스스로
//! 통과하게 두고, 링크를 만들 수 있는 환경에서만 링크를 만들어 판정을 확인한다.

use crate::tools::path_guard::{check_tool_path, escapes_symlink, is_symlink_path};

use super::super::support::TempFolder;
use super::assert_rejected;

/// 작업 폴더 안에 만든 링크는 링크 판정을 통과한다.
#[test]
fn symlink_inside_folder_is_recognized() {
    let folder = TempFolder::new("guard-link-in");
    let target = folder.write_file("keep.txt", "text");
    if !folder.try_symlink_file(&target, "link.txt") {
        // 링크를 만들 수 없는 환경(Windows 권한·개발 모드)에서는 검사하지 않는다.
        return;
    }
    let link = folder.path().join("link.txt");
    assert!(is_symlink_path(&link), "링크 자리로 봐야 합니다");
    assert!(check_tool_path(folder.path(), "link.txt").is_ok());
    assert!(
        !escapes_symlink(folder.path(), &link),
        "작업 폴더 안 링크는 허용입니다(spec AC 2)"
    );
}

/// 작업 폴더 밖 파일을 가리키는 링크는 경계 밖으로 거부한다.
#[test]
fn symlink_to_outside_file_is_rejected() {
    let folder = TempFolder::new("guard-link-out-file");
    let outside = folder
        .path()
        .parent()
        .expect("부모 폴더")
        .join("agent-snap-outside-file.txt");
    std::fs::write(&outside, "secret-value").expect("밖 파일 쓰기");
    if !folder.try_symlink_file(&outside, "out.txt") {
        return;
    }
    let link = folder.path().join("out.txt");
    assert!(
        escapes_symlink(folder.path(), &link),
        "밖을 가리키는 링크는 거부입니다(spec AC 2)"
    );
    assert_rejected(folder.path(), "out.txt", "경계 밖");
    let _ = std::fs::remove_file(&outside);
}

/// 작업 폴더 밖 폴더를 가리키는 링크 폴더는 경계 밖으로 거부한다.
#[test]
fn symlink_to_outside_folder_is_rejected() {
    let folder = TempFolder::new("guard-link-out-dir");
    let outside = folder
        .path()
        .parent()
        .expect("부모 폴더")
        .join("agent-snap-outside-dir");
    std::fs::create_dir_all(&outside).expect("밖 폴더 쓰기");
    if !folder.try_symlink_dir(&outside, "escape") {
        return;
    }
    let link = folder.path().join("escape");
    assert!(
        escapes_symlink(folder.path(), &link),
        "밖 폴더 링크는 거부입니다(spec AC 2)"
    );
    assert_rejected(folder.path(), "escape/inner.txt", "경계 밖");
    let _ = std::fs::remove_dir_all(&outside);
}

/// 링크 대상이 `..`로 작업 폴더의 조상 폴더로 올라가면 같은 판정을 확인한다.
#[test]
fn symlink_parent_escape_target_is_rejected() {
    let folder = TempFolder::new("guard-link-dotdot");
    let outside = folder
        .path()
        .parent()
        .expect("부모 폴더")
        .join("agent-snap-dotdot-target.txt");
    std::fs::write(&outside, "secret-value").expect("밖 파일 쓰기");
    std::fs::create_dir_all(folder.path().join("inner")).expect("inner 폴더");
    // 링크 대상을 절대 경로로 두면 두 OS가 같은 "조상 가리킥" 상황을 봅니다(spec AC 2).
    let made = super::super::support::make_symlink(
        &outside,
        &folder.path().join("inner").join("rel.txt"),
        false,
    );
    if !made {
        return;
    }
    let escaping = folder.path().join("inner").join("rel.txt");
    assert!(
        escapes_symlink(folder.path(), &escaping),
        "`..`로 조상을 가리키는 링크는 거부입니다(spec AC 2)"
    );
    assert_rejected(folder.path(), "inner/rel.txt", "경계 밖");
    let _ = std::fs::remove_file(&outside);
}

/// 링크가 아닌 자리는 링크 판정이 참이 아니다.
#[test]
fn plain_file_is_not_symlink() {
    let folder = TempFolder::new("guard-link-plain");
    let plain = folder.write_file("plain.txt", "text");
    assert!(!is_symlink_path(&plain));
}

#[cfg(windows)]
#[test]
fn windows_gate_covers_symlink_metadata() {
    // Windows 게이트 경로가 실재 파일에서 동작함을 확인한다(개발 규칙 이식성).
    let folder = TempFolder::new("guard-link-windows");
    let plain = folder.write_file("plain.txt", "text");
    assert!(!is_symlink_path(&plain));
}
