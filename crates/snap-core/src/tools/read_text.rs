//! `read_file` 도구가 쓰는 텍스트 읽기 규칙 자리
//!
//! 도구 구현(`read_file.rs`)은 인자 받기와 경로 검사만 남김

use std::path::Path;

use crate::agent::TRUNCATION_MARK;

use super::display::{head_bytes, numbered_lines};
use super::tool::ToolOutcome;

/// 기본 읽기를 거부하는 파일 크기
const REJECT_FILE_BYTES: u64 = 256 * 1024;
const BASE_READ_BYTES: usize = 128 * 1024;
const SECTION_PAGE_BYTES: usize = 50 * 1024;
/// 첫 줄 번호. `offset`을 적지 않으면 이 줄부터 읽음
pub(super) const FIRST_LINE: usize = 1;
/// 기본 읽기를 거부할 때 남기는 안내
const BASE_REJECT_HINT: &str =
    "읽기 한도 초과(256KB 이상). offset과 limit으로 구간을 읽거나 grep으로 검색하세요";

/// 파일을 읽어 도구 결과로 바꿈. `offset`·`limit`이 둘 다 없으면 기본 읽기
///
/// 구간을 지정하면 256KB 넘는 파일도 읽을 수 있음
pub(super) fn read_text(target: &Path, offset: Option<usize>, limit: Option<usize>) -> ToolOutcome {
    let Ok(meta) = std::fs::metadata(target) else {
        return ToolOutcome::error("파일을 찾을 수 없습니다");
    };
    if !meta.is_file() {
        return ToolOutcome::error("파일이 없습니다");
    }
    let section = offset.is_some() || limit.is_some();
    if !section && meta.len() >= REJECT_FILE_BYTES {
        return ToolOutcome::error(BASE_REJECT_HINT);
    }
    let Ok(bytes) = std::fs::read(target) else {
        return ToolOutcome::error("파일을 읽지 못했습니다");
    };
    let Ok(text) = String::from_utf8(bytes) else {
        return ToolOutcome::error("바이너리 파일은 읽지 않습니다");
    };
    match (offset, limit) {
        (None, None) => ToolOutcome::Done(base_read(&text)),
        (offset, limit) => match section_read(&text, offset.unwrap_or(FIRST_LINE), limit) {
            Ok(section) => ToolOutcome::Done(section),
            Err(reason) => ToolOutcome::error(&reason),
        },
    }
}

/// 기본 읽기: 앞 128KB만 잘라서 보여주고 잘림 표식을 붙임
fn base_read(text: &str) -> String {
    let head = head_bytes(text, BASE_READ_BYTES);
    let mut out = numbered_lines(head, FIRST_LINE);
    if head.len() < text.len() {
        out.push_str(TRUNCATION_MARK);
    }
    out
}

/// 구간 읽기: `offset` 줄부터 `limit` 줄까지를 줄 번호와 함께 보여줌
///
/// 구간 한 장이 50KiB를 넘으면 읽지 않고 이유를 돌려줌
fn section_read(text: &str, offset: usize, limit: Option<usize>) -> Result<String, String> {
    let selected: Vec<&str> = text
        .lines()
        .skip(offset.saturating_sub(FIRST_LINE))
        .take(limit.unwrap_or(usize::MAX))
        .collect();
    // 한 줄에 개행 한 글자를 더한 크기가 구간 한 장의 기준
    let page_bytes: usize = selected.iter().map(|line| line.len() + 1).sum();
    if page_bytes > SECTION_PAGE_BYTES {
        return Err(format!(
            "구간 한 장이 {}KiB를 넘습니다. limit을 줄이세요",
            SECTION_PAGE_BYTES / 1024
        ));
    }
    let joined = selected.join("\n");
    Ok(numbered_lines(&joined, offset))
}
