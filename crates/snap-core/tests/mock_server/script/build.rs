//! 응답 스크립트를 만드는 창구 자리. 생성자와 스크립트 상수만 담당.
use std::time::Duration;

use super::spec::HeaderMode;
use super::spec::MockScript;
use super::spec::StreamStep;
use super::spec::StreamStop;

/// 급히 끊기 전에 헤더와 첫 조각이 클라이언트로 옮겨가는 대기.
pub const ABORT_GAP: Duration = Duration::from_millis(20);

impl StreamStep {
    /// 조각 규칙을 만든다. 조각 뒤 대기는 0.
    pub fn chunk(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Chunk {
            bytes: bytes.into(),
            gap: Duration::ZERO,
        }
    }

    /// 조각 규칙에 조각 뒤 대기 시간을 입힌다.
    pub fn chunk_after(self, gap: Duration) -> Self {
        match self {
            Self::Chunk { bytes, .. } => Self::Chunk { bytes, gap },
            step => step,
        }
    }
}

impl MockScript {
    /// 빈 응답 스크립트. 상태 200, 헤더 즉시, 추가 헤더 없음, 본문 없음, 정상 종료.
    pub fn ok() -> Self {
        Self {
            status: 200,
            header: HeaderMode::Immediate,
            headers: Vec::new(),
            body: Vec::new(),
            stop: StreamStop::End,
        }
    }

    /// 지정한 상태 코드의 빈 응답 스크립트.
    pub fn with_status(status: u16) -> Self {
        Self {
            status,
            ..Self::ok()
        }
    }

    /// 헤더를 지연해 내보내는 응답 스크립트로 바꿈.
    pub fn with_header_delay(mut self, wait: Duration) -> Self {
        self.header = HeaderMode::Delay(wait);
        self
    }

    /// 응답 본문 규칙을 채운다.
    pub fn with_body(mut self, body: Vec<StreamStep>) -> Self {
        self.body = body;
        self
    }

    /// 응답에 실을 추가 헤더를 채운다.
    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_owned(), value.to_owned()));
        self
    }

    /// 본문 규칙이 끝난 뒤 연결 종료 규칙을 채운다.
    pub fn with_stop(mut self, stop: StreamStop) -> Self {
        self.stop = stop;
        self
    }
}
