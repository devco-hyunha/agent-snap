//! 2안 왕복 기록의 잘림 상한 확인입니다(spec AC 5).
//!
//! 기본 4,000자 상수를 고정으로 쓰던 자리를 고쳤습니다(리뷰 1차 1번 지적 보정). 설정을 줄이면
//! messages에 보내는 글자뿐 아니라 기록의 결과 칸도 같은 상한으로 잘립니다(spec AC 5, 1안 규칙과 동일).

use snap_core::agent::{AgentLoop, LoopEnd, LoopOutcome, TRUNCATION_MARK, ToolCallFormat};
use snap_core::config::ToolCallForm;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{cancel_token, content_stream, form_config, registry_with_named_fixed};

/// 2안 기록 잘림이 설정값(`with_tool_result_max_chars`)을 따지는지 확인합니다.
#[tokio::test]
async fn text_record_truncates_at_configured_max_chars() {
    let long_output = "0123456789".repeat(30);
    let registry = registry_with_named_fixed("echo", &long_output);
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(content_stream("{\"name\":\"echo\",\"arguments\":{}}")),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&form_config(registry, ToolCallForm::Text).with_tool_result_max_chars(24))
    .await;
    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    let record = &result.tool_calls[0];
    assert_eq!(record.format, ToolCallFormat::Text);
    assert_eq!(
        record.result.chars().count(),
        24,
        "설정값 24자로 잘려야 합니다: {}",
        record.result
    );
    assert!(
        record.result.ends_with(TRUNCATION_MARK),
        "잘림 표식이 남아야 합니다: {}",
        record.result
    );
    let requests = server.recorded_requests().await;
    let second: serde_json::Value =
        serde_json::from_slice(&requests[1].body).expect("둘째 요청 본문 JSON");
    let messages = second["messages"].as_array().expect("messages 배열");
    let prefix: &str = "도구 결과 [echo]\n";
    let tool_lines: Vec<&serde_json::Value> = messages
        .iter()
        .filter(|message| message["role"] == "user")
        .filter(|message| {
            message["content"]
                .as_str()
                .is_some_and(|content| content.starts_with(prefix))
        })
        .collect();
    assert_eq!(
        tool_lines
            .iter()
            .next_back()
            .and_then(|message| message["content"].as_str())
            .map(|content| content.chars().count()),
        Some(prefix.chars().count() + 24),
        "messages의 도구 결과도 같은 상한을 따라야 합니다"
    );
}
