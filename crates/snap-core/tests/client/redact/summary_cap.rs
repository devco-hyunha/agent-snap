//! T7 요약 상한 시험: 가린 요약이 문자 상한 안에 남는지와 긴 본문에서 키가 지워지는지 봄.

use snap_core::config::SecretString;
use snap_core::llm::TransportError;

use crate::redact::materials::open_error;
use crate::script::MockScript;
use crate::script::StreamStep;
use crate::server::MockServer;
use crate::stream_echo::ECHO_KEY;

#[tokio::test]
async fn status_error_summary_stays_inside_length_cap() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    let long = format!("{} {ECHO_KEY}", "상세 ".repeat(400));
    server.enqueue(MockScript::with_status(503).with_body(vec![StreamStep::chunk(long)]));

    let key = SecretString::new(ECHO_KEY.to_owned());
    let error = open_error(&server, Some(&key)).await;

    let TransportError::Status { summary, .. } = &error else {
        panic!("test expects status error: {error}");
    };
    assert!(
        summary.to_string().chars().count() <= snap_core::llm::SUMMARY_MAX_CHARS,
        "summary stays inside the cap"
    );
    assert!(
        !summary.to_string().contains(ECHO_KEY),
        "long body hides key"
    );

    server.stop();
}
