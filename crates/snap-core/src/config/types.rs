//! snap 설정 타입 자리. 읽기·기본값만 담고 검증 규칙은 T3가 맡음.

use serde::Deserialize;

use crate::config::secret_string::SecretString;
use crate::consts::defaults::DEFAULT_BACKOFF_MAX_SECONDS;
use crate::consts::defaults::DEFAULT_BACKOFF_MIN_SECONDS;
use crate::consts::defaults::DEFAULT_CONNECT_SECONDS;
use crate::consts::defaults::DEFAULT_HEADER_WAIT_SECONDS;
use crate::consts::defaults::DEFAULT_LANGUAGE;
use crate::consts::defaults::DEFAULT_RETRIES;
use crate::consts::defaults::DEFAULT_RETRY_AFTER_MAX_SECONDS;
use crate::consts::defaults::DEFAULT_STREAM_IDLE_SECONDS;

/// 최상위 설정. 생략한 목록은 빈 배열로 정규화.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// 화면 문구 언어. 기본 `ko`.
    #[serde(default = "default_language")]
    pub language: String,
    /// 엔드포인트 목록. 생략하면 빈 배열.
    #[serde(default)]
    pub endpoints: Vec<EndpointConfig>,
    /// 샘플링 기본값. 모델별 값이 있으면 그쪽이 우선(적용은 02-7).
    #[serde(default)]
    pub sampling: Option<Sampling>,
    /// 시간·재시도 값. 생략하면 기본값.
    #[serde(default)]
    pub http: HttpSettings,
}

/// 엔드포인트 하나. 키 칸은 자격 파일에만 두므로 이 타입에 없음.
#[derive(Debug, Deserialize)]
pub struct EndpointConfig {
    /// 엔드포인트 이름.
    pub name: String,
    /// 엔드포인트 주소. Debug에 값이 나오지 않음.
    pub base_url: SecretString,
    /// 서버 형식 구분 값. 값 검증은 T3가 맡음.
    pub kind: String,
    /// 이 엔드포인트 아래 모델 목록. 생략하면 빈 배열.
    #[serde(default)]
    pub models: Vec<ModelConfig>,
}

/// 모델 하나. 서버에 보낼 `id`는 이름과 별도.
#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    /// 모델 이름.
    pub name: String,
    /// 서버에 보낼 모델 식별 값.
    pub id: String,
    /// 모델별 샘플링 값. 없으면 최상위 기본값 사용(적용은 02-7).
    #[serde(default)]
    pub sampling: Option<Sampling>,
    /// 생각 강도 값. 값 검증은 T3가 맡음.
    #[serde(default)]
    pub reasoning: Option<String>,
    /// 공통 옵션에 없는 값. 요청 본문 합치기는 02-5가 맡음.
    #[serde(default)]
    pub extra: Option<serde_json::Map<String, serde_json::Value>>,
}

/// 샘플링 값. 없는 항목은 두지 않아 서버 기본을 따름.
#[derive(Debug, Deserialize)]
pub struct Sampling {
    /// 온도. 범위 검증은 T3가 맡음.
    #[serde(default)]
    pub temperature: Option<f64>,
    /// 상위 확률 합계. 범위 검증은 T3가 맡음.
    #[serde(default)]
    pub top_p: Option<f64>,
    /// 생성 최대 토큰 수. 범위 검증은 T3가 맡음.
    #[serde(default)]
    pub max_tokens: Option<u64>,
}

/// 시간·재시도 값. 키 7종만 인정하고 값 검증은 T3가 맡음.
#[derive(Debug, Deserialize)]
pub struct HttpSettings {
    /// 재시도 횟수.
    #[serde(default = "default_retries")]
    pub retries: u64,
    /// 재시도 대기 최소 초.
    #[serde(default = "default_backoff_min")]
    pub backoff_min: f64,
    /// 재시도 대기 최대 초.
    #[serde(default = "default_backoff_max")]
    pub backoff_max: f64,
    /// 서버가 권유한 재시도 대기 최대 초.
    #[serde(default = "default_retry_after_max")]
    pub retry_after_max: f64,
    /// 연결 시도 최대 초.
    #[serde(default = "default_connect_seconds")]
    pub connect_seconds: f64,
    /// 응답 헤더 대기 최대 초.
    #[serde(default = "default_header_wait_seconds")]
    pub header_wait_seconds: f64,
    /// 응답 스트림 대기 최대 초.
    #[serde(default = "default_stream_idle_seconds")]
    pub stream_idle_seconds: f64,
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            retries: default_retries(),
            backoff_min: default_backoff_min(),
            backoff_max: default_backoff_max(),
            retry_after_max: default_retry_after_max(),
            connect_seconds: default_connect_seconds(),
            header_wait_seconds: default_header_wait_seconds(),
            stream_idle_seconds: default_stream_idle_seconds(),
        }
    }
}

/// 언어 기본값 창구.
fn default_language() -> String {
    DEFAULT_LANGUAGE.to_owned()
}

/// 재시도 횟수 기본값 창구.
fn default_retries() -> u64 {
    DEFAULT_RETRIES
}

/// 재시도 대기 최소 기본값 창구.
fn default_backoff_min() -> f64 {
    DEFAULT_BACKOFF_MIN_SECONDS
}

/// 재시도 대기 최대 기본값 창구.
fn default_backoff_max() -> f64 {
    DEFAULT_BACKOFF_MAX_SECONDS
}

/// 재시도 권유 최대 기본값 창구.
fn default_retry_after_max() -> f64 {
    DEFAULT_RETRY_AFTER_MAX_SECONDS
}

/// 연결 시도 기본값 창구.
fn default_connect_seconds() -> f64 {
    DEFAULT_CONNECT_SECONDS
}

/// 헤더 대기 기본값 창구.
fn default_header_wait_seconds() -> f64 {
    DEFAULT_HEADER_WAIT_SECONDS
}

/// 스트림 대기 기본값 창구.
fn default_stream_idle_seconds() -> f64 {
    DEFAULT_STREAM_IDLE_SECONDS
}
