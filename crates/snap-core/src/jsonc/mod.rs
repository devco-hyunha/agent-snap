//! JSONC 창구 모듈 자리. 해석·편집·저장·저장 오류를 책임별 파일로 나눔.
mod edit;
mod error;
mod parse;
mod save;

pub use edit::edit_jsonc;
pub use error::JsoncError;
pub use parse::parse_jsonc;
pub use save::error::SaveError;
pub use save::write_text_atomic;
// 편집 창구가 받는 값 타입. 소비자가 같은 이름으로 쓰게 재수출.
pub use jsonc_parser::cst::CstInputValue;
