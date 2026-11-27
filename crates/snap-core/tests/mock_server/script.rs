//! 응답 스크립트 골격 선언 자리. 규격 타입 파일과 생성 창구 파일을 함께 엶.
//!
//! `tests/mock_server/main.rs`의 `mod script;`가 이 파일을 열고, 아래 파일을 함께 컴파일한다.
//! `tests/client/main.rs`가 같은 파일을 경로나열로 쓰므로 자식 경로를 명시한다.

#[path = "script/build.rs"]
mod build;
#[path = "script/spec.rs"]
mod spec;

#[allow(unused_imports)]
pub use build::ABORT_GAP;
#[allow(unused_imports)]
pub use spec::HeaderMode;
pub use spec::MockScript;
pub use spec::StreamStep;
pub use spec::StreamStop;
