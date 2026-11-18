//! 저장 시험 무: 제품 `save/`와 같은 관심사로 묶음. Windows 전용 APIs를 쓰는 실측 시험 파일은 Windows 전용 cfg 게이트로 컴파일.

mod atomic;
mod config;
mod credentials;
mod credentials_rename;
mod redaction;
mod rename;
#[cfg(windows)]
mod windows_replace;
