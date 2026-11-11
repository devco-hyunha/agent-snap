//! 설정에 적는 문자열로 해석하는 값 타입(색상·형식)입니다.
//!
//! `parse`는 설정에 적는 이름으로 값을 찾고, `to_name`은 같은 표기를 쓰는 저장 쪽과 짝으로 씁니다.

/// 엔드포인트가 따르는 서버 종류입니다(D20).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    /// OpenAI.
    OpenAi,
    /// OpenRouter.
    OpenRouter,
    /// Ollama.
    Ollama,
    /// vLLM.
    Vllm,
    /// llama.cpp 서버.
    LlamaCpp,
    /// LM Studio.
    LmStudio,
}

impl EndpointKind {
    /// 설정에 적는 이름으로 서버 종류를 찾습니다. 모르는 이름이면 `None`입니다.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "openai" => Some(Self::OpenAi),
            "openrouter" => Some(Self::OpenRouter),
            "ollama" => Some(Self::Ollama),
            "vllm" => Some(Self::Vllm),
            "llamacpp" => Some(Self::LlamaCpp),
            "lmstudio" => Some(Self::LmStudio),
            _ => None,
        }
    }

    /// 설정 파일에 적는 서버 종류 이름을 돌려줍니다(`parse`와 같은 표기를 짝으로 씁니다).
    pub(in crate::config) fn to_name(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::OpenRouter => "openrouter",
            Self::Ollama => "ollama",
            Self::Vllm => "vllm",
            Self::LlamaCpp => "llamacpp",
            Self::LmStudio => "lmstudio",
        }
    }
}

/// 모델의 공통 생각 옵션입니다(D15). 서버별 요청 필드로 바꾸는 일은 02-5가 합니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reasoning {
    /// 끔.
    Off,
    /// 낮음.
    Low,
    /// 중간.
    Medium,
    /// 높음.
    High,
}

impl Reasoning {
    /// 설정에 적는 이름으로 생각 옵션을 찾습니다. 모르는 이름이면 `None`입니다.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "off" => Some(Self::Off),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    /// 설정 파일에 적는 생각 옵션 이름을 돌려줍니다(`parse`와 같은 표기를 짝으로 씁니다).
    pub(in crate::config) fn to_name(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// 도구 호출 형식 선언입니다(D50 결정 3·결정 7).
///
/// `openai`·`text`는 감지 없이 선언대로만 쓰고, `auto`일 때만 4x·2회 연속 감지를 씁니다.
/// 루프가 쓰는 형식 상태(`agent::ToolCallFormat`)와 다르고, 이 값은 설정에 적히는 선언만 다룹니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolCallForm {
    /// 1안 OpenAI `tools` 경로를 선언합니다(기본). 감지는 하지 않습니다.
    #[default]
    OpenAi,
    /// 2안 텍스트 프로토콜을 선언합니다. 감지 없이 2안 조립으로 나갑니다(spec AC 7).
    Text,
    /// 1안으로 시작하고 4x·2회 연속 감지로만 2안에 들어갑니다.
    Auto,
}

impl ToolCallForm {
    /// 설정에 적는 이름으로 호출 형식 선언을 찾습니다. 모르는 이름이면 `None`입니다.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "openai" => Some(Self::OpenAi),
            "text" => Some(Self::Text),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }

    /// 설정 파일에 적는 호출 형식 선언 이름을 돌려줍니다(`parse`와 같은 표기를 짝으로 씁니다).
    #[must_use]
    pub const fn to_name(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Text => "text",
            Self::Auto => "auto",
        }
    }
}
