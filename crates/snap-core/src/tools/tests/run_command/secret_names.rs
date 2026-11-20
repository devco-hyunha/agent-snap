//! 비밀 이름 검사 시험: 승인 표식이 채워진 뒤에도 명령 속 비밀 파일 이름은 보조 검사로 거부

use crate::tools::command::tool::RunCommandTool;
use crate::tools::tool::{Tool, ToolOutcome};

use super::super::approved_context::approved_run_context;
use super::super::args::command_args;
use super::super::support::TempFolder;

/// 승인자가 채워진 상태에서도 명령 문자열의 비밀 이름은 보조 검사로 거부
///
/// 셸이 명령을 나누는 문자(`;`·`|`·탭·줄바꿈)로 이어진 조각도 이름 판정 대상
#[tokio::test]
async fn secret_path_in_command_is_rejected_even_after_approval() {
    let folder = TempFolder::new("run-secret");
    let ctx = approved_run_context(&folder, 20);
    ctx.mark_approved();
    // 승인 표식은 호출마다 소비되므로 시험은 호출 전에 매번 채움
    for target in [
        "credentials.jsonc",
        "./sub/agent-snap.jsonc",
        ".env.local",
        "nested/dir/credentials.jsonc",
        "a;credentials.jsonc",
        "a|credentials.jsonc",
        "a\tcredentials.jsonc",
        "a\ncredentials.jsonc",
    ] {
        ctx.mark_approved();
        let outcome = RunCommandTool
            .call(&command_args(&format!("type {target}")), &ctx)
            .await;
        let ToolOutcome::Error(text) = outcome else {
            panic!("거부되어야 합니다({target}): {outcome:?}");
        };
        assert!(text.contains("비밀 파일"), "{target}: {text}");
    }
}
