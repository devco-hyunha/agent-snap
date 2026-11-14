//! 결과 표시를 돕는 자리입니다(03-3 T1, dev-principles 3절 DRY).
//!
//! 경로 표시와 목록·검색 공통 줄 상한을 이 파일 한 곳에서만 만듭니다.
//! 경로 판단은 `path_guard`가 맡고, 이 파일은 표시만 맡습니다(책임 분리).

use std::path::Path;

use crate::agent::TRUNCATION_MARK;

/// 작업 폴더를 기준으로 한 경로 표시를 만들어 줍니다.
#[must_use]
pub fn relative_path(target: &Path, working_dir: &Path) -> String {
    match target.strip_prefix(working_dir) {
        Ok(rel) => display_path(rel),
        Err(_) => display_path(target),
    }
}

/// 목록·검색 결과가 모델로 가기 전에 자르는 줄 수입니다(spec AC 5).
pub const MAX_RESULT_LINES: usize = 200;

/// 줄을 상한 아래로 자르고, 잘렸으면 잘림 표식을 끝에 붙입니다.
#[must_use]
pub fn cap_lines(mut lines: Vec<String>) -> Vec<String> {
    if lines.len() > MAX_RESULT_LINES {
        lines.truncate(MAX_RESULT_LINES);
        lines.push(TRUNCATION_MARK.to_owned());
    }
    lines
}

/// 경로 구분 기호(`/`와 `\`)를 `/` 하나로 맞춰 한 줄 문자열로 만듭니다.
///
/// Windows와 Linux에서 같은 표시가 나오게 하는 자리입니다(개발 규칙: 두 OS 동등).
fn display_path(path: &Path) -> String {
    let raw = path.to_string_lossy();
    raw.replace('\\', "/")
}

/// 텍스트에서 앞 `max_bytes` 바이트까지를 글자 경계에서 잘라 줍니다.
///
/// UTF-8 글자 한가운데에서 자르면 결과가 깨지므로 글자 경계까지 내려갑니다.
#[must_use]
pub(super) fn head_bytes(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }
    let mut cut = max_bytes;
    while cut > 0 && !text.is_char_boundary(cut) {
        cut -= 1;
    }
    &text[..cut]
}

/// 줄마다 번호를 붙여 한 줄씩 이어 씁니다. 번호는 `start`부터 매깁니다.
///
/// 구간 읽기도 기본 읽기도 같은 표기를 쓰므로 이 한 곳에서만 만듭니다(DRY).
#[must_use]
pub(super) fn numbered_lines(text: &str, start: usize) -> String {
    text.lines()
        .enumerate()
        .map(|(index, line)| format!("{}: {line}\n", start + index))
        .collect()
}
