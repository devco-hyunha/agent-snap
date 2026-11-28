//! 응답 스크립트 규격 타입 자리. 응답 시점과 연결 종료 규칙 타입만 담는다.
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

    /// 헤더를 보낸 뒤 본문을 급히 끊음. 본문 끝 표시를 보내지 않는다.
    Abort,
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

/// 응답 스크립트 규격. 상태 코드·응답 헤더·헤더 지연·본문 규칙·연결 종료를 담는다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockScript {
    /// 응답 상태 코드.
    pub status: u16,

    /// 응답 헤더 지연 규칙.
    pub header: HeaderMode,

    /// 응답에 실는 추가 헤더. 이름·값을 그대로 보냄.
    pub headers: Vec<(String, String)>,

    /// 응답 본문 규칙.
    pub body: Vec<StreamStep>,

    /// 본문 규칙이 끝난 뒤 연결 종료 규칙.
    pub stop: StreamStop,
}
