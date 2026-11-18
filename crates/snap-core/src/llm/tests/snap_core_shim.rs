//! 라이브러리 단위 시험이 바깥 시험과 같은 이름(`snap_core`)으로 같은 크레이트를 보게 하는 자리재내기.
//!
//! 같은 타입을 한 이름으로 통일해 지원·시험 코드를 한 경로로 묶음.

#[allow(unused_imports)]
pub use crate::{config, llm, secrets};
