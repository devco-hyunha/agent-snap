//! ripgrep 연동 자리입니다(03-3 T2·T3).
//!
//! ripgrep이 있으면 `--json` 출력으로 찾고, 없거나 결과를 못 읽으면 `None`으로
//! 알려 내장 검색으로 넘깁니다(조용한 대체).
//!
//! 순회 기준은 `walk`과 같은 규칙을 씁니다(T2 리뷰 3항, spec AC 2): `--no-follow`(기본값)로
//! 링크 폴더를 순회하지 않습니다(`walk`의 링크 폴더 비순회와 동일). 링크를 통한
//! 경로 탈출은 모든 도구가 부르는 `path_guard`의 경로 검사 한 자리에서 막습니다.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::config::{CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME};

use super::command_probe::command_works;
use super::display::relative_path;
use super::path_guard::is_secret_name;

/// ripgrep이 있으면 JSON 출력으로 찾고, 없거나 실패하면 `None`입니다.
///
/// `--json`을 못 쓰는 옛 버전·실행 불가는 내장 검색으로 넘어갑니다(조용한 대체).
pub(crate) fn search_with_ripgrep(
    working_dir: &Path,
    start: &Path,
    pattern: &str,
    case_sensitive: bool,
    ext: Option<&str>,
    format: &str,
    include_hidden: bool,
) -> Option<Vec<String>> {
    if !command_works("rg", &["--version"]) {
        return None;
    }
    let mut command = Command::new("rg");
    command
        .arg("--json")
        .arg("--no-ignore")
        .arg("--no-follow")
        .arg("--glob-case-insensitive");
    if include_hidden {
        command.arg("--hidden");
    }
    if !case_sensitive {
        command.arg("--ignore-case");
    }
    if let Some(ext) = ext {
        // 확장자 기준을 `ext_matches`(대소문자 무시)와 같은 기준으로 맞춥니다.
        command.arg("--glob").arg(format!("*.{ext}"));
    }
    // 비밀 파일은 rg가 읽지도 않게 인자 단계에서 막습니다. ripgrep는 `--glob` 규칙을
    // 마지막에 맞는 것으로 판단하므로, 거르기 줄을 확장자 필터 뒤에 반드시 둡니다.
    // `.env` 계열(AC 8)은 이름이 `.env`로 시작하는 자리만 막습니다(과잉 차단 금지).
    // 결과-parser에도 같은 거르기를 두어 방어선을 겹치게 합니다(D17·spec AC 4).
    command
        .arg("--glob")
        .arg(format!("!{CREDENTIALS_FILE_NAME}"))
        .arg("--glob")
        .arg(format!("!{CONFIG_FILE_NAME}"))
        .arg("--glob")
        .arg("!.env")
        .arg("--glob")
        .arg("!.env*");
    command.arg("-e").arg(pattern).arg("--").arg(start);
    let output = command.current_dir(working_dir).output().ok()?;
    if !output.status.success() && output.status.code() != Some(1) {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    parse_ripgrep_json(&stdout, working_dir, format)
}

/// `rg --json` 줄들을 결과 형식으로 바꿉니다. 모양이 다르면 `None`(내장 대체)입니다.
pub(crate) fn parse_ripgrep_json(
    stdout: &str,
    working_dir: &Path,
    format: &str,
) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut counts: Vec<(String, usize)> = Vec::new();
    for raw_line in stdout.lines() {
        let event: Value = serde_json::from_str(raw_line).ok()?;
        if event.get("type")?.as_str()? != "match" {
            continue;
        }
        let data = event.get("data")?;
        let path_text = data.get("path")?.get("text")?.as_str()?;
        let raw = data.get("lines")?.get("text")?.as_str()?;
        let line = raw.trim_end_matches(&['\n', '\r'][..]);
        // 방어선: 인자 거르기를 통과한 결과가 와도 비밀 자리는 한 번 더 거릅니다.
        // 자리와 내용 둘 다 결과에 남지 않게 하는 것이 목적입니다(D17·spec AC 4).
        if is_secret_name(Path::new(path_text)) {
            continue;
        }
        let rel = relative_path(&PathBuf::from(path_text), working_dir);
        if format == "content" {
            let line_no = event
                .get("line")
                .or_else(|| event.get("line_number"))
                .and_then(Value::as_u64)
                .or_else(|| {
                    event
                        .get("data")
                        .and_then(|data| data.get("line_number"))
                        .and_then(Value::as_u64)
                })?;
            out.push(format!("{rel}:{line_no}:{line}"));
        } else if format == "files" {
            if !files.iter().any(|seen| seen == &rel) {
                files.push(rel);
            }
        } else {
            match counts.iter_mut().find(|(seen, _)| seen == &rel) {
                Some((_, count)) => *count += 1,
                None => counts.push((rel, 1)),
            }
        }
    }
    if format == "content" {
        Some(out)
    } else if format == "files" {
        Some(files)
    } else {
        Some(
            counts
                .into_iter()
                .map(|(path, count)| format!("{path}:{count}"))
                .collect(),
        )
    }
}
