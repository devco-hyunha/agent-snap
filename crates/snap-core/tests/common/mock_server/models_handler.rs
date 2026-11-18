#![allow(dead_code, unreachable_pub)]

//! `GET /models` 요청을 받아 목록 JSON으로 응답하는 핸들러
//!
//! 헤더 지연·행(헤더를 영원히 안 보내기)·본문 조각 지연은 채팅 핸들러와 같은 스크립트 규격
//! 본문 스트림은 `body_stream`의 기존 구현을 그대로 재사용

use std::sync::Arc;
use std::time::Instant;

use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, Method, Uri};
use axum::response::Response;

use super::body_stream::{GuardedBody, ScriptBody};
use super::chat_handler::BodyStreamBox;
use super::guard::ConnectionGuard;
use super::models_response::{join_body_parts, list_body_parts, list_response};
use super::record::new_record;
use super::response::response_builder;
use super::shared::{self, Shared};

/// `GET /models` 요청. 스크립트가 정한 상태·헤더·본문 순서와 지연 그대로 답함
///
/// 채팅 핸들러와 같은 순서로 스크립트를 따름. 본문이 여러 조각이면 지연을 지키며 스트림으로 보냄
pub(super) async fn handle_models(
    State(shared): State<Arc<Shared>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let guard = ConnectionGuard::new(&shared);

    // 응답 만들기 전에 그 상태를 요청 기록에 함께 남김. 채팅과 같은 순서
    let script = shared::current_script(&shared).await;
    shared::record(
        &shared,
        new_record(
            method,
            &uri,
            headers,
            body.to_vec(),
            Instant::now(),
            script.status,
        ),
    )
    .await;

    if script.hang_before_headers {
        // 클라이언트가 끊을 때까지 헤더를 보내지 않음
        let _guard = guard;
        return std::future::pending::<Response>().await;
    }

    if !script.header_delay.is_zero() {
        tokio::time::sleep(script.header_delay).await;
    }

    // 지연 있는 목록 스크립트는 시간 초과·취소 조건을 지켜야 하므로 순서를 지킴
    if list_body_parts(&script)
        .iter()
        .any(|part| !part.delay_before.is_zero())
    {
        let inner: BodyStreamBox = Box::pin(ScriptBody::new(list_body_parts(&script).to_vec()));
        let guarded = GuardedBody {
            inner,
            guard: Some(guard),
        };
        return response_builder(&script)
            .header("content-type", "application/json")
            .body(Body::new(guarded))
            .expect("모의 목록 스트림 응답 조립");
    }

    // 끊김 표식이 없는 목록은 JSON 한 덩어리로 바로 답함
    let _guard = guard;
    let list_body = join_body_parts(list_body_parts(&script));
    list_response(&script, list_body)
}
