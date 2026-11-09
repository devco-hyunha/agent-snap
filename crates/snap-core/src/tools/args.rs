//! 파일 도구가 받는 인자 JSON 해석을 한 곳에서만 다루는 자리입니다(03-3 T1, dev-principles 3절 DRY).

use serde_json::Value;

/// 도구 인자 JSON을 해석합니다. 루프가 앞 단계에서 복구한 JSON을 받는 자리라 형식 오류만 짧게 밝힙니다.
///
/// `read_file`·`write_file`·`edit_file`이 같은 문장을 쓰므로 도구마다 해석기를 다시 두지 않습니다.
pub fn parse_tool_args(args_json: &str) -> Result<Value, &'static str> {
    serde_json::from_str(args_json).map_err(|_| "인자 JSON 형식 오류")
}
