//! 요청 주소 조립 자리. 주소 만드는 한 책임만 담음.
//!
//! `/v1` 보정은 `ensure_v1` 한곳 정본만 쓰고, completions·모델 목록 경로가 같은 규칙을 따른다.

use crate::consts::request::CHAT_COMPLETIONS_PATH;
use crate::consts::request::MODELS_PATH;
use crate::consts::request::PATH_SEPARATOR;
use crate::consts::request::VERSION_PATH;

/// `base_url`에 버전 경로를 한 번만 넣은 주소. `/v1`로 끝나면 원문만 씀.
pub fn ensure_v1(base_url: &str) -> String {
    let has_version = base_url
        .strip_suffix(VERSION_PATH)
        .is_some_and(|head| head.ends_with(PATH_SEPARATOR));
    if has_version {
        return base_url.to_owned();
    }
    format!("{base_url}/{VERSION_PATH}")
}

/// `base_url`을 채팅 completions 주소로 조립함. scheme을 붙이지 않음.
pub fn completions_url(base_url: &str) -> String {
    let version = ensure_v1(base_url);
    format!("{version}/{CHAT_COMPLETIONS_PATH}")
}

/// `base_url`을 모델 목록 주소로 조립함. scheme을 붙이지 않음.
pub fn models_url(base_url: &str) -> String {
    let version = ensure_v1(base_url);
    format!("{version}/{MODELS_PATH}")
}
