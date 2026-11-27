//! 02-7 T3 모델 목록 시험 공통 재료. 조회 창구·시간 값·응답 본문 조립만 담음.
//! 같은 시험 묶음의 정상·형식·상태·경로 시험이 재사용하는 재료 파일.

use std::time::Duration;

use snap_core::config::SecretString;
use snap_core::llm::ModelsError;
use snap_core::llm::RetrySettings;
use snap_core::llm::SystemRandomSource;
use snap_core::llm::TransportTimings;
use snap_core::llm::list_models;
use tokio_util::sync::CancellationToken;

use crate::script::ABORT_GAP;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;

/// 시험용 시간 값. 연결은 넉넉히, 헤더 대기는 짧게 둔다.
pub(crate) const TIMINGS: TransportTimings = TransportTimings {
    connect: Duration::from_secs(5),
    header_wait: Duration::from_secs(2),
    stream_idle: Duration::from_secs(2),
};

/// 시험용 재시도 설정. 재시도 대기는 0, 추가 시도는 한 번만.
pub(crate) const SETTINGS: RetrySettings = RetrySettings {
    max_retries: 1,
    backoff_min: Duration::ZERO,
    backoff_max: Duration::from_millis(1),
    retry_after_max: 60,
};

/// 시험용 키. 모의 서버가 받은 헤더로만 확인.
pub(crate) const KEY: &str = "sk-modelsvalue123";

/// 모의 서버 주소를 `base_url` 형태로 만듦. `/v1`까지 포함.
pub(crate) fn base_url(server: &MockServer) -> String {
    format!("{}/v1", server.base_url())
}

/// JSON 본문을 chunked 조각으로 보내고 끝 표시로 닫는 응답 스크립트.
pub(crate) fn json_script(status: u16, body: &str) -> MockScript {
    MockScript::with_status(status).with_body(vec![
        StreamStep::chunk(body.to_owned()),
        StreamStep::Stop(crate::script::StreamStop::End),
    ])
}

/// 헤더를 보낸 뒤 본문을 급히 끊는 응답 스크립트. 본문 읽기 실패 재료.
pub(crate) fn abort_script(status: u16, body: &str) -> MockScript {
    MockScript::with_status(status)
        .with_body(vec![
            StreamStep::chunk(body.to_owned()).chunk_after(ABORT_GAP),
            StreamStep::Stop(crate::script::StreamStop::Abort),
        ])
        .with_stop(crate::script::StreamStop::Abort)
}

/// 모의 서버에서 모델 목록을 읽어서 결과를 그대로 돌려줌. 시험 키를 함께 넘김.
pub(crate) async fn models_of(server: &MockServer) -> Result<Vec<String>, ModelsError> {
    let key = SecretString::new(KEY.to_owned());
    models_of_keyed(&base_url(server), Some(&key)).await
}

/// 키 없는 주소로 모델 목록을 읽어서 결과를 그대로 돌려줌. `/v1` 없는 서버 주소를 그대로 씀.
pub(crate) async fn models_of_without_key(server: &MockServer) -> Result<Vec<String>, ModelsError> {
    models_of_keyed(&server.base_url(), None).await
}

/// 조회 창구 한곳. 키 유무를 그대로 넘겨 키 있음·키 없음 경로를 재료에서 함께 볼 수 있게 함.
pub(crate) async fn models_of_keyed(
    url: &str,
    key: Option<&SecretString>,
) -> Result<Vec<String>, ModelsError> {
    list_models(
        url,
        key,
        &TIMINGS,
        &SETTINGS,
        SystemRandomSource::default(),
        &CancellationToken::new(),
    )
    .await
}
