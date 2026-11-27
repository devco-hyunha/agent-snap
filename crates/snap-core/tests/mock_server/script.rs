//! 응답 스크립트 규격 타입 자리. 응답 시점과 연결 종료 규칙만 담는다.
use std::time::Duration;

/// 응답 헤더가 나가는 시점 규칙.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderMode {
    /// 지연 없이 바로 내보냄.
    Immediate,

    /// 지정한 시간만큼 기다린 뒤 내보냄.
    Delay(Duration),
}

impl HeaderMode {
    /// 헤더가 나가는 시각까지의 대기 시간.
    pub fn wait(&self) -> Duration {
        match self {
            Self::Immediate => Duration::ZERO,
            Self::Delay(wait) => *wait,
        }
    }
}

/// 스트림이 멈추는 규칙.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamStop {
    /// 조각을 다 보낸 뒤 정상 종료.
    End,

    /// 조각 없이 연결만 끊음.
    Drop,

    /// 조각을 다 보낸 뒤 연결을 끊음.
    Cut,
}

/// 스트림이 흘려보낼 규칙.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamStep {
    /// SSE 조각 하나. 조각을 보낸 뒤 기다릴 시간을 함께 담음.
    Chunk {
        /// 보낼 조각.
        bytes: Vec<u8>,

        /// 조각을 보낸 뒤 기다릴 시간.
        gap: Duration,
    },

    /// 이 지점에서 스트림을 멈춤.
    Stop(StreamStop),
}

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

/// 응답 스크립트 규격. 상태 코드·헤더 지연·본문 규칙·연결 종료를 담는다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockScript {
    /// 응답 상태 코드.
    pub status: u16,

    /// 응답 헤더 지연 규칙.
    pub header: HeaderMode,

    /// 응답 본문 규칙.
    pub body: Vec<StreamStep>,

    /// 본문 규칙이 끝난 뒤 연결 종료 규칙.
    pub stop: StreamStop,
}

impl MockScript {
    /// 빈 응답 스크립트. 상태 200, 헤더 즉시, 본문 없음, 정상 종료.
    pub fn ok() -> Self {
        Self {
            status: 200,
            header: HeaderMode::Immediate,
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

    /// 본문 규칙이 끝난 뒤 연결 종료 규칙을 채운다.
    pub fn with_stop(mut self, stop: StreamStop) -> Self {
        self.stop = stop;
        self
    }
}
