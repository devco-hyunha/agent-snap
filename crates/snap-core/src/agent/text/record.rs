//! 2안 텍스트 프로토콜의 도구 실행 기록을 채우는 자리입니다(03-4 spec AC 5).
//!
//! 1안 실행(`executor.rs`)과 같은 규칙(잘림·시간·형식 칸)으로 기록을 채우되, 호출 형식 칸은
//! 2안으로 채웁니다. 안내 글자는 `notice.rs`, messages 줄은 `messages.rs`가 맡습니다
//! (관심사 분리, dev-principles 5절).

use crate::tools::tool::{ToolContext, ToolOutcome};

use super::notice::unregistered_notice;
use super::recover::RecoveredCall;
use crate::agent::executor::now_unix_secs;
use crate::agent::fallback::ToolCallFormat;
use crate::agent::registry::ToolRegistry;
use crate::agent::truncate::truncate_tool_result;
use crate::agent::types::ToolCallRecord;

/// 2안 왕복에서 도구 호출 한 개를 실행해 03-2 기록 규칙으로 채우는 자리입니다(AC 5).
///
/// 1안 실행(`executor.rs`)과 같은 규칙으로 잘림·시간·형식 칸을 채우되, 호출 형식 칸은 2안으로 채웁니다.
/// 인자 스키마 검증은 `validate`의 `validate_found`가 끝난 뒤 부르므로 여기서 다시 검사하지
/// 않습니다(중복 검증 금지, dev-principles 3절).
pub async fn run_text_call(
    registry: &ToolRegistry,
    context: &ToolContext,
    call: &RecoveredCall,
) -> ToolCallRecord {
    let executed_at = now_unix_secs();
    let outcome = match registry.find(&call.name) {
        Some(tool) => tool.call(&call.arguments, context).await,
        None => ToolOutcome::Done(unregistered_notice(&call.name, registry)),
    };
    fill_record(
        &call.name,
        &call.arguments,
        outcome.text(),
        executed_at,
        context.tool_result_max_chars,
    )
}

/// 2안 안내(여러 호출 뒤쪽·복원 실패)를 모델에 넘길 글자로 남기고 기록으로 채우는 자리입니다.
#[must_use]
pub fn notice_record(
    name: &str,
    arguments: &str,
    notice: &str,
    max_chars: usize,
) -> ToolCallRecord {
    fill_record(name, arguments, notice, now_unix_secs(), max_chars)
}

/// 2안 기록 항목을 채우는 한 곳입니다(같은 필드 조립을 다시 적지 않습니다, DRY).
///
/// 잘림 상한은 1안(`executor.rs`)과 같은 실행 판 값(`ToolContext::tool_result_max_chars`)을
/// 넘겨받습니다. 기본값 상수를 고정으로 쓰지 않아야 설정을 늘린 사용자에게도
/// messages와 기록 잘림이 같은 값으로 맞습니다(AC 5).
fn fill_record(
    name: &str,
    arguments: &str,
    result: &str,
    executed_at: u64,
    max_chars: usize,
) -> ToolCallRecord {
    ToolCallRecord {
        call_id: text_call_id(name, executed_at),
        tool_name: name.to_owned(),
        arguments: arguments.to_owned(),
        result: truncate_tool_result(result, max_chars),
        executed_at_unix_secs: executed_at,
        format: ToolCallFormat::Text,
    }
}

/// 2안 호출 id 글자입니다. 9자리 알파뉴머릭 16진수입니다.
///
/// llama.cpp·mlx-lm은 9자리가 넘는 id를 다음 턴에 재투입하면 템플릿 검증에 걸려 400을 내므로
/// (12번 8절 함정, D50 재볼점) 서버가 준 id를 쓰지 않고 여기서 자체로 만듭니다. 시각과 이름
/// 해시를 섞어 왕복마다 다른 값을 만듭니다.
#[must_use]
pub fn text_call_id(name: &str, executed_at: u64) -> String {
    let mut hash: u32 = 0;
    for ch in name.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(u32::from(ch));
    }
    format!(
        "{:09x}",
        u32::try_from(executed_at).unwrap_or_default() ^ hash
    )
}
