//! 선택 키로 준비한 값과 조립된 클라이언트를 확인하는 시험 본문입니다(02-7 AC 1).
//!
//! 준비 값의 엔드포인트 주소·서버용 id와 클라이언트 값이 선택 키의 값 그대로인지 봅니다.
//! 설정 골격은 `support.rs`를 재사용합니다(dev-principles 3절 DRY).

use super::support::config_with_two_endpoints;
use crate::credentials::Credentials;

/// AC 1: 조립된 클라이언트는 준비 값의 엔드포인트 주소와 모델 값을 그대로 갖는다.
#[test]
fn client_for_carries_selected_values() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let client = config
        .client_for("pc2/fast", &credentials)
        .expect("pc2/fast");
    let selected = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(client.model_id, selected.model_id);
    assert_eq!(client.sampling, selected.sampling);
    assert_eq!(client.reasoning, selected.reasoning);
    assert_eq!(client.extra, selected.extra);
}

/// AC 1: 선택 키가 다른 같은 별칭은 서로 다른 엔드포인트와 서버용 id로 준비된다.
#[test]
fn select_model_splits_same_alias_by_selection_key() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let first = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");
    let second = config
        .select_model("pc2/fast", &credentials)
        .expect("pc2/fast");

    assert_eq!(first.model_id, "id-first");
    assert_eq!(second.model_id, "id-second");
    assert_eq!(first.base_url.reveal(), "https://pc1.example.invalid/v1");
    assert_eq!(second.base_url.reveal(), "https://pc2.example.invalid/v1");
}
