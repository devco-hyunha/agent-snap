#![allow(dead_code, unreachable_pub)]

//! `GET /models` 요청을 받아 목록 JSON으로 응답하는 핸들러 하나만 다룹니다.
//!
//! 채팅 핸들러(`chat_handler`)와 핸들러 파일 책임을 나눴다(SRP, dev-principles 2절).
//! 목록 본문 조립은 `models_response`, 응답 시작 조립은 `response`, 요청 기록은 `record`가 맡는다.
//! 헤더 지연·행(헤더를 영원히 안 보내기)·본문 조각 지연은 채팅 핸들러와 같은 스크립트 규격을
//! 따르며, 본문 스트림은 `body_stream`의 기존 구현을 그대로 재사용한다(DRY).

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

/// `GET /models` 요청입니다. 스크립트가 정한 상태·헤더·본문 순서와 지연 그대로 답합니다.
///
/// 채팅 핸들러와 같은 순서로 스크립트를 따릅니다(취소·시간 초과 시험이 같은 조건을
/// 목록 경로에서도 볼 수 있게). 본문이 여러 조각이면 지연을 지키며 스트림으로 보내고,
/// 한 덩어리로 읽는 시험(정상·형식 오류)은 본문 전체가 올 때까지 읽기만 하면 됩니다.
pub(super) async fn handle_models(
    State(shared): State<Arc<Shared>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let guard = ConnectionGuard::new(&shared);

    // 응답을 만들기 전에 그 상태를 요청 기록에 함께 남깁니다(채팅과 같은 순서).
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
        // 클라이언트가 끊을 때까지 헤더를 보내지 않습니다.
        let _guard = guard;
        return std::future::pending::<Response>().await;
    }

    if !script.header_delay.is_zero() {
        tokio::time::sleep(script.header_delay).await;
    }

    // 지연이 있는 목록 스크립트는 시험의 시간 초과·취소 조건을 지켜야 하므로 순서를 지키며 보냅니다.
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

    // 끊김 표식이 없는 목록은 JSON 한 덩어리로 바로 답한다(기록 본문과 동일하게).
    let _guard = guard;
    let list_body = join_body_parts(list_body_parts(&script));
    list_response(&script, list_body)
}
