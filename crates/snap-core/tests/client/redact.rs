//! T7 전송 오류 가리기 골격 선언 자리. 상태 오류 가리기 시험과 본문 상한 시험을 책임별 파일로 나눔.
//!
//! `tests/client/main.rs`의 `mod redact;`가 이 파일을 열고, 아래 파일을 함께 컴파일한다.

mod materials;
mod status_redaction;
mod summary_cap;
