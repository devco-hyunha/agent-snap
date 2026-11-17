//! 03-8 execute 브리지: JS가 넘긴 값을 Rust 요약 글자로 바꾸는 판정만 둡니다(AC 4).
//!
//! 결론: 값→글자 변환은 이 파일 한 자리(`js_value_text`)에만 있고, 봉지·프로미스 체인을
//! 세어 최종값을 만드는 판정은 `script_value`에 있습니다. JS 얅개는 봉지 한 겹만 열고,
//! 최종값을 정하는 판정은 Rust 쪽에만 둡니다(dev-principles 3절 DRY, 보정 3).
//!
//! 03-8 T2 최종값 보정(보정 라운드 1 미달 A): 코드 본문을 `eval_promise`에 그대로 넘깁니다.
//! 평가 판은 코드를 함수 몸체로 만들어 마지막 문장값을 결승 값으로 주므로, 직접 함수로
//! 감싸지 말아야 합니다. 감싸면 최종값이 `undefined`가 되어 요약이 깨진 글자로 나옵니다.
//! 최종값을 문자열로 만드는 판정은 `js_value_text` 한 자리에만 있고, 값에 따라 `null`·`123`
//! 같은 글자로 바꾸므로 `undefined`가 결승 값인 코드는 `null`로 표시됩니다.
//!
//! 03-8 T2 값 변환 보정(라운드 3 major 5): JSON 변환 실패를 `unwrap_or_default()`로
//! 기본값에 넘기지 않고, 실패한 자리를 `오류: ...` 글자로 돌려줍니다(dev-principles 7절).

use rquickjs::Ctx;
use rquickjs::Function;
use rquickjs::Value;

use super::policy::MAX_PROMISE_ROUNDS;
use super::policy::SCRIPT_VALUE_KEY;

/// JS가 넘긴 값 글자를 요약 글자로 바꾸는 자리입니다(AC 4: 요약 1개만).
///
/// 결론: 문자열은 그대로 올리고, 문자열이 아니면 JSON 글자로 바꿉니다.
/// JSON 변환이 실패하면 `None`을 돌려주지 않고 `오류:` 글자로 남깁니다. JSON이 값을
/// 못 만드는 자리(함수·심볼·순환 객체)는 JS `String(v)`을 거친 결과를 JSON 글자로
/// 담아 돌려보냅니다. 이 자리까지 실패하면 오류 글자로 보고합니다(7절: 삼키지 않음).
pub(in crate::tools) fn js_value_text<'js>(ctx: &Ctx<'js>, value: &Value<'js>) -> String {
    if let Ok(text) = value.clone().get::<String>() {
        return text;
    }
    let json_text = match json_text_of(ctx, value) {
        Ok(text) => text,
        Err(reason) => return format!("오류: {reason}"),
    };
    // 앞 경로에서 JSON 글자가 정해지므로, 이 경로는 JSON 글자가 없는 JSON이 값을
    // 못 만든 값(`None`)만 지나갑니다.
    match json_text {
        Some(text) => text,
        None => match stringified_json_text(ctx, value) {
            Ok(Some(text)) => text,
            Ok(None) => "null".to_owned(),
            Err(reason) => format!("오류: {reason}"),
        },
    }
}

/// 코드 최종값을 값 하나로 만듭니다(AC 4: 요약은 값 1개).
///
/// 봉지를 벗기는 판정은 이 파일의 `unwrap_envelope` 한 자리에만 둡니다(보정 3 DRY).
/// 봉지를 한 겹 벗기면 값이 프로미스로 남고, 그 프로미스를 Rust 미래로 기다리면
/// 결과 값이 나옵니다. 이를 16겹까지 풀고, 넘치면 오류로 봅니다.
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

/// 값에 `value` 칸이 있으면 그 칸의 값으로 한 겹 벗깁니다(AC 4의 QuickJS 봉지).
fn unwrap_envelope<'js>(value: &Value<'js>) -> Value<'js> {
    let boxed = value
        .as_object()
        .and_then(|object| object.get::<_, Value>(SCRIPT_VALUE_KEY).ok());
    match boxed {
        Some(inner) if !inner.is_undefined() => inner,
        _ => value.clone(),
    }
}

/// 값을 JSON 글자로 만드는 자리입니다. 엔진이 던진 실패를 글자로 남깁니다(7절).
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

/// JSON이 값을 못 만든 값(함수·심볼·순환 객체)을 JS `String(v)`으로 만들어 JSON 글자로 넘깁니다.
///
/// `String(v)`이 던지면 실패 글자로 보고하고, `String(v)` 결과가 JSON 글자가 안 되면
/// `None`(결승 값 없음)으로 봅니다(7절: 삼키지 않음).
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
