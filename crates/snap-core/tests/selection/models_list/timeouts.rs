//! 선택 키 클라이언트의 `GET /models` 시간 초과 시험
//!
//! 목록 전용 경로 두 곳을 다룸
//! - 헤더 대기 전: 헤더를 늦게·아니 보내는 자리에서 헤더 대기 초과
//! - 본문 읽기 중: 본문이 멈춘 자리에서 본문 읽기 유휴 초과

use std::time::{Duration, Instant};

use snap_core::config::Config;
use snap_core::llm::CancellationToken;
use snap_core::llm::TimeoutSegment;
use snap_core::llm::TransportError;

use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};

use super::support::{
    DEFAULT_LIST_IDS, FAST_SELECTION_KEY, ac3_credentials, ac3_http_config_text,
    models_list_body_json,
};

/// 헤더가 유휴 설정보다 늦게 와도 헤더 대기 안에 오면 목록 조회 성공
#[tokio::test]
async fn list_models_header_later_than_idle_setting_succeeds() {
    let script = MockResponseScript {
        header_delay: Duration::from_millis(400),
        ..MockResponseScript::models_list()
    };
    let server = spawn_models_script_server(script).await;
    let ids = list_models_at(server.base_url()).await;
    assert_eq!(ids, DEFAULT_LIST_IDS);
}

/// 헤더가 끝내 오지 않으면 헤더 대기 구간 이름으로 시간 초과 발생
///
/// 구간 `HeaderWait`, 설정한 초 값(2초)이 오류에 찍히고 표시에 주소가 없음
#[tokio::test]
async fn list_models_header_needing_timeout_is_header_wait_segment() {
    let server = spawn_models_script_server(MockResponseScript::hang_headers()).await;
    let started = Instant::now();
    let error = list_models_error_at(server.base_url()).await;
    let text = format!("{error}");

    assert!(
        matches!(
            &error,
            TransportError::Timeout {
                segment: TimeoutSegment::HeaderWait,
                seconds: 2,
            }
        ),
        "헤더 대기 초과여야 합니다: {error:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(6));
    assert!(!text.contains("127.0.0.1"));
}

/// 헤더를 받고 본문이 멈추면 본문 읽기 유휴 구간 이름으로 시간 초과 발생
///
/// 유휴 구간 `StreamIdle`과 초 값 1을 내놓는지 확인
#[tokio::test]
async fn list_models_body_stall_is_stream_idle_segment() {
    // 헤더·목록 본문은 바로 주고, 그 뒤에 오는 조각은 5초 뒤로 미룸
    let script = MockResponseScript::with_body_parts(vec![
        MockBodyPart::immediate(models_list_body_json().into_bytes()),
        MockBodyPart::after(Duration::from_secs(5), b"never".to_vec()),
    ]);
    let server = spawn_models_script_server(script).await;
    let started = Instant::now();
    let timeout_error = list_models_error_at(server.base_url()).await;
    assert!(
        matches!(
            &timeout_error,
            TransportError::Timeout {
                segment: TimeoutSegment::StreamIdle,
                seconds: 1,
            }
        ),
        "본문 읽기 유휴 초과여야 합니다: {timeout_error:?}"
    );
    assert!(started.elapsed() < Duration::from_secs(4));
    assert!(!format!("{timeout_error}").contains("127.0.0.1"));
}

/// 설정 `http`의 헤더 대기 초가 조회 오류의 초 값에 그대로 찍힘
#[tokio::test]
async fn list_models_timeout_seconds_follow_config_http() {
    let server = spawn_models_script_server(MockResponseScript::hang_headers()).await;
    let started = Instant::now();
    let error = list_models_error_at(server.base_url()).await;

    assert!(
        matches!(
            &error,
            TransportError::Timeout {
                segment: TimeoutSegment::HeaderWait,
                seconds: 2,
            }
        ),
        "설정 헤더 대기 초가 오류에 찍혀야 합니다: {error:?}"
    );
    // 설정 초(2초) 안에 종료. 기본값 300초를 기다리지 않음
    assert!(
        started.elapsed() < Duration::from_secs(6),
        "설정 헤더 대기 시간 안에 끝나야 합니다: {:?}",
        started.elapsed()
    );
}

/// 짧은 시간 설정으로 목록 조회를 한 번 부름
async fn list_models_at(base_url: &str) -> Vec<String> {
    let client = ac3_short_http_client(base_url);
    client
        .list_models(CancellationToken::new())
        .await
        .expect("목록 조회 성공")
}

/// 같은 조립으로 목록 조회를 부르고 오류만 받아 옴
async fn list_models_error_at(base_url: &str) -> TransportError {
    let client = ac3_short_http_client(base_url);
    client
        .list_models(CancellationToken::new())
        .await
        .expect_err("시간 초과여야 합니다")
}

/// 짧은 시간 설정으로 선택 키 클라이언트를 만듦
fn ac3_short_http_client(base_url: &str) -> snap_core::llm::selection::SelectedClient {
    let config = Config::parse(&ac3_http_config_text(base_url)).expect("AC 3 설정");
    config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트")
}

/// `/models` 라우트에 지정한 스크립트를 따라 응답하는 서버를 띄움
///
/// 목록 전용 핸들러가 채팅 핸들러와 같은 스크립트 규격(헤더 지연·행·본문 조각 지연)을 따라감
async fn spawn_models_script_server(script: MockResponseScript) -> MockServer {
    MockServer::spawn_models_script(script).await
}
