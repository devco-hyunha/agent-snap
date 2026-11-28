//! 자격 파일 텍스트 읽기 자리. 없는 파일은 새 자격 파일의 밑바탕으로 읽는다.

use std::fs::read_to_string;
use std::path::Path;

use crate::config::save::error::ConfigSaveError;
use crate::consts::defaults::API_KEYS_KEY;

/// 자격 파일이 없을 때 쓰는 밑바탕 텍스트. 최상위 `api_keys` 객체를 가진다.
/// 칸 이름은 정본 상수 `API_KEYS_KEY`로만 조립한다.
fn new_credentials_text() -> String {
    format!("{{\n  \"{API_KEYS_KEY}\": {{}}\n}}")
}

/// 자격 파일 텍스트 읽음. 파일이 없으면 새 자격 파일 밑바탕을 돌려 새 파일로 만들게 한다.
pub(super) fn read_credentials_text(path: &Path) -> Result<String, ConfigSaveError> {
    match read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(new_credentials_text()),
        Err(_) => Err(ConfigSaveError::Read {
            path: path.display().to_string(),
        }),
    }
}
