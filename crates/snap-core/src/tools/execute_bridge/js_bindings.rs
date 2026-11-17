//! 03-8 execute 브리지: JS에 도구 창구를 다는 얅개 조립만 둡니다(AC 1·AC 2).
//!
//! 결론: Rust 미래는 `Async` 창구로 붙이고, 그 함수값은 1칸 얅개(`__snapTool`)로 감쌉니다.
//! QuickJS가 `Async` 함수값을 `{value: 프로미스}` 봉지로 감싸기 때문에, 얅개는 `value` 칸을
//! 한 번 열어 안의 프로미스만 JS에 돌려줍니다. 봉지를 열어내는 판정(`value` 칸 열기)의
//! 정본은 `js_values.rs`의 `unwrap_envelope`(Rust) 한 자리이고, JS 얅개는 봉지를 여는 규칙을
//! Rust 정본과 같은 방식으로 읽습니다(dev-principles 3절 DRY, 보정 3).
//! 얅개 조립은 이 파일 한 자리(`bind_bridge_functions`)에만 두어 목록 4개와 `run_tool`이
//! 같은 얅개를 재사용합니다. 걸려 있는 목록은 JS 배열(`SNAP_TOOLS`)로 보여 줍니다(AC 1).
//!
//! 03-8 T2 라운드 3 보정(major 6·9): 16겹 상한의 정본은 Rust의 `policy::MAX_PROMISE_ROUNDS`
//! 한 자리이고, JS 얅개는 그 정본을 Rust에서 조립해 같은 숫자로 씁니다(dev-principles 3절 DRY).
//! 매개 얅개(`__snapArg`)는 `JSON.stringify`가 순환 객체에서 던지는 예외를 잡습니다(8절 보안,
//! 7절 에러 처리). 실패 글자를 JS가 `Promise`로 받듯 거절로 돌려 엔진 정지로 번지지 않게 합니다.

use rquickjs::Array;
use rquickjs::Ctx;
use rquickjs::Error as JsError;
use rquickjs::Function;
use rquickjs::Result as JsResult;
use rquickjs::function::Async;

use super::policy::JS_ARG_NAME;
use super::policy::JS_RAW_PREFIX;
use super::policy::JS_RAW_RUN_TOOL_NAME;
use super::policy::JS_RUN_TOOL_NAME;
use super::policy::JS_WRAPPER_NAME;
use super::policy::MAX_PROMISE_ROUNDS;
use super::policy::READONLY_TOOLS;
use super::policy::SCRIPT_VALUE_KEY;
use super::tool_refs::ToolRefs;

/// 1차 허용 목록 4개와 `run_tool` 창구를 JS에 다는 자리입니다(AC 1·AC 2).
pub(in crate::tools) fn bind_bridge_functions<'js>(ctx: &Ctx<'js>, refs: ToolRefs) -> JsResult<()> {
    let globals = ctx.globals();
    let _: () = ctx.eval(js_tool_wrapper_source().as_str())?;
    for name in READONLY_TOOLS {
        let bound = BoundCall {
            refs: refs.clone(),
            name: (*name).to_owned(),
        };
        let raw = Function::new(
            ctx.clone(),
            Async(move |args_json: String| {
                let bound = bound.clone();
                async move { Ok::<String, JsError>(bound.refs.call(&bound.name, args_json).await) }
            }),
        )?;
        let raw_name = format!("{JS_RAW_PREFIX}{name}");
        globals.set(raw_name.as_str(), raw)?;
        // 코드가 `read_file({path: 'x'})`처럼 JS 값을 주면 JSON 글자로 바꿔 Rust에 넘깁니다.
        // 코드가 이미 JSON 글자를 준 호출은 두 번 감싸지 않게, 얅개 한 자리(`__snapArg`)에서
        // 문자열은 그대로 두고 JS 값만 JSON 글자로 바꿉니다(AC 1의 매개 규격 한 자리).
        let wrapper = ctx.eval::<Function<'js>, _>(format!(
            "(args) => globalThis.{WRAPPER_NAME}(globalThis.{raw_name}(globalThis.{ARG_NAME}(args)))",
            WRAPPER_NAME = JS_WRAPPER_NAME,
            raw_name = raw_name,
            ARG_NAME = JS_ARG_NAME,
        ))?;
        globals.set((*name).to_owned(), wrapper)?;
    }
    let dynamic = refs.clone();
    let raw_run_tool = Function::new(
        ctx.clone(),
        Async(move |name: String, args_json: String| {
            let dynamic = dynamic.clone();
            async move { Ok::<String, JsError>(dynamic.call(&name, args_json).await) }
        }),
    )?;
    globals.set(JS_RAW_RUN_TOOL_NAME, raw_run_tool)?;
    let run_tool = ctx.eval::<Function<'js>, _>(format!(
        "(name, args) => globalThis.{WRAPPER_NAME}(globalThis.{RAW_RUN_TOOL}(name, globalThis.{ARG_NAME}(args)))",
        WRAPPER_NAME = JS_WRAPPER_NAME,
        RAW_RUN_TOOL = JS_RAW_RUN_TOOL_NAME,
        ARG_NAME = JS_ARG_NAME,
    ))?;
    globals.set(JS_RUN_TOOL_NAME, run_tool)?;
    globals.set("SNAP_TOOLS", readonly_names(ctx)?)?;
    Ok(())
}

/// JS에 다는 얅개 문자열을 조립하는 자리입니다(AC 1·AC 2·AC 4의 흐름).
///
/// 얅개가 봉지를 연 자리는 JS 쪽 한 자리이고, Rust 쪽의 봉지 열기 판정(`js_values.rs`의
/// `unwrap_envelope`)과 같은 칸 이름(`SCRIPT_VALUE_KEY`)을 씁니다. JS 얅개는 봉지 한 겹만
/// 열고, 봉지·프로미스 체인을 세어 최종값을 만드는 판정은 Rust 한 자리에만 둡니다
/// (dev-principles 3절 DRY, 보정 3).
///
/// 16겹 상한은 Rust 정본(`MAX_PROMISE_ROUNDS`)에서 값을 옮겨 조립하므로 JS 쪽에 숫자가
/// 따로 없습니다(보정 3, major 6).
///
/// 매개 얅개(`__snapArg`)는 `JSON.stringify(args)`가 순환 객체에서 던지는 예외를 잡습니다.
/// 거절로 돌려 엔진이 멈추는 자리로 번지지 않게 합니다(minor 11, dev-principles 7절·8절).
pub(in crate::tools) fn js_tool_wrapper_source() -> String {
    let rounds = MAX_PROMISE_ROUNDS;
    format!(
        r#"
globalThis.{WRAPPER_NAME} = function (v) {{
  return new Promise((resolve, reject) => {{
    const step = (x, depth) => {{
      if (depth > {ROUNDS}) {{ resolve(x); return; }}
      if (x && typeof x === 'object' && Object.prototype.hasOwnProperty.call(x, '{VALUE_KEY}')) {{
        const inner = x.{VALUE_KEY};
        if (inner && typeof inner.then === 'function') {{ inner.then((y) => step(y, depth + 1), reject); }}
        else {{ step(inner, depth + 1); }}
        return;
      }}
      resolve(x);
    }};
    if (v && typeof v.then === 'function') {{ v.then((x) => step(x, 0), reject); }}
    else {{ step(v, 0); }}
  }});
}};
globalThis.{ARG_NAME} = function (args) {{
  if (typeof args === 'string') {{ return args; }}
  try {{
    return JSON.stringify(args);
  }} catch (e) {{
    throw new Error('execute 매개 JSON 변환 실패: ' + e.message);
  }}
}};
globalThis.{WRAPPER_NAME};
"#,
        WRAPPER_NAME = JS_WRAPPER_NAME,
        ARG_NAME = JS_ARG_NAME,
        ROUNDS = rounds,
        VALUE_KEY = SCRIPT_VALUE_KEY,
    )
}

/// 1차 허용 목록 이름을 JS 배열로 만들어 주는 자리입니다(AC 1).
fn readonly_names<'js>(ctx: &Ctx<'js>) -> JsResult<Array<'js>> {
    let names = Array::new(ctx.clone())?;
    for (index, name) in READONLY_TOOLS.iter().enumerate() {
        names
            .set(index, (*name).to_owned())
            .map_err(|_| JsError::new_into_js("array", "object"))?;
    }
    Ok(names)
}

/// JS에 다는 창구를 이름과 함께 묶는 자리입니다(같은 조립을 4개에 다시 쓰지 않게).
#[derive(Clone)]
struct BoundCall {
    refs: ToolRefs,
    name: String,
}
