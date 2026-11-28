//! 받은 요청 기록 타입 자리. 필드만 담는 파일. 저장 로직은 `store.rs`가 맡음.
use std::sync::Arc;
use std::sync::Mutex;

/// 모의 서버가 받은 요청 하나.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRequest {
    /// 요청 메서드.
    pub method: String,

    /// 요청 주소. 경로와 쿼리.
    pub path: String,

    /// 받은 헤더. 이름과 값.
    pub headers: Vec<(String, String)>,

    /// 받은 요청 본문.
    pub body: Vec<u8>,
}

/// 모의 서버가 받은 요청 기록. 핸들러와 테스트가 같은 묶음을 공유함.
#[derive(Debug, Clone, Default)]
pub struct RequestLog(pub(crate) Arc<Mutex<Vec<RecordedRequest>>>);
