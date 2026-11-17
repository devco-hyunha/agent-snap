//! 03-8 execute 브리지: JS가 도구를 부르는 창구(`ToolRefs`)와 그 창구가 도구를 부르는 자리만 둡니다
//! (AC 1·AC 2·AC 3).
//!
//! 결론: 03-3·03-6 `Tool` 구현을 그대로 부르는 창구 하나만 JS에 건니다. 1차 허용 목록
//! 읽기 4개(read_file·grep·glob·list_dir)는 이름 그대로 JS에 걸고, 그 밖의 이름은
//! 같은 자리에서 막힌 글자를 돌려줍니다(spec.md 19행 1차 목록).
//! 승인을 기다리는 자리는 Rust 미래이고, rquickjs는 Rust 미래와 JS 프로미스를 왕복해 줍니다.
//! 코드가 `await`하면 승인 대기가 끝난 뒤에 진행합니다(AC 2: `for` 반복도 호출마다 게이트).
//! 매개·게이트·잘림은 03-3·03-6이 같은 검사를 다시 하게 해 그대로 재사용합니다(DRY).
//!
//! 03-8 T2 승인 창구 보정(보정 라운드 1 미달 A·B): `run_tool`은 읽기 4개를 승인 없이
//! 부르고, 쓰기·명령 이름(`write_file`·`edit_file`·`run_command`)은 03-6 승인 창구를
//! 호출마다 탄 뒤에 부릅니다(spec AC 2·AC 5). 등록되지 않은 이름은 승인 창구 앞에
//! 같은 자리에서 막습니다. `write_file`·`edit_file`은 03-6 도구가 승인 표식을 보지
//! 않으므로 승인 창구가 승인 1개만 셈니다. `run_command`은 승인 표식을 소비하는 도구라
//! 창구가 승인한 뒤 두 번 셈지지 않게 같은 자리가 표식을 지웁니다(AC 5의 승인 2개).
//!
//! 03-8 T2 lifetime 보정: JS에 다는 함수는 엔진 확인 판이 `'static`보다 오래 사는 값만 요구합니다.
//! 그래서 등록 목록·실행 판·기록 창구를 빌린 값이 아니라 `Arc` 소유본으로 자게 합니다.
//!
//! 03-8 T2 라운드 3 보정(major 4): 승인 뒤에 도구를 부르는 순서(표식 채우기 → 호출 →
//! 지우기 → 재승인 거부)는 03-2 `agent::executor::run_approved_call`이 정본이고, 이 자리는
//! 그 정본을 그대로 부릅니다(dev-principles 3절 DRY). 정본 쪽 가시(범위)만 넓혔고, 이 자리
//! 순서를 다시 정의하지 않습니다.

use std::sync::Arc;

use crate::agent::executor::run_approved_call;
use crate::agent::record::registry::ToolRegistry;
use crate::tools::tool::Tool;
use crate::tools::tool::ToolContext;
use crate::tools::tool::ToolOutcome;

use super::approval_gate::ScriptApproval;
use super::policy::GATED_TOOLS;
use super::policy::READONLY_TOOLS;
use super::script_log::ScriptLog;
use super::text_parts::outcome_text;
use super::text_parts::write_summary;

/// JS가 도구를 부르는 창구입니다. 등록 목록·실행 판·기록 창구를 한곳에서 자게 합니다.
///
/// 엔진이 `'static`으로 오래 쓰므로 빌린 수명을 두지 않습니다(03-8 T2 lifetime 보정).
#[derive(Clone)]
pub(in crate::tools) struct ToolRefs {
    registry: Arc<ToolRegistry>,
    context: Arc<ToolContext>,
    log: Arc<ScriptLog>,
}

impl ToolRefs {
    /// 창구를 등록 목록·실행 판·기록 창구 복제본과 함께 만듭니다.
    pub(in crate::tools) const fn new(
        registry: Arc<ToolRegistry>,
        context: Arc<ToolContext>,
        log: Arc<ScriptLog>,
    ) -> Self {
        Self {
            registry,
            context,
            log,
        }
    }

    /// 코드가 이름으로 도구를 불러 결과를 글자로 받아옵니다(AC 1·AC 2·AC 3).
    ///
    /// 열려 있는 이름은 1차 허용 목록 읽기 4개와 03-6이 승인 대상으로 정한 이름입니다.
    /// 읽기 4개는 승인 없이 바로 부르고, 승인 대상 이름은 승인 창구를 호출마다 탄 뒤에
    /// 부릅니다. 그 밖의 이름은 승인 창구 앞에 같은 자리에서 막습니다(AC 2: 게이트 전 차단).
    /// 기록은 결과를 받은 뒤에 매번 쌓입니다(AC 3).
    pub(in crate::tools) async fn call(self, name: &str, args_json: String) -> String {
        let is_open = READONLY_TOOLS.contains(&name) || GATED_TOOLS.contains(&name);
        if !is_open {
            return format!("오류: 1차 허용 목록 밖 도구({name})");
        }
        let Some(tool) = self.registry.find(name) else {
            return format!("도구 없음: {name}");
        };
        let outcome = self.run_tool(tool, name, &args_json).await;
        let text = outcome_text(&outcome);
        self.log.push(name, &args_json, &text);
        text
    }

    /// 03-3·03-6 구현을 그대로 부르는 자리입니다(AC 2 게이트를 같은 자리에 둡니다).
    ///
    /// 쓰기·명령 도구는 03-6 승인 창구를 호출마다 열고, 승인 응답이 승인한 호출에만
    /// 03-2 정본(`run_approved_call`)로 표식을 채워 부릅니다. 거부는 03-6 정본 문구로
    /// 결과를 남깁니다.
    async fn run_tool(
        &self,
        tool: &(dyn Tool + Send + Sync),
        name: &str,
        args_json: &str,
    ) -> ToolOutcome {
        if !tool.executes_writes() {
            return tool.call(args_json, &self.context).await;
        }
        let summary = write_summary(name, args_json);
        let approval = ScriptApproval::new(self.context.clone());
        let reply = approval.gate(&summary).await;
        if !reply.is_approved() {
            return ToolOutcome::Error(reply.rejection_text(&summary));
        }
        run_approved_call(tool, args_json, &self.context, &summary).await
    }
}
