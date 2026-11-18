//! 선택 키 클라이언트의 `GET /models` 취소 통합 시험
//!
//! 채팅 전송 취소 규칙을 목록 조회가 그대로 따르는지 확인
//! 취소는 목록 전용 경로(헤더 받기 전·본문 읽는 중)에서 취소 오류 하나만 내고 곧 끝남
//! 목록은 재시도하지 않으므로 취소로 끝나면 요청이 한 번으로 멈춤

use std::time::{Duration, Instant};

use snap_core::config::Config;
use snap_core::llm::CancellationToken;
use snap_core::llm::TransportError;

use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};

use super::support::{
    CANCEL_DELAY, CANCEL_UPPER_BOUND, FAST_SELECTION_KEY, ac3_credentials, ac3_http_config_text,
    models_list_body,
};

/// 헤더를 받기 전에 취소하면 500ms 안에 취소 오류가 나고 요청이 한 번으로 멈춤
///
/// 목록 조회의 헤더 경로는 채팅 전송과 같은 취소 경합(`send_headers`)을 사용
#[tokio::test]
async fn cancel_before_headers_reports_cancelled_quickly() {
    // 헤더를 영원히 보내지 않는 서버라 취소가 헤더 경로의 유일한 종료 지점
    let server = MockServer::spawn(MockResponseScript::hang_headers()).await;
    let cancel = CancellationToken::new();
    let config = Config::parse(&ac3_http_config_text(server.base_url())).expect("AC 3 설정");
    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");

    // 취소는 `CANCEL_DELAY` 뒤로 미룸. 그 사이에 요청이 나가 헤더 대기 경로를 탑
    let started = Instant::now();
    let later = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(CANCEL_DELAY).await;
        later.cancel();
    });

    let error = client
        .list_models(cancel)
        .await
        .expect_err("헤더 대기 중 취소는 오류");

    assert!(
        matches!(error, TransportError::Cancelled),
        "취소 오류여야 합니다: {error}"
    );
    assert!(
        started.elapsed() < CANCEL_UPPER_BOUND,
        "취소가 헤더 대기를 방치하고 끝났습니다: {:?}",
        started.elapsed()
    );

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        1,
        "취소로 끝나서 재시도 요청이 나면 안 됩니다"
    );
    // 서버 쪽 연결이 3초 안에 닫힘
    assert!(server.wait_until_idle(Duration::from_secs(3)).await);
}

/// 헤더를 받고 본문을 읽는 도중 취소하면 500ms 안에 취소 오류가 남
///
/// 목록 본문 읽기는 채팅 스트림 유휴 경합과 같은 취소·유휴 경합(`fetch_models_body`)을 사용
#[tokio::test]
async fn cancel_mid_body_reports_cancelled_quickly() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(models_list_body()),
        MockBodyPart::after(Duration::from_secs(30), b"never".to_vec()),
    ]))
    .await;
    let cancel = CancellationToken::new();
    let config = Config::parse(&ac3_http_config_text(server.base_url())).expect("AC 3 설정");
    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");

    // 본문 앞 덩어리는 바로 오므로 그 뒤 `CANCEL_DELAY`에 취소
    let started = Instant::now();
    let later = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(CANCEL_DELAY).await;
        later.cancel();
    });

    let error = client
        .list_models(cancel)
        .await
        .expect_err("본문 읽기 중 취소는 오류");

    assert!(
        matches!(error, TransportError::Cancelled),
        "취소 오류여야 합니다: {error}"
    );
    assert!(
        started.elapsed() < CANCEL_UPPER_BOUND,
        "취소가 본문 읽기를 방치하고 끝났습니다: {:?}",
        started.elapsed()
    );

    let recorded = server.recorded_requests().await;
    assert_eq!(
        recorded.len(),
        1,
        "취소로 끝나서 재시도 요청이 나면 안 됩니다"
    );
    assert!(server.wait_until_idle(Duration::from_secs(3)).await);
}
