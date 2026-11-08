//! 03-2 루프 시험 재료: 가짜 도구·모의 서버 본문·루프 설정 골격입니다.
//!
//! 도구 결과는 고정 문자열로 두고, 시간은 하한·상한으로만 확인합니다(테스트 계획).

#![allow(dead_code, unreachable_pub)]

use std::collections::BTreeMap;

use snap_core::agent::{LoopConfig, ToolCallProfile, ToolRegistry};
use snap_core::config::{ExtraValue, Sampling};
use snap_core::llm::{CancellationToken, ChatRequest};
use snap_core::tools::tool::{ToolCallFuture, ToolContext, ToolOutcome};

use crate::common::mock_server::MockBodyPart;

/// 내용만 있는 스트림 줄을 모의 서버 본문 조각으로 만듭니다.
#[must_use]
pub fn content_stream(content: &str) -> Vec<MockBodyPart> {
    let line = format!(
        "data: {}\n\n",
        serde_json::json!({"choices":[{"delta":{"content":content}}]})
    );
    vec![
        MockBodyPart::immediate(line.into_bytes()),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}

/// 도구 호출 델타가 있는 스트림 줄을 모의 서버 본문 조각으로 만듭니다.
#[must_use]
pub fn tool_call_stream(id: &str, name: &str, arguments: &str) -> Vec<MockBodyPart> {
    let line = format!(
        "data: {}\n\n",
        serde_json::json!({"choices":[{"delta":{"tool_calls":[
            {"index":0,"id":id,"function":{"name":name,"arguments":arguments}}
        ]}}]})
    );
    vec![
        MockBodyPart::immediate(line.into_bytes()),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}

/// 내용도 도구 호출도 없는 빈 스트림입니다(AC 4 미지원 감지용).
#[must_use]
pub fn empty_stream() -> Vec<MockBodyPart> {
    vec![MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec())]
}

/// 도구 호출을 여러 개(병렬 index) 한 번에 알리는 모의 서버 본문 조각입니다(AC 3 여러 갈래 확인용).
#[must_use]
pub fn tool_calls_stream(calls: &[(&str, &str, &str)]) -> Vec<MockBodyPart> {
    let delta_calls: Vec<serde_json::Value> = calls
        .iter()
        .enumerate()
        .map(|(index, (id, name, arguments))| {
            serde_json::json!({"index": index, "id": id, "function": {"name": name, "arguments": arguments}})
        })
        .collect();
    vec![
        MockBodyPart::immediate(
            format!(
                "data: {}\n\n",
                serde_json::json!({"choices":[{"delta":{"tool_calls": delta_calls}}]})
            )
            .into_bytes(),
        ),
        MockBodyPart::immediate(b"data: [DONE]\n\n"[..].to_vec()),
    ]
}

/// 정해진 문자열을 그대로 도구 결과로 돌려주는 가짜 도구입니다.
pub struct FixedTool {
    pub name: &'static str,
    pub output: &'static str,
}

impl snap_core::tools::tool::Tool for FixedTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "고정 결과를 돌려주는 가짜 도구"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type":"object","properties":{}})
    }

    fn call<'a>(&'a self, _args_json: &'a str, _ctx: &'a ToolContext) -> ToolCallFuture<'a> {
        let output = self.output;
        Box::pin(async move { ToolOutcome::Done(output.to_owned()) })
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
    registry.register(Box::new(FixedTool { name, output }));
    registry
}

/// 같은 이름의 도구를 두 개 등록한 목록을 만듭니다(등록 중복 검사 시험용).
#[must_use]
pub fn registry_with_fixed_twice(name: &'static str, output: &'static str) -> ToolRegistry {
    let mut registry = registry_with_fixed(name, output);
    registry.register(Box::new(FixedTool { name, output }));
    registry
}

/// 승인 요청만 돌려주는 도구를 한 개 등록한 목록을 만듭니다.
#[must_use]
pub fn registry_with_approval(name: &'static str) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(ApprovalTool { name }));
    registry
}

/// 빈 취소 토큰입니다.
#[must_use]
pub fn cancel_token() -> CancellationToken {
    CancellationToken::new()
}

/// 루프가 쓸 요청 골격입니다. 모델 `extra` 값은 비어 있고 루프가 채웁니다.
#[must_use]
pub fn base_config() -> ChatRequest {
    ChatRequest {
        model: "loop-test-model".into(),
        messages: Vec::new(),
        tools: Vec::new(),
        sampling: Sampling::default(),
        reasoning: None,
        extra: BTreeMap::new(),
    }
}

/// 요청 골격에 `extra` 값을 넣은 골격을 만듭니다(AC 7 시험용).
#[must_use]
pub fn base_config_with(extra: BTreeMap<String, ExtraValue>) -> ChatRequest {
    ChatRequest {
        extra,
        ..base_config()
    }
}

/// 왕복 상한과 프로파일까지 채운 루프 설정입니다.
#[must_use]
pub fn loop_config(
    registry: ToolRegistry,
    max_turns: u32,
    profile: Option<ToolCallProfile>,
) -> LoopConfig {
    let mut config = LoopConfig::new(
        registry,
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        "C:\\workspace\\agent-snap",
    );
    config.max_turns = max_turns;
    config.profile = profile;
    config
}
