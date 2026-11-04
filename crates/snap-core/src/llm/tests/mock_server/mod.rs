//! 전송 통합 테스트용 모의 HTTP 서버입니다.
//!
//! 헤더를 늦추거나, 본문 중간에 멈추거나, 연결이 닫힌 때를
//! 테스트가 조종하고 확인할 수 있습니다. 받은 요청도 기록합니다.
//! 제품 공개 API에는 내보내지 않습니다.
//!
//! 스크립트 규격(`script`), 서버 수명·기록(`server`), 본문 스트림 구현(`handler`)으로
//! 관심사별로 나눕니다.

// 자식 파일의 항목을 한 칸 위로 올리기 위해 각 파일의 가시 범위를 tests로 정했습니다.
// 경로는 세 파일이 정본이고, 한 자리에서 쓰는 쪽을 위해 여기서 다시 내보냅니다.
pub(in crate::llm::tests) mod handler;
pub(in crate::llm::tests) mod script;
pub(in crate::llm::tests) mod server;

pub(in crate::llm::tests) use script::{MockBodyPart, MockResponseScript};
pub(in crate::llm::tests) use server::MockServer;

#[cfg(test)]
mod self_tests;
