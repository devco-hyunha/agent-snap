//! 요청 기록 확인. 모의 서버가 메서드·경로·헤더·본문을 남기는지 본다.
use crate::fixtures::RawClient;
use crate::script::MockScript;
use crate::server::MockServer;

#[tokio::test]
async fn request_record_keeps_method_path_headers_body() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::with_status(201));

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let head = client
        .send(
            "POST",
            "/v1/chat/completions",
            &[("authorization", "Bearer test-token")],
            r#"{"model":"m1"}"#,
        )
        .await
        .unwrap();
    assert_eq!(head.status, 201);

    assert!(
        server
            .base_url()
            .starts_with(&format!("http://{}", server.address())),
        "base url points at the server address"
    );

    let recorded = server.requests().snapshot();
    assert_eq!(recorded.len(), 1);
    #[allow(clippy::unwrap_used)]
    let request = recorded.first().unwrap();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/v1/chat/completions");
    assert_eq!(request.header("authorization"), Some("Bearer test-token"));
    assert_eq!(request.body_text(), r#"{"model":"m1"}"#);

    server.stop();
}

#[tokio::test]
async fn request_record_keeps_each_request_in_order() {
    #[allow(clippy::unwrap_used)]
    let server = MockServer::start().await.unwrap();
    server.enqueue(MockScript::ok());
    server.enqueue(MockScript::with_status(500));

    #[allow(clippy::unwrap_used)]
    let mut client = RawClient::connect(server.address()).await.unwrap();
    let head = client.send("GET", "/v1/models", &[], "").await.unwrap();
    assert_eq!(head.status, 200);

    #[allow(clippy::unwrap_used)]
    let mut second = RawClient::connect(server.address()).await.unwrap();
    let head = second
        .send("POST", "/v1/chat/completions", &[], "body")
        .await
        .unwrap();
    assert_eq!(head.status, 500);

    let recorded = server.requests().snapshot();
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].method, "GET");
    assert_eq!(recorded[0].path, "/v1/models");
    assert!(recorded[0].body.is_empty());
    assert_eq!(recorded[1].method, "POST");
    assert_eq!(recorded[1].body_text(), "body");

    server.stop();
}
