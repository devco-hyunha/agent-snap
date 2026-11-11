//! 03-2 루프 시험의 가짜 도구와 등록 목록 헬퍼입니다.
//!
//! 도구 결과는 고정 문자열로 두고, 승인이 필요한 도구는 즉시 거부 확인에 씁니다(테스트 계획).

#![allow(dead_code, unreachable_pub)]

use snap_core::agent::ToolRegistry;
use snap_core::tools::tool::{ToolCallFuture, ToolContext, ToolOutcome};

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
