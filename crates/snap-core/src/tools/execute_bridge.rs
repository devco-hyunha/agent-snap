//! execute 브리지 지원 파일 폴더.
//!
//! 각 파일이 정책 값·엔진 구동·워치독·JS 얅개·JS 값 판정·승인 대기·기록·글자 보조·호출 창구 책임을 하나씩 맡음.

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
