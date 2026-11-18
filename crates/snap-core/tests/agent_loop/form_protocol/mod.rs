//! 형식 전환 시험 파일들의 루트.
//!
//! 골격·모의 서버 본문·도구 등록 헬퍼는 `tests/agent_loop/support/` 트리를 재사용.

mod block;
mod declared;
mod notices;
mod records;
mod switch;

/// 요청 본문 JSON의 `tools` 목록. 도구가 비면 `tools` 칸을 생략하므로 빈 목록으로 봄.
pub(super) fn tools_of(request_body: &serde_json::Value) -> Vec<serde_json::Value> {
    request_body["tools"]
        .as_array()
        .cloned()
        .unwrap_or_default()
}
