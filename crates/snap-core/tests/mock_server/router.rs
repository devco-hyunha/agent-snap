//! 모의 서버 라우터 자리. 요청을 받아 응답하는 경로 연결과 핸들러 흐름만 담당.
use std::sync::Arc;

use axum::Router;
use axum::extract::Request;
use axum::extract::State;
use axum::response::Response;
use tokio::time::sleep;

use crate::recorder::record_request;
use crate::response::build_response;
use crate::script::StreamStop;
use crate::state::ServerState;

/// 요청을 받아 응답하는 라우터. 모든 경로를 같은 핸들러로 봄.
pub fn build_router(state: Arc<ServerState>) -> Router {
    Router::new().fallback(handle).with_state(state)
}

/// 요청을 기록하고 응답 스크립트대로 응답함. 끊기 규칙은 응답 뒤 서버를 끝낸다.
/// 급히 끊는 규칙은 본문 읽기 실패로 끝나면 되고, 서버는 살아 있어 다음 요청을 받을 수 있다.
async fn handle(State(state): State<Arc<ServerState>>, request: Request) -> Response {
    let script = state.take_script();
    let recorded = record_request(request).await;
    state.log.push(recorded);
    sleep(script.header.wait()).await;
    let response = build_response(&script);
    if matches!(script.stop, StreamStop::Cut | StreamStop::Drop) {
        state.cancel_shutdown();
    }
    response
}
