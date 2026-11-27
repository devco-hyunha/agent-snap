//! 받은 요청 기록 로직 자리. 요청에서 기록을 뽑는 일만.
use axum::body::to_bytes;
use axum::extract::Request;

use crate::types::RecordedRequest;

/// 요청 본문 크기 상한. 시험 본문은 작다.
const BODY_LIMIT: usize = 1024 * 1024;

/// 요청에서 메서드·경로·헤더·본문만 뽑아 기록으로 만듦.
pub(crate) async fn record_request(request: Request) -> RecordedRequest {
    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, BODY_LIMIT).await.unwrap_or_default();
    RecordedRequest {
        method: parts.method.as_str().to_owned(),
        path: parts.uri.path().to_owned(),
        headers: parts
            .headers
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_owned(),
                    value.to_str().unwrap_or_default().to_owned(),
                )
            })
            .collect(),
        body: bytes.to_vec(),
    }
}
