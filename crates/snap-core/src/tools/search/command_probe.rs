//! 외부 프로그램이 실제로 동작하는지 한 번 보고 확인하는 자리입니다(03-6 R3 SRP 분리).
//!
//! 도구마다 외부 프로그램을 부르기에 앞서 같은 확인을 쓰므로, 이 한 곳에서만 만듭니다
//! (dev-principles 3절 DRY).

use std::process::Command;

/// 명령 한 번이 성공했는지 봅니다.
pub(crate) fn command_works(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .output()
        .is_ok_and(|out| out.status.success())
}
