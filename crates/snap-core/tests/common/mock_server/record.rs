//! 모의 서버가 받아 둔 요청 기록 타입과 그 조립
//!
//! 두 경로 핸들러가 같은 필드 조립을 다시 적지 않게 `new_record`로 합침

use std::time::Instant;

use axum::http::{HeaderMap, Method, StatusCode, Uri};

/// 모의 서버가 받아 둔 요청 한 건
#[derive(Clone, Debug)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    /// 요청이 도착한 단조 시각. 반복 요청 간격 확인에 사용
    pub received_at: Instant,
    /// 요청에 대해 이 서버가 보낸 응답 상태 코드. 시험이 요청별로 상태까지 대조
    pub status: StatusCode,
}

/// 요청 기록 항목을 채움. 두 경로 핸들러가 같은 필드 조립을 다시 적지 않음
pub fn new_record(
    method: Method,
    uri: &Uri,
    headers: HeaderMap,
    body: Vec<u8>,
    received_at: Instant,
    status: StatusCode,
) -> RecordedRequest {
    RecordedRequest {
        method: method.to_string(),
        path: uri.path().to_string(),
        headers,
        body,
        received_at,
        status,
    }
}
