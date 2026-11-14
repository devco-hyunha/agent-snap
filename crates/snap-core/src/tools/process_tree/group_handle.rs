//! OS마다 같은 이름의 묶음 핸들을 골라 주는 자리입니다(03-7, spec AC 3의 OS 게이트).
//!
//! 결론: `#[cfg(windows)]`에서는 Job Object 핸들을, `#[cfg(unix)]`에서는 process group 핸들을
//! `GroupHandle`이라는 한 이름으로 내보냅니다. 그래서 `process_tree.rs`가 OS를 구분하지 않고
//! 같은 창구로 묶음을 다루게 됩니다.

#[cfg(unix)]
pub use crate::tools::process_tree::unix_tree::ProcessGroup as GroupHandle;
#[cfg(windows)]
pub use crate::tools::process_tree::windows_tree::JobHandle as GroupHandle;
// 두 운영체제 밖의 OS에는 묶음 타입만 두는 자리로 둡니다(개발 규칙 지원 대상은 두 OS).
// 이 OS에서는 `ProcessTree::start`가 지원 없음 오류로 끝나므로 묶음이 만들어지지 않습니다.
#[cfg(not(any(windows, unix)))]
#[derive(Debug)]
pub struct GroupHandle;
