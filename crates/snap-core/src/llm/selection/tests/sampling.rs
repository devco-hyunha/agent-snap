//! 샘플링 우선순위 시험 본문입니다(02-7 AC 1).
//!
//! 모델별 값이 루트 기본값보다 앞서고, 모델에 없는 항목만 루트를 따라 가는지 봅니다.
//! 설정 골격은 `support.rs`를 재사용합니다(dev-principles 3절 DRY).

use super::support::config_with_two_endpoints;
use crate::credentials::Credentials;

/// AC 1: 샘플링은 모델별 값이 루트 기본값보다 앞서고, 모델에 없는 항목만 루트를 따라 간다.
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
