//! 이 요청에서 실제로 알고 있는 키·base_url로 한 번 더 가리는 확인 가리기 도구입니다(D28).
//!
//! 어림 가리기를 지난 문구를 [`super::redact::summarize_stream_error`]로 먼저 거친 뒤,
//! 알고 있는 값과 그 호스트 문자열로 한 번 더 가립니다.
//! 값은 `Debug`·`Display`에 그대로 나타나지 않습니다. 서버가 되돌려 준 문구를 가릴 때만
//! 원문을 쓰고, 가린 결과에만 원문이 섞이지 않게 합니다.

use std::fmt;

use crate::secrets::SecretValue;

use super::redact::{SUMMARIZE_MAX_LEN, summarize_stream_error, truncate_chars};

/// 이 요청에서 알고 있는 키·base_url을 기억하는 가림 도구입니다(D28).
#[derive(Clone)]
pub(in crate::llm) struct SecretScrub {
    key: Option<SecretValue>,
    base_url: SecretValue,
}

impl SecretScrub {
    /// 요청에 쓴 키와 base_url로 가림 도구를 만듭니다.
    pub(in crate::llm) fn new(key: Option<&SecretValue>, base_url: &SecretValue) -> Self {
        Self {
            key: key.cloned(),
            base_url: base_url.clone(),
        }
    }

    /// 어림 가리기를 지난 문구를, 알고 있는 키·base_url과 그 호스트로 한 번 더 가립니다.
    ///
    /// 결과는 항상 어림 가리기 최대 길이로 잘립니다.
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

/// `http://`·`https://` 다음에 오는 호스트(포트 포함) 조각을 꺼냅니다.
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

/// 원문 문자열과 똑같아 보이는 조각을 골라 가린 문자열로 바꿉니다.
///
/// 대소문자·숫자는 자리 수만 유지해 옮기므로, 가린 결과에서 원문 글자를 되읽을 수 없습니다.
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
