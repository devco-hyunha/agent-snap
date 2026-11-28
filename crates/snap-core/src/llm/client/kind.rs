//! 엔드포인트 종류 타입 자리. 서버 형식 6종과 이름 매핑 한곳만 담음.

/// 설정 파일 `kind` 값과 요청 변환이 쓰는 서버 형식.
///
/// 설정 검증이 허용하는 값의 한곳 정본. `PAIRS`가 곧 허용 목록.
/// 요청 변환은 이 타입으로만 종류를 받음.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    /// OpenAI 형식. `reasoning_effort` 사용.
    Openai,

    /// OpenRouter. `reasoning` 중첩 객체 사용.
    Openrouter,

    /// Ollama. 로컬 서버라 인증 헤더 없이도 동작.
    Ollama,

    /// vLLM 서버.
    Vllm,

    /// llama.cpp 서버.
    Llamacpp,

    /// LM Studio 서버. 공통 thinking 필드 없음.
    Lmstudio,
}

impl EndpointKind {
    /// 설정 값과 종류를 잇는 짝. 허용값과 이름 매핑의 한곳 정본.
    pub const PAIRS: [(Self, &'static str); 6] = [
        (Self::Openai, "openai"),
        (Self::Openrouter, "openrouter"),
        (Self::Ollama, "ollama"),
        (Self::Vllm, "vllm"),
        (Self::Llamacpp, "llamacpp"),
        (Self::Lmstudio, "lmstudio"),
    ];

    /// 설정 검증이 허용하는 `kind` 값. `PAIRS`에서 만든 목록.
    pub const NAMES: [&'static str; 6] = Self::names();

    /// `PAIRS`의 이름만 순서대로 모음.
    const fn names() -> [&'static str; 6] {
        let mut names = [""; 6];
        let mut index = 0;
        while index < Self::PAIRS.len() {
            names[index] = Self::PAIRS[index].1;
            index += 1;
        }
        names
    }

    /// 설정 값 문자열을 종류로 옮김. 목록 밖은 `None`.
    pub fn from_setting(value: &str) -> Option<Self> {
        Self::PAIRS
            .iter()
            .find(|(_, name)| *name == value)
            .map(|(kind, _)| *kind)
    }

    /// 설정 파일과 오류 문구가 쓰는 이름.
    pub fn as_str(self) -> &'static str {
        Self::PAIRS
            .iter()
            .find(|(kind, _)| *kind == self)
            .map(|(_, name)| *name)
            .unwrap_or(Self::PAIRS[0].1)
    }
}

impl std::fmt::Display for EndpointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
