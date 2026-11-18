#![allow(dead_code, unreachable_pub)]

//! 모델 목록 조회 시험이 같이 쓰는 설정·자격·스크립트 조립
//!
//! 설정 문자열을 여러 시험에서 다시 적지 않게 한곳에 둠

use std::time::Duration;

use snap_core::credentials::Credentials;

use crate::common::mock_server::{MockBodyPart, MockResponseScript};

/// 자격 키 값 확인용 마킹 조각
///
/// 목록 오류 표시에 이 값이 드러나면 안 됨
pub const KEY_MARK: &str = "key-ac3-models-list";

/// 목록 스크립트가 알려 주는 기본 id 두 개.
pub const DEFAULT_LIST_IDS: [&str; 2] = ["mock-model-a", "mock-model-b"];

/// 모르는 필드·`null`이 섞인 목록 응답에서 읽어야 할 id 두 개
pub const EXTRA_IDS: [&str; 2] = ["alpha", "beta"];

/// `pc1` 엔드포인트 인증 헤더 확인용 키 이름
pub const KEY_HEADER_NAME: &str = "authorization";

pub const FAST_SELECTION_KEY: &str = "pc1/fast";

/// 설정에 미리 넣어 놓은 모델 id
///
/// 목록 조회 결과에 섞이면 안 됨
pub const KEEPING_ID: &str = "keep-from-config";

pub const FIRST_ID_MARK: &str = "mock-model-a";

pub const SECOND_ID_MARK: &str = "mock-model-b";

pub const CANCEL_CONNECT_SECONDS: u64 = 5;
pub const CANCEL_HTTP_SECONDS: u64 = 2;
pub const CANCEL_IDLE_SECONDS: u64 = 1;

pub const CANCEL_DELAY: Duration = Duration::from_millis(250);
pub const CANCEL_UPPER_BOUND: Duration = Duration::from_millis(500);

/// Bearer 헤더 값 접두사.
pub const BEARER_PREFIX: &str = "Bearer ";

/// 설정 골격. `pc1` 엔드포인트를 모의 서버 주소로 연결
pub fn ac3_config_text(base_url: &str) -> String {
    format!(
        r#"{{"endpoints":[
            {{"name":"pc1","base_url":"{base_url}","kind":"openai","models":[
                {{"name":"fast","id":"{KEEPING_ID}"}}]}}]}}"#
    )
}

/// `pc1`에 마킹 키가 들어간 자격 파일 사본
pub fn ac3_credentials() -> Credentials {
    Credentials::parse(&format!(r#"{{"api_keys":{{"pc1":"{KEY_MARK}"}}}}"#)).expect("자격 파일")
}

/// 응답 본문 JSON을 200 목록 응답 스크립트로 조립
pub fn list_script(body_json: &str) -> MockResponseScript {
    MockResponseScript::with_body_parts(vec![MockBodyPart::immediate(
        body_json.as_bytes().to_vec(),
    )])
}

/// 취소·시간 초과 시험이 쓰는 `pc1` 설정. 짧은 시간 값(`http`)을 더함
pub fn ac3_http_config_text(base_url: &str) -> String {
    format!(
        r#"{{"endpoints":[
            {{"name":"pc1","base_url":"{base_url}","kind":"openai","models":[
                {{"name":"fast","id":"{KEEPING_ID}"}}]}}],
            "http":{{"connect_seconds":{CANCEL_CONNECT_SECONDS},
            "header_wait_seconds":{CANCEL_HTTP_SECONDS},
            "stream_idle_seconds":{CANCEL_IDLE_SECONDS}}}}}"#
    )
}

/// 기본 목록 응답 본문 JSON (헤더·본문 지연 본문 조립 시 같은 근거로 사용)
pub fn models_list_body() -> Vec<u8> {
    models_list_body_json().into_bytes()
}

pub fn models_list_body_json() -> String {
    format!(
        r#"{{"object":"list","data":[{{"id":"{}","object":"model","owned_by":"mock"}},{{"id":"{}","object":"model","owned_by":"mock"}}]}}"#,
        FIRST_ID_MARK, SECOND_ID_MARK
    )
}
