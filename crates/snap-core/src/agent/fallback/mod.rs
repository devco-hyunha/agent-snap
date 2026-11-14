//! 2안 폴백 지원 파일 4종을 모은 폴더입니다(T6 F2 폴더 정리).
//!
//! 선언만 모였고, 파일 이름과 가시성은 그대로 두었습니다. 폴백 지점 타입·표기·연속 판별
//! 파일은 폴더와 같은 이름이 되면 clippy의 module-inception에 걸리므로 `connector.rs`로
//! 이름을 정정했습니다(폴백 연결 지점 표기가 이 파일의 중심 책임입니다).

pub mod args;
pub mod connector;
pub mod tool_call_form;
pub mod unsupported;
