//! execute 브리지: JS에 도구 창구를 다는 얅개 조립.
//!
//! Rust 미래는 `Async` 창구로 붙이고 함수값은 1칸 얅개(`__snapTool`)로 감쌈. QuickJS가 `Async` 함수값을 `{value: 프로미스}` 봉지로 감싸므로 얅개가 `value` 칸을 열어 안의 프로미스만 JS에 반환.
//! 정본(봉지 열기 판정·16겹 상한)은 Rust 한 자리에 두고 JS 얅개는 같은 칸 이름·같은 값으로 따라감 → JS 쪽 중복 없음.

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

/// 1차 허용 목록 4개와 `run_tool` 창구를 JS에 연결.
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
        // 코드가 `read_file({path: 'x'})`처럼 JS 값을 주면 JSON 글자로 바꿔 Rust에 전달.
        // 이미 JSON 글자를 준 호출은 두 번 감싸지 않게 얅개 한 자리(`__snapArg`)에서
        // 문자열은 그대로 두고 JS 값만 JSON 글자로 변환.
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

/// JS에 다는 얅개 문자열을 조립.
///
/// 봉지 열기와 16겹 상한의 정본은 Rust(`js_values.rs`·`MAX_PROMISE_ROUNDS`)에 두고 JS 얅개는 같은 칸 이름·같은 값으로 따라감 → 체인 판정·숫자가 JS 쪽에 중복되지 않음.
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

/// 1차 허용 목록 이름을 JS 배열로 반환.
fn readonly_names<'js>(ctx: &Ctx<'js>) -> JsResult<Array<'js>> {
    let names = Array::new(ctx.clone())?;
    for (index, name) in READONLY_TOOLS.iter().enumerate() {
        names
            .set(index, (*name).to_owned())
            .map_err(|_| JsError::new_into_js("array", "object"))?;
    }
    Ok(names)
}

/// JS에 다는 창구를 이름과 함께 묶음.
#[derive(Clone)]
struct BoundCall {
    refs: ToolRefs,
    name: String,
}
