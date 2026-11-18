//! 검증·저장 경로가 파일을 옮기는 교체 지점과 어긋난 상태 견본.

use std::path::Path;

use crate::config::{ConfigError, SaveError};

/// 검증·저장 경로가 표준대로 파일을 옮기는 교체 지점.
pub(crate) fn standard_replacement(from: &Path, target: &Path) -> std::io::Result<()> {
    std::fs::rename(from, target)
}

/// 두 파일을 함께 고치는 저장이 밝히는 어긋난 상태 변형 견본.
pub(crate) fn side_effect_errors() -> Vec<(&'static str, ConfigError)> {
    vec![
        (
            "이름 바꾸기 어긋난 상태",
            ConfigError::Save(SaveError::EndpointRenameAfterKeySide { source: None }),
        ),
        (
            "삭제 어긋난 상태",
            ConfigError::Save(SaveError::EndpointRemoveAfterKeySide { source: None }),
        ),
    ]
}

/// `target`의 파일 이름이 `blocked`와 같으면 교체를 막고, 나머지는 표준대로 옮김.
pub(crate) fn blocking_replacement(
    blocked: &'static str,
) -> impl Fn(&Path, &Path) -> std::io::Result<()> {
    move |from: &Path, target: &Path| {
        // `file_name()`는 `OsStr`을 돌려주므로 같은 종류로 비교.
        let blocked_name = Path::new(blocked).as_os_str();
        if target.file_name() == Some(blocked_name) {
            return Err(std::io::Error::other("교체 시험"));
        }
        std::fs::rename(from, target)
    }
}
