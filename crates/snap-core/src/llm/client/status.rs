//! 비정상 HTTP 상태 구분 자리. 상태 코드와 가린 본문 요약만 담음.
//!
//! 본문은 읽기 상한까지만 읽고, 읽지 못한 본문은 빈 요약으로 변환해 상태 코드로만 보고한다.

use reqwest::Response;

use crate::consts::status::BODY_MAX_BYTES;
use crate::llm::client::error::TransportError;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::redact::redact_known;
use crate::llm::summary::redact_summary;

use crate::llm::SUMMARY_MAX_CHARS;

/// 응답을 읽어서 상태 오류로 만듦. 본문은 어림 가리기 뒤 알고 있는 키·호스트로 한 번 더 가림.
pub(super) async fn status_error(
    response: &mut Response,
    secrets: &KnownSecrets,
) -> TransportError {
    let status = response.status().as_u16();
    let raw = body_text(response).await;
    let summary = redact_known(&redact_summary(&raw), secrets);
    TransportError::status(status, summary)
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
