//! 안쪽 클라이언트를 감싸는 재시도 래퍼 자리. 래퍼 타입·트레이트 구현 한 책임만 담음.
//!
//! 열기 루프·대기 경합·판단 조립은 각각 `run`·`backoff`·`decision`이 맡는다.

use std::future::Future;
use std::pin::Pin;

use tokio_util::sync::CancellationToken;

use crate::llm::ChatRequest;
use crate::llm::TransportError;
use crate::llm::client::run::open_with_retry;
use crate::llm::client::traits::LlmClient;
use crate::llm::retry::RandomSource;
use crate::llm::retry::RetrySettings;

/// 안쪽 클라이언트를 감싸는 재시도 래퍼. 에이전트 루프는 안쪽과 같은 트레이트로 본다.
///
/// 난수원은 래퍼가 만든 값만 쓰고, 시도는 한 번만 연다.
#[derive(Clone, Debug)]
pub struct RetryClient<Inner, Rng> {
    /// 재시도 전에 여는 안쪽 클라이언트.
    inner: Inner,

    /// 02-4 판단이 받는 재시도 설정.
    settings: RetrySettings,

    /// 백오프가 뽑는 난수원.
    rng: Rng,
}

impl<Inner, Rng> RetryClient<Inner, Rng> {
    /// 안쪽 클라이언트와 판단 설정, 래퍼가 만든 난수원으로 재시도 래퍼 만들기.
    pub const fn new(inner: Inner, settings: RetrySettings, rng: Rng) -> Self {
        Self {
            inner,
            settings,
            rng,
        }
    }
}

impl<Inner, Rng> LlmClient for RetryClient<Inner, Rng>
where
    Inner: LlmClient + Clone + Send + Sync + 'static,
    Inner::Receiver: Send,
    for<'a> Inner::Open<'a>: Send,
    Rng: RandomSource + Clone + Send + 'static,
{
    type Open<'a> =
        Pin<Box<dyn Future<Output = Result<Inner::Receiver, TransportError>> + Send + 'a>>;

    type Receiver = Inner::Receiver;

    fn open(&self, request: &ChatRequest, cancel: CancellationToken) -> Self::Open<'_> {
        let inner = self.inner.clone();
        let rng = self.rng.clone();
        let request = request.clone();
        Box::pin(open_with_retry(inner, self.settings, rng, request, cancel))
    }
}
