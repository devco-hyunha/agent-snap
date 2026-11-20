//! 백그라운드 전환 시험 트리를 책임별 파일로 나눈 등록 자리
//!
//! wake 주기는 짧게 주고, 확인은 넉넉한 하한·상한으로만 봄

#[path = "check.rs"]
mod check;
#[path = "drop.rs"]
#[allow(dead_code)]
mod drop;
#[path = "register.rs"]
mod register;
#[path = "roundtrip.rs"]
mod roundtrip;
#[path = "settings.rs"]
mod settings;
#[path = "shared.rs"]
mod shared;
#[path = "wake.rs"]
mod wake;
#[path = "wake_helpers.rs"]
mod wake_helpers;
