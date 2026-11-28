//! T7 전송 오류 가리기 시험 재료 자리. 되돌린 키·호스트 본문과 전송 조립만 담음.
//!
//! 시험용 키·호스트 값은 어림 가리기를 그대로 지나, 2차 가리기로만 지워지는 형태다.

use snap_core::config::SecretString;
use snap_core::llm::LlmClient;
use snap_core::llm::TransportError;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_echo::ECHO_KEY;
use crate::stream_echo::echo_transport;

/// 어림 가리기를 그대로 지나가는 시험용 호스트 문자열.
pub(crate) const ECHO_HOST: &str = "api.echo-host.invalid";

/// 2차 가리기로만 지워지는 값 두 종을 되돌려 주는 본문.
pub(crate) fn echoed_body() -> String {
    format!("upstream {ECHO_HOST} failed for {ECHO_KEY} with quota exceeded")
}

/// 지정한 상태 코드로 오류 본문을 되돌리는 응답 스크립트.
pub(crate) fn echo_script(status: u16) -> MockScript {
    MockScript::with_status(status).with_body(vec![StreamStep::chunk(echoed_body())])
}

/// 키가 있는 전송으로 요청을 보내고 상태를 받아냄.
#[allow(clippy::expect_used)]
pub(crate) async fn open_error(server: &MockServer, key: Option<&SecretString>) -> TransportError {
    echo_transport(server, key)
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("server error status ends the stream")
}
