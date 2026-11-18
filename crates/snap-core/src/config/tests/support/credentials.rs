//! 자격 파일 해석·키 값 꺼내기·키 칸 개수 세기·자격 쪽 오류 확인·자격 파일 텍스트 견본. 설정 파일 견본(`save_samples.rs`)과 구분하려고 상수 이름에 `KEYS`를 붙이는 제약.

use crate::config::ConfigError;
use crate::credentials::Credentials;

use super::checks::{KEY_VALUE, assert_error_hides};

/// 자격 파일 해석을 왕복용으로 읽음.
pub(crate) fn parse_credentials(text: &str) -> Credentials {
    Credentials::parse(text).expect("02-1 해석이 통과해야 합니다")
}

/// 해석이 읽은 키 값을 문자열로 꺼냄.
pub(crate) fn key_value(credentials: &Credentials, endpoint: &str) -> Option<String> {
    credentials
        .find_key(endpoint)
        .map(|secret| secret.reveal().to_owned())
}

/// 자격 파일 텍스트에서 특정 키 이름 칸의 개수를 셈.
pub(crate) fn count_key_entries(text: &str, endpoint: &str) -> usize {
    let prefix = format!(r#""{endpoint}":"#);
    text.lines()
        .filter(|line| line.trim_start().starts_with(&prefix))
        .count()
}

/// 자격 쪽 오류에 키 값이 가려진지 확인. 주소는 자격 저장에서 다루지 않아 가리기 대상이 키 값뿐.
pub(crate) fn assert_error_hides_key_value(label: &str, error: &ConfigError) {
    assert_error_hides(label, error, &[KEY_VALUE]);
}

/// 자격 파일 견본: 주석 3종, 손대지 않은 키, 손대지 않은 최상위 칸 포함.
pub(crate) const KEYS_ANNOTATED_FILE: &str = r#"// 사용자 주석입니다.
{
  /* api_keys 밖 항목은 무시합니다 */
  "endpoint": [],
  "other_root_key": 1,
  "api_keys": {
    // 메인 서버
    "main": "main-key-111",
    // 다른 서버 키
    "stage_ep": "stage-key-222",
  }
}"#;
/// 이름이 겹치는 칸이 둘 있고 해석이 마지막 칸의 값으로 정하는 자격 파일 견본.
pub(crate) const KEYS_DUPLICATE_FILE: &str = r#"{
  /* api_keys 밖 항목 */
  "other_root_key": 1,
  "api_keys": {
    // 중복 설명 주석
    "main": "first-value",
    "main": "last-value",
    "stage_ep": "stage-key-222",
  }
}"#;
/// 자격 파일의 `main` 키 값.
pub(crate) const MAIN_KEY_VALUE: &str = "main-key-111";
/// 자격 파일의 다른 서버 키 값.
pub(crate) const OTHER_KEY_VALUE: &str = "stage-key-222";
/// 자격 파일의 다른 서버 키 칸 원문.
pub(crate) const OTHER_KEY_TEXT: &str = r#""stage_ep": "stage-key-222""#;
/// 자격 파일의 손대지 않은 최상위 칸 원문.
pub(crate) const KEYS_ROOT_EXTRA_TEXT: &str = r#""other_root_key": 1"#;

/// 자격 파일 쓰기가 설정 파일을 안 건드림을 확인하는 설정 쪽 텍스트.
pub(crate) const CONFIG_SIDE_TEXT: &str = r#"{
  // 설정 쪽 주석
  "endpoints": [{"name": "main", "base_url": "http://local.invalid/v1", "kind": "ollama"}]
}"#;
