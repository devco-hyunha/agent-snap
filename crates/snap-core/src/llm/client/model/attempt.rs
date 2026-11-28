//! 모델 목록 시도 타입 자리. 재시도 루프가 다시 여는 시도 값만 담음.
//!
//! 전송 열기와 `id` 모으기는 각각 `open`·`format`이 맡음.

use crate::config::secret_string::SecretString;
use crate::llm::client::known_secrets::KnownSecrets;
use crate::llm::client::model::error::ModelsError;
use crate::llm::client::model::format::collect_ids;
use crate::llm::client::model::open::open_models_json;
use crate::llm::client::timings::TransportTimings;
use tokio_util::sync::CancellationToken;

/// 재시도 루프가 다시 부르는 시도 한 번의 값. 소유한 값만 담음.
#[derive(Clone)]
pub(super) struct ModelsAttempt {
    pub(super) base_url: String,
    pub(super) key: Option<SecretString>,
    pub(super) timings: TransportTimings,
    pub(super) cancel: CancellationToken,
}

impl ModelsAttempt {
    /// 시도 하나. 전송을 열고 응답 JSON에서 `id` 목록만 모음.
    pub(super) async fn open(&self) -> Result<Vec<String>, ModelsError> {
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
