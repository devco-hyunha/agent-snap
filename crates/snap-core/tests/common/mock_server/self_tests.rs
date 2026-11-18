#![allow(dead_code, unreachable_pub)]

//! 모의 서버 자체 시험
//!
//! 모의 서버가 지연·멈춤·연결 종료·요청 기록·응답 상태·모델 목록을 지정한 스크립트대로 내보내는지 확인

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
    // 요청 시작 시각. 도착 시각이 앞서는지 확인
    let started = Instant::now();
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

    let checked = Instant::now();
    assert!(
        req.received_at >= started,
        "도착 시각이 요청 시작보다 앞섭니다"
    );
    assert!(
        req.received_at <= checked,
        "도착 시각이 기록 확인보다 뒤섭니다"
    );
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

    // 블록을 벗어나며 요청 Future를 버려 연결을 끊음
    // `tokio::pin!` 값만 drop 하면 실제 Future는 살아 남음
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

#[tokio::test]
async fn body_disconnect_is_observed() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(b"first".to_vec()),
        MockBodyPart::disconnect(),
    ]))
    .await;
    let client = reqwest::Client::new();
    let mut response = client
        .post(server.chat_completions_url())
        .body("{}")
        .send()
        .await
        .expect("본문 끊김 요청");
    assert_eq!(response.status(), StatusCode::OK);

    let first = response.chunk().await.expect("첫 조각").expect("첫 바이트");
    assert_eq!(&first[..], b"first");

    // 끊김을 밝힌 말이 오류 텍스트에 들어 있는지만 확인
    let closed = response
        .chunk()
        .await
        .expect_err("끊김 표식 뒤에는 본문 읽기가 실패해야 합니다");
    // 맨 윗말은 "본문 읽기 실패"만 밝히고 저수준 끊김 말은 아래 체인에 있음. Debug로 전체를 확인
    let text = format!("{closed:?}").to_lowercase();
    assert!(
        text.contains("closed") || text.contains("eof"),
        "끊김을 밝히는 오류어야 합니다: {text}"
    );

    // 끊긴 연결은 서버 쪽에서도 정리됨
    assert!(
        server.wait_until_idle(Duration::from_secs(3)).await,
        "끊긴 연결이 3초 안에 정리되어야 합니다"
    );
}

#[tokio::test]
async fn records_response_status() {
    let server = MockServer::spawn(MockResponseScript::status_with_retry_after(
        StatusCode::TOO_MANY_REQUESTS,
        "0",
    ))
    .await;
    let client = reqwest::Client::new();
    let response = client
        .post(server.chat_completions_url())
        .body("{}")
        .send()
        .await
        .expect("429 요청");
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

    server
        .set_script(MockResponseScript::status_with_retry_after(
            StatusCode::SERVICE_UNAVAILABLE,
            "0",
        ))
        .await;
    let response = client
        .post(server.chat_completions_url())
        .body("{}")
        .send()
        .await
        .expect("503 요청");
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 2);
    assert_eq!(
        recorded[0].status,
        StatusCode::TOO_MANY_REQUESTS,
        "첫 기록 상태가 첫 응답 상태와 같아야 합니다"
    );
    assert_eq!(
        recorded[1].status,
        StatusCode::SERVICE_UNAVAILABLE,
        "둘째 기록 상태가 둘째 응답 상태와 같아야 합니다"
    );
}

#[tokio::test]
async fn models_list_is_served() {
    let server = MockServer::spawn(MockResponseScript::models_list()).await;
    let client = reqwest::Client::new();
    let response = client
        .get(server.models_url())
        .send()
        .await
        .expect("모델 목록 요청");
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.expect("목록 본문 JSON");
    let ids: Vec<&str> = body["data"]
        .as_array()
        .expect("data 배열")
        .iter()
        .map(|item| item["id"].as_str().expect("id 문자열"))
        .collect();
    assert_eq!(ids, ["mock-model-a", "mock-model-b"]);

    // 목록 요청 기록에도 메서드·경로·상태가 함께 찍힘. 기록 본문은 요청 본문(GET이라 비어 있음).
    let recorded = server.recorded_requests().await;
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].method, "GET");
    assert_eq!(recorded[0].path, "/models");
    assert!(
        recorded[0].body.is_empty(),
        "GET 목록 요청의 요청 본문이 비어 있어야 합니다"
    );
    assert_eq!(recorded[0].status, StatusCode::OK);
}
