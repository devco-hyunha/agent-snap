//! T6 취소 골격 선언 자리. 취소 시험 본문과 공용 재료를 책임별 파일로 나눔.
//!
//! `tests/client/main.rs`의 `mod cancel;`가 이 파일을 열고, 아래 파일을 함께 컴파일한다.

pub mod materials;
mod stream;
