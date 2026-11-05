//! 재시도 판단 내부 항목을 보는 단위 시험 묶음입니다.
//!
//! 판단 규칙 자체(`after`·`backoff`·`classify`)를 제품 내부 경로로 직접 시험합니다.

mod after;
mod backoff;
mod classify;
