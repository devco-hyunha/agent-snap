//! 서버 오류 본문 2차 가리기 자리. 어림 가리기를 지난 뒤에 알고 있는 키·호스트만 한 번 더 가림.
//!
//! 상태 오류 본문과 HTTP 200 안의 스트림 오류 요약이 같은 자리의 2차 가리기를 거침.

use reqwest::Url;

use crate::llm::SseError;
use crate::llm::TransportError;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::redacted::RedactedSummary;
use crate::llm::summary::SUMMARY_MAX_CHARS;
use crate::llm::summary::clamp_len;
use crate::locale::config::SECRET_REDACTED;

/// 가린 요약에서 알고 있는 키·호스트 문자열을 가렸음 표시로 바꿈.
///
/// 빈 문자열은 건너뛰고, 결과를 길이 상한으로 한 번 더 자름.
pub(super) fn redact_known(summary: &RedactedSummary, secrets: &KnownSecrets) -> RedactedSummary {
    let needles = secret_needles(secrets);
    let text = summary.expose();
    let replaced = needles
        .iter()
        .filter(|needle| !needle.is_empty())
        .fold(text.to_owned(), |text, needle| {
            text.replace(needle.as_str(), SECRET_REDACTED)
        });
    let capped = &replaced[..clamp_len(&replaced, SUMMARY_MAX_CHARS)];
    RedactedSummary::new(capped.to_owned())
}

/// 스트림 오류의 서버 요약을 2차로 가린 전송 오류로 바꿈.
///
/// HTTP 200 안의 최상위 `error` 청크는 어림 가리기만 지난 상태라 여기서 한 번 더 가림.
/// 그 밖의 전송 오류는 그대로 둠.
pub(super) fn hide_stream(error: TransportError, secrets: &KnownSecrets) -> TransportError {
    let inner = match &error {
        TransportError::Stream(SseError::Stream { summary }) => summary,
        _ => return error,
    };
    let hidden = redact_known(inner, secrets);
    TransportError::stream(SseError::stream(hidden))
}

/// 2차로 가릴 문자열 목록. 키·주소·주소의 호스트.
///
/// 호스트가 없으면 주소 원문만 목록에 담는다.
fn secret_needles(secrets: &KnownSecrets) -> Vec<String> {
    let mut needles = Vec::new();
    if let Some(key) = secrets.key() {
        needles.push(key.expose().to_owned());
    }
    let base_url = secrets.base_url();
    needles.push(base_url.to_owned());
    if let Some(host) = url_host(base_url) {
        needles.push(host.to_owned());
    }
    needles
}

/// 주소에서 호스트만. scheme·포트·경로를 뺀 값을 원문 부분 문자열로 돌려줌.
fn url_host(base_url: &str) -> Option<&str> {
    let host = Url::parse(base_url).ok()?.host_str()?.to_owned();
    let index = base_url.find(&host)?;
    Some(&base_url[index..index + host.len()])
}
