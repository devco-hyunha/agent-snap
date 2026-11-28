//! 재시도 루프 자리. 시도를 열고 판단이 재시도를 주면 대기를 채운 뒤 같은 작업을 다시 연다.
//!
//! 헤더를 받은 뒤 실패는 판단에 넣지 않고 그대로 돌려준다. 판단이 `None`인 경로가 그 경계.
//! 채팅 스트림 열기와 모델 목록 열기가 같은 루프를 쓴다. 판단은 `decide`로 넘긴다.

use std::future::Future;

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
    rng: Rng,
    request: ChatRequest,
    cancel: CancellationToken,
) -> Result<Inner::Receiver, TransportError>
where
    Inner: LlmClient,
    Inner::Receiver: Send,
    Rng: RandomSource,
{
    let open = || inner.open(&request, cancel.clone());
    retry_loop(open, open_decision, settings, rng, &cancel).await
}

/// 작업을 열고, 판단이 재시도를 주면 대기를 채운 뒤 같은 작업을 다시 연다.
///
/// 첫 시도를 시도 1로 센다. 포기가 정해지면 안쪽 오류를 그대로 돌려준다.
/// `decide`가 판단 입력을 만드는 자리까지 맡는다. 판단이 `None`이면 첫 오류를 그대로 낸다.
pub(super) async fn retry_loop<T, E, Open, Fut, Decide, Rng>(
    mut open: Open,
    mut decide: Decide,
    settings: RetrySettings,
    mut rng: Rng,
    cancel: &CancellationToken,
) -> Result<T, E>
where
    Open: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    Decide: FnMut(&E, u64, &RetrySettings, &mut Rng) -> Option<RetryDecision>,
    Rng: RandomSource,
    E: From<TransportError>,
{
    let mut attempt = FIRST_ATTEMPT;
    loop {
        match open().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                let Some(decision) = decide(&error, attempt, &settings, &mut rng) else {
                    return Err(error);
                };
                match decision {
                    RetryDecision::Wait { wait } => {
                        attempt = next_attempt(attempt);
                        wait_backoff(wait, cancel).await?;
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
