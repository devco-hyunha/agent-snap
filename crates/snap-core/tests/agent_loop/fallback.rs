//! 03-2 폴백 연결 지점 시험: AC 4의 4x 판별과 03-2 범위 확인.
//!
//! 2안 프로토콜 구현은 03-4의 몫이고, 여기서는 4x 판별과 상태 단정만 봅니다.
//! 03-4 모듈을 import하지 않음을 파일 경로가 증명합니다(전환 지점 인터페이스 확인).

use snap_core::agent::{
    AgentLoop, FallbackPoint, LoopConfig, LoopEnd, LoopError, LoopOutcome, ToolCallFormat,
    UNSUPPORTED_STATUS_CANDIDATES, first_turn_fallback_point, is_unsupported_status,
};
use snap_core::config::HttpSettings;

use axum::http::StatusCode;

use crate::common::mock_server::{MockBodyPart, MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    base_config, cancel_token, content_stream, registry_with_fixed, test_working_dir,
    tool_call_stream,
};

/// 미지원 후보 4x 목록이 02-6 재시도 후보(429·500/502/503/504)와 겹치지 않는지 확인합니다.
#[test]
fn unsupported_status_table_excludes_retry_candidates() {
    for status in UNSUPPORTED_STATUS_CANDIDATES {
        assert!(is_unsupported_status(*status), "{status}는 후보여야 합니다");
    }
    for status in [429u16, 500, 502, 503, 504, 401] {
        assert!(
            !is_unsupported_status(status),
            "{status}는 후보가 아닙니다(재시도 후보이거나 폴백 판단 대상 아님)"
        );
    }
}

/// 첫 턴 판정 1종(부름 없음 + 텍스트 1회)만 만드는지 단위로 봅니다(할 일 6·7).
/// 첫 왕복이 아니거나 호출 기록이 있으면 전환 판단을 만들지 않습니다.
#[test]
fn first_turn_text_makes_one_fallback_point() {
    assert_eq!(
        first_turn_fallback_point(true, 1, false),
        Some(FallbackPoint::NoToolCallText)
    );
    // 내용 없는 응답은 빈 응답 판정이 따로 맡습니다(reply.rs AC 4 시험).
    assert_eq!(first_turn_fallback_point(false, 1, false), None);
    // 첫 왕복이 아니면 세션 단위 판단이라 03-4 기록으로 넘깁니다.
    assert_eq!(first_turn_fallback_point(true, 2, false), None);
    // 도구 호출 기록이 있으면 정상 왕복입니다.
    assert_eq!(first_turn_fallback_point(true, 1, true), None);
}

/// 첫 턴 연결 확인: 루프가 호출 없는 텍스트 1회로 stop 종료할 때, 판정 함수가 만든 판단 값을
/// 결과에 담아 03-4에 넘기는지 봅니다. 4x 경로는 `transport_failure`이 같은 판단 타입으로 연결합니다(리뷰 1차 N2).
#[tokio::test]
async fn loop_stop_carries_first_turn_fallback_point() {
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream("답"))).await;
    let outcome = run_loop_with_no_retries(&server).await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    // 첫 왕복 텍스트 1회 판정이 루프를 통해 결과에 실려 03-4 소비 자리로 연결됩니다.
    assert_eq!(result.fallback_point, Some(FallbackPoint::NoToolCallText));
}

/// 왕복이 쌓인 뒤 텍스트로 끝나면 첫 턴 판정이 아니므로 판단 값을 담지 않습니다(None 유지).
#[tokio::test]
async fn later_turn_stop_has_no_fallback_point() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(tool_call_stream("call_a", "echo", "{\"x\":1}")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = run_loop_with_no_retries(&server).await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.final_text, "최종 답");
    assert_eq!(
        result.fallback_point, None,
        "첫 왕복이 아니면 03-4 기록 판단으로 넘기므로 판단 값을 담지 않습니다"
    );
}

/// AC 4 4x 갈래: 미지원 후보 400 응답이면 판단 값이 상태를 담고, 요청은 한 번만 나갑니다(재전송 없음).
#[tokio::test]
async fn unsupported_status_400_ends_with_status_point_once() {
    let script = MockResponseScript {
        status: StatusCode::BAD_REQUEST,
        body_parts: vec![MockBodyPart::immediate(b"tools not supported".to_vec())],
        ..MockResponseScript::empty_ok()
    };
    let server = MockServer::spawn(script).await;
    let outcome = run_loop_with_no_retries(&server).await;
    let LoopOutcome::Failed(LoopError::Unsupported { point, .. }) = outcome else {
        panic!("미지원 오류로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(point, FallbackPoint::UnsupportedStatus { status: 400 });
    assert_eq!(
        server.recorded_requests().await.len(),
        1,
        "02-6이 한 번 보낸 결과를 그대로 받고 루프가 재전송하지 않습니다"
    );
}

/// 후보에 없는 4x 전송 실패(401)는 상태를 보존한 `Transport`로 그대로 넘깁니다(폴백 판단 대상 아님).
#[tokio::test]
async fn non_candidate_401_keeps_transport_with_status() {
    let script = MockResponseScript {
        status: StatusCode::UNAUTHORIZED,
        ..MockResponseScript::empty_ok()
    };
    let server = MockServer::spawn(script).await;
    let outcome = run_loop_with_no_retries(&server).await;
    let LoopOutcome::Failed(LoopError::Transport { status, .. }) = outcome else {
        panic!("전송 오류로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(
        status,
        Some(401),
        "미지원 후보가 아닌 4x는 `Transport`의 상태 코드로 보존합니다"
    );
}

/// 도구 왕복 기록의 호출 형식 칸은 항상 1안입니다. 2안 기록은 03-4가 만들며, 여기서 2안 값은 만들지 않습니다(할 일 4).
#[tokio::test]
async fn tool_call_record_format_is_openai() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(tool_call_stream("call_a", "echo", "{\"x\":1}")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = run_loop_with_no_retries(&server).await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.final_text, "최종 답");
    assert_eq!(result.tool_calls[0].format, ToolCallFormat::OpenAi);
}

/// 재시도 0(한 번 보내고 포기) 설정으로 루프를 돌립니다(할 일 5: 재전송 없음 확인 전제).
async fn run_loop_with_no_retries(server: &MockServer) -> LoopOutcome {
    let http = HttpSettings {
        retries: 0,
        ..test_http()
    };
    let config = LoopConfig::new(
        registry_with_fixed("echo", "ok"),
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        // 도구가 파일을 쓰지 않는 시험이라 현재 위치를 가리키는 상대 표기면 충분합니다(리뷰 2차 N7).
        test_working_dir(),
    );
    AgentLoop::new(
        client_at_with(server.base_url(), None, http),
        cancel_token(),
    )
    .run(&config)
    .await
}
