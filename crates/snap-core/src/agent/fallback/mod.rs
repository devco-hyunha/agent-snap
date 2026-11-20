//! 2안 폴백 지원 파일을 모은 폴더
//!
//! 폴백 연결 지점 표기가 중심 책임이고, 전환 판단 값은 `switch`가 담당

pub mod args;
pub mod connector;
pub mod switch;
pub mod tool_call_form;
pub mod unsupported;

pub use connector::FallbackConnector;
pub use switch::{
    FallbackPoint, TEXT_STREAK_SWITCH_COUNT, first_turn_fallback_point, is_text_switch_streak,
};
