//! 저장 시험 도움: 교체 지점, 어긋난 상태 견본, 저장 정보 조립.
//!
//! 검증·저장 경로가 파일을 옮기는 방식과 교체 지점을 모아 두고, 여러 시험 파일이 같은 조립을
//! 다시 적지 않게 합니다.

use std::path::Path;

use crate::config::{ConfigError, EndpointKind, EndpointSave, SaveError};

/// 검증·저장 경로가 표준대로 파일을 옮기는 교체 지점입니다.
pub(crate) fn standard_replacement(from: &Path, target: &Path) -> std::io::Result<()> {
    std::fs::rename(from, target)
}

/// 두 파일을 함께 고치는 저장이 밝히는 어긋난 상태 변형들을 견본으로 만듭니다(D40 규칙 3).
/// 가리기 검사 여러 파일에서 같은 견본을 다시 적지 않게 하려는 자리입니다.
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

/// `target`의 파일 이름이 `blocked`와 같으면 교체를 막고, 나머지는 표준대로 옮깁니다.
/// 교체가 막히는 시험을 보는 테스트가 여러 파일에서 같은 지점을 다시 적지 않게 하려는 자리입니다.
pub(crate) fn blocking_replacement(
    blocked: &'static str,
) -> impl Fn(&Path, &Path) -> std::io::Result<()> {
    move |from: &Path, target: &Path| {
        // `file_name()`는 `OsStr`을 돌려주므로 같은 종류로 비교합니다.
        let blocked_name = Path::new(blocked).as_os_str();
        if target.file_name() == Some(blocked_name) {
            return Err(std::io::Error::other("교체 시험"));
        }
        std::fs::rename(from, target)
    }
}

/// 키 값 없이 저장하는 엔드포인트 저장 정보입니다(모델 없음). 설정 파일 저장 쪽 견본에서
/// 여러 파일이 같은 조립을 다시 적지 않게 하려는 자리입니다.
pub(crate) fn endpoint_save(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::OpenAi,
        models: Vec::new(),
    }
}
