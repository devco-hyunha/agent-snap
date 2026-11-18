//! 모델별 값과 루트 기본값을 잇는 샘플링 병합 규칙.

use crate::config::{ModelConfig, Sampling};

/// 모델별 값이 루트 기본보다 앞서는 샘플링으로 이음.
///
/// 모델에 없는 항목만 루트를 따르고, 루트에도 없으면 본문에서 빼 서버 기본을 따름.
pub(in crate::llm) fn merged_sampling(model: &ModelConfig, root: &Sampling) -> Sampling {
    Sampling {
        temperature: model.sampling.temperature.or(root.temperature),
        top_p: model.sampling.top_p.or(root.top_p),
        max_tokens: model.sampling.max_tokens.or(root.max_tokens),
    }
}
