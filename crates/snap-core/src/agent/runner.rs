//! 루프 골격: 왕복을 반복하고 도구 호출을 처리하는 에이전트 루프입니다(spec AC 1~7).
//!
//! 같은 요청 재시도는 클라이언트 한 층에만 있습니다(D13, 02-6). 이 파일은 그 층을
//! 그대로 쓰고, 왕복이 끝나면 도구만 처리합니다(중복 재시도 금지).
//! 상태 값(`loop_state`)·1안 왕복 판단(`openai_turn`)·스트림 드레인과 결과 채움(`loop_stream`)·
//! 2안 조립(text::protocol)·2안 왕복 한 바퀴(text::turn)·첫 요청 조립(`loop_request`)은
//! 각각 그 파일의 책임이고, 등록 변환(definitions)·프롬프트 조립(prompt)·도구 실행(executor)·
//! 잘림(truncate)은 그대로입니다(D50 결정 1, 03-6 리뷰 SRP 지적).

use crate::llm::{CancellationToken, ChatRequest, LlmClient, ToolCall};

use super::config::LoopConfig;
use super::executor::run_tool_calls;
use super::fallback::FallbackPoint;
use super::loop_request::first_request;
use super::loop_state::LoopState;
use super::loop_stream::{drain_stream, loop_result};
use super::openai_turn::{TextStep, append_openai_roundtrip, apply_text_only_turn};
use super::prompt::tool_result_messages;
use super::text::turn::{TextTurn, run_text_turn};
use super::tool_call_form::{ToolCallFormat, detects_switch};
use super::types::{LoopEnd, LoopError, LoopOutcome};
use super::unsupported::{transport_failure, unsupported_point};

/// 도구 왕복 루프를 돌리는 자리입니다. 클라이언트와 취소 토큰만 있으면 어디서도 돕니다.
pub struct AgentLoop<C: LlmClient> {
    client: C,
    cancel: CancellationToken,
}

impl<C: LlmClient> AgentLoop<C> {
    /// 클라이언트와 취소 토큰으로 루프를 만듭니다.
    pub const fn new(client: C, cancel: CancellationToken) -> Self {
        Self { client, cancel }
    }

    /// 도구 왕복 루프를 돌립니다(1안 OpenAI 경로, AC 1).
    ///
    /// # 흐름
    /// 프롬프트 앞머리 조립 → 왕복 → `tool_calls` 처리 → 결과를 messages에 붙임 → `stop`까지 반복.
    /// 같은 요청 전송 재시도는 02-6 한 층(`stream_chat`)이 하고, 이 파일은 같은 요청을 다시 보내지 않습니다(D13).
    ///
    /// # 오류
    /// - 전송이 실패하면 그 이유를 그대로 알립니다.
    /// - 내용도 도구 호출도 없는 응답은 03-4 폴백 연결 지점으로 넘기는 오류로 알립니다(AC 4).
    pub async fn run(&self, config: &LoopConfig) -> LoopOutcome {
        match first_request(config) {
            Ok(request) => self.run_turns(request, config).await,
            Err(error) => LoopOutcome::Failed(error),
        }
    }

    /// 왕복을 반복합니다. 왕복마다 messages를 키우고, 같은 요청을 다시 보내지 않습니다(D13).
    ///
    /// 왕복의 갈래는 4갈래이고, 각 판단은 `LoopState`의 메서드와 `openai_turn`의 작은 함수들이 맡습니다:
    /// 전송 실패 4x → `LoopState::switch_to_text`, 호출 없는 텍스트 → `apply_text_only_turn`,
    /// 도구 호출 결과 이어쓰기 → `append_openai_roundtrip`.
    async fn run_turns(&self, first: ChatRequest, config: &LoopConfig) -> LoopOutcome {
        let mut state = LoopState::start(first, config.tool_call_form);
        // 1안 왕복에서 도구 호출 없이 이어진 텍스트 횟수(2안 전환 확정 계산, D50 결정 2).
        let mut text_turns: u32 = 0;
        // 2안에서 JSON을 못 찾았을 때 넘기는 짧은 오류를 한 번만 보내는 표식입니다(AC 3, 12번 9절 (a) 권고).
        let mut text_notice_sent = false;
        let detects = detects_switch(config.tool_call_form);
        let mut turns: u32 = 0;
        loop {
            let stream = match self
                .client
                .stream_chat(state.request.clone(), self.cancel.clone())
                .await
            {
                Ok(stream) => stream,
                Err(error) => {
                    let failure = transport_failure(&error);
                    // 4x 전환은 `auto`와 1안 왕복에서만 합니다(D50 결정 3, 선언 우선).
                    if !(detects && matches!(state.format, ToolCallFormat::OpenAi)) {
                        return LoopOutcome::Failed(failure);
                    }
                    let Some(point) = unsupported_point(&failure) else {
                        return LoopOutcome::Failed(failure);
                    };
                    state.switch_to_text(&config.registry, point);
                    continue;
                }
            };
            turns += 1;
            let assembled = drain_stream(stream).await;
            let content = assembled.content.clone();
            let calls: Vec<ToolCall> = assembled.tool_calls.values().cloned().collect();
            // 2안으로 전환한 왕복은 글자에서 JSON을 복원하는 경로로 들어갑니다(할 일 3a).
            if matches!(state.format, ToolCallFormat::Text) {
                let outcome = run_text_turn(
                    &mut state.request,
                    config,
                    &content,
                    &mut state.records,
                    &mut text_notice_sent,
                )
                .await;
                if matches!(outcome, TextTurn::Stop) {
                    return LoopOutcome::Finished(LoopEnd::Stop {
                        result: loop_result(content, state.records, turns, state.switch_point),
                    });
                }
                if turns >= config.max_turns {
                    return LoopOutcome::Finished(LoopEnd::TurnLimit {
                        partial: loop_result(content, state.records, turns, state.switch_point),
                    });
                }
                continue;
            }
            if calls.is_empty() {
                if content.is_empty() {
                    // 내용도 호출도 없는 응답만 미지원 판단으로 넘깁니다(AC 4).
                    return LoopOutcome::Failed(LoopError::unsupported(
                        FallbackPoint::NoToolCallText,
                    ));
                }
                match apply_text_only_turn(
                    &mut state,
                    &config.registry,
                    &content,
                    &calls,
                    turns,
                    &mut text_turns,
                    detects,
                ) {
                    TextStep::Switch | TextStep::KeepCandidate => continue,
                    TextStep::DeclaredStop => {
                        return LoopOutcome::Finished(LoopEnd::Stop {
                            result: loop_result(content, state.records, turns, state.switch_point),
                        });
                    }
                }
            }
            // 03-6 cancel 연결(spec AC 4): 루프가 자기는 받은 02 토큰을 도구 실행 판에 붙입니다.
            config.context.attach_cancel(&self.cancel);
            let run = run_tool_calls(&config.registry, &config.context, &calls).await;
            let tool_messages = tool_result_messages(&run.records);
            let (mut turn_records, rejection) = run.into_parts();
            state.records.append(&mut turn_records);
            let Some(stopped_record) = rejection else {
                if turns >= config.max_turns {
                    return LoopOutcome::Finished(LoopEnd::TurnLimit {
                        partial: loop_result(content, state.records, turns, state.switch_point),
                    });
                }
                text_turns = 0;
                append_openai_roundtrip(&mut state, &content, &calls, tool_messages);
                continue;
            };
            return LoopOutcome::Finished(LoopEnd::ApprovalRejected {
                stopped_record,
                partial: loop_result(content, state.records, turns, state.switch_point),
            });
        }
    }
}
