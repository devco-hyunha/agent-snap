//! 서버 상태(429·503·400) 재시도 통합 시험(AC 1·2·3): 요청 수와 요청 간격과 포기 이유를 모의 서버로 확인합니다.
//!
//! 상한 이하의 `Retry-After` 429는 성공 재시도, 헤더 없는 503은 횟수 초과 포기,
//! 상한 초과 `Retry-After` 429와 그 밖의 4xx(400)는 요청 한 번으로 포기합니다.
//! 시험 값은 짧게 주고 백오프 난수는 1.0으로 고정해 간격이 흔들리지 않게 합니다.

use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::http::{HeaderMap, StatusCode};

use super::super::common::mock_server::RecordedRequest;
use super::super::common::mock_server::{MockResponseScript, MockServer};
use super::super::common::support::{
    body_parts_sse, content_events, sample_request, success_stream_body,
};
use super::{client_at_with, drain, retry_status_http, set_unit_sample};
use snap_core::llm::{
    CancellationToken, LlmClient, NotCandidateDetail, RetryGiveUpReason, TransportError,
};

/// 200과 내용 한 줄과 `[DONE]`으로 바로 답하는 성공 스크립트입니다.
///
/// AC 1의 재시도 끝 성공 응답으로 씁니다.
fn success_script(content: &str) -> MockResponseScript {
    MockResponseScript::with_body_parts(body_parts_sse(&success_stream_body(&[content])))
}

/// 헤더 없이 주어진 상태 코드로 바로 답하는 스크립트입니다.
///
/// 조립하는 스크립트인지 읽는 쪽에서 바로 알게 칸을 적습니다.
/// `header_delay`는 0이라 상태줄을 바로 보내고, `response_headers`는 비워 헤더를 붙이지 않습니다.
fn plain_status(status: StatusCode) -> MockResponseScript {
    MockResponseScript {
        status,
        response_headers: HeaderMap::new(),
        header_delay: Duration::ZERO,
        ..MockResponseScript::empty_ok()
    }
}

/// 요청 기록에서 `at`번째 요청이 도착한 시각부터 다음 요청이 도착한 시각까지의 간격입니다.
///
/// 요청 수와 간격은 서버에 도착한 시각으로만 재고, 확인은 넉넉한 하한으로만 합니다.
fn gap_after(recorded: &[RecordedRequest], at: usize) -> Duration {
    recorded[at + 1]
        .received_at
        .duration_since(recorded[at].received_at)
}

/// 상태 코드까지 온 마지막 전송 오류의 포기 이유입니다.
///
/// 서버 상태가 아닌 오류(연결 실패 등)는 시험 전제가 어긋난 것이라 즉시 멈춥니다.
fn give_up_of(error: &TransportError) -> RetryGiveUpReason {
    match error {
        TransportError::ServerStatus {
            give_up: Some(reason),
            ..
        } => *reason,
        other => panic!("서버 상태 포기 봉지여야 합니다: {other:?}"),
    }
}

// AC 1: 상한 이하 2초짜리 `Retry-After`를 단 한 번의 429 뒤에 성공 — 요청 2회, 두 요청 사이 간격 ≥ `Retry-After`.
// 모의 서버는 다음 요청부터 답을 바꿀 수 있어, 첫 429 뒤 재시도 요청 시각 안에 성공 응답으로 바꿉니다.
#[tokio::test]
async fn retry_after_within_cap_retries_once_and_succeeds() {
    let server = Arc::new(
        MockServer::spawn(MockResponseScript::status_with_retry_after(
            StatusCode::TOO_MANY_REQUESTS,
            "2",
        ))
        .await,
    );
    let swap = server.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;
        swap.set_script(success_script("첫성공")).await;
    });

    let client = set_unit_sample(
        client_at_with(server.base_url(), None, retry_status_http(3)),
        1.0,
    );
    let mut stream = client
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect("열려야 합니다");
    let events = drain(&mut stream).await;
    assert_eq!(content_events(&events), "첫성공");

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        2,
        "요청이 429 한 번과 재시도 한 번으로 2회여야 합니다"
    );
    let gap = gap_after(&recorded, 0);
    assert!(
        gap >= Duration::from_secs(2),
        "두 요청 사이 간격이 `Retry-After`(2초)보다 짧습니다: {gap:?}"
    );
}

// AC 2: 헤더 없는 503만 주는 서버 — 최초 1회 + 최대 재시도(시험 값 1) 요청 후 횟수 초과로 포기.
#[tokio::test]
async fn plain_503_gives_up_after_attempt_limit() {
    let server = MockServer::spawn(plain_status(StatusCode::SERVICE_UNAVAILABLE)).await;
    let client = set_unit_sample(
        client_at_with(server.base_url(), None, retry_status_http(1)),
        1.0,
    );
    let error = client
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect_err("헤더 없는 503은 포기해야 합니다");

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        2,
        "최초 1회와 최대 재시도 1회(시험 값)로 요청이 끝나야 합니다"
    );
    let gap = gap_after(&recorded, 0);
    assert!(
        gap >= Duration::from_millis(10),
        "재시도 사이 간격이 백오프 하한(10밀리초)보다 짧습니다: {gap:?}"
    );
    assert_eq!(
        give_up_of(&error),
        RetryGiveUpReason::AttemptsExceeded { attempt: 2 }
    );
}

// AC 3 앞부분: 상한(2초)을 넘는 3초짜리 `Retry-After` 429 — 요청 1회로 끝나고 대기 초를 밝힌 포기.
// 뒤에도 같은 응답을 유지해, 요청이 그 자리에서 끊겨 대기(3초)를 세지 않는지 함께 봅니다.
#[tokio::test]
async fn retry_after_over_cap_ends_at_one_request() {
    let server = MockServer::spawn(MockResponseScript::status_with_retry_after(
        StatusCode::TOO_MANY_REQUESTS,
        "3",
    ))
    .await;
    let client = set_unit_sample(
        client_at_with(server.base_url(), None, retry_status_http(1)),
        1.0,
    );
    let started = Instant::now();
    let error = client
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect_err("상한 초과는 포기해야 합니다");
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "요청이 상한 초과에서 끊기지 않고 재시도 대기를 갔습니다"
    );

    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 1, "상한 초과는 요청 1회로 끝나야 합니다");
    assert_eq!(
        give_up_of(&error),
        RetryGiveUpReason::RetryAfterExceedsCap {
            wait: Duration::from_secs(3)
        }
    );
}

// AC 3 뒷부분: 그 밖의 4xx(400) — 요청 1회로 끝난다.
#[tokio::test]
async fn plain_400_ends_at_one_request() {
    let server = MockServer::spawn(plain_status(StatusCode::BAD_REQUEST)).await;
    let client = set_unit_sample(
        client_at_with(server.base_url(), None, retry_status_http(3)),
        1.0,
    );
    let error = client
        .stream_chat(sample_request(None), CancellationToken::new())
        .await
        .expect_err("400은 포기해야 합니다");

    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 1, "400은 요청 1회로 끝나야 합니다");
    assert_eq!(
        give_up_of(&error),
        RetryGiveUpReason::NotCandidate(NotCandidateDetail::HttpStatus { status: 400 })
    );
}
