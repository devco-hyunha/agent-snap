//! 오류 문구에서 URL·시크릿처럼 보이는 조각을 가리고 길이를 제한합니다.
//!
//! 이 모듈은 문구 조립과 길이 제한만 맡습니다.
//! 모양만 보고 가리는 어림 가리기 규칙은 [`super::mask`]에 두고,
//! 이 요청에서 실제로 알고 있는 키·base_url로 한 번 더 가리는 확인 가리기는
//! [`super::scrub`]에 둡니다(D28, 03-6 R2 감사 분리).

use super::mask;

/// 어림·확인 가리기를 모두 거친 뒤 유지할 최대 글자 수입니다.
pub(super) const SUMMARIZE_MAX_LEN: usize = 200;

/// 서버 오류 문구에서 URL·키처럼 보이는 조각을 어림 규칙으로 가리고 길이를 제한합니다.
pub(in crate::llm) fn summarize_stream_error(raw: &str) -> String {
    let without_urls = mask::redact_urls(raw);
    let without_secrets = mask::redact_secret_like(&without_urls);
    truncate_chars(&without_secrets, SUMMARIZE_MAX_LEN)
}

/// 주어진 글자 수를 넘으면 잘르고 끝에 생략 표시를 붙입니다.
pub(in crate::llm) fn truncate_chars(input: &str, max_chars: usize) -> String {
    let count = input.chars().count();
    if count <= max_chars {
        return input.to_string();
    }
    let truncated: String = input.chars().take(max_chars).collect();
    format!("{truncated}…")
}
