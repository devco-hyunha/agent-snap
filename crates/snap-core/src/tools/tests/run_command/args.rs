//! `run_command` 인자 검증 시험: 인자가 없거나 JSON이 깨지면 한 줄 오류만 남김 확인

use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::support::{TempFolder, context_of};

/// `command` 인자가 없거나 인자 JSON이 깨지면 한 줄 오류만 반환
#[tokio::test]
async fn missing_command_arg_is_one_line_error() {
    let folder = TempFolder::new("run-args");
    let ctx = context_of(&folder);
    for args in [r#"{ "path": "a.txt" }"#, "{\"command\":"] {
        let ToolOutcome::Error(text) = RunCommandTool.call(args, &ctx).await else {
            panic!("거부되어야 합니다: {args}");
        };
        assert!(text.starts_with("오류: "), "오류 한 줄 규격: {text}");
        assert_eq!(text.lines().count(), 1, "한 줄만: {text}");
    }
}
