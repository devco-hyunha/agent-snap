//! 도구 인자 JSON 해석·복구: 깨진 JSON은 한 번 복구를 시도

/// 인자 JSON을 해석. 깨진 JSON은 `repair_json`으로 한 번 복구한 뒤 확인
///
/// # 오류
/// - 복구 실패: `Err(이유)` 한 줄 문자열. `오류: ` 접두는 호출부가 붙임
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

/// 글자에서 첫 중괄호의 짝에 맞는 닫힘 기호 위치를 탐색
///
/// 문자열 안의 중괄호는 세지 않고, 짝이 안 맞으면 `None`
/// 2안 복원 쪽(`agent/text/recover.rs`)이 같은 지식을 다시 적지 않고 함께 사용
pub(crate) fn balanced_object_end(text: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    let mut inside_string = false;
    let mut escaped = false;
    for (index, ch) in text.char_indices() {
        if inside_string {
            if ch == '"' && !escaped {
                inside_string = false;
            }
            escaped = ch == '\\' && !escaped;
            continue;
        }
        match ch {
            '"' => inside_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// 닫힘 기호를 채워 한 번 고친 JSON 생성
///
/// 문자열 안의 `"`를 구분 창으로 쓰고, `{}`·`[]` 닫힘 기호만 채움
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
