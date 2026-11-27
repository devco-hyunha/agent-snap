//! 서버 쪽 연결 정리 표시 확인. 연결이 끝났을 때 표시가 켜지는지만 본다.
use std::time::Duration;

use crate::fixtures::RawClient;
use crate::script::MockScript;
use crate::server::MockServer;

#[tokio::test]
async fn closed_flag_turns_on_when_client_connection_ends() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());

    assert!(
        !server.is_connection_closed(),
        "no connection yet means no close flag"
    );

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let head = client
        .send("POST", "/v1/chat/completions", &[], "")
        .await
        .unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    client
        .shutdown()
        .await
        .expect("test closes its own connection");

    let started = tokio::time::Instant::now();
    while !server.is_connection_closed() {
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "server side flag turns on after the client connection ends"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    server.stop();
}
