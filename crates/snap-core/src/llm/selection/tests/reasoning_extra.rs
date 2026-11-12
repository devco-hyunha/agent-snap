//! `reasoning`과 `extra`가 그대로 담리는지 보는 시험 본문입니다(02-7 AC 1).
//!
//! 설정 골격은 `support.rs`를 재사용합니다(dev-principles 3절 DRY).

use super::support::config_with_two_endpoints;
use crate::config::ExtraValue;
use crate::config::Reasoning;
use crate::credentials::Credentials;

/// AC 1: `reasoning`과 `extra`는 설정 값 그대로 준비 값에 담긴다.
#[test]
fn select_model_keeps_reasoning_and_extra() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let selected = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");

    assert_eq!(selected.reasoning, Some(Reasoning::Low));
    assert_eq!(
        selected.extra.get("keep_alive"),
        Some(&ExtraValue::Number("0".into()))
    );
    assert!(
        config
            .select_model("pc2/fast", &credentials)
            .expect("pc2/fast")
            .reasoning
            .is_none()
    );
}
