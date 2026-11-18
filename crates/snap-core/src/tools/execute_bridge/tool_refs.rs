//! execute 브리지: JS가 도구를 부르는 창구.
//!
//! 1차 허용 목록 읽기 4개는 승인 없이 부르고, 쓰기·명령 이름은 승인을 호출마다 받은 뒤 부름. 그 밖의 이름은 승인 전에 차단.
//! 엔진 확인 판은 `'static`보다 오래 사는 값을 요구하므로 등록 목록·실행 판·기록 창구를 `Arc` 소유본으로 가짐. 승인 뒤 호출 순서는 `run_approved_call` 정본을 그대로 사용.

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

/// JS가 도구를 부르는 창구.
///
/// 엔진이 오래 쓰므로 등록 목록·실행 판·기록 창구를 `Arc` 소유본으로 가짐.
#[derive(Clone)]
pub(in crate::tools) struct ToolRefs {
    registry: Arc<ToolRegistry>,
    context: Arc<ToolContext>,
    log: Arc<ScriptLog>,
}

impl ToolRefs {
    /// 창구를 등록 목록·실행 판·기록 창구 복제본과 함께 만듦.
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

    /// 코드가 이름으로 도구를 불러 결과를 글자로 받아옴.
    ///
    /// 읽기 4개는 승인 없이 부르고, 승인 대상 이름은 승인을 호출마다 받은 뒤 부르며, 그 밖의 이름은 승인 전에 차단. 기록은 결과를 받은 뒤에 매번 쌓음.
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

    /// 도구 구현을 그대로 부르는 자리. 게이트를 같은 자리에 둠.
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
