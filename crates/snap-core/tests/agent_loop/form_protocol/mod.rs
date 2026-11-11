//! 03-4 형식 전환 시험을 AC 주제별로 나눈 파일들의 루트입니다.
//!
//! 골격·모의 서버 본문·도구 등록 헬퍼는 `tests/agent_loop/support/` 트리를 그대로 재사용합니다.

mod block;
mod declared;
mod notices;
mod records;
mod switch;

/// 요청 본문 JSON의 `tools` 목록입니다. 본문은 도구가 비면 `tools` 칸을 생략하므로 빈 목록으로 봅니다(AC 확인용).
pub(super) fn tools_of(request_body: &serde_json::Value) -> Vec<serde_json::Value> {
    request_body["tools"]
        .as_array()
        .cloned()
        .unwrap_or_default()
}
