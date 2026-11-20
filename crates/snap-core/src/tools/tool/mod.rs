//! `Tool` 트레이트 골격. 필드·메서드 이름은 고정
//!
//! 실행 판은 `context`에, 결과 타입과 미래 상자는 `outcome`에 따로 둠

mod context;
mod outcome;

pub use context::ToolContext;
pub use outcome::{ToolCallFuture, ToolOutcome};

/// 에이전트가 쓰는 도구가 지켜야 할 트레이트
pub trait Tool {
    /// 등록 이름. 루프가 `tool_calls`의 이름과 같은지 비교
    fn name(&self) -> &str;

    /// 모델에게 보이는 한 줄 설명
    fn description(&self) -> &str;

    /// 인자 JSON Schema 객체
    fn parameters_schema(&self) -> serde_json::Value;

    /// 쓰기·명령을 하는 도구인지 알림. 승인 구분 기준
    fn executes_writes(&self) -> bool {
        false
    }

    /// 인자 JSON 원문을 받아 도구를 실행
    ///
    /// 등록 목록이 `Box<dyn Tool>`로 트레이트 객체를 쓰려면 미래 타입을 상자로 고정해야 함
    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a>;
}
