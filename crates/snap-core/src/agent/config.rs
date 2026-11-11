//! 루프가 쓸 설정 값(`LoopConfig`)과 그 기본값을 채우는 자리입니다.
//!
//! 설정 값은 왕복을 시작하기 전의 준비 값만 담는 타입이라, 변경 이유가 하나입니다.
//! 기본값 상수 두 개와 설정 값을 채우는 setter를 같은 책임을 가진 이 파일에 두고,
//! 결과·오류 타입만 `types.rs`에 남깁니다(dev-principles 2절 SRP).

use std::path::PathBuf;

use crate::config::ToolCallForm;
use crate::llm::ChatRequest;
use crate::tools::tool::ToolContext;

use super::profile::ToolCallProfile;
use super::registry::ToolRegistry;

/// 기본 반복 상한입니다. 설정에 값이 없으면 이 값을 씁니다(spec AC 2).
pub const DEFAULT_MAX_TURNS: u32 = 32;

/// 기본 도구 결과 잘림 상한입니다. UTF-8 4,000자(spec AC 5).
pub const DEFAULT_TOOL_RESULT_MAX_CHARS: usize = 4_000;

/// 루프가 쓸 설정 값입니다.
pub struct LoopConfig {
    /// 도구 등록 목록입니다.
    pub registry: ToolRegistry,
    /// 도구 경로 검사 기준·잘림 상한·승인자가 들어 있는 실행 판입니다.
    pub context: ToolContext,
    /// `stop`·`tool_calls`를 제외한 골격 값(model·sampling·reasoning)입니다.
    pub base: ChatRequest,
    /// 시스템 프롬프트 앞머리(`agent/system_prompt.txt` 내용)입니다.
    pub system_prompt: String,
    /// 사용자 과제 한 장입니다.
    pub user_message: String,
    /// 반복 상한입니다.
    pub max_turns: u32,
    /// `tool_call` 낮은 온도 프로파일입니다. 없으면 합치지 않습니다.
    pub profile: Option<ToolCallProfile>,
    /// 호출 형식 선언입니다. `openai`·`text`는 선언대로만 쓰고, `auto`일 때만 감지를 씁니다(D50 결정 3).
    pub tool_call_form: ToolCallForm,
    /// `AGENTS.md` 지시 파일 상한(바이트)입니다. 넘으면 표시 한 줄만 붙이고 원문은 그대로 둡니다(spec AC 4).
    pub agents_md_max_bytes: usize,
}

impl LoopConfig {
    /// 등록 목록과 기본값으로 설정을 만듭니다.
    ///
    /// `base`는 `model`·`sampling`·`reasoning`을 채운 요청 골격이고, messages·tools·extra는 루프가 채웁니다.
    pub fn new(
        registry: ToolRegistry,
        base: ChatRequest,
        system_prompt: impl Into<String>,
        user_message: impl Into<String>,
        working_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            registry,
            context: ToolContext::new(working_dir, DEFAULT_TOOL_RESULT_MAX_CHARS),
            base,
            system_prompt: system_prompt.into(),
            user_message: user_message.into(),
            max_turns: DEFAULT_MAX_TURNS,
            profile: None,
            tool_call_form: ToolCallForm::default(),
            agents_md_max_bytes: crate::agent::agents_md::DEFAULT_AGENTS_MD_MAX_BYTES,
        }
    }

    /// 반복 상한을 정합니다.
    #[must_use]
    pub const fn with_max_turns(mut self, max_turns: u32) -> Self {
        self.max_turns = max_turns;
        self
    }

    /// 도구 결과 잘림 상한을 정합니다.
    #[must_use]
    pub const fn with_tool_result_max_chars(mut self, max_chars: usize) -> Self {
        self.context.tool_result_max_chars = max_chars;
        self
    }

    /// `tool_call` 낮은 온도 프로파일을 정합니다.
    #[must_use]
    pub const fn with_profile(mut self, profile: ToolCallProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    /// 호출 형식 선언을 정합니다(선언 우선 → 감지, D50 결정 3).
    #[must_use]
    pub const fn with_tool_call_form(mut self, form: ToolCallForm) -> Self {
        self.tool_call_form = form;
        self
    }

    /// `AGENTS.md` 지시 파일 상한을 정합니다(spec AC 4, 설정으로 변경 가능).
    #[must_use]
    pub const fn with_agents_md_max_bytes(mut self, max_bytes: usize) -> Self {
        self.agents_md_max_bytes = max_bytes;
        self
    }
}
