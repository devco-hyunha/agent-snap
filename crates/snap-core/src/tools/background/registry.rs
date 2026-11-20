//! 백그라운드 목록: 백그라운드로 옮긴 명령을 담는 한 정의
//!
//! 목록 등록·확인·해제·재등록을 이 한 곳에만 두고, wake 한 번은 에이전트 턴 하나와 대응

use std::collections::VecDeque;

use tokio::task::JoinHandle;

use super::state::BackgroundState;
use super::text::background_transition_line;
use crate::tools::process_tree::ProcessTree;

/// 백그라운드로 옮긴 명령 목록(등록·확인·해제·재등록)
#[derive(Default)]
pub struct BackgroundRegistry {
    items: VecDeque<BackgroundState>,
}

impl BackgroundRegistry {
    /// 빈 목록 생성
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// 만료 판이 넘기는 트리와 읽기 태스크로 백그라운드 명령을 등록하고 전환 표시 한 줄을 넘김
    ///
    /// wake는 목록의 맨 앞 항목부터 확인
    pub fn register(
        &mut self,
        tree: ProcessTree,
        stdout: Option<JoinHandle<String>>,
        stderr: Option<JoinHandle<String>>,
    ) -> String {
        let pid = tree.pid();
        self.items
            .push_back(BackgroundState::new(tree, stdout, stderr));
        background_transition_line(pid)
    }

    /// wake 대상 한 개를 목록에서 빼서 가져옴. 목록이 비면 `None`
    #[must_use]
    pub fn take_for_wake(&mut self) -> Option<BackgroundState> {
        self.items.pop_front()
    }

    /// wake를 마친 명령을 목록 맨 앞에 다시 둠. 끝나지 않은 명령이 순서를 유지하게 함
    pub fn reinsert(&mut self, command: BackgroundState) {
        self.items.push_front(command);
    }

    /// wake 대상이 있는지 확인
    #[must_use]
    pub fn has_next(&self) -> bool {
        !self.items.is_empty()
    }

    /// 백그라운드로 옮긴 명령 수
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 백그라운드로 옮긴 명령이 없으면 wake를 부르지 않은 상태
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Drop for BackgroundRegistry {
    fn drop(&mut self) {
        // 목록이 비면 백그라운드 명령도 함께 끝나 트리가 묶음 종료 경로를 밟음
        drop(self.items.drain(..));
    }
}
