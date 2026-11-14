//! 모델별 값과 루트 기본값을 잇는 샘플링 병합 규칙만 둡니다(03-6 R2 감사 분리).
//!
//! 값 계산 책임은 선택 준비가 바뀌는 이유와 다르므로 따로 둡니다(dev-principles 2절 SRP).

use crate::config::{ModelConfig, Sampling};

/// 모델별 값이 루트 기본값보다 앞서는 샘플링으로 잇습니다(D14).
///
/// 모델에 없는 항목만 루트 기본을 따라 가고, 루트에도 없으면 본문에서 빼 서버 기본을 따릅니다.
pub(in crate::llm) fn merged_sampling(model: &ModelConfig, root: &Sampling) -> Sampling {
    Sampling {
        temperature: model.sampling.temperature.or(root.temperature),
        top_p: model.sampling.top_p.or(root.top_p),
        max_tokens: model.sampling.max_tokens.or(root.max_tokens),
    }
}
