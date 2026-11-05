//! SSE 파이너·스트림 복원 통합 테스트 타깃의 루트입니다.
//!
//! 함께 쓰는 지원(`common`)은 여기에서 선언하고, 실제 시험은 아래 파일에 분리해 둡니다.

pub mod common;

mod sse {
    mod chunks;
    mod lines;
    mod stream_decoder_contract;
}
