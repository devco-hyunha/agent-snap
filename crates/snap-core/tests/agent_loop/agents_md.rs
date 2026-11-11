//! 03-5 루프 계층 시험: `AGENTS.md`가 읽혀 첫 요청 system 앞머리에 규범 → 원문 순으로 들어가는지 봅니다(spec AC 1·AC 2·AC 4).
//!
//! 03-2 지원부 골격(`loop_config_base` 계층인 `agents_md_loop_config`)과 모의 서버 재료를 그대로 재사용하고,
//! 작업 폴더만 임시 작업 폴더로 바꿉니다.

use snap_core::agent::{
    AGENTS_MD_LONG_NOTICE, AgentLoop, DEFAULT_AGENTS_MD_MAX_BYTES, LoopEnd, LoopOutcome,
};

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{
    LoopTempWorkdir, agents_md_loop_config, cancel_token, content_stream, registry_with_fixed,
};

/// AC 1: 작업 폴더 `AGENTS.md` 원문이 규범 앞머리 뒤에 조립된 첫 요청을 보냅니다.
#[tokio::test]
async fn agents_md_reaches_first_request_system_head() {
    let folder = LoopTempWorkdir::new("loop-agents-md");
    folder.write("AGENTS.md", "규칙 원문 첫째 줄");
    let config = agents_md_loop_config(
        registry_with_fixed("echo", "ok"),
        folder.path().to_path_buf(),
        64,
    );
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream(
        "도구 호출 없는 답",
    )))
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    assert!(
        matches!(outcome, LoopOutcome::Finished(LoopEnd::Stop { .. })),
        "지시가 있어도 루프는 AC 1대로 종료합니다"
    );

    let requests = server.recorded_requests().await;
    let body: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("요청 본문 JSON");
    let system = body["messages"][0]["content"]
        .as_str()
        .expect("system 내용")
        .to_owned();
    assert!(
        system.starts_with("규범 앞머리"),
        "규범 앞머리가 먼저 와야 합니다: {system}"
    );
    assert!(
        system.contains("규칙 원문 첫째 줄"),
        "AGENTS.md 원문이 뒤에 따라야 합니다: {system}"
    );
}

/// AC 2: `AGENTS.md`가 없는 작업 폴더에서는 규범 앞머리만 보냅니다(지시 없는 상태로 루프 계속).
#[tokio::test]
async fn loop_without_agents_md_keeps_only_norm_head() {
    let folder = LoopTempWorkdir::new("loop-no-agents-md");
    let config = agents_md_loop_config(
        registry_with_fixed("echo", "ok"),
        folder.path().to_path_buf(),
        DEFAULT_AGENTS_MD_MAX_BYTES,
    );
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream("답"))).await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    assert!(
        matches!(outcome, LoopOutcome::Finished(LoopEnd::Stop { .. })),
        "지시 없이도 루프는 계속 돕니다"
    );
    let requests = server.recorded_requests().await;
    let body: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("요청 본문 JSON");
    assert_eq!(body["messages"][0]["content"], "규범 앞머리");
}

/// AC 4: 상한을 넘는 지시 파일은 원문 그대로 가고 끝에 표시 한 줄만 붙습니다.
#[tokio::test]
async fn long_agents_md_keeps_text_and_adds_notice_line() {
    let folder = LoopTempWorkdir::new("loop-long-agents-md");
    let text = "긴 규칙 ".repeat(64);
    folder.write("AGENTS.md", &text);
    let config = agents_md_loop_config(
        registry_with_fixed("echo", "ok"),
        folder.path().to_path_buf(),
        64,
    );
    let server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream("답"))).await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;
    assert!(
        matches!(outcome, LoopOutcome::Finished(LoopEnd::Stop { .. })),
        "긴 지시도 루프는 계속 돕니다"
    );
    let requests = server.recorded_requests().await;
    let body: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("요청 본문 JSON");
    let system = body["messages"][0]["content"]
        .as_str()
        .expect("system 내용")
        .to_owned();
    assert!(
        system.contains(&text),
        "원문을 잘라 넣지 않고 그대로 보내야 합니다"
    );
    assert!(
        system.ends_with(AGENTS_MD_LONG_NOTICE),
        "표시 한 줄이 끝에 붙어야 합니다"
    );
    assert_eq!(
        system.lines().count(),
        text.lines().count() + 2,
        "규범 앞머리 줄과 표시 한 줄을 더한 줄 수여야 합니다"
    );
}
