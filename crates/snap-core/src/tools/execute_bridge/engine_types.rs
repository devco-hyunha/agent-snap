//! 03-8 execute 브리드가 엔진 스레드에 넘기는 재료 정본입니다(AC 1·AC 6).
//!
//! 결론: rquickjs 엔진 확인 판은 `'static`보다 오래 사는 값만 받습니다(03-8 T2 lifetime 보정).
//! 그래서 엔진이 쓰는 등록 목록·실행 판·기록 창구를 소유본(`Arc`) 하나로 묶은 타입을 이 파일 한 곳에 두고,
//! 엔진 구동 창구(`engine_drive.rs`)와 JS 함수 연결 창구(`js_bindings.rs`)가 같은 정의를 씁니다(dev-principles 3절 DRY).
//! 등록 목록과 실행 판을 복사할 때 등록 순서·승인자·취소 토큰이 갈라지지 않게, 만들기 창구도 이 한 곳에 둡니다.

use std::sync::Arc;

use crate::agent::record::registry::ToolRegistry;
use crate::tools::tool::ToolContext;

use super::script_log::ScriptLog;

/// 엔진 스레드가 하나씩 받아 쓰는 재료입니다.
///
/// 등록 목록은 엔진이 도구 이름으로 찾아 부르려는 복사본이고, 실행 판은 03-3이 만든 값을
/// 엔진 스레드가 같은 상태로 빌려 쓰는 복제본이며, 기록 창구는 호출 기록을 엔진 바깥에
/// 쌓는 공유 창구입니다(AC 3).
pub(in crate::tools) struct EngineBridge {
    /// 엔진이 도구를 찾을 등록 목록 복사본입니다.
    pub(in crate::tools) registry: Arc<ToolRegistry>,
    /// 엔진이 도구 호출에 쓰는 실행 판 복제본입니다.
    pub(in crate::tools) context: Arc<ToolContext>,
    /// 엔진이 남기는 03-8 호출 기록 창구 복제본입니다.
    pub(in crate::tools) log: Arc<ScriptLog>,
}

impl EngineBridge {
    /// 실행 판·기록 창구와 함께 등록 목록 복사본을 만들어 묶습니다.
    ///
    /// 등록 목록을 만드는 곳은 이 한 자리입니다. 엔진이 전용 스레드마다 자기 등록 목록을 쓰므로 호출마다 새로 만듭니다.
    /// 목록을 조립하는 코드는 `agent::record::registry::ToolRegistry::with_agent_tools`(registry.rs 한 정의)만 쓰므로
    /// 이름·순서가 갈라지지 않습니다(DRY, dev-principles 3절).
    pub(in crate::tools) fn new(context: Arc<ToolContext>, log: Arc<ScriptLog>) -> Self {
        Self {
            registry: Arc::new(ToolRegistry::with_agent_tools()),
            context,
            log,
        }
    }
}
