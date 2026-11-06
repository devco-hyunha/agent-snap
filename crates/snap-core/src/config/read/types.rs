//! 검증을 통과한 설정 값의 타입입니다. 값은 읽는 시점에 검증되므로 이후 단계는 그대로 믿고 씁니다.

use std::collections::BTreeMap;
use std::time::Duration;

use crate::secrets::SecretValue;

/// `target_language`를 생략했을 때의 기본값입니다.
pub(in crate::config) const DEFAULT_TARGET_LANGUAGE: &str = "ko";

/// `agent-snap.jsonc`를 읽어 검증한 설정입니다(DECISIONS D10, D20).
///
/// 내부 진실은 `endpoints[].models` 중첩이며, 모델 조회는 [`Config::models`]로 합니다.
/// Debug에는 base_url이 [`SecretValue`]로 가려져 나옵니다.
#[derive(Debug)]
pub struct Config {
    /// 대상 언어 코드입니다(기본 `ko`).
    pub target_language: String,
    /// 엔드포인트 목록입니다. 생략하면 빈 목록입니다.
    pub endpoints: Vec<EndpointConfig>,
    /// 루트 `sampling` 기본값입니다. 적지 않은 항목은 서버 기본을 따릅니다(D14).
    pub sampling: Sampling,
    /// 시간·재시도 값입니다.
    pub http: HttpSettings,
}

impl Config {
    /// 모든 엔드포인트 아래 모델을 `(엔드포인트, 모델)` 쌍으로 펼쳐 돌려줍니다.
    pub fn models(&self) -> impl Iterator<Item = (&EndpointConfig, &ModelConfig)> {
        self.endpoints
            .iter()
            .flat_map(|endpoint| endpoint.models.iter().map(move |model| (endpoint, model)))
    }
}

/// 엔드포인트 한 개의 설정입니다. 키 칸은 없고, 키는 자격 파일에서 이름으로 찾습니다(D10).
#[derive(Debug)]
pub struct EndpointConfig {
    /// trim한 엔드포인트 이름입니다. 자격 파일의 키 이름으로 그대로 쓰입니다.
    pub name: String,
    /// 서버 기본 주소입니다. 호스트가 출력에 새지 않도록 비밀 값 래퍼로 담습니다(D19).
    pub base_url: SecretValue,
    /// 서버 종류입니다.
    pub kind: EndpointKind,
    /// 이 엔드포인트 아래 모델 목록입니다. 생략하면 빈 목록입니다.
    pub models: Vec<ModelConfig>,
}

/// 모델 한 개의 설정입니다.
#[derive(Debug)]
pub struct ModelConfig {
    /// trim한 모델 이름입니다. 같은 엔드포인트 안에서 유일합니다.
    pub name: String,
    /// 서버에 보낼 모델 id입니다(trim한 값).
    pub id: String,
    /// 모델별 샘플링 값입니다. 모델별 값이 루트 값보다 앞섭니다.
    pub sampling: Sampling,
    /// 생각 옵션입니다. 없으면 요청에 더하지 않습니다.
    pub reasoning: Option<Reasoning>,
    /// 요청 본문에 합칠 추가 값입니다. 없으면 빈 맵입니다.
    pub extra: BTreeMap<String, ExtraValue>,
}

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

/// 샘플링 값입니다. 적지 않은 항목은 `None`이며 요청 본문에서 빼 서버 기본을 따릅니다(D14).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sampling {
    /// 0 이상 2 이하.
    pub temperature: Option<f64>,
    /// 0 초과 1 이하.
    pub top_p: Option<f64>,
    /// 1 이상 정수.
    pub max_tokens: Option<u64>,
}

/// 시간·재시도 값입니다. 빠진 값은 [`HttpSettings::default`]의 값이 됩니다(D20).
#[derive(Debug, Clone, PartialEq)]
pub struct HttpSettings {
    /// 최초 요청 외 재시도 횟수입니다.
    pub retries: u32,
    /// 백오프 시작값입니다.
    pub backoff_min: Duration,
    /// 백오프 상한입니다.
    pub backoff_max: Duration,
    /// `Retry-After`를 따르는 상한입니다.
    pub retry_after_max: Duration,
    /// 연결 단계 제한입니다.
    pub connect_seconds: Duration,
    /// 요청 후 응답 헤더를 기다리는 제한입니다.
    pub header_wait_seconds: Duration,
    /// 스트림에서 다음 조각을 기다리는 제한입니다.
    pub stream_idle_seconds: Duration,
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            retries: 3,
            backoff_min: Duration::from_secs(1),
            backoff_max: Duration::from_secs(30),
            retry_after_max: Duration::from_secs(60),
            connect_seconds: Duration::from_secs(10),
            header_wait_seconds: Duration::from_secs(300),
            stream_idle_seconds: Duration::from_secs(120),
        }
    }
}

/// 모델 `extra`에 적힌 JSON 값입니다. 하위 객체까지 그대로 담습니다(D15).
#[derive(Debug, Clone, PartialEq)]
pub enum ExtraValue {
    /// `null`.
    Null,
    /// 불리언.
    Bool(bool),
    /// 숫자입니다. 적힌 모양 그대로 문자열로 보관합니다.
    Number(String),
    /// 문자열.
    String(String),
    /// 배열.
    Array(Vec<ExtraValue>),
    /// 객체.
    Object(BTreeMap<String, ExtraValue>),
}
