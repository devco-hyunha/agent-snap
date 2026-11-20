//! 백그라운드 확인 도구가 루프의 도구 결과 규칙으로 모델에 넘어가는지 확인.
//!
//! wake 한 번은 에이전트 턴 하나와 대응하므로, 도구 호출 한 바퀴의 기록으로 넘어감만 본다.

use snap_core::agent::{AgentLoop, LoopEnd, LoopOutcome, ToolRegistry};
use snap_core::tools::check_background::CheckBackgroundTool;

use crate::common::mock_server::{MockResponseScript, MockServer};
use crate::common::support::{client_at_with, test_http};
use crate::support::{cancel_token, content_stream, loop_config_base, tool_call_stream};

/// 모의 서버가 `check_background`를 부르고, 확인 결과가 도구 기록으로 모델에 넘어감.
#[tokio::test]
async fn check_background_round_trip_reaches_model_record() {
    let server = MockServer::spawn_queue(vec![
        MockResponseScript::with_body_parts(tool_call_stream(
            "call_check",
            "check_background",
            "{}",
        )),
        MockResponseScript::with_body_parts(content_stream("최종 답")),
    ])
    .await;

    let mut registry = ToolRegistry::new();
    registry.register(Box::new(CheckBackgroundTool));
    let config = loop_config_base(registry);
    let outcome = AgentLoop::new(
        client_at_with(server.base_url(), None, test_http()),
        cancel_token(),
    )
    .run(&config)
    .await;

    let LoopOutcome::Finished(LoopEnd::Stop { result }) = outcome else {
        panic!("stop으로 끝나야 합니다: {outcome:?}");
    };
    assert_eq!(result.tool_calls.len(), 1);
    let record = &result.tool_calls[0];
    assert_eq!(record.tool_name, "check_background");
    assert_eq!(record.result, "백그라운드 명령이 없습니다");
}
