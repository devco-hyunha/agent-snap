//! 셸 명령 프로세스를 플랫폼별 트리 묶음(Job Object·process group)에 넣고, 만료·cancel·프로그램 종료 시점에 묶음째로 끝냄
//!
//! OS 고유 코드는 `#[cfg(windows)]`·`#[cfg(unix)]`로 게이트를 채워 두 경로를 함께 봄

use std::io;
use std::path::Path;

use tokio::process::{Child, ChildStderr, ChildStdout};

#[cfg(windows)]
mod windows_tree;

#[cfg(unix)]
mod unix_tree;

mod group_handle;
mod shell_command;
mod terminate;

use group_handle::GroupHandle;
use shell_command::shell_command;

/// 셸 명령 프로세스 트리를 감싸는 실행 판
///
/// 트리에 넣은 채로 명령을 시작하고, [`ProcessTree::terminate`]가 묶음째로 끝냄
pub struct ProcessTree {
    child: Child,
    group: GroupHandle,
}

impl ProcessTree {
    /// 명령을 실행해 트리 묶음에 넣고 시작
    ///
    /// Windows는 셸 시작 전 Job Object를 만들어 두므로 손자까지 묶음에 들어감. Unix는 셸 프로세스만 그룹에 넣음
    pub fn start(command: &str, root: &Path) -> io::Result<Self> {
        let mut cmd = shell_command(command, root);
        #[cfg(windows)]
        {
            let group = GroupHandle::create()?;
            let child = cmd.spawn()?;
            group.assign(child.id())?;
            Ok(Self { child, group })
        }
        #[cfg(unix)]
        {
            cmd.process_group(0);
            let child = cmd.spawn()?;
            let group = GroupHandle::from_shell(child.id())?;
            Ok(Self { child, group })
        }
        #[cfg(not(any(windows, unix)))]
        {
            let _ = cmd;
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "이 운영체제에서는 셸 명령을 지원하지 않습니다",
            ))
        }
    }

    /// 트리 묶음째 한 번에 끝냄(deadline 만료·cancel·프로그램 종료)
    ///
    /// 요청 결과를 붙이는 자리에서 이 결과를 적음. 종료 경로는 [`terminate`]가 담당
    pub async fn terminate(&mut self) -> io::Result<()> {
        terminate::terminate(self).await
    }

    /// 표준 출력·표준 오류를 한 번만 뽑아냄. 읽기 태스크가 파이프를 따로 읽게 함
    pub fn take_pipes(&mut self) -> (Option<ChildStdout>, Option<ChildStderr>) {
        (self.child.stdout.take(), self.child.stderr.take())
    }

    /// 명령이 끝나기를 대기(정상 종료). cancel·만료는 [`ProcessTree::terminate`] 사용
    pub async fn wait(&mut self) -> io::Result<std::process::ExitStatus> {
        self.child.wait().await
    }

    /// 셸 프로세스의 식별 번호. 백그라운드 목록이 확인 결과를 가르는 데 씀
    #[must_use]
    pub fn pid(&self) -> Option<u32> {
        self.child.id()
    }
}

impl Drop for ProcessTree {
    fn drop(&mut self) {
        terminate::close_group_on_drop(self);
    }
}
