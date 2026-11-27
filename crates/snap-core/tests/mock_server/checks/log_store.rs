//! 요청 기록 저장소 확인. 기록 추가·순서·비움 동작을 본다.
use crate::types::RecordedRequest;
use crate::types::RequestLog;

/// 시험용 요청 기록. 본문과 헤더는 인자로 채운다.
fn request(method: &str, path: &str, body: &str) -> RecordedRequest {
    RecordedRequest {
        method: method.to_owned(),
        path: path.to_owned(),
        headers: Vec::new(),
        body: body.as_bytes().to_vec(),
    }
}

#[test]
fn empty_log_has_no_request() {
    let log = RequestLog::new();

    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
    assert!(log.snapshot().is_empty());
}

#[test]
fn snapshot_keeps_requests_in_push_order() {
    let log = RequestLog::new();

    log.push(request("GET", "/v1/models", ""));
    log.push(request("POST", "/v1/chat/completions", "body"));

    let recorded = log.snapshot();
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].method, "GET");
    assert_eq!(recorded[1].method, "POST");
    assert_eq!(recorded[1].body_text(), "body");
    assert!(!log.is_empty());
}

#[test]
fn header_lookup_ignores_name_case() {
    let log = RequestLog::new();
    let mut record = request("POST", "/v1/chat/completions", "");
    record
        .headers
        .push(("Authorization".to_owned(), "Bearer x".to_owned()));

    log.push(record);

    let recorded = log.snapshot();
    #[allow(clippy::unwrap_used)]
    let request = recorded.first().unwrap();
    assert_eq!(request.header("authorization"), Some("Bearer x"));
    assert_eq!(request.header("content-type"), None);
}
