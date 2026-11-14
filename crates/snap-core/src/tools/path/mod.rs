//! 경로 검사 지원 파일 4종을 모은 폴더입니다(T6 F1 폴더 정리).
//!
//! 각 파일의 가시성과 이름은 그대로 두고 선언만 모읍니다. 공개 창구는 `tools.rs`가
//! 기존 이름 그대로 다시 내보냅니다.

pub(crate) mod link_guard;
pub mod path_guard;
pub(crate) mod path_norm;
pub(crate) mod secret_names;
