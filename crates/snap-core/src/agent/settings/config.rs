//! 루프가 쓸 설정 값(`LoopConfig`)과 그 기본값을 채우는 자리
//!
//! 왕복 시작 전 준비 값만 담는 타입이라 변경 이유가 하나

use std::path::PathBuf;

use crate::config::ToolCallForm;
use crate::llm::ChatRequest;
use crate::tools::tool::ToolContext;

use crate::agent::record::registry::ToolRegistry;

use super::profile::ToolCallProfile;

/// 기본 반복 상한. 설정에 값이 없으면 이 값 사용
pub const DEFAULT_MAX_TURNS: u32 = 32;

/// 기본 도구 결과 잘림 상한. UTF-8 4,000자
pub const DEFAULT_TOOL_RESULT_MAX_CHARS: usize = 4_000;

/// 루프가 쓸 설정 값
pub struct LoopConfig {
    /// 도구 등록 목록
    pub registry: ToolRegistry,
    /// 도구 경로 검사 기준·잘림 상한·승인자가 들어 있는 실행 판
    pub context: ToolContext,
    /// `stop`·`tool_calls`를 제외한 골격 값(model·sampling·reasoning)
    pub base: ChatRequest,
    /// 시스템 프롬프트 앞머리(`agent/system_prompt.txt` 내용)
    pub system_prompt: String,
    /// 사용자 과제 한 장
    pub user_message: String,
    /// 반복 상한
    pub max_turns: u32,
    /// `tool_call` 낮은 온도 프로파일. 없으면 합치지 않음
    pub profile: Option<ToolCallProfile>,
    /// 호출 형식 선언. `openai`·`text`는 선언대로만 쓰고, `auto`일 때만 감지 사용
    pub tool_call_form: ToolCallForm,
    /// `AGENTS.md` 지시 파일 상한(바이트). 넘으면 표시 한 줄만 붙이고 원문은 그대로 둠
    pub agents_md_max_bytes: usize,
}

impl LoopConfig {
    /// 등록 목록과 기본값으로 설정 생성
    ///
    /// `base`는 `model`·`sampling`·`reasoning`을 채운 요청 골격이고, messages·tools·extra는 루프가 채움
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
            agents_md_max_bytes: crate::agent::prompt::agents_md::DEFAULT_AGENTS_MD_MAX_BYTES,
        }
    }

    /// 반복 상한 설정
    #[must_use]
    pub const fn with_max_turns(mut self, max_turns: u32) -> Self {
        self.max_turns = max_turns;
        self
    }

    /// 도구 결과 잘림 상한 설정
    #[must_use]
    pub const fn with_tool_result_max_chars(mut self, max_chars: usize) -> Self {
        self.context.tool_result_max_chars = max_chars;
        self
    }

    /// `tool_call` 낮은 온도 프로파일 설정
    #[must_use]
    pub const fn with_profile(mut self, profile: ToolCallProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    /// 호출 형식 선언 설정(선언 우선 → 감지)
    #[must_use]
    pub const fn with_tool_call_form(mut self, form: ToolCallForm) -> Self {
        self.tool_call_form = form;
        self
    }

    /// `AGENTS.md` 지시 파일 상한 설정(설정으로 변경 가능)
    #[must_use]
    pub const fn with_agents_md_max_bytes(mut self, max_bytes: usize) -> Self {
        self.agents_md_max_bytes = max_bytes;
        self
    }

    /// `run_command`의 승인 대기·실행 시간 상한과 백그라운드 확인 주기를 설정 값(`agent` 섹션)에서 받아 채움
    ///
    /// 설정 읽기 자리(`config::AgentSettings`)를 그대로 재사용하고, 도구 실행 판 한 곳에만 넣음
    #[must_use]
    pub fn with_run_command_deadline(mut self, settings: &crate::config::AgentSettings) -> Self {
        self.context.run_command =
            crate::tools::command::settings::RunCommandSettings::from_settings(settings);
        self.context.wake_interval = settings.wake_interval_seconds;
        self
    }

    /// CLI에서 주입하는 승인자를 도구 실행 판에 채움
    #[must_use]
    pub fn with_approver(mut self, approver: crate::tools::approval::request::ApproverBox) -> Self {
        self.context.approver = Some(approver);
        self
    }
}
