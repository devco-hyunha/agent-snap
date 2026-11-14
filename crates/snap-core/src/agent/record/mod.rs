//! 기록·등록 파일 3종을 모은 폴더입니다(T6 F2 폴더 정리).
//!
//! 각 파일의 이름과 가시성은 그대로 두었습니다. 기록 채움 파일(`record_fill`)은
//! 에이전트 모듈 안에서 `executor`가 쓰는 자리라 `pub(super)`로 두었습니다.
//! 폴더 이름과 같은 파일은 없습니다.

pub(super) mod record_fill;
pub mod registry;
pub mod truncate;
