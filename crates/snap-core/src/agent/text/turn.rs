//! 2안 텍스트 프로토콜 왕복 한 바퀴를 조립하는 자리입니다(03-4 spec AC 3~AC 5, D50 결정 1).
//!
//! `loop/runner.rs`가 1안 왕복 골격과 전환 판단만 남기고 이 파일로 옮긴 분할입니다(리뷰 SRP 지적).
//! 복원·검증·기록·messages 줄은 각각 `recover`·`validate`·`record`·
//! `messages`·`notice`의 책임이고, 이 파일은 그 재료로 왕복 한 바퀴를 잇는 조립만 맡습니다
//! (dev-principles 2절 SRP·5절 관심사 분리).

use crate::llm::ChatRequest;

use super::messages::{text_assistant_message, text_result_message};
use super::notice::{missing_json_notice, multi_call_notice, problem_notice};
use super::record::{notice_record, run_text_call};
use super::recover::{RecoverOutcome, recover_text_call};
use super::validate::validate_found;
use crate::agent::config::LoopConfig;
use crate::agent::types::ToolCallRecord;

/// 2안 왕복 한 바퀴가 다음 왕복을 이어갈지 알리는 표식입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTurn {
    /// 도구 호출을 복원해 실행했습니다. 다음 왕복을 이어갑니다.
    Called,
    /// 짧은 오류를 넘겼습니다. 같은 왕복을 한 번 더 이어갑니다(오류 안내 1회 규칙).
    Again,
    /// 왕복을 끝냅니다. 2안에서 도구를 쓰지 않고 답을 낸 자리입니다.
    Stop,
}

/// 2안 왕복 한 바퀴: assistant 글자에서 JSON 호출을 복원해 스키마 검증 후 실행하고, 결과를
/// `role: user`의 `도구 결과 [name]` 글자로 messages에 붙입니다(03-4 spec AC 3~AC 5).
///
/// JSON을 찾지 못하면 짧은 오류 한 줄을 같은 자리에 넣어 한 번만 되알립니다(12번 9절 (a) 권고,
/// 오류 안내 1회까지만 허용). 스키마에 안 맞는 인자는 도구 실행 전에 짧은 오류로 돌아갑니다(AC 4).
/// 여러 호출(2개 이상)이 한 번에 오면 첫 호출만 실행하고 나머지는 안내로 넘깁니다(범위 Out).
pub async fn run_text_turn(
    request: &mut ChatRequest,
    // 루프 설정의 등록 목록·실행 판·잘림 상한을 그대로 쓰기 위해 통째로 넘깁니다.
    config: &LoopConfig,
    content: &str,
    records: &mut Vec<ToolCallRecord>,
    notice_sent: &mut bool,
) -> TextTurn {
    let max_chars = config.context.tool_result_max_chars;
    let RecoverOutcome::Found { call, extra_calls } = recover_text_call(content) else {
        // JSON을 찾지 못한 자리입니다. 안내는 왕복 한 바퀴만 허용합니다(12번 9절 (a), AC 3).
        // 이미 안내를 넘긴 자리라면 모델이 도구를 쓰지 않고 답을 낸 것으로 보고 끝냅니다.
        if *notice_sent {
            return TextTurn::Stop;
        }
        *notice_sent = true;
        let notice = notice_record("json", content, missing_json_notice(), max_chars);
        request.messages.push(text_assistant_message(content));
        request
            .messages
            .push(text_result_message("json", &notice.result, max_chars));
        records.push(notice);
        return TextTurn::Again;
    };
    *notice_sent = false;
    // 스키마 검증은 도구 실행 전에 합니다(spec AC 4). 걸러진 표기는 `notice`가 정본 글자로 만듭니다.
    let record = match validate_found(&config.registry, &call) {
        Some(problem) => notice_record(
            &call.name,
            &call.arguments,
            &problem_notice(&call.name, &problem, &config.registry),
            max_chars,
        ),
        None => run_text_call(&config.registry, &config.context, &call).await,
    };
    let extras: Vec<ToolCallRecord> = extra_calls
        .iter()
        .map(|extra| {
            notice_record(
                &extra.name,
                &extra.arguments,
                multi_call_notice(),
                max_chars,
            )
        })
        .collect();
    let notices = std::iter::once(record.clone())
        .chain(extras.iter().cloned())
        .map(|record| text_result_message(&record.tool_name, &record.result, max_chars))
        .collect::<Vec<_>>();
    request.messages.push(text_assistant_message(content));
    request.messages.extend(notices);
    records.push(record);
    records.extend(extras);
    TextTurn::Called
}
