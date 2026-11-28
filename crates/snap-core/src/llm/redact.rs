//! 서버 오류 요약 가림 대상 판정 자리. 키·토큰·시크릿·주소성 칸을 가렸음 표시로 바꿀지 봄.

use crate::locale::config::SECRET_REDACTED;

const KEY_VALUE_SEPARATORS: [char; 2] = [':', '='];
const BEARING_NAMES: [&str; 7] = [
    "key",
    "token",
    "secret",
    "credential",
    "authorization",
    "base_url",
    "bearer",
];
const SCHEME_SEP: &str = "://";
const KEY_PREFIXES: [&str; 2] = ["sk-", "pk-"];
const OPAQUE_MIN_CHARS: usize = 32;
// 키가 붙을 수 있는 칸 구분자. 쉼표·따옴표·괄호·중괄호·역슬래시까지 포함.
const TOKEN_SEPARATORS: [char; 12] = [':', '=', '(', ')', '[', ']', '{', '}', ',', '"', '\'', '\\'];

/// 주소성 칸. `https://host/v1`처럼 scheme과 host가 붙거나,
/// `api.example-host.test/v1`처럼 scheme 없이 host에 `.`과 `/`가 붙으면 가림 대상.
fn is_address(token: &str) -> bool {
    token.contains(SCHEME_SEP) || has_host_path(token)
}

/// scheme 없는 host과 경로 조각. host에 `.`이 있고 `/`로 경로가 붙은 형태만 host로 봄.
fn has_host_path(token: &str) -> bool {
    let Some((host, path)) = token.split_once('/') else {
        return false;
    };
    !host.is_empty() && !path.is_empty() && host.contains('.')
}

/// 키·토큰·시크릿 이름과 값이 한 칸에 붙은 형태.
fn is_bearing_pair(token: &str) -> bool {
    let Some((name, value)) = split_pair(token) else {
        return false;
    };
    let name = name.to_lowercase();
    let is_bearing = BEARING_NAMES.iter().any(|needle| name.contains(needle));
    is_bearing && !value.is_empty()
}

/// 키 접두가 붙은 맨 단독 토큰(`sk-…`). 구분자 없이 값만 온 형태.
fn is_key_prefix_token(token: &str) -> bool {
    let lowered = token.to_ascii_lowercase();
    KEY_PREFIXES
        .iter()
        .any(|prefix| lowered.starts_with(prefix))
}

/// 32자 이상 hex·base64·base64url 토큰. 숫자 섞임 또는 대·소문자 섞임이면 opaque로 봄.
fn is_opaque_token(token: &str) -> bool {
    if token.len() < OPAQUE_MIN_CHARS || !token.chars().all(is_base64_char) {
        return false;
    }
    let has_digit = token.chars().any(|c| c.is_ascii_digit());
    let has_case_mix = token.chars().any(|c| c.is_ascii_uppercase())
        && token.chars().any(|c| c.is_ascii_lowercase());
    has_digit || has_case_mix
}

/// hex·base64·base64url 문자 집합. `-`·`_`는 base64url 키용 문자.
fn is_base64_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '-' | '_')
}

/// 가림 대상 칸인지.
pub(super) fn is_hidden(token: &str) -> bool {
    is_address(token)
        || is_bearing_pair(token)
        || is_key_prefix_token(token)
        || is_opaque_token(token)
}

/// 칸을 구분자 조각으로 나눈 뒤 키로 보이는 조각만 가렸음 표시로 바꿈.
/// 구분자는 원문 그대로 자리에 남긴다.
pub(super) fn redact_key_parts(token: &str) -> String {
    let mut out = String::new();
    let mut part = String::new();
    for c in token.chars() {
        if !TOKEN_SEPARATORS.contains(&c) {
            part.push(c);
            continue;
        }
        flush_part(&mut out, &mut part);
        out.push(c);
    }
    flush_part(&mut out, &mut part);
    out
}

/// 쌓아 둔 조각이 키로 보이면 가렸음 표시로 바꿔 out에 붙임. 조각은 초기화.
fn flush_part(out: &mut String, part: &mut String) {
    if !part.is_empty() {
        if is_hidden(part) {
            out.push_str(SECRET_REDACTED);
        } else {
            out.push_str(part);
        }
        part.clear();
    }
}

/// 구분자 기준으로 앞이름·값 둘로 나눔.
fn split_pair(token: &str) -> Option<(&str, &str)> {
    let separator = KEY_VALUE_SEPARATORS
        .into_iter()
        .find(|c| token.contains(*c))?;
    token.split_once(separator)
}
