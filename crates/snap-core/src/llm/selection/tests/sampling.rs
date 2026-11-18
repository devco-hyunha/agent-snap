//! 샘플링 우선순위 시험 본문.
//!
//! 모델별 값이 루트보다 앞서고, 없는 항목만 루트를 따르는지 확인. 골격은 `support.rs` 재사용.

use super::support::config_with_two_endpoints;
use crate::credentials::Credentials;

/// 샘플링은 모델별 값이 루트보다 앞서고, 없는 항목만 루트를 따름.
#[test]
fn select_model_merges_sampling_with_model_priority() {
    let config = config_with_two_endpoints();
    let credentials = Credentials::absent();

    let selected = config
        .select_model("pc1/fast", &credentials)
        .expect("pc1/fast");

    assert_eq!(selected.sampling.temperature, Some(0.7));
    assert_eq!(selected.sampling.top_p, Some(0.9));
    assert_eq!(selected.sampling.max_tokens, Some(64));
}
