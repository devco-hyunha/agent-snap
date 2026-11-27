//! 재시도 열기 루프 자리. 시도를 열고 판단이 재시도를 주면 대기를 채운 뒤 같은 요청을 다시 연다.
//!
//! 헤더를 받은 뒤 실패는 판단에 넣지 않고 그대로 돌려준다. 판단이 `None`인 경로가 그 경계.

use crate::llm::ChatRequest;
use crate::llm::TransportError;
use crate::llm::client::backoff::wait_backoff;
use crate::llm::client::decision::open_decision;
use crate::llm::client::traits::LlmClient;
use crate::llm::retry::RandomSource;
use crate::llm::retry::RetryDecision;
use crate::llm::retry::RetrySettings;
use tokio_util::sync::CancellationToken;

/// 첫 요청의 시도 번호.
const FIRST_ATTEMPT: u64 = 1;

/// 시도를 열고, 판단이 재시도를 주면 대기를 채운 뒤 같은 요청을 다시 연다.
///
/// 첫 요청을 시도 1로 센다. 포기가 정해지면 안쪽 오류를 그대로 돌려준다.
pub(super) async fn open_with_retry<Inner, Rng>(
    inner: Inner,
    settings: RetrySettings,
    mut rng: Rng,
    request: ChatRequest,
    cancel: CancellationToken,
) -> Result<Inner::Receiver, TransportError>
where
    Inner: LlmClient,
    Inner::Receiver: Send,
    Rng: RandomSource,
{
    let mut attempt = FIRST_ATTEMPT;
    loop {
        match inner.open(&request, cancel.clone()).await {
            Ok(receiver) => return Ok(receiver),
            Err(error) => {
                let Some(decision) = open_decision(&error, attempt, &settings, &mut rng) else {
                    return Err(error);
                };
                match decision {
                    RetryDecision::Wait { wait } => {
                        attempt = next_attempt(attempt);
                        wait_backoff(wait, &cancel).await?;
                    }
                    RetryDecision::GiveUp { .. } => return Err(error),
                }
            }
        }
    }
}

/// 시도 번호를 한 칸 올림.
const fn next_attempt(attempt: u64) -> u64 {
    attempt + 1
}
