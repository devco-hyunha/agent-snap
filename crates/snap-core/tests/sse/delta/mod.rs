//! 02-3 델타 복원 시험 묶음. 책임별 파일(순서·생각·도구 호출·모르는 필드·사용량·되풀이)만 봄.

mod finish;
mod order;
mod reasoning;
mod tool_calls;
mod unknown;
mod usage;

/// delta 객체를 payload JSON 문자열로 만든다. `choices[0].delta` 골격을 함께 쓴다.
pub(super) fn delta_payload(delta: serde_json::Value) -> String {
    serde_json::json!({ "choices": [{ "delta": delta }] }).to_string()
}

/// 최상위 필드를 골격에 얹은 payload JSON 문자열.
pub(super) fn payload_with(extra: serde_json::Value) -> String {
    let mut object = serde_json::json!({ "choices": [{ "delta": {} }] });
    let Some(pairs) = extra.as_object().cloned() else {
        return object.to_string();
    };
    let Some(target) = object.as_object_mut() else {
        return object.to_string();
    };
    for (key, value) in pairs {
        target.insert(key, value);
    }
    object.to_string()
}
