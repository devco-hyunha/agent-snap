//! process group 핸들: 그 밖의 운영체제(Linux)에서 셸 명령과 그 자식을 한 번에 끝내는 창구(03-7).
//!
//! 결론: 셸을 새 그룹으로 시작하고(process_tree의 `process_group(0)`), 그 그룹 번호를 기억해
//! 만료·cancel 때 그룹 전체에 시그널을 보낸다. 그룹에 남은 자식은 끝나기까지 확인한다(spec 동작 규칙 2).
//! 셸이 `setsid`로 만든 손자처럼 그룹 밖으로 나간 경로는 시그널이 닿지 않는다(동작 규칙 5).
//! 이 창구는 자식 정리(수명)만 맞추는 용도이고, 권한 격리(D9)와 목적이 다르다(D22 2.3절 35행).

use std::io;
use std::time::Duration;
use std::time::Instant;

use tokio::process::Child;

/// 그룹 번호를 기억하는 핸들입니다(Job Object와 같은 구실을 한다).
///
/// 셸은 새 그룹의 선두 pid로 시작하므로 그룹 번호가 셸 pid와 같습니다. 셸을 끝낸 뒤에도
/// 그룹이 남아 있으면 잔존으로 보고 확인합니다.
pub struct ProcessGroup {
    pgid: libc::pid_t,
}

impl ProcessGroup {
    /// 셸을 새 그룹으로 시작하고 그 그룹 번호를 기억합니다.
    ///
    /// 셸 pid는 그룹 선두 pid로 쓰입니다. 셸이 `setsid`로 그룹을 따로 만드는 경로는
    /// 그룹 밖으로 나가는 경로로 취급합니다(spec 동작 규칙 5에 표시한 자리).
    pub fn from_shell(shell_pid: Option<u32>) -> io::Result<Self> {
        // tokio가 pid를 주지 않는 OS에서는 그룹 번호를 알 수 없으므로 오류로 먼저 돌려준다.
        let Some(pid) = shell_pid else {
            return Err(io::Error::other("셸 프로세스 번호가 없습니다"));
        };
        // 그룹 번호는 운영체제가 정한 수(i32)로만 씁니다. tokio의 셸 pid가 범위를 넘으면
        // 그룹 시그널을 보낼 수 없으므로 오류로 먼저 돌려줍니다.
        let Ok(pgid) = libc::pid_t::try_from(pid) else {
            return Err(io::Error::other(
                "셸 프로세스 번호가 그룹 번호 범위를 넘었습니다",
            ));
        };
        Ok(Self { pgid })
    }

    /// 셸을 먼저 끝내고 그룹 전체를 시그널로 끊습니다(spec 동작 규칙 2·동작 규칙 5).
    ///
    /// 셸을 먼저 끝낸 뒤 그룹에 남은 자식을 시그널로 끊고, 그룹이 사라지는지 확인합니다.
    /// 셸이 자식을 그룹과 나눈 경로에서는 셸 종료 뒤에 그룹 시그널로 자식을 거둡니다.
    pub async fn terminate_shell_first(&mut self, shell_child: &mut Child) -> io::Result<()> {
        let _ = shell_child.kill().await;
        let _ = shell_child.wait().await;
        self.kill_group(libc::SIGKILL)?;
        self.wait_group_gone();
        Ok(())
    }

    /// 그룹을 끝내는 요청만 백그라운드로 보냅니다(프로그램 종료, spec AC 2).
    ///
    /// 드롭은 async 맥락이 아니라 셸을 기다릴 수 없습니다. 셸 pid는 종료 뒤 재사용될 수
    /// 있어 셸 pid 단독 종료는 쓰지 않고 그룹에만 시그널을 보냅니다.
    pub fn terminate_background(&self) {
        let _ = self.kill_group(libc::SIGKILL);
    }

    /// 그룹에 속한 모든 프로세스(남은 자식 포함)에 시그널을 보냅니다.
    fn kill_group(&self, signal: libc::c_int) -> io::Result<()> {
        // 시그널 0은 존재 확인만 하는 요청이라 그룹이 남아 있는지 보는 데 쓸 수 있습니다.
        let sent = unsafe { libc::killpg(self.pgid, signal) };
        if sent == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        // 그룹이 이미 사라진 뒤의 잔존 없음(ESRCH)은 정리가 끝난 것으로 봅니다.
        if error.raw_os_error() == Some(libc::ESRCH) {
            return Ok(());
        }
        Err(error)
    }

    /// 그룹이 사라지는지 확인합니다(spec 동작 규칙 2의 "끝까지 확인").
    ///
    /// 시그널 0 요청을 보내는 방식이고, 시그널을 받고도 죽지 않는 자식에 대비해 상한만 둡니다.
    fn wait_group_gone(&self) {
        let deadline = Instant::now() + GROUP_GONE_WAIT;
        while self.kill_group(0).is_ok() {
            if deadline.elapsed() >= GROUP_GONE_WAIT {
                return;
            }
            std::thread::sleep(GROUP_POLL_INTERVAL);
        }
    }
}

/// 그룹 잔존 확인을 기다리는 상한입니다. 죽지 않는 자식 앞에서 무한대기를 막습니다.
const GROUP_GONE_WAIT: Duration = Duration::from_millis(2_000);

/// 그룹 잔존 확인 간격입니다. 시그널 0은 존재 확인만 하는 요청이라 가볍게 돌면 됩니다.
const GROUP_POLL_INTERVAL: Duration = Duration::from_millis(20);
