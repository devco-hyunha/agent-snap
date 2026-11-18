//! execute 브리지: JS가 넘긴 값을 Rust 요약 글자로 바꾸는 판정.
//!
//! 값→글자 변환과 봉지·프로미스 체인 판정은 Rust 한 자리에만 둠. JS 얅개는 봉지 한 겹만 열고 Rust 정본과 같은 규칙을 따라감.
//! 코드 본문을 `eval_promise`에 그대로 넘김 — 직접 함수로 감싸면 최종값이 `undefined`가 되어 요약이 깨진 글자로 나옴.

use rquickjs::Ctx;
use rquickjs::Function;
use rquickjs::Value;

use super::policy::MAX_PROMISE_ROUNDS;
use super::policy::SCRIPT_VALUE_KEY;

/// JS가 넘긴 값 글자를 요약 글자로 변환.
///
/// 문자열은 그대로 올리고 문자열이 아니면 JSON 글자로 변환. JSON 변환 실패는 기본값에 넘기지 않고 `오류:` 글자로 남기며, JSON이 값을 못 만드는 값(함수·심볼·순환 객체)은 JS `String(v)` 결과를 JSON 글자로 담아 반환.
pub(in crate::tools) fn js_value_text<'js>(ctx: &Ctx<'js>, value: &Value<'js>) -> String {
    if let Ok(text) = value.clone().get::<String>() {
        return text;
    }
    let json_text = match json_text_of(ctx, value) {
        Ok(text) => text,
        Err(reason) => return format!("오류: {reason}"),
    };
    match json_text {
        Some(text) => text,
        None => match stringified_json_text(ctx, value) {
            Ok(Some(text)) => text,
            Ok(None) => "null".to_owned(),
            Err(reason) => format!("오류: {reason}"),
        },
    }
}

/// 코드 최종값을 값 하나로 만듦.
///
/// 봉지 열기 판정은 `unwrap_envelope` 한 자리에만 둠. 한 겹 벗긴 뒤 남은 프로미스를 Rust 미래로 기다려 결과 값을 얻고, 16겹까지 풀며 넘치면 오류로 보고.
pub(in crate::tools) async fn script_value<'js>(outer: Value<'js>) -> Result<Value<'js>, String> {
    let mut current = unwrap_envelope(&outer);
    let mut rounds = 0;
    while let Some(promise) = current.as_promise().cloned() {
        rounds += 1;
        if rounds > MAX_PROMISE_ROUNDS {
            return Err(format!("프로미스 중첩 {MAX_PROMISE_ROUNDS} 초과"));
        }
        current = promise
            .into_future::<Value>()
            .await
            .map_err(|error| format!("execute JS 코드: {error}"))?;
        current = unwrap_envelope(&current);
    }
    Ok(current)
}

/// 값에 `value` 칸이 있으면 그 칸의 값으로 한 겹 벗김.
fn unwrap_envelope<'js>(value: &Value<'js>) -> Value<'js> {
    let boxed = value
        .as_object()
        .and_then(|object| object.get::<_, Value>(SCRIPT_VALUE_KEY).ok());
    match boxed {
        Some(inner) if !inner.is_undefined() => inner,
        _ => value.clone(),
    }
}

/// 값을 JSON 글자로 만듦. 엔진이 던진 실패를 글자로 남김.
fn json_text_of<'js>(ctx: &Ctx<'js>, value: &Value<'js>) -> Result<Option<String>, String> {
    ctx.json_stringify(value.clone())
        .map_err(|error| format!("execute JS 값의 JSON 글자 만들기 실패: {error}"))?
        .map(|coerced| {
            coerced
                .get::<String>()
                .map_err(|error| format!("execute JS 값의 JSON 글자 읽기 실패: {error}"))
        })
        .transpose()
}

/// JSON이 값을 못 만든 값(함수·심볼·순환 객체)을 JS `String(v)`로 만들어 JSON 글자로 변환.
///
/// `String(v)`이 던지면 실패 글자로 보고하고, 결과가 JSON 글자가 안 되면 값 없음으로 봄.
fn stringified_json_text<'js>(
    ctx: &Ctx<'js>,
    value: &Value<'js>,
) -> Result<Option<String>, String> {
    let stringify = ctx
        .globals()
        .get::<_, Function>("String")
        .map_err(|_| "execute JS 요약 생성 함수(String)이 없습니다".to_owned())?;
    let text_value = stringify
        .call::<_, Value>((value.clone(),))
        .map_err(|error| format!("execute JS 값 요약 생성 실패: {error}"))?;
    json_text_of(ctx, &text_value)
}
