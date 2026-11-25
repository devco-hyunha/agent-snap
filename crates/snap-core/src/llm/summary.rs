//! 서버 오류 요약 조립 자리. 가린 요약으로 만들고 길이 상한만 더함.

use serde_json::Value;

use crate::llm::redact::is_hidden;
use crate::llm::redact::redact_key_parts;
use crate::llm::redacted::RedactedSummary;
use crate::locale::config::SECRET_REDACTED;

const ERROR_KEY: &str = "error";
const MESSAGE_KEY: &str = "message";

/// 서버 오류 요약 표시 길이 상한.
pub const SUMMARY_MAX_CHARS: usize = 120;

/// 최상위 `error` 객체가 있으면 서버가 준 오류 요약 텍스트.
/// `message`가 없는 폴백은 JSON 원문 전체. 가리기 전 단계라 원문을 그대로 둠.
pub fn stream_error_text(value: &Value) -> Option<String> {
    let error = value.get(ERROR_KEY)?;
    let message = error.get(MESSAGE_KEY).and_then(Value::as_str);
    let Some(message) = message else {
        return Some(error.to_string());
    };
    Some(message.to_owned())
}

/// 서버가 준 오류 텍스트를 가린 요약으로 옮김.
/// 칸 전체가 키·토큰·시크릿·주소면 가렸음 표시로 바꾸고,
/// 칸 전체가 아니면 구분자·괄호로 나눈 조각 중 키로 보이는 조각만 가렸음 표시로 바꿈.
/// 그 밖의 칸은 순서대로 이어 붙이고 길이 상한으로 자름.
pub fn redact_summary(raw: &str) -> RedactedSummary {
    let mut kept: Vec<String> = Vec::new();
    let mut used = 0usize;
    for token in raw.split_whitespace() {
        if used >= SUMMARY_MAX_CHARS {
            break;
        }
        let piece = if is_hidden(token) {
            SECRET_REDACTED.to_owned()
        } else {
            redact_key_parts(token)
        };
        let piece = piece[..clamp_len(&piece, SUMMARY_MAX_CHARS - used)].to_owned();
        if piece.is_empty() {
            break;
        }
        used += piece.len();
        kept.push(piece);
    }
    let joined = kept.join(" ");
    let capped = &joined[..clamp_len(&joined, SUMMARY_MAX_CHARS)];
    RedactedSummary::new(capped.to_owned())
}

/// 길이 상한까지의 문자열 인덱스. 문자 경계를 지킴.
fn clamp_len(text: &str, max_len: usize) -> usize {
    if text.len() <= max_len {
        return text.len();
    }
    let mut end = 0usize;
    for (index, ch) in text.char_indices() {
        if index + ch.len_utf8() > max_len {
            break;
        }
        end = index + ch.len_utf8();
    }
    end
}
