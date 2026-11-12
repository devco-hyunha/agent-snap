//! 선택 키로 정하는 키와 선택 오류를 확인하는 시험 본문입니다(02-7 AC 1·AC 2 경로).
//!
//! 자격 파일 `api_keys`의 엔드포인트 이름 찾기와, 오류 문자열에 주소·키 값이 드러나지 않는
//! 가리기를 봅니다. 설정 골격은 `support.rs`를 재사용합니다(dev-principles 3절 DRY).

use super::super::SelectError;
use super::support::{KEY_MARK, config_with_two_endpoints};
use crate::credentials::Credentials;

/// AC 1: 키는 02-2의 엔드포인트 이름 찾기(자격 파일 `api_keys`)로 정하고, 없는 엔드포인트는 없음이다.
#[test]
fn select_model_finds_key_by_endpoint_name() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::parse(&format!(r#"{{"api_keys":{{"pc1":"{KEY_MARK}"}}}}"#))
        .expect("자격 파일");

    let first = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");
    let second = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(first.api_key.expect("pc1 키").reveal(), KEY_MARK);
    assert!(second.api_key.is_none());
}

/// AC 2 경로: 없는 선택 키를 고를 때 선택 오류가 그대로 올라오고, 오류 문자열에 주소가 드러나지 않는다.
#[test]
fn select_model_selection_error_hides_hosts() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let error = match config.select_model("pc9/fast", &credentials) {
        Ok(_) => panic!("설정에 없는 키는 거부해야 합니다"),
        Err(error) => error,
    };

    assert!(matches!(error, SelectError::Selection(_)));
    let text = format!("{error} {error:?}");
    assert!(text.contains("pc9/fast"), "입력 키가 없습니다: {text}");
    assert!(
        !text.contains("pc1.example.invalid") && !text.contains("pc2.example.invalid"),
        "오류에 주소가 있습니다: {text}"
    );
    assert!(!text.contains(KEY_MARK), "오류에 키 값이 있습니다: {text}");
}
