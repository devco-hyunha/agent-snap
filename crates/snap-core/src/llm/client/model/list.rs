//! 모델 목록 조회 자리. 재시도 루프·전송·형식 파싱을 잇는 한 책임만 담음.
//!
//! `data` 배열 규칙은 `format`이, 전송과 상태 구분은 `open`이, 재시도 판단은 `run`이 맡는다.
//! reqwest 타입은 밖에 내지 않는다.

use crate::config::secret_string::SecretString;
use crate::llm::client::decision::open_decision;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::model::error::ModelsError;
use crate::llm::client::model::format::collect_ids;
use crate::llm::client::model::open::open_models_json;
use crate::llm::client::run::retry_loop;
use crate::llm::client::timings::TransportTimings;
use crate::llm::retry::RandomSource;
use crate::llm::retry::RetryDecision;
use crate::llm::retry::RetrySettings;
use tokio_util::sync::CancellationToken;

/// 엔드포인트 주소와 키로 모델 목록을 읽어서 `data` 배열의 문자열 `id` 목록만 돌려줌.
///
/// 서버 HTTP 오류는 02-5 전송 오류·가리기를 거치고, 02-6 재시도 규칙으로 후보만 재시도한다.
pub async fn list_models<Rng>(
    base_url: &str,
    key: Option<&SecretString>,
    timings: &TransportTimings,
    settings: &RetrySettings,
    rng: Rng,
    cancel: &CancellationToken,
) -> Result<Vec<String>, ModelsError>
where
    Rng: RandomSource,
{
    let attempt = ModelsAttempt {
        base_url: base_url.to_owned(),
        key: key.cloned(),
        timings: *timings,
        cancel: cancel.clone(),
    };
    let open = || attempt.open();
    retry_loop(open, models_decision, *settings, rng, cancel).await
}

/// 재시도 루프가 다시 부르는 시도 한 번의 값. 소유한 값만 담음.
#[derive(Clone)]
struct ModelsAttempt {
    base_url: String,
    key: Option<SecretString>,
    timings: TransportTimings,
    cancel: CancellationToken,
}

impl ModelsAttempt {
    /// 시도 하나. 전송을 열고 응답 JSON에서 `id` 목록만 모음.
    async fn open(&self) -> Result<Vec<String>, ModelsError> {
        let Self {
            base_url,
            key,
            timings,
            cancel,
        } = self;
        let key = key.as_ref();
        let secrets = KnownSecrets::new(key.cloned(), base_url);
        let value = open_models_json(base_url, key, timings, cancel, &secrets).await?;
        Ok(collect_ids(&value)?)
    }
}

/// 모델 목록 실패를 02-4 판단으로 옮김. 02-5 전송 오류만 판단 입력이 되고, 형식 오류는 `None`.
fn models_decision<Rng>(
    error: &ModelsError,
    attempt: u64,
    settings: &RetrySettings,
    rng: &mut Rng,
) -> Option<RetryDecision>
where
    Rng: RandomSource,
{
    let ModelsError::Transport(transport) = error else {
        return None;
    };
    open_decision(transport, attempt, settings, rng)
}
