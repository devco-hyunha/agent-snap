//! T3 검증 시험 공통 재료. 설정 텍스트를 읽고 오류 문구만 꺼냄.
use std::path::Path;

use snap_core::config::Config;
use snap_core::config::ConfigError;
use snap_core::config::config_from_str;

/// 시험용 설정 파일 경로 표시.
pub const PATH: &str = "dir/snap.jsonc";

/// 시험용 엔드포인트 주소. 오류 문구에 안 나오는지를 확인하는 재료.
pub const HOST: &str = "secret-host.invalid";

/// 설정 텍스트 읽기.
pub fn parse(text: &str) -> Result<Config, ConfigError> {
    config_from_str(text, Path::new(PATH))
}

/// 거부된 경우에만 오류 문구 반환. 성공하면 시험을 세움.
pub fn reject_message(text: &str) -> String {
    match parse(text) {
        Ok(_) => panic!("test expects rejection, got ok: {text}"),
        Err(err) => err.to_string(),
    }
}

/// 엔드포인트 목록 블록으로 만드는 설정 텍스트.
pub fn endpoints(block: &str) -> String {
    format!(r#"{{ "endpoints": {block} }}"#)
}
