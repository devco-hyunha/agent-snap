//! 도구 인자 JSON 해석·복구(spec AC 3, 2609.07370): 깨진 JSON은 한 번 복구를 시도한다.
//!
//! 성공하면 고친 JSON 문자열을 돌려주고, 실패하면 이유만 한 줄로 만든다.
//! `오류: ` 접두는 `ToolOutcome::error`가 붙이므로 여기서 다시 붙이지 않는다.
//! 복구 시도는 이 함수 한 번만 하며, 02-6 전송 재시도와는 별이다(D13).

/// 인자 JSON을 해석한다. 깨진 JSON은 `repair_json`으로 한 번 복구한 뒤 확인한다(spec AC 3).
///
/// # 결과
/// - 그대로·복구된 JSON 문자열: `Ok(사용할 JSON)`.
/// - 복구 실패: `Err(이유)` 한 줄 문자열. `오류: ` 접두는 호출부가 붙인다.
pub fn recover_arguments(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok("{}".to_owned());
    }
    if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
        return Ok(trimmed.to_owned());
    }
    repair_json(trimmed).ok_or_else(|| "인자 JSON 형식 오류".to_owned())
}

/// 닫힘 기호를 채워 한 번 고친 JSON을 만든다.
///
/// 문자열 안의 `"`를 구분 창으로 쓰고, `{}`·`[]` 닫힘 기호만 채운다.
/// 열림 기호의 짝이 맞지 않으면 `None`이고, 짝이 모두 맞아도 해석이 안 되면 `None`이다.
fn repair_json(raw: &str) -> Option<String> {
    let mut opens: Vec<char> = Vec::new();
    let mut inside_string = false;
    let mut escaped = false;
    for ch in raw.chars() {
        if inside_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                inside_string = false;
            }
            continue;
        }
        match ch {
            '"' => inside_string = true,
            '{' | '[' => opens.push(ch),
            '}' | ']' => {
                opens.pop()?;
            }
            _ => {}
        }
    }
    if opens.is_empty() {
        return None;
    }
    let mut repaired = String::from(raw.trim());
    for open in opens.iter().rev() {
        repaired.push(match open {
            '{' => '}',
            '[' => ']',
            other => *other,
        });
    }
    serde_json::from_str::<serde_json::Value>(&repaired)
        .is_ok()
        .then_some(repaired)
}
