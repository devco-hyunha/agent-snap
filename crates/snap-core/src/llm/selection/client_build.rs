//! 선택 키 클라이언트 조립. 엔드포인트 정보와 선택한 모델 값을 가짐.
//!
//! 값 채우기·스트림 재사용은 `selection/apply.rs`. HTTP 조립은 `OpenAiCompatClient::new` 재사용.

use std::collections::BTreeMap;
use std::future::Future;

use crate::config::{ExtraValue, Reasoning, Sampling};
use crate::llm::client::{CancellationToken, TransportError};
use crate::llm::transport::OpenAiCompatClient;

use super::{SelectError, SelectedModel};

/// 선택 키가 가리키는 엔드포인트로만 요청하는 클라이언트.
///
/// 본문에는 선택한 모델의 서버용 id·샘플링·`reasoning`·`extra`가 들어감.
pub struct SelectedClient {
    /// 선택 키 엔드포인트 주소만 담은 전송 클라이언트.
    pub(super) inner: OpenAiCompatClient,
    /// 서버에 보낼 모델 id.
    pub(super) model_id: String,
    /// 모델별 값이 루트 기본보다 앞선 샘플링.
    pub(super) sampling: Sampling,
    /// 생각 옵션. 없으면 요청에 더하지 않음.
    pub(super) reasoning: Option<Reasoning>,
    /// 본문에 합칠 추가 값.
    pub(super) extra: BTreeMap<String, ExtraValue>,
}

impl SelectedClient {
    /// 선택 키 준비 값(`SelectedModel`)에서 클라이언트를 만듦.
    ///
    /// 엔드포인트 정보와 시간 설정은 `OpenAiCompatClient::new`에 그대로 넘김.
    pub(super) fn from_selected(selected: &SelectedModel) -> Result<Self, SelectError> {
        let inner = OpenAiCompatClient::new(
            selected.base_url.clone(),
            selected.kind,
            selected.api_key.clone(),
            selected.http.clone(),
        )?;
        Ok(Self {
            inner,
            model_id: selected.model_id.clone(),
            sampling: selected.sampling,
            reasoning: selected.reasoning,
            extra: selected.extra.clone(),
        })
    }

    /// 선택 키 엔드포인트의 `GET /models` 목록 조회를 이 클라이언트에서 부를 수 있게 이음.
    ///
    /// 채팅과 같은 base_url에 `/models`를 붙이고 전송 클라이언트의 조회를 그대로 거침.
    pub fn list_models(
        &self,
        cancel: CancellationToken,
    ) -> impl Future<Output = Result<Vec<String>, TransportError>> + Send + '_ {
        // 재시도 없이 한 번만 조회. 같은 시간·취소·오류 가리기를 씀.
        self.inner.list_models(cancel)
    }
}

/// 전송 조립 실패를 선택 키 조립 오류로 올림. 주소·키 텍스트는 담지 않음.
impl From<TransportError> for SelectError {
    fn from(_error: TransportError) -> Self {
        Self::ClientBuild
    }
}
