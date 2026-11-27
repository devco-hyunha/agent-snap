//! 모의 서버로 스트림을 열어 종료 결과를 읽는 창구 재료. 사용량·시각 시험이 함께 쓰는 공용 조립만 담음.
use std::time::Duration;

use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;
use snap_core::llm::LlmClient;
use snap_core::llm::StreamOutcome;
use snap_core::llm::StreamReceiver;
use snap_core::llm::TransportTimings;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::script::StreamStop;
use crate::server::MockServer;

/// 시각 확인에 필요한 넉넉한 시험용 시간 값.
pub(crate) const OPEN_TIMINGS: TransportTimings = TransportTimings {
    connect: Duration::from_millis(500),
    header_wait: Duration::from_secs(2),
    stream_idle: Duration::from_secs(2),
};

/// 조각 사이 대기 없음.
pub(crate) const NO_GAP: Duration = Duration::ZERO;

/// 첫 토큰과 마지막 토큰 시각을 떼는 조각 사이 대기.
pub(crate) const CHUNK_GAP: Duration = Duration::from_millis(60);

/// 시험용 시간 값 묶음 조립 창구. 같은 시험 묶음의 시간·취소·연결 시험이 함께 씀.
pub(crate) const fn transport_timings(
    connect: Duration,
    header_wait: Duration,
    stream_idle: Duration,
) -> TransportTimings {
    TransportTimings {
        connect,
        header_wait,
        stream_idle,
    }
}

/// 모의 서버 주소를 `base_url` 형태로 만듦. `/v1`까지 포함.
pub(crate) fn base_url(server: &MockServer) -> String {
    format!("{}/v1", server.base_url())
}

/// 모의 서버 본문 규칙. 조각을 하나씩 보내고 끝 표시로 닫음.
fn body_steps(chunks: Vec<String>, gap: Duration) -> Vec<StreamStep> {
    let mut steps: Vec<StreamStep> = chunks
        .into_iter()
        .map(|chunk| StreamStep::chunk(chunk).chunk_after(gap))
        .collect();
    steps.push(StreamStep::Stop(StreamStop::End));
    steps
}

/// 모의 서버로 스트림을 열어 종료 결과를 읽음.
#[allow(clippy::expect_used)]
pub(crate) async fn outcome_from(
    server: &MockServer,
    chunks: Vec<String>,
    gap: Duration,
) -> StreamOutcome {
    server.enqueue(MockScript::ok().with_body(body_steps(chunks, gap)));
    let transport =
        HttpTransport::with_timings(&base_url(server), EndpointKind::Openai, None, OPEN_TIMINGS);
    let mut receiver = transport
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect("test opens stream");
    receiver.finish().expect("test finishes stream")
}
