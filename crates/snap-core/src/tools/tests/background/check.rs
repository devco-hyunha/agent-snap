//! 백그라운드 목록이 비었을 때 확인 도구 결과 한 줄 시험

use crate::tools::check_background::CheckBackgroundTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::support::{TempFolder, context_of};

/// 백그라운드 목록이 비면 wake를 부르지 않은 상태로 한 줄만 남김
#[tokio::test]
async fn check_background_tool_reports_empty_registry() {
    let folder = TempFolder::new("bg-check-empty");
    let ctx = context_of(&folder);
    let outcome = CheckBackgroundTool.call("{}", &ctx).await;
    let ToolOutcome::Done(text) = outcome else {
        panic!("확인 결과가 있어야 합니다: {outcome:?}");
    };
    assert!(text.contains("백그라운드 명령이 없습니다"), "{text}");
    assert_eq!(CheckBackgroundTool.name(), "check_background");
    assert!(
        !CheckBackgroundTool.executes_writes(),
        "확인은 쓰기 도구 아님"
    );
}
