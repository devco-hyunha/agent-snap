//! 시험용 도구 인자 JSON 문자열 재료 정본

#![allow(dead_code)]

use serde_json::json;

/// `command` 인자 하나를 지정한 도구 인자 JSON 문자열 만들기
#[must_use]
pub fn command_args(command: &str) -> String {
    json!({ "command": command }).to_string()
}
