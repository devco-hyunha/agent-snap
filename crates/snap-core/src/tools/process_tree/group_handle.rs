//! OS마다 같은 이름의 묶음 핸들을 골라 주는 자리
//!
//! `GroupHandle` 한 이름으로 내보내 `process_tree.rs`가 OS 구분 없이 묶음을 다룸

#[cfg(unix)]
pub use crate::tools::process_tree::unix_tree::ProcessGroup as GroupHandle;
#[cfg(windows)]
pub use crate::tools::process_tree::windows_tree::JobHandle as GroupHandle;
// 두 운영체제 밖 OS에는 묶음 타입만 둠. `ProcessTree::start`가 지원 없음으로 끝나 묶음이 만들어지지 않음
#[cfg(not(any(windows, unix)))]
#[derive(Debug)]
pub struct GroupHandle;
