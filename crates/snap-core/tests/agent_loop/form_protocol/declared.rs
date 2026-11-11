use snap_core::agent::{AgentLoop, FallbackPoint, LoopEnd, LoopOutcome};
use snap_core::config::{HttpSettings, ToolCallForm};

use axum::http::StatusCode;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{cancel_token, content_stream, form_config, registry_with_fixed};

use super::tools_of;

/// AC 7: `text` 선언은 감지 없이 첫 요청부터 2안 조립이고, `openai` 선언은 전환을 쓰지 않는다.
#[tokio::test]
async fn declared_text_skips_detection_and_openai_stays_in_first_form() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(content_stream("{\"name\":\"echo\",\"arguments\":{}}")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let text_config = form_config(registry_with_fixed("echo", "ok"), ToolCallForm::Text);
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&text_config)
    .await;
    assert!(matches!(
        outcome,
        LoopOutcome::Finished(LoopEnd::Stop { .. })
    ));
    let requests = server.recorded_requests().await;
    let first: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("첫째 요청 본문 JSON");
    assert!(
        tools_of(&first).is_empty(),
        "`text` 선언은 감지 없이 2안 조립으로 나갑니다"
    );
    let system = first["messages"].as_array().expect("messages")[0]["content"]
        .as_str()
        .expect("system content");
    assert!(
        system.contains("JSON"),
        "지시문에 JSON 글자가 없습니다: {system}"
    );

    let openai_server = MockServer::spawn(MockResponseScript::with_body_parts(content_stream(
        "도구 호출 없는 답",
    )))
    .await;
    let openai_config = form_config(registry_with_fixed("echo", "ok"), ToolCallForm::OpenAi);
    let openai_outcome = AgentLoop::new(
        client_at_with(openai_server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&openai_config)
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = openai_outcome else {
        panic!("1안 선언은 텍스트 1회에서 AC 1대로 종료합니다: {openai_outcome:?}");
    };
    assert_eq!(
        result.fallback_point,
        Some(FallbackPoint::NoToolCallText),
        "`openai` 선언은 감지를 쓰지 않으므로 전환 기록만 남습니다"
    );
    let openai_requests = openai_server.recorded_requests().await;
    let openai_first: serde_json::Value =
        serde_json::from_slice(&openai_requests[0].body).expect("첫째 요청 본문 JSON");
    assert!(
        !tools_of(&openai_first).is_empty(),
        "`openai` 선언은 1안 tools를 유지합니다"
    );
}

/// AC 1 규칙: `auto` 선언의 4x는 2안으로 전환한 뒤 같은 자리에 1회를 쓰고, `text` 선언은 4x를 그대로 넘긴다.
#[tokio::test]
async fn declared_text_does_not_switch_on_4xx() {
    let script = MockResponseScript {
        status: StatusCode::NOT_FOUND,
        ..MockResponseScript::empty_ok()
    };
    let server = MockServer::spawn(script).await;
    let outcome = AgentLoop::new(
        client_at_with(
            server.base_url(),
            None,
            HttpSettings {
                retries: 0,
                ..test_http()
            },
        ),
        cancel_token(),
    )
    .run(&form_config(
        registry_with_fixed("echo", "ok"),
        ToolCallForm::Text,
    ))
    .await;
    assert!(matches!(outcome, LoopOutcome::Failed(_)));
    assert_eq!(
        server.recorded_requests().await.len(),
        1,
        "선언 값은 감지를 쓰지 않으므로 재전송하지 않습니다"
    );
}
