//! 엔드포인트·모델 설정 값 타입입니다.
//!
//! 기본값 상수는 부모(`types/mod.rs`)에 두고, 여기서는 타입 선언만 다룹니다.

use std::collections::BTreeMap;

use crate::secrets::SecretValue;

use super::forms::{EndpointKind, Reasoning, ToolCallForm};
use super::http::HttpSettings;
use super::sampling::{ExtraValue, Sampling};

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
    /// 클라이언트가 맞출 맥락 창 한도(토큰)입니다. 생략 시 [`super::DEFAULT_CONTEXT_WINDOW`]이고, 요청 본문에는 싣지 않습니다(D29).
    pub context_window: u64,
    /// 도구 호출 형식 선언입니다. 생략 시 [`super::DEFAULT_TOOL_CALL_FORM`]이고, 03-4의 2안 전환 경로를 정합니다(D50 결정 7).
    pub tool_call_form: ToolCallForm,
}
