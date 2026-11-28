//! 토큰 시각 기록 자리. 응답이 늘어난 때만 첫·마지막 토큰 시각을 채움.

use std::time::Instant;

use crate::llm::client::stream::StreamTimings;

/// 요청 시작과 토큰 시각을 함께 재는 기록.
#[derive(Debug)]
pub struct TokenTimes {
    /// 요청을 시작한 시각.
    request_start: Instant,

    /// 첫 토큰을 받은 시각.
    first_token: Option<Instant>,

    /// 마지막 토큰을 받은 시각.
    last_token: Option<Instant>,

    /// 합친 응답에서 늘어난 글자 수. 토큰 시각을 채울지 판단에 씀.
    merged_len: usize,
}

impl TokenTimes {
    /// 요청 시작 시각만 받아 기록 열기.
    pub const fn new(request_start: Instant) -> Self {
        Self {
            request_start,
            first_token: None,
            last_token: None,
            merged_len: 0,
        }
    }

    /// 합친 응답 글자 수를 받아 늘었을 때만 토큰 시각을 채움.
    pub fn record(&mut self, merged_len: usize) {
        if merged_len == self.merged_len {
            return;
        }
        self.merged_len = merged_len;
        let now = Instant::now();
        if self.first_token.is_some() {
            self.last_token = Some(now);
            return;
        }
        self.first_token = Some(now);
        self.last_token = Some(now);
    }

    /// 지금까지의 시각 묶음.
    pub const fn timings(&self) -> StreamTimings {
        StreamTimings {
            request_start: self.request_start,
            first_token: self.first_token,
            last_token: self.last_token,
        }
    }
}
