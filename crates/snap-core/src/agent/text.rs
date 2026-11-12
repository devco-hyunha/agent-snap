//! 2안 텍스트 프로토콜 모듈: 한 도메인의 파일들을 한 폴더로 묶고 공개면을 한곳에서 내보냅니다.
//!
//! 지시 조립(`protocol`)·왕복 한 바퀴(`turn`)·복원 래더(`recover`)·스키마 검증(`validate`)·
//! 안내 글자(`notice`)·기록 채움(`record`)·messages 줄(`messages`)·스키마 글자(`schema_text`)가
//! 각 파일의 책임입니다(dev-principles 2절 SRP·5절 관심사 분리).

pub mod messages;
pub mod notice;
pub mod protocol;
pub mod record;
pub mod recover;
pub mod schema_text;
pub mod turn;
pub mod validate;

pub use messages::{TEXT_RESULT_PREFIX, text_assistant_message, text_result_message};
pub use notice::{missing_json_notice, multi_call_notice, problem_notice};
pub use protocol::{TEXT_PROTOCOL_HEAD, TEXT_PROTOCOL_RULES, text_protocol_block};
pub use record::{notice_record, run_text_call, text_call_id};
pub use recover::{RecoverOutcome, RecoveredCall, recover_text_call};
pub use turn::TextTurn;
pub use validate::{CallProblem, validate_found};
