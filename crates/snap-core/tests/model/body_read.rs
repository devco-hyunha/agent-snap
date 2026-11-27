//! 02-7 T3 본문 읽기 책임 시험. 본문 읽기·JSON 해석 실패가 헤더 이후 읽기 실패로 나가는지만 봄.

use snap_core::llm::ModelsError;
use snap_core::llm::TransportError;

use crate::materials::KEY;
use crate::materials::abort_script;
use crate::materials::json_script;
use crate::materials::models_of;
use crate::server::MockServer;

/// 본문 급중단으로 읽기가 실패한 경우. 헤더 이후 읽기 실패로 끝남. 재시도 후보가 아니라 시도는 한 번만 열림.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_body_abort_ends_as_stream_read() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(abort_script(200, r#"{"data":[{"id":"qwen-id""#));

    let error = models_of(&server)
        .await
        .expect_err("aborted body ends the list");

    assert_stream_read(&error);
    assert_eq!(
        server.requests().len(),
        1,
        "body read failure is not a retry candidate"
    );

    server.stop();
}

/// JSON이 아닌 본문. 해석 실패로 헤더 이후 읽기 실패에 담김. 재시도는 한 번만 열림.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_body_not_json_ends_as_stream_read() {
    let server = MockServer::start().await.unwrap();
    server.enqueue(json_script(200, "not a json body"));

    let error = models_of(&server)
        .await
        .expect_err("non-json body ends the list");

    assert_stream_read(&error);
    assert_eq!(
        server.requests().len(),
        1,
        "json parse failure is not a retry candidate"
    );

    server.stop();
}

/// 본문 읽기·해석 실패가 연결 설정 초 값만 담은 헤더 이후 읽기 실패인지 확인.
fn assert_stream_read(error: &ModelsError) {
    let ModelsError::Transport(TransportError::StreamRead { connect_seconds }) = error else {
        panic!("test expects stream read error: {error}");
    };
    assert!(
        *connect_seconds > 0.0,
        "stream read carries the connect setting"
    );

    let display = error.to_string();
    let debug = format!("{error:?}");
    for secret in [KEY, "127.0.0.1", "qwen-id"] {
        assert!(
            !display.contains(secret) && !debug.contains(secret),
            "stream read error hides body text and key: {display}"
        );
    }
}
