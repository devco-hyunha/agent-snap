//! 비정상 HTTP 상태 구분 자리. 상태 코드·가린 본문 요약·가린 `Retry-After` 값만 담음.
//!
//! 본문은 읽기 상한까지만 읽고, 읽지 못한 본문은 빈 요약으로 변환해 상태 코드로만 보고한다.
//! `Retry-After` 헤더는 한 번만 읽고 2차 가리기 뒤 담는다.

use reqwest::Response;

use crate::consts::status::BODY_MAX_BYTES;
use crate::consts::status::RETRY_AFTER_HEADER;
use crate::llm::client::error::TransportError;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::redact::redact_known;
use crate::llm::client::retry_after::RetryAfterHeader;
use crate::llm::redacted::RedactedSummary;
use crate::llm::summary::redact_summary;

use crate::llm::SUMMARY_MAX_CHARS;

/// 응답을 읽어서 상태 오류로 만듦. 본문은 어림 가리기 뒤 알고 있는 키·호스트로 한 번 더 가림. 헤더는 2차 가리기로만 옮김.
pub(super) async fn status_error(
    response: &mut Response,
    secrets: &KnownSecrets,
) -> TransportError {
    let status = response.status().as_u16();
    let retry_after = retry_after_header(response, secrets);
    let raw = body_text(response).await;
    let summary = redact_known(&redact_summary(&raw), secrets);
    TransportError::status(status, summary, retry_after)
}

/// `Retry-After` 헤더를 한 번만 읽어 2차 가리기로 옮김. 헤더가 없으면 없음.
fn retry_after_header(response: &Response, secrets: &KnownSecrets) -> Option<RetryAfterHeader> {
    let raw = response.headers().get(RETRY_AFTER_HEADER)?.to_str().ok()?;
    let hidden = redact_known(&RedactedSummary::new(raw.to_owned()), secrets);
    Some(RetryAfterHeader::new(hidden.expose().to_owned()))
}

/// 응답 본문을 상한까지만 읽고 텍스트로 옮김. 읽기 실패는 읽은 만큼만 쓴다.
async fn body_text(response: &mut Response) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    while bytes.len() < BODY_MAX_BYTES {
        match response.chunk().await {
            Ok(Some(chunk)) => bytes.extend_from_slice(&chunk),
            Ok(None) | Err(_) => break,
        }
    }
    let text = String::from_utf8_lossy(&bytes);
    text.chars().take(SUMMARY_MAX_CHARS).collect()
}
