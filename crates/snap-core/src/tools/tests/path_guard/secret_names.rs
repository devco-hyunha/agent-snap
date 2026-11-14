//! 비밀 파일 이름과 `.env` 계열 차단 시험(`secret_names` 판정, spec AC 4·AC 8, 03-3 T1·T4).
//!
//! 비밀 목록(`agent-snap.jsonc`·`credentials.jsonc`)과 `.env`로 시작하는 이름이
//! 읽기·쓰기 검사에서 한 자리에서 거부됨을 확인한다. 이름 비교는 `path_norm`의
//! `names_match` 관례를 따르므로 OS마다 결과가 다르다(개발 규칙 이식성).

use crate::tools::path_guard::{check_tool_path, check_write_path};

use super::super::support::TempFolder;
use super::assert_rejected;

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
