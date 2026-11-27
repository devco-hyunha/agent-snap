//! 요청 주소 조립 자리. 주소 만드는 한 책임만 담음.

use crate::consts::request::CHAT_COMPLETIONS_PATH;

/// 검증이 통과한 `base_url`에 채팅 completions 경로를 이어 붙임. scheme을 붙이지 않음.
pub fn completions_url(base_url: &str) -> String {
    format!("{base_url}/{CHAT_COMPLETIONS_PATH}")
}
