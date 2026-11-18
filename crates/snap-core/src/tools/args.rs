//! 파일 도구가 받는 인자 JSON 해석 자리

use serde_json::Value;

/// 도구 인자 JSON 해석 자리. 루프가 복구한 JSON을 받음
///
/// `read_file`·`write_file`·`edit_file`이 같은 문장을 쓰므로 도구마다 해석기를 다시 두지 않음
pub fn parse_tool_args(args_json: &str) -> Result<Value, &'static str> {
    serde_json::from_str(args_json).map_err(|_| "인자 JSON 형식 오류")
}

/// 필수 문자열 인자 읽기
pub fn str_arg<'a>(args: &'a Value, key: &'a str) -> Result<&'a str, String> {
    match args.get(key) {
        Some(Value::String(text)) => Ok(text),
        _ => Err(format!("{key}는 문자열이어야 합니다")),
    }
}

/// 선택 문자열 인자 읽기. 없거나 null이면 `None`, 문자열이 아니면 오류
pub fn opt_str<'a>(args: &'a Value, key: &'a str) -> Result<Option<&'a str>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text)),
        Some(_) => Err(format!("{key}는 문자열이어야 합니다")),
    }
}

/// 참·거짓 인자 읽기. 없거나 null이면 `default`, 모양이 다르면 오류
pub fn bool_arg(args: &Value, key: &str, default: bool) -> Result<bool, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(default),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => Err(format!("{key}는 참·거짓이어야 합니다")),
    }
}
