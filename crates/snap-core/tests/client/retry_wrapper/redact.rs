//! 02-4 재시도 래퍼 가리기 시험. 래퍼가 내보내는 오류 문자열에서 키·호스트가 가려지는지 봄.

use snap_core::llm::LlmClient;
use tokio_util::sync::CancellationToken;

use crate::request::request_with_user_message;
use crate::retry_wrapper::materials::SETTINGS;
use crate::retry_wrapper::materials::wrapper;
use crate::script::MockScript;
use crate::server::MockServer;
use crate::stream_open::base_url;

#[tokio::test]
async fn wrapper_error_hides_base_url_host() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::with_status(429));

    let base_url = base_url(&server);
    let wrapper = wrapper(&server, SETTINGS);
    #[allow(clippy::expect_used)]
    let error = wrapper
        .open(&request_with_user_message(), CancellationToken::new())
        .await
        .expect_err("rate limit ends as status failure");

    let display = error.to_string();
    let debug = format!("{error:?}");
    let host = base_url.split_once("://").unwrap_or(("", "")).1;
    assert!(
        !display.contains(&base_url) && !debug.contains(&base_url),
        "display and debug hide the base_url: {display}"
    );
    assert!(
        !display.contains(host) && !debug.contains(host),
        "display and debug hide the host: {debug}"
    );

    server.stop();
}
