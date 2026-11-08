//! 루프 골격: 왕복을 반복하고 도구 호출을 처리하는 에이전트 루프입니다(spec AC 1~7).
//!
//! 같은 요청 재시도는 클라이언트 한 층에만 있습니다(D13, 02-6). 이 파일은 그 층을
//! 그대로 쓰고, 왕복이 끝나면 도구만 처리합니다(중복 재시도 금지).
//! 등록 변환(definitions)·프롬프트 조립·도구 실행·잘림은 각각 definitions·prompt·executor·truncate의 책임입니다(SRP).

use crate::llm::{
    AssembledResponse, CancellationToken, ChatRequest, ChatStream, LlmClient, ToolCall,
};

use super::config::LoopConfig;
use super::definitions::registry_to_definitions;
use super::executor::run_tool_calls;
use super::prompt::{
    EMPTY_AGENTS_MD, assemble_messages, assistant_turn_message, tool_result_messages,
};
use super::types::{LoopEnd, LoopError, LoopOutcome, LoopResult, LoopStartError, ToolCallRecord};

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
        match self.first_request(config) {
            Ok(request) => self.run_turns(request, config).await,
            Err(error) => LoopOutcome::Failed(error),
        }
    }

    /// 첫 요청 골격을 만듭니다. 등록 변환·프롬프트 조립·프로파일 합치기를 한곳에서 묶습니다.
    ///
    /// 등록 이름 중복은 첫 요청을 만들기 전에 막습니다. 같은 이름이면 `find`가 앞의 도구만
    /// 찾아 뒤의 도구가 조용히 가려지므로, 요청을 보내고 나서 막으면 결과가 어긋납니다.
    fn first_request(&self, config: &LoopConfig) -> Result<ChatRequest, LoopError> {
        if let Some((name, _position)) = config.registry.find_duplicate_name() {
            return Err(LoopError::Start(LoopStartError::DuplicateToolName { name }));
        }
        let mut request = config.base.clone();
        request.messages =
            assemble_messages(&config.system_prompt, EMPTY_AGENTS_MD, &config.user_message);
        request.tools = registry_to_definitions(&config.registry);
        if let Some(profile) = &config.profile {
            profile.apply(&mut request).map_err(LoopError::Profile)?;
        }
        Ok(request)
    }

    /// 왕복을 반복합니다. 왕복마다 messages를 키우고, 같은 요청을 다시 보내지 않습니다(D13).
    async fn run_turns(&self, first: ChatRequest, config: &LoopConfig) -> LoopOutcome {
        let mut request = first;
        let mut records: Vec<ToolCallRecord> = Vec::new();
        let mut turns: u32 = 0;
        loop {
            let stream = match self
                .client
                .stream_chat(request.clone(), self.cancel.clone())
                .await
            {
                Ok(stream) => stream,
                Err(error) => return LoopOutcome::Failed(LoopError::transport(&error)),
            };
            turns += 1;
            let assembled = drain_stream(stream).await;
            let content = assembled.content.clone();
            let calls: Vec<ToolCall> = assembled.tool_calls.values().cloned().collect();
            if calls.is_empty() {
                if content.is_empty() {
                    return LoopOutcome::Failed(LoopError::unsupported());
                }
                return LoopOutcome::Finished(LoopEnd::Stop {
                    result: loop_result(content, records, turns),
                });
            }
            let run = run_tool_calls(&config.registry, &config.context, &calls).await;
            let tool_messages = tool_result_messages(&run.records);
            let (mut turn_records, rejection) = run.into_parts();
            records.append(&mut turn_records);
            let Some(stopped_record) = rejection else {
                if turns >= config.max_turns {
                    return LoopOutcome::Finished(LoopEnd::TurnLimit {
                        partial: loop_result(content, records, turns),
                    });
                }
                request.messages = request
                    .messages
                    .iter()
                    .cloned()
                    .chain(std::iter::once(assistant_turn_message(&content, &calls)))
                    .chain(tool_messages)
                    .collect();
                continue;
            };
            return LoopOutcome::Finished(LoopEnd::ApprovalRejected {
                stopped_record,
                partial: loop_result(content, records, turns),
            });
        }
    }
}

/// 스트림을 끝까지 읽고 합친 응답을 받아옵니다.
async fn drain_stream(mut stream: ChatStream) -> AssembledResponse {
    while let Some(event) = stream.next_event().await {
        if event.is_err() {
            break;
        }
    }
    stream.assembled().clone()
}

/// 왕복 한 바퀴의 결과를 채우는 한 곳입니다(같은 조립을 다시 적지 않습니다, DRY).
fn loop_result(final_text: String, tool_calls: Vec<ToolCallRecord>, turns: u32) -> LoopResult {
    LoopResult {
        final_text,
        tool_calls,
        turns,
    }
}
