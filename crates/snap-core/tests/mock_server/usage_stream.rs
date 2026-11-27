//! 02-5 T8 모의 서버 재료. content 조각과 사용량 종료 조각 조립만 담당.
use serde_json::json;

/// content 델타 조각 하나.
fn content_chunk(text: &str) -> String {
    let payload = json!({ "choices": [{ "delta": { "content": text } }] }).to_string();
    format!("data: {payload}\n\n")
}

/// 최상위 `usage`를 얹은 종료 조각 하나.
fn usage_chunk(prompt: u64, completion: u64, total: u64) -> String {
    let payload = json!({
        "choices": [{ "delta": {}, "finish_reason": "stop" }],
        "usage": {
            "prompt_tokens": prompt,
            "completion_tokens": completion,
            "total_tokens": total,
        }
    })
    .to_string();
    format!("data: {payload}\n\n")
}

/// 끝 표시 조각.
pub(crate) fn done_chunk() -> String {
    "data: [DONE]\n\n".to_owned()
}

/// content 두 조각과 사용량 종료 조각 순서.
pub(crate) fn chunks_with_usage() -> Vec<String> {
    vec![
        content_chunk("가"),
        content_chunk("나"),
        usage_chunk(3, 2, 5),
        done_chunk(),
    ]
}

/// content 두 조각만 있고 사용량 종료 조각이 없는 순서.
pub(crate) fn chunks_without_usage() -> Vec<String> {
    vec![content_chunk("가"), content_chunk("나"), done_chunk()]
}

/// content가 한 번도 늘지 않고 사용량 종료 조각만 있는 순서.
pub(crate) fn usage_only_chunks() -> Vec<String> {
    vec![usage_chunk(4, 1, 5), done_chunk()]
}
