//! 로케일 모듈 자리. 언어별 문구를 키 목록으로 묶음.
pub mod en;
pub mod ko;

// 언어 전환 지점. 이 한 줄만 바꾸면 문구가 바뀜.
pub use ko::config;
pub use ko::jsonc;
pub use ko::models;
pub use ko::selection;
pub use ko::sse;
pub use ko::transport;
