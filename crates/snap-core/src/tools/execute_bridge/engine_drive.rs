//! 03-8 execute 브리지: JS 코드를 굴리는 전용 스레드의 창구만 둡니다(AC 1·AC 4·AC 6).
//!
//! 결론: 코드 한 덩어리마다 엔진을 새로 만들고, 전용 스레드의 독립 runtime에서 굴립니다.
//! rquickjs 엔진은 한 스레드가 쓰는 값이고, 승인을 기다리는 Rust 미래와 JS 프로미스가
//! 같은 엔진 락을 쓰기 때문입니다.
//!
//! 검사 3종(길이·시간 상한·중단)을 걸고, 결과 요약 1개만 올립니다. 시간 상한과 취소는
//! 03-6 `run_command`이 쓰는 값·토큰을 그대로 재사용합니다(spec AC 6). 엔진은 rquickjs
//! 0.14.0(D51 승인, features std·futures)입니다.
//!
//! 03-8 T2 lifetime 보정: rquickjs의 엔진 확인 판(`async_with`)은 클로저가 `'static`보다
//! 오래 사는 값을 요구합니다. 그래서 등록 목록·실행 판·기록 창구를 빌린 값 대신 소유본
//! (`Arc`)으로 옮겨 담습니다. `parallel` 특징이 없는 rquickjs는 엔진을 한 스레드에서만
//! 쓰므로, 전용 스레드 안에서 엔진·등록 목록·승인 대기를 한 스레드에 묶어 굴립니다.
//!
//! AC 6 트리 정리 한계(라운드 3 major 7): 03-7 `ProcessTree`는 셸 자식 프로세스를 OS
//! 묶음에 담는 창구인데, JS 엔진 스레드는 OS 자식을 만들지 않습니다. 얹을 자리가 없는
//! 자리라 워치독과 엔진 스레드 안의 상한이 그 확인을 대신합니다(이 주석이 그 사유 한 줄).
//!
//! 값→요약 변환과 프로미스 판정은 `js_values.rs`, 워치독 판정은 `watchdog.rs`가 맡습니다
//! (dev-principles 2절 SRP). 엔진 구동은 동작 단위마다 이름 한 개씩의 함수로 나눕니다.

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

/// JS 코드를 굴리고 결과 요약 1개를 받아옵니다.
///
/// `bridge`는 03-3·03-6 도구가 등록된 목록·03-3이 만든 실행 판·03-8 호출 기록 창구를
/// 한 스레드에 넘기는 재료입니다(`engine_types::EngineBridge`). 호출 전체 확인 1승인은
/// 03-2 `executor`가 채워 넘깁니다.
#[must_use]
pub(in crate::tools) fn run_script(bridge: EngineBridge, script: &str) -> String {
    if script.len() > SCRIPT_MAX_BYTES {
        return format!("오류: execute JS 코드 길이 초과({SCRIPT_MAX_BYTES}자)");
    }
    let cancel = bridge.context.cancel_token();
    // 02 취소 토큰이 이미 끊겨 있으면 엔진을 켜지 않고 즉시 `중단`으로 끝냅니다(AC 6).
    if cancel.is_cancelled() {
        return "오류: 중단: execute JS 코드를 중지했습니다".to_owned();
    }
    // 03-6 `run_command`이 쓰는 시간 상한을 그대로 재사용합니다(spec AC 6).
    let run_limit_secs = bridge.context.run_command.run_limit.as_secs();
    let code = script.to_owned();
    std::thread::scope(|scope| {
        let (sender, receiver) = mpsc::channel();
        let worker =
            scope.spawn(move || drive_engine(code, bridge, cancel, run_limit_secs, sender));
        let watchdog = scope.spawn(move || watch_outcome(run_limit_secs, &receiver));
        // 보정 2(AC 6): 워치독 판정을 먼저 기다립니다. 엔진 스레드가 결과를 안 보내는
        // 스크립트에서도 시간 상한 안에 끝나야 하기 때문입니다. 엔진 스레드는 안에서
        // 상한을 지키므로 스레드 합치기 대기에는 걸리지 않습니다.
        let watchdog_verdict = watchdog
            .join()
            .unwrap_or_else(|_| Err("execute 감시 창구가 예상 밖으로 멈췄습니다".to_owned()));
        let outcome = watchdog_verdict.and_then(|verdict| match verdict {
            // 시간 상한·취소 판정이 먼저 오면 엔진 결과를 버립니다.
            Some(reason) => Err(reason),
            // 워치독이 엔진 알림을 받은 뒤이므로 엔진 결과를 기다립니다.
            None => worker
                .join()
                .unwrap_or_else(|_| Err("execute 엔진이 코드를 예상 밖으로 멈췄습니다".to_owned())),
        });
        outcome.unwrap_or_else(|reason| format!("오류: {reason}"))
    })
}

/// 코드를 굴리는 자리 전체를 한 스레드에서 돌리는 창구입니다(동작 단위 함수를 잇습니다).
///
/// 엔진 생성 → 확인 판과 함수 연결 → 코드 평가와 프로미스 대기 → 상한 감시 → 채널 알림
/// → drain 순서로 이름 한 개씩의 함수를 이어 부릅니다. JS 실패는 짧은 오류 글자로만
/// 돌려줍니다(AC 6: 원문을 결과에 통째로 넣지 않음).
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
    // 보정 1(E0382): `Sender`는 복제가 안 되는 창구라 복제본을 만들어 씁니다.
    // async 블록에는 복제본만 넘기고, 원본은 블록 밖 마지막 알림에 씁니다.
    let notify = sender.clone();
    let outcome = local.block_on(&rt, async move {
        let engine = new_engine().await?;
        let context = new_context(&engine).await?;
        // 보정 2(AC 6): 엔진 스레드 안에 시간 상한을 직접 걸어, 이 스레드가 반드시
        // 상한 안에 마칠 있게 합니다. 그래야 워치독 판정 뒤 스레드를 합치는 자리가
        // 결과를 안 보내는 스크립트에서 영구 대기되지 않습니다.
        // 엔진 확인 판의 `'static` 수명 조건(파일 위 주석의 lifetime 보정)을 이 한 자리에서
        // 채웁니다. 도구 연결·평가·프로미스 대기를 같은 순서로 두는 자리입니다.
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
        // 결판 값을 만들기 전에 취소를 확인합니다: 승인 대기 future가 취소로
        // 끝나면 엔진이 그 future를 마저 굴러야 프로미스가 닫힙니다(AC 6).
        if outcome.is_err() {
            let _ = notify.send(Ok("엔진 결과 알림".to_owned()));
            drain_engine(&engine).await;
        }
        outcome
    });
    let _ = sender.send(outcome.clone());
    outcome
}

/// QuickJS 엔진을 만드는 자리입니다(엔진 생성 동작 한 개).
async fn new_engine() -> Result<AsyncRuntime, String> {
    AsyncRuntime::new().map_err(|_| "execute 엔진을 만들지 못했습니다".to_owned())
}

/// 엔진 확인 판을 만드는 자리입니다(확인 판 생성 동작 한 개).
async fn new_context(engine: &AsyncRuntime) -> Result<AsyncContext, String> {
    AsyncContext::full(engine)
        .await
        .map_err(|_| "execute 엔진 확인 판을 만들지 못했습니다".to_owned())
}

/// 코드 본문을 JS 프로미스로 평가해 값이 담긴 프로미스로 받아옵니다(AC 1·AC 4).
///
/// 코드 본문을 `eval_promise`에 그대로 넘깁니다. 이 평가 판은 코드를 함수 몸체로 만들어
/// 마지막 문장의 값(마지막 표현식)을 결승 값으로 돌려줍니다. `(async () => { ... })()`처럼
/// 직접 감싸면 그 함수의 결승 값이 없어 최종값이 항상 `undefined`가 됩니다(보정 라운드 1
/// 미달 A의 원인).
async fn eval_script_code<'js>(ctx: &Ctx<'js>, code: &str) -> Result<Value<'js>, String> {
    ctx.eval_promise(code)
        .map_err(|error| format!("execute JS 코드 평가 오류: {error}"))?
        .into_future::<Value>()
        .await
        .map_err(|error| format!("execute JS 코드: {error}"))
}

/// JS가 도구 호출 future를 엔진 안에 세울 때 그 future를 마저 굴리는 창구입니다.
///
/// `drive`를 계속 빌려야 코드가 진행하므로 select 갈래에 늘 넣습니다(AC 6 흐름).
async fn pump_engine(engine: &AsyncRuntime) {
    engine.drive().await;
}

/// 엔진 실행을 엔진 진행·취소·시간 상한과 겨루는 자리입니다(상한 감시 동작 한 개, AC 6).
///
/// 취소를 안에서 살피는 것은 빠른 종료용이고, 시간 상한은 워치독과 여기서 함께 지킵니다.
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

/// 엔진이 남긴 future를 마저 굴려 프로미스를 닫는 자리입니다(drain 동작 한 개, AC 6: 상한 안).
///
/// drive가 끝나거나 여유 시간이 먼저 오면 마칩니다.
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
