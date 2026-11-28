//! T5 복원 연결 시험 재료 정본. SSE payload 골격·조각 원문·스트림 원문 조립·스크립트 시간 상수 정본만 담음.
//! 같은 시험 묶음의 복원 단위·계약·전송 통합·시간 초과·취소 시험 재사용 재료.

use std::time::Duration;

use serde_json::Value;
use serde_json::json;

/// 본문이 멈춘 상태로 남는 시간 정본. 시간 초과·취소 시험이 함께 씀.
pub(crate) const STOPPED_BODY: Duration = Duration::from_millis(2_000);

/// 첫 content 조각 원문 정본. 시간 초과·취소 시험이 함께 씀.
pub(crate) const FIRST_CHUNK: &str = "data: {\"choices\":[{\"delta\":{\"content\":\"가\"}}]}\n\n";

/// 두 번째 content 조각 원문 정본. 시간 초과·취소 시험이 함께 씀.
pub(crate) const SECOND_CHUNK: &str = "data: {\"choices\":[{\"delta\":{\"content\":\"나\"}}]}\n\n";

/// delta 객체를 payload JSON 문자열로 조립. `choices[0].delta` 골격.
pub(crate) fn delta_payload(delta: Value) -> String {
    json!({ "choices": [{ "delta": delta }] }).to_string()
}

/// 최상위 필드를 골격에 얹은 payload JSON 문자열.
pub(crate) fn payload_with(extra: Value) -> String {
    let Some(pairs) = extra.as_object().cloned() else {
        return json!({ "choices": [{ "delta": {} }] }).to_string();
    };
    let mut object = json!({ "choices": [{ "delta": {} }] });
    let Some(target) = object.as_object_mut() else {
        return object.to_string();
    };
    for (key, value) in pairs {
        target.insert(key, value);
    }
    object.to_string()
}

/// 서버 오류 청크 payload. `error.message` 형식.
pub(crate) fn error_payload(message: &str) -> String {
    json!({ "error": { "message": message } }).to_string()
}

/// data 이벤트 하나를 나타내는 SSE 줄 묶음.
pub(crate) fn data_line(payload: &str) -> String {
    format!("data: {payload}\n\n")
}

/// 02-3 AC 1과 같은 형식의 스트림 원문. 주석·content·생각·도구 호출·사용량을 섞음.
pub(crate) fn mixed_stream() -> String {
    let content = delta_payload(json!({ "content": "가나다" }));
    let reasoning = delta_payload(json!({ "reasoning": "생각" }));
    let tool_call = delta_payload(json!({
        "tool_calls": [{ "index": 0, "id": "call-1", "function": { "name": "run", "arguments": "{}" } }]
    }));
    let accounting = payload_with(json!({
        "finish_reason": "stop",
        "usage": { "prompt_tokens": 3, "completion_tokens": 2, "total_tokens": 5 }
    }));
    format!(
        ": keep-alive\n\n{}{}{}{}data: [DONE]\n\n",
        data_line(&content),
        data_line(&reasoning),
        data_line(&tool_call),
        data_line(&accounting),
    )
}
