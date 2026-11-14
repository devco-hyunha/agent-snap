//! 오류·Debug 가리기 테스트(명세 AC 4, AC 5) 계통별 창구: 값이 남지 않는 확인을 계통별로 나눕니다.
//!
//! 값 가리기 확인은 AC 4 계통(`key_rejection.rs`)과 AC 5 계통(`debug_values.rs`)으로 나눴고,
//! 예시 파일 검사는 가리기 확인과 다른 책임이라 `example_file.rs`로 따로 뒀습니다.

mod debug_values;
mod example_file;
mod key_rejection;
