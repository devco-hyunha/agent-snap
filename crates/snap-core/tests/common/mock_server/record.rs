//! 모의 서버가 받아 둔 요청 기록 타입과 그 조립만 다룹니다.
//!
//! 기록 타입(`RecordedRequest`)과 필드 채우기(`new_record`)가 이 파일의 책임입니다.
//! 응답 스크립트 규격(`script`)·서버 수명(`server`)과 책임을 나눴다(SRP, dev-principles 2절).
//! 두 경로 핸들러가 같은 필드 조립을 다시 적지 않게 `new_record`로 합쳤다(DRY).

use std::time::Instant;

use axum::http::{HeaderMap, Method, StatusCode, Uri};

/// 모의 서버가 받아 둔 요청 한 건입니다.
#[derive(Clone, Debug)]
pub struct RecordedRequest {
    /// HTTP 메서드 문자열입니다.
    pub method: String,
    /// 요청 경로입니다.
    pub path: String,
    /// 요청 헤더입니다.
    pub headers: HeaderMap,
    /// 요청 본문 바이트입니다.
    pub body: Vec<u8>,
    /// 요청이 도착한 단조 시각입니다. 반복 요청 사이의 간격 확인에 씁니다.
    pub received_at: Instant,
    /// 요청에 대해 이 서버가 보낸 응답 상태 코드입니다. 시험이 요청별로 상태까지 대조하는 데 씁니다.
    pub status: StatusCode,
}

/// 요청 기록 항목을 채운다. 두 경로 핸들러가 같은 필드 조립을 다시 적지 않게 한다.
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
