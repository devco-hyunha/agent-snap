//! 스트림 오류 2차 가리기 시험 재료. 어림 가리기를 그대로 지나가는 키·호스트 값과 전송 조립만 담음.
//! 같은 시험 묶음의 스트림 오류 가리기 시험이 재사용하는 재료 파일.

use snap_core::config::SecretString;
use snap_core::llm::EndpointKind;
use snap_core::llm::HttpTransport;

use serde_json::json;

use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_open::OPEN_TIMINGS;
use crate::stream_open::base_url;

/// 어림 가리기를 그대로 지나가는 시험용 키 값. 2차 가리기 대상만 확인.
pub(crate) const ECHO_KEY: &str = "echoedkeyvalue9999";

/// 지정한 호스트가 서버 오류 문구에 그대로 실린 형태. base_url 호스트 바늘 시험 재료.
pub(crate) fn echoed_message_for_host(host: &str) -> String {
    format!("upstream {host} failed for {ECHO_KEY} with quota exceeded")
}

/// 지정한 호스트가 들어간 최상위 `error` 청크 payload.
pub(crate) fn echoed_error_line_for_host(host: &str) -> String {
    let payload = json!({ "error": { "message": echoed_message_for_host(host) } }).to_string();
    format!("data: {payload}\n\n")
}

/// 지정한 호스트가 서버 오류 문구에 실린 HTTP 200 스트림 응답 스크립트.
pub(crate) fn echo_stream_script_for_host(host: &str) -> MockScript {
    MockScript::ok().with_body(vec![StreamStep::chunk(echoed_error_line_for_host(host))])
}

/// 키가 있는 시험용 전송 구현.
pub(crate) fn echo_transport(server: &MockServer, key: Option<&SecretString>) -> HttpTransport {
    HttpTransport::with_timings(
        &base_url(server),
        EndpointKind::Openai,
        key.cloned(),
        OPEN_TIMINGS,
    )
}
