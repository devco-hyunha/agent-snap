//! execute 브리드가 엔진 스레드에 넘기는 재료 정본.
//!
//! 엔진 확인 판은 `'static`보다 오래 사는 값만 받으므로 등록 목록·실행 판·기록 창구를 `Arc` 소유본 하나로 묶은 타입을 한 곳에 둠 → 엔진 구동·JS 함수 연결이 같은 정의를 사용. 등록 순서·승인자·취소 토큰이 갈라지지 않게 만들기 창구도 한 곳에 둠.

use std::sync::Arc;

use crate::agent::record::registry::ToolRegistry;
use crate::tools::tool::ToolContext;

use super::script_log::ScriptLog;

/// 엔진 스레드가 하나씩 받아 쓰는 재료.
pub(in crate::tools) struct EngineBridge {
    /// 엔진이 도구를 찾는 등록 목록 복사본.
    pub(in crate::tools) registry: Arc<ToolRegistry>,
    /// 엔진이 도구 호출에 쓰는 실행 판 복제본.
    pub(in crate::tools) context: Arc<ToolContext>,
    /// 엔진이 남기는 호출 기록 창구 복제본.
    pub(in crate::tools) log: Arc<ScriptLog>,
}

impl EngineBridge {
    /// 실행 판·기록 창구와 함께 등록 목록 복사본을 만듦.
    ///
    /// 엔진이 전용 스레드마다 자기 등록 목록을 쓰므로 호출마다 새로 만드며, 조립은 `ToolRegistry::with_agent_tools` 한 정의만 따라감 → 이름·순서 중복 없음.
    pub(in crate::tools) fn new(context: Arc<ToolContext>, log: Arc<ScriptLog>) -> Self {
        Self {
            registry: Arc::new(ToolRegistry::with_agent_tools()),
            context,
            log,
        }
    }
}
