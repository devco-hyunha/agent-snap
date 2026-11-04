//! 모의 서버 자체 시험: 지연·멈춤·연결 종료·요청 기록을 확인합니다(작업 2 DoD).

use axum::http::StatusCode;

use super::script::{MockBodyPart, MockResponseScript};
use super::server::MockServer;
use std::time::Duration;
use std::time::Instant;

#[tokio::test]
async fn records_request_headers_and_body() {
    let server = MockServer::spawn(MockResponseScript::hang_headers()).await;
    assert!(server.base_url().starts_with("http://127.0.0.1:"));
    server.set_script(MockResponseScript::empty_ok()).await;

    let client = reqwest::Client::new();
    let response = client
        .post(server.chat_completions_url())
        .header("Authorization", "Bearer test-key-value")
        .header("Content-Type", "application/json")
        .body(r#"{"model":"demo","stream":true}"#)
        .send()
        .await
        .expect("모의 서버 요청");
    assert_eq!(response.status(), StatusCode::OK);

    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 1);
    let req = &recorded[0];
    assert_eq!(req.method, "POST");
    assert_eq!(req.path, "/chat/completions");
    let auth = req
        .headers
        .get("authorization")
        .expect("Authorization 헤더")
        .to_str()
        .expect("Authorization 문자열");
    assert_eq!(auth, "Bearer test-key-value");
    assert_eq!(req.body, br#"{"model":"demo","stream":true}"#);
}

#[tokio::test]
async fn header_delay_is_observed() {
    let delay = Duration::from_millis(200);
    let server = MockServer::spawn(MockResponseScript {
        header_delay: delay,
        ..MockResponseScript::empty_ok()
    })
    .await;
    let client = reqwest::Client::new();
    let started = Instant::now();
    let response = client
        .post(server.chat_completions_url())
        .body("{}")
        .send()
        .await
        .expect("지연 헤더 요청");
    let elapsed = started.elapsed();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        elapsed >= Duration::from_millis(150),
        "헤더 지연이 너무 짧음: {elapsed:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "헤더 지연이 너무 김: {elapsed:?}"
    );
}

#[tokio::test]
async fn body_stall_is_observed() {
    let stall = Duration::from_millis(200);
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(b"first".to_vec()),
        MockBodyPart::after(stall, b"second".to_vec()),
    ]))
    .await;
    let client = reqwest::Client::new();
    let mut response = client
        .post(server.chat_completions_url())
        .body("{}")
        .send()
        .await
        .expect("본문 멈춤 요청");
    assert_eq!(response.status(), StatusCode::OK);

    let first = response.chunk().await.expect("첫 조각").expect("첫 바이트");
    assert_eq!(&first[..], b"first");

    let started = Instant::now();
    let second = response
        .chunk()
        .await
        .expect("둘째 조각")
        .expect("둘째 바이트");
    let elapsed = started.elapsed();
    assert_eq!(&second[..], b"second");
    assert!(
        elapsed >= Duration::from_millis(150),
        "본문 멈춤이 너무 짧음: {elapsed:?}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "본문 멈춤이 너무 김: {elapsed:?}"
    );
}

#[tokio::test]
async fn connection_close_is_observed() {
    let server = MockServer::spawn(MockResponseScript::hang_headers()).await;
    assert_eq!(server.open_connections(), 0);

    // 블록을 벗어나며 요청 Future를 버려 연결을 끊습니다.
    // `tokio::pin!` 값만 drop 하면 실제 Future는 살아 남습니다.
    {
        let client = reqwest::Client::new();
        let send = client.post(server.chat_completions_url()).body("{}").send();
        tokio::pin!(send);

        tokio::select! {
            result = &mut send => {
                panic!("헤더를 안 보내야 하는데 응답이 왔습니다: {result:?}");
            }
            _ = tokio::time::sleep(Duration::from_millis(200)) => {}
        }
        assert!(server.open_connections() >= 1, "연결이 열려 있어야 합니다");
    }

    let closed = server.wait_until_idle(Duration::from_secs(3)).await;
    assert!(closed, "3초 안에 서버 쪽 연결이 닫혀야 합니다");
    assert_eq!(server.open_connections(), 0);
}
