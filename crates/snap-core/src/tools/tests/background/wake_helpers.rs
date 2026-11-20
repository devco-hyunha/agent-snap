//! 왕복 시험 재사용 재료: wake를 반복해 완료 한 줄을 받아옴
//!
//! 왕복 계열 시험이 wake 상한·반복 횟수를 다시 적지 않게 한 정의로 둠

use crate::tools::check_background::CheckBackgroundTool;
use crate::tools::tool::{Tool, ToolContext, ToolOutcome};

/// wake를 반복해 완료 결과가 나오면 그 한 줄로 빠져나옴
pub(super) async fn wake_until_finished(ctx: &ToolContext, tool: &CheckBackgroundTool) -> String {
    for _ in 0..12 {
        let outcome = tool.call("{}", ctx).await;
        let ToolOutcome::Done(text) = outcome else {
            panic!("wake 결과가 있어야 합니다: {outcome:?}");
        };
        if text.contains("완료: 종료 코드") {
            return text;
        }
    }
    panic!("wake 상한 안에 완료가 나와야 합니다")
}
