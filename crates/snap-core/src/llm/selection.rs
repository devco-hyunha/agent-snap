//! 선택 키로 클라이언트를 만드는 준비(02-7 작업 1): 선택 키로 정한 값과 엔드포인트 정보를 모는다.
//!
//! 클라이언트 조립(`selection/client_build.rs`)과 값 채우기·전송 재사용(`selection/apply.rs`)은
//! 바뀌는 이유가 다르므로 파일을 나눈다(프로젝트 개발 규칙 SRP).

use std::collections::BTreeMap;

use crate::config::{
    Config, EndpointKind, ExtraValue, HttpSettings, ModelConfig, Reasoning, Sampling,
    SelectionError,
};
use crate::credentials::Credentials;
use crate::secrets::SecretValue;

mod apply;
mod client_build;

pub use client_build::SelectedClient;

#[cfg(test)]
mod tests;

/// 선택 키로 클라이언트를 만드는 준비가 실패했습니다.
///
/// Display와 Debug에 자격 키와 base_url 호스트가 드러나는 값은 없습니다(선택 오류는 입력 키만 담고,
/// 조립 실패는 02-5 규칙을 따릅니다).
#[derive(thiserror::Error, Debug)]
pub enum SelectError {
    /// 선택 키가 설정에 없거나 형식이 틀리거나 고를 모델이 없습니다.
    #[error(transparent)]
    Selection(#[from] SelectionError),
    /// HTTP 클라이언트 조립에 실패했습니다.
    #[error("전송 클라이언트를 만들지 못했습니다")]
    ClientBuild,
}

/// 선택 키가 고른 모델과 요청 준비 값입니다.
///
/// 샘플링은 모델별 값이 루트 기본값보다 앞서고(D14), `reasoning`과 `extra`는 설정 값 그대로입니다.
/// 키는 02-2의 키 정하기(자격 파일 `api_keys`의 엔드포인트 이름 찾기)로 정했습니다.
pub struct SelectedModel {
    /// 선택 키가 가리키는 엔드포인트 주소입니다.
    pub base_url: SecretValue,
    /// 엔드포인트 서버 종류입니다.
    pub kind: EndpointKind,
    /// 요청에 쓸 키입니다. 자격 파일에 없으면 없음(인증 헤더 없이 요청)입니다.
    pub api_key: Option<SecretValue>,
    /// 요청 시간·재시도 값입니다. 설정의 `http` 값입니다.
    pub http: HttpSettings,
    /// 서버에 보낼 모델 id입니다.
    pub model_id: String,
    /// 모델별 값이 루트 기본값보다 앞선 샘플링입니다. 없는 항목은 본문에서 빼 서버 기본을 따릅니다.
    pub sampling: Sampling,
    /// 생각 옵션입니다. 없으면 요청에 더하지 않습니다.
    pub reasoning: Option<Reasoning>,
    /// 요청 본문에 합칠 추가 값입니다. 없으면 빈 맵입니다.
    pub extra: BTreeMap<String, ExtraValue>,
}

impl Config {
    /// 선택 키(`엔드포인트이름/모델이름`)로 클라이언트 만들 준비를 합니다.
    ///
    /// 설정에서 모델 찾기(`config/selection.rs`)와 키 정하기(02-2: 자격 파일 `api_keys`의 엔드포인트
    /// 이름 찾기)를 함께 하고, 요청 본문에 쓸 값(서버용 id·샘플링·`reasoning`·`extra`)을 모아 돌려줍니다.
    /// 모델 이름만으로는 고르지 않습니다(선택 키 필수, D20).
    ///
    /// # 오류
    /// - [`SelectError::Selection`]: 설정에 없는 키·형식이 틀린 키·고를 모델 없음. 오류에는 입력 키만 담습니다.
    /// - [`SelectError::ClientBuild`]: HTTP 클라이언트 조립에 실패했습니다.
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

    /// 선택 키(`엔드포인트이름/모델이름`)로 요청 클라이언트를 만듭니다.
    ///
    /// [`Config::select_model`]로 준비하고 클라이언트를 조립니다. 요청은 그 선택 키의
    /// 엔드포인트로만 가고, 본문에는 그 모델의 서버용 id와 샘플링·`reasoning`·`extra`가 실립니다(02-7 AC 1).
    ///
    /// # 오류
    /// - [`SelectError::Selection`]: 설정에 없는 키·형식이 틀린 키·고를 모델 없음.
    /// - [`SelectError::ClientBuild`]: HTTP 클라이언트 조립에 실패했습니다.
    pub fn client_for(
        &self,
        selection_key: &str,
        credentials: &Credentials,
    ) -> Result<SelectedClient, SelectError> {
        let selected = self.select_model(selection_key, credentials)?;
        SelectedClient::from_selected(&selected)
    }
}

/// 모델별 값이 루트 기본값보다 앞서는 샘플링으로 잇습니다(D14).
///
/// 모델에 없는 항목만 루트 기본을 따라 가고, 루트에도 없으면 본문에서 빼 서버 기본을 따릅니다.
fn merged_sampling(model: &ModelConfig, root: &Sampling) -> Sampling {
    Sampling {
        temperature: model.sampling.temperature.or(root.temperature),
        top_p: model.sampling.top_p.or(root.top_p),
        max_tokens: model.sampling.max_tokens.or(root.max_tokens),
    }
}
