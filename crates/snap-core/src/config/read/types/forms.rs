//! 설정에 적는 이름으로 해석하는 값 타입 3종(서버 종류·생각 옵션·호출 형식).

/// 엔드포인트 서버 종류.
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
    /// 설정에 적는 이름으로 서버 종류를 찾음. 모르는 이름이면 `None`.
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

    /// 설정에 적는 서버 종류 이름 반환. `parse`와 같은 표기를 짝으로 씀.
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

/// 모델 공통 생각 옵션.
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
    /// 설정에 적는 이름으로 생각 옵션을 찾음. 모르는 이름이면 `None`.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "off" => Some(Self::Off),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    /// 설정에 적는 생각 옵션 이름 반환. `parse`와 같은 표기를 짝으로 씀.
    pub(in crate::config) fn to_name(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// 도구 호출 형식 선언.
///
/// `openai`·`text`는 감지 없이 선언대로만 쓰고, `auto`는 4x·2회 연속 감지 뒤에 2안으로 전환함.
/// 루프가 쓰는 형식 상태(`agent::ToolCallFormat`)와 다르고, 이 타입은 설정에 적히는 선언만 다룸.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolCallForm {
    /// OpenAI `tools` 경로 선언(기본).
    #[default]
    OpenAi,
    /// 텍스트 프로토콜 선언. 2안 조립으로 감지 없이 진행함.
    Text,
    /// 1안 시작 후 4x·2회 연속 감지 시에만 2안으로 전환.
    Auto,
}

impl ToolCallForm {
    /// 설정에 적는 이름으로 호출 형식 선언을 찾음. 모르는 이름이면 `None`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "openai" => Some(Self::OpenAi),
            "text" => Some(Self::Text),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }

    /// 설정에 적는 호출 형식 선언 이름 반환. `parse`와 같은 표기를 짝으로 씀.
    #[must_use]
    pub const fn to_name(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Text => "text",
            Self::Auto => "auto",
        }
    }
}
