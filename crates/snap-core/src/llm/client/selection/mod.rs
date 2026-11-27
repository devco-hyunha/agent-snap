//! 선택 키 자리. 타입 정의·형식 해석·설정 조회·오류를 책임별 파일로 나눔.

mod error;
mod format;
mod key;
mod lookup;
mod target;

pub use error::SelectionError;
pub use error::SelectionFormatProblem;
pub use format::parse_selection_key;
pub use key::SelectionKey;
pub use lookup::find_target;
pub use target::SelectionTarget;
