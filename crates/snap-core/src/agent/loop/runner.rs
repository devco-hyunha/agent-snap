//! 루프 골격: 왕복을 반복하고 도구 호출을 처리하는 에이전트 루프
//!
//! 같은 요청 재시도는 클라이언트 한 층에만 있으므로 왕복이 끝나면 도구만 처리

use crate::llm::{CancellationToken, ChatRequest, LlmClient, ToolCall};

use crate::agent::executor::run_tool_calls;
use crate::agent::fallback::connector::FallbackPoint;
use crate::agent::fallback::tool_call_form::{ToolCallFormat, detects_switch};
use crate::agent::fallback::unsupported::{transport_failure, unsupported_point};
use crate::agent::prompt::assemble::tool_result_messages;
use crate::agent::schema::types::{LoopEnd, LoopError, LoopOutcome};
use crate::agent::settings::config::LoopConfig;
use crate::agent::text::turn::{TextTurn, run_text_turn};

use super::loop_request::first_request;
use super::loop_state::LoopState;
use super::loop_stream::{drain_stream, loop_result};
use super::openai_turn::{TextStep, append_openai_roundtrip, apply_text_only_turn};

/// 도구 왕복 루프를 돌리는 자리. 클라이언트와 취소 토큰만 있으면 어디서도 동작
pub struct AgentLoop<C: LlmClient> {
    client: C,
    cancel: CancellationToken,
}

impl<C: LlmClient> AgentLoop<C> {
    /// 클라이언트와 취소 토큰으로 루프 생성
    pub const fn new(client: C, cancel: CancellationToken) -> Self {
        Self { client, cancel }
    }

    /// 도구 왕복 루프 실행(1안 OpenAI 경로)
    ///
    /// 같은 요청 전송 재시도는 클라이언트 한 층(`stream_chat`)이 담당
    /// # 오류
    /// - 전송 실패 시 그 이유를 그대로 알림
    /// - 내용도 도구 호출도 없는 응답은 폴백 연결 지점 오류로 알림
    pub async fn run(&self, config: &LoopConfig) -> LoopOutcome {
        match first_request(config) {
            Ok(request) => self.run_turns(request, config).await,
            Err(error) => LoopOutcome::Failed(error),
        }
    }

    /// 왕복을 반복하며 messages를 키우고, 같은 요청을 다시 보내지 않음
    ///
    /// 판단 갈래는 `LoopState` 메서드와 `openai_turn`의 작은 함수들이 담당
    async fn run_turns(&self, first: ChatRequest, config: &LoopConfig) -> LoopOutcome {
        let mut state = LoopState::start(first, config.tool_call_form);
        // 1안 왕복에서 도구 호출 없이 이어진 텍스트 횟수(2안 전환 확정 계산)
        let mut text_turns: u32 = 0;
        // 2안에서 JSON을 못 찾았을 때 넘기는 짧은 오류를 한 번만 보내는 표식
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
                    // 4x 전환은 `auto`와 1안 왕복에서만 함(선언 우선)
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
            // 2안으로 전환한 왕복은 글자에서 JSON을 복원하는 경로로 진입
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
                    // 내용도 호출도 없는 응답만 미지원 판단으로 넘김
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
            // 취소 연결: 루프가 받은 취소 토큰을 도구 실행 판에 붙임
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
