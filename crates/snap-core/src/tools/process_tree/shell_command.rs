//! 셸을 두 운영체제에서 같은 의미로 시작하는 `Command` 생성
//!
//! Windows는 `cmd.exe /C`, 그 밖의 운영체제는 `sh -c`를 쓰고 명령 문자열은 승인 화면에 그대로 보이는 값이므로 신뢰 창구는 승인자

use std::path::Path;
use std::process::Stdio;

use tokio::process::Command;

#[must_use]
pub fn shell_command(command: &str, root: &Path) -> Command {
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
