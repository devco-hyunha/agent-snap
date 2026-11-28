//! 스트리밍 응답 열기 자리. 연결 시간 설정·헤더 대기 시간·취소 경합·상태 구분만 담당.
//!
//! reqwest `timeout`·`read_timeout`은 구간을 가리지 못해 미사용.
//! 연결 시간은 reqwest 연결 시간 설정에만 맡기고, 헤더 대기는 이 파일이 tokio로 측정.
//! 본문 읽기와 가리기는 `status`가 맡고, 이 파일은 헤더까지 받은 응답만 넘김.

use reqwest::Client;
use reqwest::Method;
use reqwest::Response;
use serde_json::Value;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::config::secret_string::SecretString;
use crate::llm::TransportError;
use crate::llm::client::auth::auth_header;
use crate::llm::client::connect::connect_error;
use crate::llm::client::endpoint::completions_url;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::status::status_error;
use crate::llm::client::timings::TransportTimings;

/// 검증이 통과한 엔드포인트 정보로 스트리밍 응답을 열고 응답 헤더까지 받음.
///
/// 헤더가 `header_wait` 안에 오지 않으면 헤더 대기 시간 초과로 끝난다.
/// 헤더 대기 중 취소는 같은 자리에서 경합.
/// 헤더 이후 상태가 정상이면 상태 오류로 만들고, 본문은 가린 요약만 담는다.
pub async fn open_stream(
    base_url: &str,
    key: Option<&SecretString>,
    timings: &TransportTimings,
    body: &Value,
    cancel: &CancellationToken,
    secrets: &KnownSecrets,
) -> Result<Response, TransportError> {
    let client = build_client(timings)?;
    let builder = build_request(&client, base_url, key, body);
    let sent = tokio::select! {
        () = cancel.cancelled() => return Err(TransportError::cancelled()),
        sent = timeout(timings.header_wait, builder.send()) => sent,
    };
    let mut response = match sent {
        Ok(Ok(response)) => response,
        Ok(Err(error)) => {
            return Err(connect_error(&error, timings.connect_seconds()));
        }
        Err(_) => return Err(TransportError::header_wait(timings.header_wait_seconds())),
    };
    if !response.status().is_success() {
        return Err(status_error(&mut response, secrets).await);
    }
    Ok(response)
}

/// 연결 시간 설정만 실은 클라이언트. 요청 전체·읽기 시간 설정은 두지 않음.
pub(super) fn build_client(timings: &TransportTimings) -> Result<Client, TransportError> {
    Client::builder()
        .connect_timeout(timings.connect)
        .build()
        .map_err(|error| connect_error(&error, timings.connect_seconds()))
}

/// 스트리밍 채팅 요청 조립. 인증 헤더는 키가 있을 때만 붙임.
fn build_request(
    client: &Client,
    base_url: &str,
    key: Option<&SecretString>,
    body: &Value,
) -> reqwest::RequestBuilder {
    let mut builder = client.request(Method::POST, completions_url(base_url));
    if let Some((name, value)) = auth_header(key) {
        builder = builder.header(name, value);
    }
    builder.json(body)
}
