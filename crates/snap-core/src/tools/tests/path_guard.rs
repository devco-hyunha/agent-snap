//! 경로 검사와 심볼릭 링크 기준 시험(spec AC 2·AC 4, 03-3 T1·T3).
//!
//! 경계(바깥·`..`·절대경로)와 비밀 파일 이름 차단, 그리고 T3가 확정한 링크 검사
//! 기준("작업 폴더 안에서 생성된 링크만 허용")을 확인한다.
//! 링크를 만들 수 없는 환경(Windows: 권한·개발 모드)에서는 링크 시험이 스스로
//! 통과하게 두고, 링크를 만들 수 있는 환경에서만 링크를 만들어 판정을 확인한다.

use crate::tools::path_guard::{
    check_tool_path, check_write_path, escapes_symlink, is_symlink_path,
};

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

// 아래는 03-3 T4 `.env` 계열 차단 시험입니다(spec AC 8).

/// `.env`와 `.env`로 시작하는 파일을 읽기·쓰기 검사가 한 자리에서 거부합니다(AC 8).
#[test]
fn env_files_are_blocked_for_read_and_write() {
    let folder = TempFolder::new("guard-env");
    for name in [".env", ".env.local", ".env.production"] {
        folder.write_file(name, "value=1");
        assert_rejected(folder.path(), name, "비밀 파일");
        let write_error = check_write_path(folder.path(), name)
            .expect_err("`.env` 계열은 쓰기 검사도 거부되어야 합니다");
        assert_eq!(
            write_error.message(),
            "비밀 파일은 읽을 수도 쓸 수도 없습니다"
        );
        // 검사가 거부했을 뿐이고 파일은 그대로 있습니다(AC 8).
        assert!(!folder.read_file(name).is_empty());
    }
}

/// `.env` 계열은 하위 폴더 자리도 거부합니다(AC 8).
#[test]
fn env_files_in_subfolders_are_blocked() {
    let folder = TempFolder::new("guard-env-nested");
    folder.write_file("config/.env.local", "value=1");
    assert_rejected(folder.path(), "config/.env.local", "비밀 파일");
    assert!(check_write_path(folder.path(), "config/.env.local").is_err());
}

#[cfg(windows)]
#[test]
fn env_file_name_is_case_insensitive_on_windows() {
    // Windows는 이름 비교가 대소문자를 가리지 않습니다(names_match 관례, AC 8).
    let folder = TempFolder::new("guard-env-case");
    folder.write_file(".ENV.local", "value=1");
    assert_rejected(folder.path(), ".ENV.local", "비밀 파일");
}

#[cfg(not(windows))]
#[test]
fn env_file_name_is_case_sensitive_on_unix() {
    // UNIX는 문자 그대로 비교하므로 대문자 이름은 통과합니다(AC 8 이식성).
    let folder = TempFolder::new("guard-env-case");
    folder.write_file(".ENV.local", "value=1");
    assert!(check_tool_path(folder.path(), ".ENV.local").is_ok());
}

/// `.env`로 시작하지 않는 이름은 차단하지 않습니다(AC 8 과잉 차단 방지).
#[test]
fn names_outside_env_family_are_allowed() {
    let folder = TempFolder::new("guard-env-similar");
    for name in [
        "app.env.json",
        "env.txt",
        "the.env.local",
        "envlocal",
        "ENVlocal",
        "env",
        "env.local",
    ] {
        folder.write_file(name, "value=1");
        assert!(
            check_tool_path(folder.path(), name).is_ok(),
            "{name}은 통과"
        );
    }
}

/// `.env`로 시작하는 접두어 판정 기준을 확인합니다(AC 8: 접두어 정본).
/// `.envlocal`은 점(env) 접두어로 시작하므로 차단하고, `envlocal`은 접두어가 없어 통과합니다.
#[test]
fn env_prefix_boundary_decides_block() {
    let folder = TempFolder::new("guard-env-prefix");
    folder.write_file(".envlocal", "value=1");
    assert_rejected(folder.path(), ".envlocal", "비밀 파일");
    folder.write_file("envlocal", "value=1");
    assert!(
        check_tool_path(folder.path(), "envlocal").is_ok(),
        "접두어 없는 이름은 통과"
    );
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

// 아래는 03-3 T3 심볼릭 링크 기준 시험이다(spec AC 2: "작업 폴더 안에서 생성된 링크만 허용").

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
    let made = super::support::make_symlink(
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
