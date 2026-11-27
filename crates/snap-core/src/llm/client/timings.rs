//! 전송 시간 값 타입 자리. 세 구간 값만 담는 파일.

use std::time::Duration;

use crate::config::types::HttpSettings;

/// 전송이 재는 시간 세 칸. 연결·헤더 대기·스트림 유휴.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportTimings {
    /// 연결 시도 최대 시간.
    pub connect: Duration,

    /// 응답 헤더 대기 최대 시간.
    pub header_wait: Duration,

    /// 응답 스트림 유휴 최대 시간.
    pub stream_idle: Duration,
}

impl TransportTimings {
    /// 설정 `http` 값에서 전송 시간만 옮김.
    pub fn from_settings(settings: &HttpSettings) -> Self {
        Self {
            connect: Duration::from_secs_f64(settings.connect_seconds),
            header_wait: Duration::from_secs_f64(settings.header_wait_seconds),
            stream_idle: Duration::from_secs_f64(settings.stream_idle_seconds),
        }
    }

    /// 연결 시도 설정 초 값.
    pub const fn connect_seconds(&self) -> f64 {
        self.connect.as_secs_f64()
    }

    /// 헤더 대기 설정 초 값.
    pub const fn header_wait_seconds(&self) -> f64 {
        self.header_wait.as_secs_f64()
    }

    /// 스트림 유휴 설정 초 값.
    pub const fn stream_idle_seconds(&self) -> f64 {
        self.stream_idle.as_secs_f64()
    }
}
