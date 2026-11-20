//! 백그라운드 도메인: 목록·상태·읽기 확인·표시·공유 창구
//!
//! wake 한 번은 에이전트 턴 하나와 대응하므로 목록은 앞 항목부터 한 개씩 확인

pub mod registry;
pub mod shared;
pub mod state;

pub(crate) mod pipe_read;
mod text;

pub use registry::BackgroundRegistry;
pub use shared::{SharedBackgroundRegistry, lock_background, shared_background_registry};
pub use state::BackgroundState;
