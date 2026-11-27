//! 선택 대상·자격 키·`http` 설정으로 클라이언트를 조립하는 자리. 조립 한 책임만 담음.
//!
//! 재시도 판단 로직은 고치지 않고 02-6 래퍼를 부르기만 한다.

use crate::config::secret_string::SecretString;
use crate::config::types::HttpSettings;
use crate::llm::client::build::bundle::ClientBundle;
use crate::llm::client::kind::EndpointKind;
use crate::llm::client::retry::RetryClient;
use crate::llm::client::selection::SelectionTarget;
use crate::llm::client::settings::retry_settings_of;
use crate::llm::client::transport::HttpTransport;
use crate::llm::retry::SystemRandomSource;

/// T1이 찾은 엔드포인트·모델과 자격 키로 클라이언트를 조립함.
///
/// `HttpTransport`에 주소·종류·키·`http` 설정을 넣고, 그 위에 02-6 재시도 래퍼를 씌운다.
/// 조립 실패는 이 함수가 내지 않는다. 대상이 정해지지 않으면 T1의 선택 오류가 이미 거른다.
pub fn assemble_client(
    target: &SelectionTarget<'_>,
    key: Option<SecretString>,
    http: &HttpSettings,
) -> ClientBundle {
    let transport = HttpTransport::new(
        target.endpoint.base_url.expose(),
        endpoint_kind(target),
        key,
        http,
    );
    let client = RetryClient::new(
        transport,
        retry_settings_of(http),
        SystemRandomSource::default(),
    );
    ClientBundle {
        client,
        model_id: target.model.id.clone(),
    }
}

/// 설정 `kind` 값을 서버 형식으로 옮김.
///
/// 설정 검증이 허용 목록 밖을 이미 거부하므로, 목록 밖 값은 OpenAI 형식으로 본다.
fn endpoint_kind(target: &SelectionTarget<'_>) -> EndpointKind {
    EndpointKind::from_setting(target.endpoint.kind.trim()).unwrap_or(EndpointKind::Openai)
}
