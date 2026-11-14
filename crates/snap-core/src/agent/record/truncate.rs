//! 도구 결과 잘림(spec AC 5): UTF-8 4,000자 초과 시 앞을 자르고 표식을 붙입니다.
//!
//! 잘림 상한은 03-2가 정본이고, 도구(`Tool::call`)는 자르지 않습니다(03-1 결과 규격).
//! 글자 수는 바이트가 아니라 문자 수로 세고, 자른 뒤 붙이는 표식까지 합친 총 길이가
//! 상한을 넘지 않게 합니다.

/// 잘린 결과에 붙이는 표식입니다(spec AC 5).
pub const TRUNCATION_MARK: &str = "…[truncated]";

/// 도구 결과를 상한 문자 수로 자릅니다(AC 5).
///
/// `max_chars` 이하이면 그대로 두고, 넘으면 앞만 남기고 `…[truncated]`를 붙입니다.
/// 상한이 표식 길이보다 작으면 표식만 남깁니다(잘림 표식을 모델이 항상 알 수 있게).
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
