#![allow(dead_code, unreachable_pub)]

//! `POST /chat/completions` 요청 핸들러와 본문 스트림 조립 하나만 다룹니다.
//!
//! 목록 핸들러(`models_handler`)와 핸들러 파일 책임을 나눴다(SRP, dev-principles 2절).
//! 스크립트 규격은 `super::script`, 본문 스트림 구현은 `super::body_stream`,
//! 응답 시작 조립은 `super::response`, 서버 수명과 연결 수 세기는 `super::server`가 맡습니다.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, Method, Uri};
use axum::response::Response;

use super::body_stream::{DisconnectBody, GuardedBody, ScriptBody};
use super::disconnect::BodyDisconnected;
use super::guard::ConnectionGuard;
use super::record::new_record;
use super::response::response_builder;
use super::script::MockResponseScript;
use super::shared::{self, Shared};

/// `pub(super)`로 열어 목록 핸들러도 같은 스트림 조립을 쓰게 한다(DRY, dev-principles 3절).
pub(super) type BodyStreamBox =
    Pin<Box<dyn http_body::Body<Data = Bytes, Error = BodyDisconnected> + Send + Sync>>;

/// `POST /chat/completions` 요청입니다. 스크립트대로 본문을 스트림으로 보내며,
/// 연결을 끊거나 헤더를 늦게 보내는 시험도 이 경로로 본다.
pub(super) async fn handle_chat_completions(
    State(shared): State<Arc<Shared>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let guard = ConnectionGuard::new(&shared);

    // 요청을 받기 시작하는 순간을 요청 도착 시각으로 기록합니다. 반복 요청 간격 확인의 기준입니다.
    let received_at = Instant::now();
    let script = shared::current_script(&shared).await;

    // 응답을 만들기 전에 그 상태를 요청 기록에 함께 남깁니다. 시험이 요청별로 상태까지 대조합니다.
    shared::record(
        &shared,
        new_record(
            method,
            &uri,
            headers,
            body.to_vec(),
            received_at,
            script.status,
        ),
    )
    .await;

    if script.hang_before_headers {
        // 클라이언트가 끊을 때까지 헤더를 보내지 않습니다.
        let _guard = guard;
        return std::future::pending::<Response>().await;
    }

    if !script.header_delay.is_zero() {
        tokio::time::sleep(script.header_delay).await;
    }

    let builder = response_builder(&script);

    // 응답 헤더까지 보내는 본문 스트림(지연·끊김·조각 전송)은 채팅 핸들러가 만드는 조립을 씁니다.
    response_with_body_stream(builder, script, guard).expect("모의 응답 본문 조립")
}

/// 스크립트 본문 스트림을 붙여 응답을 조립합니다. 채팅 핸들러 한곳에서 쓰는 만들기입니다
/// (목록은 본문 스트림을 쓰지 않습니다).
///
/// 끊김 표식이 섞인 스크립트면 끊김 본문을 씁니다. 끊김은 hyper가 보내는 chunked 끝 마커 없이
/// 연결이 닫히는 자리에서 나와야 합니다(02-6 AC 5).
fn response_with_body_stream(
    builder: axum::http::response::Builder,
    script: MockResponseScript,
    guard: ConnectionGuard,
) -> Result<Response, axum::http::Error> {
    let inner: BodyStreamBox = if script.body_parts.iter().any(|part| part.disconnect) {
        Box::pin(DisconnectBody::new(script.body_parts))
    } else {
        Box::pin(ScriptBody::new(script.body_parts))
    };
    let guarded = GuardedBody {
        inner,
        guard: Some(guard),
    };
    builder.body(Body::new(guarded))
}
