//! 전송 통합 테스트를 관심사별로 나눈 모듈입니다(AC 1·3·4·5·6).
//!
//! 복원·시간 초과·취소·연결 오류·사용량·가리기를 파일로 나눕니다.

mod cancel;
mod connect_errors;
mod redact;
mod restore;
mod timeouts;
mod usage;

use std::time::Duration;

pub(super) use super::support::test_http;

use crate::config::HttpSettings;
use crate::llm::{ChatStream, OpenAiCompatClient, StreamEvent};

/// 헤더 전 연결 세부를 보는 시험용 시간 값입니다.
///
/// Windows는 닫힌 포트에 대한 연결을 몇 초 다시 시도합니다. 헤더 대기(2초)가 먼저
/// 끝나면 시간 초과로 보여 연결 세부를 못 봅니다. 헤더 대기를 넉넉히 둬 연결 실패가 먼저 오게 합니다.
pub(super) fn connect_check_http() -> HttpSettings {
    HttpSettings {
        header_wait_seconds: Duration::from_secs(30),
        ..test_http()
    }
}

pub(super) fn client_at_with(
    base_url: &str,
    api_key: Option<&str>,
    http: HttpSettings,
) -> OpenAiCompatClient {
    super::support::client_at(base_url, crate::config::EndpointKind::OpenAi, api_key, http)
}

/// 스트림을 끝까지 읽고 이벤트만 모읍니다.
pub(super) async fn drain(stream: &mut ChatStream) -> Vec<StreamEvent> {
    let mut events = Vec::new();
    while let Some(event) = stream.next_event().await {
        match event {
            Ok(event) => events.push(event),
            Err(error) => panic!("스트림 오류: {error}"),
        }
    }
    events
}
