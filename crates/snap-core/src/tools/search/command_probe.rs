//! 외부 프로그램이 실제로 동작하는지 확인하는 자리

use std::process::Command;

/// 명령 한 번이 성공했는지 확인
pub(crate) fn command_works(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .output()
        .is_ok_and(|out| out.status.success())
}
