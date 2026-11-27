//! 02-4 판단 입력 조립 자리. 전송 오류를 판단 입력으로 옮기고 헤더 값을 얹어 판단에 넘김.

use std::time::SystemTime;

use crate::llm::TransportError;
use crate::llm::client::failure_map::map_failure;
use crate::llm::retry::RandomSource;
use crate::llm::retry::RetryDecision;
use crate::llm::retry::RetrySettings;
use crate::llm::retry::decide_retry;

/// `open` 단계 실패를 02-4 판단으로 옮김. 판단 입력이 아니면 `None`.
pub(super) fn open_decision<Rng>(
    error: &TransportError,
    attempt: u64,
    settings: &RetrySettings,
    rng: &mut Rng,
) -> Option<RetryDecision>
where
    Rng: RandomSource,
{
    let input = map_failure(error)?;
    let header = retry_after_of(error);
    let decision = decide_retry(
        input,
        header.as_deref(),
        attempt,
        settings,
        SystemTime::now(),
        rng,
    );
    Some(decision)
}

/// 상태 오류가 담은 가린 `Retry-After` 값. 그 밖의 오류는 값 없음.
fn retry_after_of(error: &TransportError) -> Option<String> {
    let TransportError::Status { retry_after, .. } = error else {
        return None;
    };
    retry_after.as_ref().map(|value| value.expose().to_owned())
}
