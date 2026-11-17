//! 03-8 execute 브리지 시험을 책임별로 나눈 자리입니다(개발 규칙: 시험 본문은 제품 소스에 두지 않는다).
//!
//! 왕복·승인·타임아웃·차단·기록·길이 책임을 파일 하나로 나누고, 같은 조립을 쓰는 재료를
//! `shared.rs`에 한 자리로 둂니다. 이 트리는 크레이트 안 항목을 보는 시험입니다.

#[path = "approval.rs"]
mod approval;
#[path = "blocked.rs"]
mod blocked;
#[path = "length.rs"]
mod length;
#[path = "record.rs"]
mod record;
#[path = "roundtrip.rs"]
mod roundtrip;
pub mod shared;
#[path = "timeout.rs"]
mod timeout;
