//! SSE 파이너·스트림 복원 통합 시험 타깃의 루트.
//!
//! 지원(`common`)은 여기서 선언하고, 시험은 아래 파일에 분리.

pub mod common;

mod sse {
    mod chunks;
    mod lines;
    mod stream_decoder_contract;
}
