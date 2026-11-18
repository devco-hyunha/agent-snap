//! 2안 텍스트 프로토콜 왕복 한 바퀴를 조립하는 자리
//!
//! 재료는 `recover`·`validate`·`record`·`messages`·`notice`의 기존 파일에서 가져옴

use crate::llm::ChatRequest;

use super::messages::{text_assistant_message, text_result_message};
use super::notice::{missing_json_notice, multi_call_notice, problem_notice};
use super::record::{notice_record, run_text_call};
use super::recover::{RecoverOutcome, recover_text_call};
use super::validate::validate_found;
use crate::agent::config::LoopConfig;
use crate::agent::types::ToolCallRecord;

/// 2안 왕복 한 바퀴가 다음 왕복을 이어갈지 알리는 표식
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTurn {
    /// 도구 호출을 복원해 실행. 다음 왕복을 이어감
    Called,
    /// 짧은 오류를 넘김. 같은 왕복을 한 번 더 이어감(오류 안내 1회 규칙)
    Again,
    /// 왕복을 끝냄. 2안에서 도구를 쓰지 않고 답을 낸 자리
    Stop,
}

/// 2안 왕복 한 바퀴: assistant 글자에서 JSON 호출을 복원해 스키마 검증 후 실행하고, 결과를
/// `role: user`의 `도구 결과 [name]` 글자로 messages에 붙임
///
/// 오류 안내는 왕복 한 바퀴에 한 번만 넣음(1회 규칙)
pub async fn run_text_turn(
    request: &mut ChatRequest,
    // 루프 설정의 등록 목록·실행 판·잘림 상한을 그대로 쓰기 위해 통째로 넘김
    config: &LoopConfig,
    content: &str,
    records: &mut Vec<ToolCallRecord>,
    notice_sent: &mut bool,
) -> TextTurn {
    let max_chars = config.context.tool_result_max_chars;
    let RecoverOutcome::Found { call, extra_calls } = recover_text_call(content) else {
        // JSON을 찾지 못한 자리. 안내는 왕복 한 바퀴만 허용.
        // 이미 안내를 넘긴 자리라면 모델이 도구를 쓰지 않고 답을 낸 것으로 보고 끝냄
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
    // 스키마 검증은 도구 실행 전에 함. 걸러진 표기는 `notice`가 정본 글자로 만듦
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
