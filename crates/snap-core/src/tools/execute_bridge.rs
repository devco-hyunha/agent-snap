//! 03-8 execute 브리지 지원 파일을 모은 폴더입니다.
//!
//! 결론: 각 파일은 하나씩 책임을 맡습니다. 정책 값·엔진 구동·워치독·JS 얅개 조립·JS 값
//! 판정·승인 대기·기록 창구·글자 보조·호출 창구이며, 프로젝트 규칙(같은 관심사 파일 둘
//! 이상일 때만 폴더)에 따라 같은 관심사 파일만 이 폴더에 묶었습니다.
//! `execute` 도구 파일(`tools/execute.rs`)은 등록 골격만 두고, 나머지는 이 폴더가 맡습니다.

pub(in crate::tools) mod approval_gate;
pub(in crate::tools) mod engine_drive;
pub(in crate::tools) mod engine_types;
pub(in crate::tools) mod js_bindings;
pub(in crate::tools) mod js_values;
pub(in crate::tools) mod policy;
pub(in crate::tools) mod script_log;
pub(in crate::tools) mod text_parts;
pub(in crate::tools) mod tool_refs;
pub(in crate::tools) mod watchdog;
