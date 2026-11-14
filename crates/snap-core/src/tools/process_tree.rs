//! 03-7 프로세스 트리 종료 창구입니다(03-7 spec 동작 규칙 1~5, D22 신뢰성, AC 1~AC 3).
//!
//! 결론: 셸 명령을 실행하면서 그 프로세스를 플랫폼별 트리 묶음(Job Object · process group)에
//! 넣고, 만료·cancel·프로그램 종료 시점에 묶음째 끝냅니다.
//! 이 묶음은 자식 정리(수명)만 맞추는 용도이고, 권한 격리(D9 AppContainer)와 목적이 다릅니다
//! (D22 2.3절 35행·107행).
//!
//! Windows는 Job Object, 그 밖의 운영체제는 process group을 씁니다. OS 고유 코드는
//! `#[cfg(windows)]`·`#[cfg(unix)]`로 게이트를 채우고 같은 커밋에서 두 경로를 함께 봅니다
//! (개발 규칙 50행, spec AC 3). 묶음 밖으로 나가는 자식 경로는 Windows에서는 Job Object가
//! 막고(`windows_tree.rs`), Unix에서는 `setsid`로 나간 자리에 시그널이 닿지 않습니다.
//! 이 경로는 `group_outside_path_is_marked_not_killed` 시험으로 표시합니다(spec 동작 규칙 5).

use std::io;
use std::path::Path;
use std::process::Stdio;

use tokio::process::{Child, ChildStderr, ChildStdout, Command};

#[cfg(windows)]
mod windows_tree;

#[cfg(unix)]
mod unix_tree;

mod group_handle;

use group_handle::GroupHandle;

/// 셸 명령 프로세스 트리를 감싸는 실행 판입니다.
///
/// 트리에 넣은 채로 명령을 시작하고, [`ProcessTree::terminate`]가 묶음째 끝냅니다.
/// 판이 드롭되면 종료 요청을 백그라운드로 보냅니다(프로그램 종료, spec AC 2).
pub struct ProcessTree {
    child: Child,
    group: GroupHandle,
}

impl ProcessTree {
    /// 명령을 실행해 트리 묶음에 넣고 시작합니다(03-6 `run_command`가 호출, spec 동작 규칙 4).
    ///
    /// Windows는 셸 시작 전 Job Object를 만들어 두므로, 그 이후 셸이 만든 손자까지
    /// 묶음에 들어갑니다. Unix는 셸 프로세스만 그룹에 넣고, 그룹 밖으로 나간
    /// 자식은 [`ProcessTree::terminate`]가 셸을 끝낸 뒤 확인합니다.
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

    /// 트리 묶음째 한 번에 끝냅니다(deadline 만료·cancel·프로그램 종료, spec AC 1·AC 2).
    ///
    /// 이미 끝나면 요청만 보내고 기다립니다. 요청 결과를 붙이는 자리에서 이 결과를 적습니다.
    pub async fn terminate(&mut self) -> io::Result<()> {
        #[cfg(windows)]
        let result = {
            let result = self.group.terminate();
            // 셸 종료 확인은 묶음이 트리를 끝낸 뒤에 남는 자리이고, 여기서 받는 값은 종료
            // 코드일 뿐입니다. 결과 표기는 `run_command`의 `append_tree_note`가 종료 요청
            // 결과(`result`)로 한 자리에서 담당하므로 이 값은 버립니다.
            let _ = self.child.wait().await;
            result
        };
        #[cfg(unix)]
        let result = self.group.terminate_shell_first(&mut self.child).await;
        result
    }

    /// 표준 출력·표준 오류를 판에서 한 번씩만 뽑아줍니다(읽기 태스크가 파이프를 따로 읽게 한다).
    ///
    /// 03-6 `run_command`가 tokio 지침대로 두 파이프를 별도 태스크로 읽는 자리에서 씁니다.
    pub fn take_pipes(&mut self) -> (Option<ChildStdout>, Option<ChildStderr>) {
        (self.child.stdout.take(), self.child.stderr.take())
    }

    /// 명령이 끝나기를 기다립니다(정상 종료). cancel·만료는 [`ProcessTree::terminate`]를 씁니다.
    pub async fn wait(&mut self) -> io::Result<std::process::ExitStatus> {
        self.child.wait().await
    }

    /// 묶음을 닫는 자리입니다(프로그램 종료, spec AC 2, 동작 규칙 3).
    ///
    /// 드롭은 async 맥락이 아니므로 종료 요청만 백그라운드로 보냅니다. Windows는 Job Object
    /// 핸들(묶음의 수를 세는 창구)을 드롭에 넘기고, Unix는 기억한 그룹 번호(`pgid`)로 그룹만
    /// 향해 시그널을 보냅니다.
    #[cfg(windows)]
    fn close_group_on_drop(&mut self) {
        let _ = self.group.terminate();
    }

    /// 묶음을 닫는 자리입니다(프로그램 종료, spec AC 2, 동작 규칙 3).
    #[cfg(unix)]
    fn close_group_on_drop(&mut self) {
        self.group.terminate_background();
    }
}

impl Drop for ProcessTree {
    fn drop(&mut self) {
        self.close_group_on_drop();
    }
}

/// 셸을 두 운영체제에서 같은 의미로 시작하는 `Command`을 만듭니다(개발 규칙 이식성).
///
/// Windows는 `cmd.exe /C`, 그 밖의 운영체제는 `sh -c`를 씁니다. 03-6 `run_command`과
/// 같은 셸 규칙이고, 명령 문자열은 승인 화면에 그대로 보이는 값입니다
/// (dev-principles 8절: 신뢰할 수 없는 입력의 창구는 승인자).
fn shell_command(command: &str, root: &Path) -> Command {
    #[cfg(windows)]
    let mut cmd = {
        let mut cmd = Command::new("cmd.exe");
        cmd.arg("/C").arg(command);
        cmd
    };
    #[cfg(not(windows))]
    let mut cmd = {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(command);
        cmd
    };
    cmd.current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    cmd
}
