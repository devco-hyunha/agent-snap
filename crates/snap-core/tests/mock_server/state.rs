//! 모의 서버 상태 자리. 응답 스크립트 큐·요청 기록·서버 끝낼 표시를 공유하는 상태만 담당.
use std::collections::VecDeque;
use std::sync::Mutex;

use tokio_util::sync::CancellationToken;

use crate::script::MockScript;
use crate::types::RequestLog;

/// 핸들러가 공유하는 서버 상태. 응답 스크립트 큐·요청 기록·서버 끝낼 표시.
#[derive(Debug)]
pub struct ServerState {
    /// 응답 스크립트 큐.
    scripts: Mutex<VecDeque<MockScript>>,

    /// 받은 요청 기록.
    pub log: RequestLog,

    /// 서버를 끝낼 표시.
    shutdown: Mutex<Option<CancellationToken>>,
}

impl ServerState {
    /// 빈 상태.
    pub fn new() -> Self {
        Self {
            scripts: Mutex::new(VecDeque::new()),
            log: RequestLog::new(),
            shutdown: Mutex::new(None),
        }
    }

    /// 서버 끝낼 표시를 연결함. 끊기 규칙이 서버 연결을 정리하게 쓴다.
    pub fn attach_shutdown(&self, token: CancellationToken) {
        let Ok(mut shutdown) = self.shutdown.lock() else {
            return;
        };
        *shutdown = Some(token);
    }

    /// 서버를 끝낼 표시를 끊음. 끊기 규칙이 없으면 아무것도 하지 않음.
    pub fn cancel_shutdown(&self) {
        let Ok(shutdown) = self.shutdown.lock() else {
            return;
        };
        if let Some(token) = shutdown.as_ref() {
            token.cancel();
        }
    }

    /// 큐에 응답 스크립트를 추가함.
    pub fn push_script(&self, script: MockScript) {
        let Ok(mut scripts) = self.scripts.lock() else {
            return;
        };
        scripts.push_back(script);
    }

    /// 큐에서 응답 스크립트를 하나 뺌. 큐가 비면 404 응답.
    pub fn take_script(&self) -> MockScript {
        let Ok(mut scripts) = self.scripts.lock() else {
            return MockScript::with_status(404);
        };
        scripts
            .pop_front()
            .unwrap_or_else(|| MockScript::with_status(404))
    }
}
