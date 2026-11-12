//! 설정 테스트 공용 도구 창구: 책임별 파일로 나눈 정의들을 한 자리에서 다시 내보냅니다.
//!
//! 시험 파일들은 `support::` 이름 하나로 기존처럼 가져다 씁니다. 주소와 키는 값이 새는지
//! 확인하기 위한 가짜 표지만 씁니다.

mod checks;
mod config_text;
mod credentials;
mod rejection_cases;
mod save_failures;
mod save_path;
mod save_samples;
mod temp_dir;
mod variant_samples;

pub(crate) use self::checks::{HOST, KEY_VALUE, assert_error_hides_secrets};
pub(crate) use self::config_text::{
    base_url, config_of, config_with_endpoint_tail, config_with_model_tail, config_with_models,
    config_with_root_tail, config_without_kind, config_without_model_id, empty_env,
    endpoint_object, endpoint_object_with_kind, model_object, models_tail, parse_err, parse_ok,
};
pub(crate) use self::credentials::{
    CONFIG_SIDE_TEXT, KEYS_ANNOTATED_FILE, KEYS_DUPLICATE_FILE, KEYS_ROOT_EXTRA_TEXT,
    MAIN_KEY_VALUE, OTHER_KEY_TEXT, OTHER_KEY_VALUE, assert_error_hides_key_value,
    count_key_entries, key_value, parse_credentials,
};
pub(crate) use self::rejection_cases::{
    LEGACY_MODELS_LABEL, MODEL_ENDPOINT_FIELD_LABEL, rejection_cases,
};
pub(crate) use self::save_failures::{
    ANNOTATED_CREDENTIALS, HOST_CONFIG, bad_base_url_save, config_replace_failure,
    config_write_failure, every_save_path_error, key_replace_failure, replace_file_ok,
};
pub(crate) use self::save_path::{blocking_replacement, endpoint_save, side_effect_errors};
pub(crate) use self::save_samples::{
    ANNOTATED_FILE, BROKEN_TEXT, CONFIG_TEXT, CREDENTIALS_TEXT, EMPTY_ENDPOINTS_FILE,
    EXISTING_MAIN_FILE, HTTP_TEXT, LOCAL_ENDPOINT_TEXT, ROOT_EXTRA_TEXT, TWO_ENDPOINTS_TEXT,
    full_extra, full_save, model_save, model_with_extra_key, reasoning_only_save, save_named,
    validate_reserved_keys,
};
pub(crate) use self::temp_dir::TempDir;
pub(crate) use self::variant_samples::{
    VARIANT_COUNT, errors_outside_rejection_table, variant_name,
};
