//! 저장 테스트 무: 제품 `save/`와 같은 관심사로 묶습니다.
//!
//! Windows 실측 시험 파일은 Windows 전용 APIs를 쓰므로 Windows가 움직이는 자리에서만
//! 컴파일하게 게시를 함께 둡니다.

mod atomic;
mod config;
mod credentials;
mod credentials_rename;
mod redaction;
mod rename;
#[cfg(windows)]
mod windows_replace;
