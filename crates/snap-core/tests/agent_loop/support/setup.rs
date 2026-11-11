//! 03-2 루프 시험의 설정 골격(요청 골격·루프 설정·취소 토큰·작업 폴더)입니다.
//!
//! 같은 값을 여러 시험이 중복하지 않으려고 한 곳에 모릅니다(DRY).

#![allow(dead_code, unreachable_pub)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use snap_core::agent::{LoopConfig, ToolCallProfile, ToolRegistry};
use snap_core::config::{ExtraValue, Sampling, ToolCallForm};
use snap_core::llm::{CancellationToken, ChatRequest};

/// 빈 취소 토큰입니다.
#[must_use]
pub fn cancel_token() -> CancellationToken {
    CancellationToken::new()
}

/// 시험이 쓰는 작업 폴더 위치 표기입니다. 도구가 파일을 쓰지 않는 시험은
/// 현재 위치를 가리키는 상대 표기면 충분합니다. 절대경로를 시험마다 중복하지 않으려고 한 곳에 모읍니다.
#[must_use]
pub fn test_working_dir() -> PathBuf {
    PathBuf::from(".")
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

/// 루프 설정의 골격입니다. 앞머리·과제·작업 폴더를 같은 값으로 한 곳에서 채웁니다(DRY).
#[must_use]
pub fn loop_config_base(registry: ToolRegistry) -> LoopConfig {
    LoopConfig::new(
        registry,
        base_config(),
        "규범 앞머리",
        "과제 한 장",
        test_working_dir(),
    )
}

/// 왕복 상한과 프로파일까지 채운 루프 설정입니다.
#[must_use]
pub fn loop_config(
    registry: ToolRegistry,
    max_turns: u32,
    profile: Option<ToolCallProfile>,
) -> LoopConfig {
    let mut config = loop_config_base(registry);
    config.max_turns = max_turns;
    config.profile = profile;
    config
}

/// 호출 형식 선언(`openai`·`text`·`auto`)을 채운 루프 설정입니다(03-4 왕복 시험용).
#[must_use]
pub fn form_config(registry: ToolRegistry, form: ToolCallForm) -> LoopConfig {
    loop_config_base(registry).with_tool_call_form(form)
}
