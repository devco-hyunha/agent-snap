//! 선택 키로 클라이언트를 만드는 준비. 선택 키로 정한 값과 엔드포인트 정보를 모음.
//!
//! 조립(`selection/client_build.rs`)과 값 채우기·전송 재사용(`selection/apply.rs`)은 파일을 나눔.

use std::collections::BTreeMap;

use crate::config::{
    Config, EndpointKind, ExtraValue, HttpSettings, Reasoning, Sampling, SelectionError,
};
use crate::credentials::Credentials;
use crate::secrets::SecretValue;

pub(in crate::llm) mod models_list;
pub(in crate::llm) mod sampling_merge;

mod apply;
mod client_build;

use sampling_merge::merged_sampling;

pub use client_build::SelectedClient;

#[cfg(test)]
mod tests;

/// 선택 키로 클라이언트를 만드는 준비 실패.
///
/// Display·Debug에 자격 키와 base_url 호스트가 드러나는 값은 없음.
#[derive(thiserror::Error, Debug)]
pub enum SelectError {
    /// 선택 키가 설정에 없거나 형식이 틀리거나 고를 모델이 없음.
    #[error(transparent)]
    Selection(#[from] SelectionError),
    /// HTTP 클라이언트 조립 실패.
    #[error("전송 클라이언트를 만들지 못했습니다")]
    ClientBuild,
}

/// 선택 키가 고른 모델과 요청 준비 값.
///
/// 샘플링은 모델별 값이 루트 기본보다 앞섬. `reasoning`·`extra`는 설정 값 그대로.
pub struct SelectedModel {
    /// 선택 키가 가리키는 엔드포인트 주소.
    pub base_url: SecretValue,
    /// 엔드포인트 서버 종류.
    pub kind: EndpointKind,
    /// 요청에 쓸 키. 자격 파일에 없으면 없음(인증 헤더 없이 요청).
    pub api_key: Option<SecretValue>,
    /// 요청 시간·재시도 값. 설정의 `http` 값.
    pub http: HttpSettings,
    /// 서버에 보낼 모델 id.
    pub model_id: String,
    /// 모델별 값이 루트 기본보다 앞선 샘플링. 없는 항목은 본문에서 빼 서버 기본을 따름.
    pub sampling: Sampling,
    /// 생각 옵션. 없으면 요청에 더하지 않음.
    pub reasoning: Option<Reasoning>,
    /// 요청 본문에 합칠 추가 값. 없으면 빈 맵.
    pub extra: BTreeMap<String, ExtraValue>,
}

impl Config {
    /// 선택 키(`엔드포인트이름/모델이름`)로 클라이언트 만들 준비를 함.
    ///
    /// 모델 찾기와 키 정하기를 함께 하고, 본문에 쓸 값을 모아 반환. 모델 이름만으로는 고르지 않음.
    ///
    /// # 오류
    /// - [`SelectError::Selection`]: 설정에 없는 키·형식이 틀린 키·고를 모델 없음. 오류에는 입력 키만 담음.
    /// - [`SelectError::ClientBuild`]: HTTP 클라이언트 조립 실패.
    pub fn select_model(
        &self,
        selection_key: &str,
        credentials: &Credentials,
    ) -> Result<SelectedModel, SelectError> {
        let (endpoint, model) = self.select(selection_key)?;
        Ok(SelectedModel {
            base_url: endpoint.base_url.clone(),
            kind: endpoint.kind,
            api_key: endpoint.api_key(credentials).cloned(),
            http: self.http.clone(),
            model_id: model.id.clone(),
            sampling: merged_sampling(model, &self.sampling),
            reasoning: model.reasoning,
            extra: model.extra.clone(),
        })
    }

    /// 선택 키(`엔드포인트이름/모델이름`)로 요청 클라이언트를 만듦.
    ///
    /// [`Config::select_model`]로 준비하고 조립. 요청은 그 엔드포인트로만 가고 본문에 모델 값이 실림.
    ///
    /// # 오류
    /// - [`SelectError::Selection`]: 설정에 없는 키·형식이 틀린 키·고를 모델 없음.
    /// - [`SelectError::ClientBuild`]: HTTP 클라이언트 조립 실패.
    pub fn client_for(
        &self,
        selection_key: &str,
        credentials: &Credentials,
    ) -> Result<SelectedClient, SelectError> {
        let selected = self.select_model(selection_key, credentials)?;
        SelectedClient::from_selected(&selected)
    }
}
