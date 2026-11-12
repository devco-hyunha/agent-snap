//! 03-2 루프 시험의 가짜 도구와 등록 목록 헬퍼입니다.
//!
//! 도구 결과는 고정 문자열로 두고, 승인이 필요한 도구는 즉시 거부 확인에 씁니다(테스트 계획).

#![allow(dead_code, unreachable_pub)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use std::sync::Arc;

use snap_core::agent::{DEFAULT_TOOL_RESULT_MAX_CHARS, LoopConfig, ToolRegistry};
use snap_core::tools::approver::{ApprovalFuture, ApprovalReply, ApproverBox, CommandApprover};
use snap_core::tools::tool::{ToolCallFuture, ToolContext, ToolOutcome};

use super::setup::loop_config_base;

/// 정해진 문자열을 그대로 도구 결과로 돌려주는 가짜 도구입니다.
/// 이름·결과를 `String`으로 받긴 결과를 필요한 시험(잘림 상한 확인)에서도 씁니다.
pub struct FixedTool {
    pub name: String,
    pub output: String,
}

impl snap_core::tools::tool::Tool for FixedTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "고정 결과를 돌려주는 가짜 도구"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type":"object","properties":{}})
    }

    fn call<'a>(&'a self, _args_json: &'a str, _ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        let output = self.output.clone();
        Box::pin(async move { ToolOutcome::Done(output) })
    }
}

/// 승인이 필요한 가짜 도구입니다(03-2 골격의 즉시 거부 확인용).
pub struct ApprovalTool {
    pub name: &'static str,
}

impl snap_core::tools::tool::Tool for ApprovalTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "승인이 필요한 가짜 도구"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type":"object","properties":{}})
    }

    fn executes_writes(&self) -> bool {
        true
    }

    fn call<'a>(&'a self, _args_json: &'a str, _ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        Box::pin(async move {
            ToolOutcome::ApprovalRequired {
                summary: "승인 요약".to_owned(),
            }
        })
    }
}

/// 승인 표식이 호출마다 열리는지 확인하는 시험의 승인자입니다(03-6 T1 critical 보정 격자).
///
/// 승인자가 불린 횟수를 세고, 정해 둔 응답을 불린 순번대로 돌려줍니다. 순번 목록이 끝나면 승인으로 봅니다.
pub struct CountingApprover {
    calls: Arc<AtomicUsize>,
    replies: Vec<ApprovalReply>,
}

/// 승인자를 실행 판에 넣을 상자로 만들고, 세는 수는 바깥에서 볼 복사본으로 줍니다.
#[must_use]
pub fn counting_approver(replies: &[ApprovalReply]) -> (ApproverBox, Arc<AtomicUsize>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let boxed = Box::new(CountingApprover {
        calls: calls.clone(),
        replies: replies.to_vec(),
    }) as ApproverBox;
    (boxed, calls)
}

impl CommandApprover for CountingApprover {
    fn approve<'a>(&'a self, _summary: &'a str) -> ApprovalFuture<'a> {
        let call_index = self.calls.fetch_add(1, Ordering::Relaxed);
        let reply = self
            .replies
            .get(call_index)
            .copied()
            .unwrap_or(ApprovalReply::Approved);
        Box::pin(async move { reply })
    }
}

/// 골격 설정(`loop_config_base`)에 승인자와 작업 폴더만 채운 설정입니다(03-6 승인 주입 확인용).
#[must_use]
pub fn loop_config_with_approver(
    registry: ToolRegistry,
    working_dir: PathBuf,
    approver: ApproverBox,
) -> LoopConfig {
    let mut config = loop_config_base(registry);
    config.context = ToolContext::new(working_dir, DEFAULT_TOOL_RESULT_MAX_CHARS);
    config.max_turns = 4;
    config.with_approver(approver)
}

/// 가짜 도구를 한 개 등록한 목록을 만듭니다.
#[must_use]
pub fn registry_with_fixed(name: &'static str, output: &'static str) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FixedTool {
        name: name.to_owned(),
        output: output.to_owned(),
    }));
    registry
}

/// 인자 스키마 검증 시험에 쓰는 `list_dir` 도구 하나만 등록한 목록입니다(2안 스키마 확인용).
#[must_use]
pub fn registry_with_list_dir() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(snap_core::tools::list_dir::ListDirTool));
    registry
}

/// 같은 이름의 도구를 두 개 등록한 목록을 만듭니다(등록 중복 검사 시험용).
#[must_use]
pub fn registry_with_fixed_twice(name: &'static str, output: &'static str) -> ToolRegistry {
    let mut registry = registry_with_fixed(name, output);
    registry.register(Box::new(FixedTool {
        name: name.to_owned(),
        output: output.to_owned(),
    }));
    registry
}

/// 이름과 결과를 빌리는 문자열로 정한 고정 도구를 한 개 등록한 목록입니다(긴 결과 잘림 시험용).
#[must_use]
pub fn registry_with_named_fixed(name: &str, output: &str) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FixedTool {
        name: name.to_owned(),
        output: output.to_owned(),
    }));
    registry
}

/// 승인 요청만 돌려주는 도구를 한 개 등록한 목록을 만듭니다.
#[must_use]
pub fn registry_with_approval(name: &'static str) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ApprovalTool { name }));
    registry
}
