//! 이름 정규화 자리. 저장·자격 매칭에 쓸 값을 trim한 값으로 바꿈.

use serde_json::Value;

/// 엔드포인트·모델 이름을 trim한 값으로 바꿈. 저장·자격 매칭에 쓰는 값.
pub fn trim_names(root: &mut Value) {
    let Some(endpoints) = root.get_mut("endpoints").and_then(Value::as_array_mut) else {
        return;
    };
    for endpoint in endpoints {
        trim_field(endpoint, "name");
        let Some(models) = endpoint.get_mut("models").and_then(Value::as_array_mut) else {
            continue;
        };
        for model in models {
            trim_field(model, "name");
            trim_field(model, "id");
        }
    }
}

/// 객체의 문자열 칸을 trim한 값으로 바꿈.
fn trim_field(target: &mut Value, key: &str) {
    let Some(raw) = target.get_mut(key) else {
        return;
    };
    let Some(trimmed) = raw.as_str().map(|value| value.trim().to_owned()) else {
        return;
    };
    *raw = Value::from(trimmed);
}
