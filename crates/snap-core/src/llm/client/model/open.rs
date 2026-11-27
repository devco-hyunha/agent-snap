//! 모델 목록 응답 열기 자리. GET 전송·헤더 대기 시간·취소 경합·상태 구분·본문 JSON 읽기만 담당.
//!
//! 연결 시간과 헤더 대기는 02-5의 `build_client`·`status_error`·`connect_error`를 그대로 재사용.
//! 본문은 스트림이 아니라 한 번 읽는 JSON 문자열이라 이 파일이 읽는다.

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
use crate::llm::client::endpoint::models_url;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::open::build_client;
use crate::llm::client::status::status_error;
use crate::llm::client::timings::TransportTimings;

/// 검증이 통과한 엔드포인트 정보로 모델 목록 응답을 받고 최상위 JSON 값으로 돌려줌.
///
/// 헤더가 `header_wait` 안에 오지 않으면 헤더 대기 시간 초과로 끝난다.
/// 헤더 대기 중 취소는 같은 자리에서 경합.
/// 정상 상태가 아니면 02-5의 상태 오류(가린 요약)로 끝난다.
pub(super) async fn open_models_json(
    base_url: &str,
    key: Option<&SecretString>,
    timings: &TransportTimings,
    cancel: &CancellationToken,
    secrets: &KnownSecrets,
) -> Result<Value, TransportError> {
    let client = build_client(timings)?;
    let builder = build_get_request(&client, base_url, key);
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
    read_json(response, timings).await
}

/// 응답 본문을 텍스트로 읽고 JSON 값으로 옮김. 읽기·해석 실패는 헤더 이후 읽기 실패로 봄.
async fn read_json(
    response: Response,
    timings: &TransportTimings,
) -> Result<Value, TransportError> {
    let read = response
        .text()
        .await
        .map_err(|_| json_read_error(timings))?;
    serde_json::from_str(&read).map_err(|_| json_read_error(timings))
}

/// 응답 본문 읽기·JSON 해석 실패. 헤더 이후 읽기 실패로 봄.
fn json_read_error(timings: &TransportTimings) -> TransportError {
    TransportError::stream_read(timings.connect_seconds())
}

/// 모델 목록 GET 요청 조립. 인증 헤더는 키가 있을 때만 붙임.
fn build_get_request(
    client: &Client,
    base_url: &str,
    key: Option<&SecretString>,
) -> reqwest::RequestBuilder {
    let mut builder = client.request(Method::GET, models_url(base_url));
    if let Some((name, value)) = auth_header(key) {
        builder = builder.header(name, value);
    }
    builder
}
