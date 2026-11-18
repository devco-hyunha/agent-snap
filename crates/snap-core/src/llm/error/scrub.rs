//! 이 요청에서 알고 있는 키·base_url로 한 번 더 가리는 확인 가리기 도구.
//!
//! 어림 가리기 뒤 알고 있는 값·호스트로 한 번 더 가림. `Debug`·`Display`에 원문은 나오지 않음.

use std::fmt;

use crate::secrets::SecretValue;

use super::redact::{SUMMARIZE_MAX_LEN, summarize_stream_error, truncate_chars};

/// 이 요청에서 알고 있는 키·base_url을 기억하는 가림 도구.
#[derive(Clone)]
pub(in crate::llm) struct SecretScrub {
    key: Option<SecretValue>,
    base_url: SecretValue,
}

impl SecretScrub {
    /// 요청에 쓴 키와 base_url로 가림 도구를 만듦.
    pub(in crate::llm) fn new(key: Option<&SecretValue>, base_url: &SecretValue) -> Self {
        Self {
            key: key.cloned(),
            base_url: base_url.clone(),
        }
    }

    /// 어림 가리기 뒤 알고 있는 키·base_url·호스트로 한 번 더 가림.
    ///
    /// 결과는 항상 어림 가리기 최대 길이로 잘림.
    pub(in crate::llm) fn describe(&self, summary: &str) -> String {
        let rough = summarize_stream_error(summary);
        let mut cleaned = rough;
        if let Some(key) = &self.key {
            cleaned = replace_all_masked(&cleaned, key.reveal());
        }
        let base_url = self.base_url.reveal();
        cleaned = replace_all_masked(&cleaned, base_url);
        if let Some(authority) = web_authority(base_url) {
            cleaned = replace_all_masked(&cleaned, authority);
        }
        truncate_chars(&cleaned, SUMMARIZE_MAX_LEN)
    }
}

impl fmt::Debug for SecretScrub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretScrub")
            .field("key", &self.key)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl fmt::Display for SecretScrub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "알고 있는 키·base_url 가림 도구")
    }
}

/// `http://`·`https://` 다음에 오는 호스트(포트 포함) 조각을 꺼냄.
fn web_authority(url: &str) -> Option<&str> {
    let after_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))?;
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .filter(|part| !part.is_empty())?;
    Some(authority)
}

/// 원문과 같아 보이는 조각을 가린 문자열로 바꿈.
///
/// 대소문자·숫자는 자리 수만 유지. 가린 결과에서 원문 글자를 되읽을 수 없음.
fn replace_all_masked(text: &str, secret: &str) -> String {
    if secret.is_empty() {
        return text.to_owned();
    }
    let masked = mask_value(secret);
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(position) = rest.find(secret) {
        out.push_str(&rest[..position]);
        out.push_str(&masked);
        rest = &rest[position + secret.len()..];
    }
    out.push_str(rest);
    out
}

fn mask_value(secret: &str) -> String {
    secret
        .chars()
        .map(|ch| {
            if ch.is_ascii_digit() {
                '9'
            } else if ch.is_ascii_lowercase() {
                'a'
            } else if ch.is_ascii_uppercase() {
                'A'
            } else {
                ch
            }
        })
        .collect()
}
