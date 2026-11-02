//! 스트림 오류 문구에서 URL·시크릿처럼 보이는 조각을 가리고 길이를 제한합니다.

/// 서버 오류 문구에서 URL·키처럼 보이는 조각을 가리고 길이를 제한합니다.
pub(in crate::llm) fn summarize_stream_error(raw: &str) -> String {
    const MAX_LEN: usize = 200;
    let without_urls = redact_urls(raw);
    let without_secrets = redact_secret_like(&without_urls);
    truncate_chars(&without_secrets, MAX_LEN)
}

fn redact_urls(input: &str) -> String {
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

fn redact_secret_like(input: &str) -> String {
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

fn truncate_chars(input: &str, max_chars: usize) -> String {
    let count = input.chars().count();
    if count <= max_chars {
        return input.to_string();
    }
    let truncated: String = input.chars().take(max_chars).collect();
    format!("{truncated}…")
}
