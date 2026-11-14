//! 저장 경로 실패·대상 없음 견본 조립: 저장이 내는 여러 실패를 한 자리에서 만들어
//! 저장 가리기 시험(`save/redaction.rs`)과 설정 오류 가리기 시험이 함께 쓰게 합니다.
//!
//! 본문은 `save/redaction.rs`에서 그대로 옮겼습니다. 교체 지점 조립은 `save_path.rs`를
//! 재사용합니다(중복 금지·3절 DRY).

use std::path::Path;

use super::checks::KEY_VALUE;
use super::config_text::base_url;
use super::save_path::{blocking_replacement, standard_replacement};
use super::save_samples::save_named;
use crate::config::save::config::save_endpoint_with;
use crate::config::save::credentials::save_key_with;
use crate::config::save::store::{remove_endpoint_with, rename_endpoint_with};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointSave, KeyRename,
};
use crate::secrets::SecretValue;

/// 저장이 내는 오류를 한 목록으로 만듭니다(설정 검증 거부, 자격 이름 규칙 거부, 교체 실패,
/// 쓰기 실패, 대상 없음 포함). 설정 가리기 시험과 저장 가리기 시험이 같은 견본을 씁니다.
pub(crate) fn every_save_path_error(folder: &Path) -> Vec<(&'static str, Result<(), ConfigError>)> {
    vec![
        (
            "설정 검증 거부",
            save_endpoint_with(folder, &bad_base_url_save(), &replace_file_ok()),
        ),
        (
            "자격 이름 규칙 거부",
            save_key_with(
                folder,
                "a/b",
                &SecretValue::new(KEY_VALUE),
                &replace_file_ok(),
            ),
        ),
        ("설정 교체 실패", config_replace_failure(folder)),
        ("자격 교체 실패", key_replace_failure(folder)),
        (
            "두 파일 이름 바꾸기 교체 실패",
            rename_endpoint_with(
                folder,
                &KeyRename {
                    from: "main".to_owned(),
                    to: "main2".to_owned(),
                },
                &blocking_replacement(CONFIG_FILE_NAME),
            ),
        ),
        (
            "두 파일 삭제 교체 실패",
            remove_endpoint_with(folder, "main", &blocking_replacement(CONFIG_FILE_NAME)),
        ),
        ("설정 쓰기 실패", config_write_failure(folder)),
        (
            "이름 바꾸기 대상 없음",
            rename_endpoint_with(
                folder,
                &KeyRename {
                    from: "없는-서버".to_owned(),
                    to: "새-서버".to_owned(),
                },
                &replace_file_ok(),
            ),
        ),
        (
            "삭제 대상 없음",
            remove_endpoint_with(folder, "없는-서버", &replace_file_ok()),
        ),
    ]
}

/// 규칙에 맞지 않는 주소의 설정 저장 입력입니다. 검증 거부 견본입니다.
pub(crate) fn bad_base_url_save() -> EndpointSave {
    save_named("main", BAD_URL)
}

/// 설정 파일 교체를 막아 교체 실패를 만듭니다.
pub(crate) fn config_replace_failure(folder: &Path) -> Result<(), ConfigError> {
    save_endpoint_with(
        folder,
        &main_save(),
        &blocking_replacement(CONFIG_FILE_NAME),
    )
}

/// 자격 파일 교체를 막아 교체 실패를 만듭니다.
pub(crate) fn key_replace_failure(folder: &Path) -> Result<(), ConfigError> {
    save_key_with(
        folder,
        "main",
        &SecretValue::new(KEY_VALUE),
        &blocking_replacement(CREDENTIALS_FILE_NAME),
    )
}

/// 설정 파일 저장을 없는 하위 폴더에 시도해 임시 파일 쓰기 실패을 만듭니다.
pub(crate) fn config_write_failure(folder: &Path) -> Result<(), ConfigError> {
    save_endpoint_with(
        &folder.join("없는-하위-폴더"),
        &main_save(),
        &replace_file_ok(),
    )
}

/// 검증·대상 없음 경로를 거치게 하는 교체 지점입니다.
pub(crate) fn replace_file_ok() -> impl Fn(&Path, &Path) -> std::io::Result<()> {
    standard_replacement
}

/// 검증·저장 견본으로 쓰는 기본 엔드포인트 저장입니다.
pub(crate) fn main_save() -> EndpointSave {
    save_named("main", &base_url())
}

/// 검증 거부 입력에 쓰는 주소입니다. 가리기 대상 호스트가 들어 있어 Display에 나오면 안 됩니다.
const BAD_URL: &str = "ftp://leak-check-host.invalid/v1";

/// 사용자 주석과 다른 키가 있는 자격 파일 텍스트 견본입니다. 저장 가리기 시험들이 씁니다.
pub(crate) const ANNOTATED_CREDENTIALS: &str = r#"{
  // 사용자 주석입니다.
  "api_keys": { "main": "main-key-111", "stage_ep": "stage-key-222" }
}"#;

/// 가리기 대상 호스트가 든 엔드포인트가 있는 설정 파일 텍스트 견본입니다. 저장 가리기 시험들이 씁니다.
pub(crate) const HOST_CONFIG: &str = r#"{
  // 사용자 주석입니다.
  "endpoints": [{"name": "main", "base_url": "https://leak-check-host.invalid/v1", "kind": "ollama"}]
}"#;
