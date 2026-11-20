//! 백그라운드로 옮긴 명령 한 건의 상태: 트리·읽기 태스크·읽은 출력·종료 코드
//!
//! wake 한 번은 트리가 끝나길 wake 주기만큼 기다리고, 읽은 조각을 순서대로 붙임

use std::time::Duration;

use tokio::task::JoinHandle;

use super::pipe_read::pipe_text;
use super::text::background_result_text;
use crate::tools::process_tree::ProcessTree;

/// 백그라운드로 옮긴 명령 한 건의 상태
pub struct BackgroundState {
    tree: ProcessTree,
    pid: Option<u32>,
    stdout: Option<JoinHandle<String>>,
    stderr: Option<JoinHandle<String>>,
    stdout_text: String,
    stderr_text: String,
    exit_code: Option<i32>,
}

impl BackgroundState {
    /// 만료 판이 넘기는 트리와 읽기 태스크로 백그라운드 상태 생성
    #[must_use]
    pub fn new(
        tree: ProcessTree,
        stdout: Option<JoinHandle<String>>,
        stderr: Option<JoinHandle<String>>,
    ) -> Self {
        let pid = tree.pid();
        Self {
            tree,
            pid,
            stdout,
            stderr,
            stdout_text: String::new(),
            stderr_text: String::new(),
            exit_code: None,
        }
    }

    /// 트리가 끝나 목록에서 빼는 자리인지 여부
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.exit_code.is_some()
    }

    /// 확인 한 번(wake)의 결과 1줄
    ///
    /// 트리가 wake 주기 안에 끝나면 종료 코드를 기록하고, 끝나지 않았으면 읽은 조각만 붙임
    pub async fn wake(&mut self, wake_interval: Duration) -> String {
        let code = tokio::select! {
            status = self.tree.wait() => status.ok().and_then(|status| status.code()),
            () = tokio::time::sleep(wake_interval) => None,
        };
        self.exit_code = code.or(self.exit_code);
        self.stdout_text
            .push_str(&pipe_text(&mut self.stdout, wake_interval).await);
        self.stderr_text
            .push_str(&pipe_text(&mut self.stderr, wake_interval).await);
        background_result_text(
            self.pid,
            &self.stdout_text,
            &self.stderr_text,
            self.exit_code,
        )
    }

    /// 백그라운드 목록에서 빼는 자리. 트리가 살아 있으면 묶음째 끝내기를 걸음
    pub async fn release(mut self) {
        if self.exit_code.is_none() {
            let _ = self.tree.terminate().await;
        }
    }
}

impl Drop for BackgroundState {
    fn drop(&mut self) {
        // 읽기 태스크는 트리가 묶음을 끝낸 뒤 끝나므로 드롭에서 마저 취소
        if let Some(stdout) = self.stdout.take() {
            stdout.abort();
        }
        if let Some(stderr) = self.stderr.take() {
            stderr.abort();
        }
    }
}
