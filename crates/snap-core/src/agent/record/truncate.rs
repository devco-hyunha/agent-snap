//! 도구 결과 잘림: UTF-8 4,000자 초과 시 앞을 자르고 표식을 붙임
//!
//! 글자 수는 바이트가 아니라 문자 수로 셈

/// 잘린 결과에 붙이는 표식
pub const TRUNCATION_MARK: &str = "…[truncated]";

/// 도구 결과를 상한 문자 수로 자름
///
/// 상한이 표식 길이보다 작으면 표식만 남김
#[must_use]
pub fn truncate_tool_result(text: &str, max_chars: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_chars {
        return text.to_owned();
    }
    let mark_len = TRUNCATION_MARK.chars().count();
    let keep = max_chars.saturating_sub(mark_len);
    let mut kept: String = chars[..keep].iter().collect();
    kept.push_str(TRUNCATION_MARK);
    kept
}
