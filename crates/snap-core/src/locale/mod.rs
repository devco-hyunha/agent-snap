//! 로케일 모듈 자리. 언어별 문구를 키 목록으로 묶는다.
pub mod en;
pub mod ko;

// 언어 전환 지점. 이 한 줄만 바꾸면 문구가 바뀐다.
pub use ko::config;
