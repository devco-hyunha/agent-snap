//! 받은 요청 기록 저장 로직 자리. 헤더 찾기·본문 읽기·기록 추가만 담음.
use crate::types::RecordedRequest;
use crate::types::RequestLog;

impl RecordedRequest {
    /// 헤더 값을 이름으로 찾음. 대·소문자 구분 없이 봄.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// 요청 본문을 UTF-8 텍스트로 봄. 본문이 없으면 빈 텍스트.
    pub fn body_text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

impl RequestLog {
    /// 빈 기록.
    pub fn new() -> Self {
        Self::default()
    }

    /// 요청을 기록에 추가함.
    pub(crate) fn push(&self, request: RecordedRequest) {
        let Ok(mut recorded) = self.0.lock() else {
            return;
        };
        recorded.push(request);
    }

    /// 지금까지 기록된 요청 사본.
    pub fn snapshot(&self) -> Vec<RecordedRequest> {
        let Ok(recorded) = self.0.lock() else {
            return Vec::new();
        };
        recorded.clone()
    }

    /// 기록된 요청 개수.
    pub fn len(&self) -> usize {
        self.snapshot().len()
    }

    /// 받은 요청이 하나도 없는지.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
