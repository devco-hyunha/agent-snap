//! execute 브리지: JS 코드를 굴리는 전용 스레드 창구.
//!
//! 코드 한 덩어리마다 엔진을 새로 만들고 전용 스레드 독립 실행 판에서 굴림. rquickjs는 한 스레드가 쓰는 값이라 엔진·등록 목록·승인 대기를 한 스레드에 묶고, `Arc` 소유본으로 수명을 채움.
//! JS 엔진 스레드는 OS 자식을 만들지 않아 `ProcessTree`를 쓸 자리가 없음 → 워치독과 엔진 스레드 안 상한이 그 확인을 대체.

use std::future::Future;
use std::sync::mpsc;
use std::time::Duration;

use rquickjs::AsyncContext;
use rquickjs::AsyncRuntime;
use rquickjs::Ctx;
use rquickjs::Value;

use tokio_util::sync::CancellationToken;

use super::engine_types::EngineBridge;
use super::js_bindings::bind_bridge_functions;
use super::js_values::js_value_text;
use super::js_values::script_value;
use super::policy::SCRIPT_MAX_BYTES;
use super::policy::SCRIPT_SUMMARY_MAX_CHARS;
use super::policy::WATCHDOG_EXTRA_SECS;
use super::policy::WATCHDOG_GRACE_SECS;
use super::tool_refs::ToolRefs;
use super::watchdog::watch_outcome;
use crate::agent::record::truncate::truncate_tool_result;

/// JS 코드를 굴리고 결과 요약 1개를 받아옴.
///
/// `bridge`는 도구 등록 목록·실행 판·호출 기록 창구를 한 스레드에 넘기는 재료이며, 호출 전체 확인 승인은 `executor`가 채워 넘김.
#[must_use]
pub(in crate::tools) fn run_script(bridge: EngineBridge, script: &str) -> String {
    if script.len() > SCRIPT_MAX_BYTES {
        return format!("오류: execute JS 코드 길이 초과({SCRIPT_MAX_BYTES}자)");
    }
    let cancel = bridge.context.cancel_token();
    // 취소 토큰이 이미 끊겨 있으면 엔진을 켜지 않고 즉시 `중단`으로 끝냄.
    if cancel.is_cancelled() {
        return "오류: 중단: execute JS 코드를 중지했습니다".to_owned();
    }
    // `run_command`가 쓰는 시간 상한을 그대로 재사용.
    let run_limit_secs = bridge.context.run_command.run_limit.as_secs();
    let code = script.to_owned();
    std::thread::scope(|scope| {
        let (sender, receiver) = mpsc::channel();
        let worker =
            scope.spawn(move || drive_engine(code, bridge, cancel, run_limit_secs, sender));
        let watchdog = scope.spawn(move || watch_outcome(run_limit_secs, &receiver));
        // 워치독 판정을 먼저 기다림: 엔진 스레드가 결과를 안 보내도 시간 상한 안에 끝나야 함.
        // 엔진 스레드는 안에서 상한을 지키므로 스레드 합치기 대기에 걸리지 않음.
        let watchdog_verdict = watchdog
            .join()
            .unwrap_or_else(|_| Err("execute 감시 창구가 예상 밖으로 멈췄습니다".to_owned()));
        let outcome = watchdog_verdict.and_then(|verdict| match verdict {
            // 시간 상한·취소 판정이 먼저 오면 엔진 결과를 버림.
            Some(reason) => Err(reason),
            // 워치독이 엔진 알림을 받은 뒤이므로 엔진 결과를 기다림.
            None => worker
                .join()
                .unwrap_or_else(|_| Err("execute 엔진이 코드를 예상 밖으로 멈췄습니다".to_owned())),
        });
        outcome.unwrap_or_else(|reason| format!("오류: {reason}"))
    })
}

/// 코드를 굴리는 자리 전체를 한 스레드에서 돌리는 창구.
fn drive_engine(
    code: String,
    bridge: EngineBridge,
    cancel: CancellationToken,
    run_limit_secs: u64,
    sender: mpsc::Sender<Result<String, String>>,
) -> Result<String, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| "execute 엔진 실행 판을 만들지 못했습니다".to_owned())?;
    let local = tokio::task::LocalSet::new();
    // `Sender`는 복제가 안 되는 창구라 복제본을 만들어 사용.
    // async 블록에는 복제본만 넘기고, 원본은 블록 밖 마지막 알림에 사용.
    let notify = sender.clone();
    let outcome = local.block_on(&rt, async move {
        let engine = new_engine().await?;
        let context = new_context(&engine).await?;
        // 엔진 스레드 안에 시간 상한을 직접 걸어 이 스레드가 상한 안에 마칠 있게 함.
        // 워치독 판정 뒤 스레드를 합치는 자리가 결과를 안 보내는 스크립트에서
        // 영구 대기되지 않음. 엔진 확인 판의 `'static` 수명 조건을 이 한 자리에서 채움.
        let settled = context.async_with(async move |ctx| {
            let refs = ToolRefs::new(bridge.registry, bridge.context, bridge.log);
            bind_bridge_functions(&ctx, refs)
                .map_err(|_| "execute 도구를 JS에 연결하지 못했습니다".to_owned())?;
            let outer = eval_script_code(&ctx, &code).await?;
            let value = script_value(outer).await?;
            let text = js_value_text(&ctx, &value);
            Ok(truncate_tool_result(&text, SCRIPT_SUMMARY_MAX_CHARS))
        });
        let outcome = watch_engine_run(&engine, settled, cancel, run_limit_secs).await;
        // 결판 값을 만들기 전에 취소를 확인: 승인 대기 future가 취소로 끝나면
        // 엔진이 그 future를 마저 굴러야 프로미스가 닫힘.
        if outcome.is_err() {
            let _ = notify.send(Ok("엔진 결과 알림".to_owned()));
            drain_engine(&engine).await;
        }
        outcome
    });
    let _ = sender.send(outcome.clone());
    outcome
}

/// QuickJS 엔진 생성.
async fn new_engine() -> Result<AsyncRuntime, String> {
    AsyncRuntime::new().map_err(|_| "execute 엔진을 만들지 못했습니다".to_owned())
}

/// 엔진 확인 판 생성.
async fn new_context(engine: &AsyncRuntime) -> Result<AsyncContext, String> {
    AsyncContext::full(engine)
        .await
        .map_err(|_| "execute 엔진 확인 판을 만들지 못했습니다".to_owned())
}

/// 코드 본문을 JS 프로미스로 평가해 값이 담긴 프로미스로 받아옴.
///
/// 코드를 함수 몸체로 만들어 마지막 표현식을 결승 값으로 주므로, `(async () => { ... })()`처럼 직접 감싸면 최종값이 항상 `undefined`가 됨.
async fn eval_script_code<'js>(ctx: &Ctx<'js>, code: &str) -> Result<Value<'js>, String> {
    ctx.eval_promise(code)
        .map_err(|error| format!("execute JS 코드 평가 오류: {error}"))?
        .into_future::<Value>()
        .await
        .map_err(|error| format!("execute JS 코드: {error}"))
}

/// JS가 도구 호출 future를 엔진 안에 세울 때 그 future를 마저 굴리는 창구.
///
/// `drive`를 계속 빌려야 코드가 진행하므로 select 갈래에 늘 넣음.
async fn pump_engine(engine: &AsyncRuntime) {
    engine.drive().await;
}

/// 엔진 실행을 엔진 진행·취소·시간 상한과 겨루는 창구.
///
/// 취소를 안에서 살피는 것은 빠른 종료용이고, 시간 상한은 워치독과 여기서 함께 지킴.
async fn watch_engine_run<F>(
    engine: &AsyncRuntime,
    settled: F,
    cancel: CancellationToken,
    run_limit_secs: u64,
) -> Result<String, String>
where
    F: Future<Output = Result<String, String>>,
{
    let pump = pump_engine(engine);
    tokio::pin!(pump);
    let engine_deadline = tokio::time::sleep(Duration::from_secs(
        run_limit_secs.saturating_add(WATCHDOG_EXTRA_SECS),
    ));
    tokio::pin!(engine_deadline);
    tokio::pin!(settled);
    tokio::select! {
        result = &mut settled => result,
        () = &mut pump => Err("execute 엔진이 코드를 예상 밖으로 멈췄습니다".to_owned()),
        () = cancel.cancelled() => Err("중단: execute JS 코드를 중지했습니다".to_owned()),
        () = &mut engine_deadline => Err("미완료: execute JS 코드 실행 상한 초과".to_owned()),
    }
}

/// 엔진이 남긴 future를 마저 굴려 프로미스를 닫는 창구.
///
/// drive가 끝나거나 여유 시간이 먼저 오면 마침.
async fn drain_engine(engine: &AsyncRuntime) {
    let drain = pump_engine(engine);
    tokio::pin!(drain);
    let drain_deadline = tokio::time::sleep(Duration::from_secs(WATCHDOG_GRACE_SECS));
    tokio::pin!(drain_deadline);
    tokio::select! {
        () = &mut drain => {}
        () = &mut drain_deadline => {}
    }
}
