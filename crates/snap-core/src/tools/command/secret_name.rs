//! 명령 문자열 안에서 비밀 파일 이름을 잡아내는 보조 검사
//!
//! 경계 검사는 인자 경유 경로만 막고, 셸이 직접 여는 경로는 막지 못하므로 이름만 보는 보조 검사

use std::path::Path;

use crate::tools::path_guard::is_secret_name;

/// 명령 문자열 안의 비밀 파일 이름 조각 확인 보조 검사
///
/// 셸이 명령을 나누는 문자(`;`·`|`·탭·줄바꿈)까지 잘라 각 조각의 마지막 이름만 비교
#[must_use]
pub fn command_has_secret_name(command: &str) -> bool {
    command
        .split([' ', '"', '\\', '/', ';', '|', '\t', '\n'])
        .any(|piece| is_secret_name(Path::new(piece)))
}
