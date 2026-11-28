//! 줄 재결합 자리. 바이트 조각을 완성 줄로 바꾸고, 잘린 꼬리는 다음 조각에 넘긴다.
mod events;
mod reassemble;
mod split;

pub use events::LineState;
pub use reassemble::Lines;
