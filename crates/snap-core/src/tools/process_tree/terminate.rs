//! 셸 트리 묶음을 한 번에 끝내는 자리
//!
//! OS 종료 경로를 한 파일에 모아 `ProcessTree`가 호출 순서만 유지하게 함

use std::io;

use super::ProcessTree;

/// 트리 묶음째 한 번에 끝냄(deadline 만료·cancel·프로그램 종료)
///
/// Windows는 Job Object 핸들에 종료 요청을 보내고, 셸 종료 확인은 묶음이 트리를 끝낸 뒤에 함
#[cfg(windows)]
pub async fn terminate(tree: &mut ProcessTree) -> io::Result<()> {
    let result = tree.group.terminate();
    // 셸 종료 확인은 묶음이 트리를 끝낸 뒤에 함. 결과 표기는 `run_command`가 담당
    let _ = tree.child.wait().await;
    result
}

/// 트리 묶음째 한 번에 끝냄(deadline 만료·cancel·프로그램 종료)
///
/// Unix는 셸을 먼저 끝낸 뒤 그룹에 남은 자식을 시그널로 끊고, 그룹이 사라지는지 확인함
#[cfg(unix)]
pub async fn terminate(tree: &mut ProcessTree) -> io::Result<()> {
    tree.group.terminate_shell_first(&mut tree.child).await
}

/// 묶음을 닫는 자리(프로그램 종료). Windows는 핸들만 닫아도 트리가 끝나 종료 요청을 걸지 않음
#[cfg(windows)]
pub fn close_group_on_drop(tree: &mut ProcessTree) {
    let _ = tree.group.terminate();
}

/// 묶음을 닫는 자리(프로그램 종료). 드롭은 async가 아니라 셸을 기다릴 수 없고, 셸 pid는 종료 뒤 재사용될 수 있어 그룹에만 시그널을 발송
#[cfg(unix)]
pub fn close_group_on_drop(tree: &mut ProcessTree) {
    tree.group.terminate_background();
}
