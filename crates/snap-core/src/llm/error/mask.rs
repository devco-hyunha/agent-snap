//! 오류 문구의 URL·시크릿 모양을 가리는 어림 가리기 규칙입니다(03-6 R2 감사 분리).
//!
//! 이 모듈은 모양만 보고 가리는 어림 가리기만 맡고, 문구 조립과 길이 제한은
//! [`super::redact`]에 둡니다. 확인 가리기는 [`super::scrub`]이 맡습니다(D28).

/// `http://`·`https://`로 시작하는 조각을 `[주소]`로 바꿉니다.
pub(in crate::llm) fn redact_urls(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(end) = url_end(bytes, i) {
            out.push_str("[주소]");
            i = end;
            continue;
        }
        match input[i..].chars().next() {
            Some(ch) => {
                out.push(ch);
                i += ch.len_utf8();
            }
            None => break,
        }
    }
    out
}

/// 주소 조각의 끝(공백·따옴표·닫는 기호 전까지) 위치를 재는 자리입니다.
fn url_end(bytes: &[u8], start: usize) -> Option<usize> {
    let http = b"http://";
    let https = b"https://";
    let rest = &bytes[start..];
    let prefix_len = if rest.starts_with(https) {
        https.len()
    } else if rest.starts_with(http) {
        http.len()
    } else {
        return None;
    };
    let mut end = start + prefix_len;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_whitespace() || matches!(b, b'"' | b'\'' | b')' | b']' | b'}' | b'<' | b'>') {
            break;
        }
        end += 1;
    }
    Some(end)
}

/// 키·Bearer처럼 보이는 토근을 `[가림]`으로 바꿉니다.
pub(in crate::llm) fn redact_secret_like(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            let mut token = String::new();
            token.push(ch);
            while let Some(next) = chars.peek().copied() {
                if next.is_ascii_alphanumeric() || next == '_' || next == '-' {
                    token.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            if looks_like_secret(&token) {
                out.push_str("[가림]");
            } else {
                out.push_str(&token);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// 키 흔적으로 보이는 토근인지 모양으로만 판별합니다.
fn looks_like_secret(token: &str) -> bool {
    if token.len() < 16 {
        return false;
    }
    let lower = token.to_ascii_lowercase();
    if lower.starts_with("sk-") || lower.starts_with("bearer") {
        return true;
    }
    let has_digit = token.chars().any(|c| c.is_ascii_digit());
    let has_alpha = token.chars().any(|c| c.is_ascii_alphabetic());
    has_digit && has_alpha && token.len() >= 24
}
