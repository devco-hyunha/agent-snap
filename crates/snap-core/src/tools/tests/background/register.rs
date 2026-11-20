//! 백그라운드 목록 등록 시험: 등록이 표시 한 줄과 목록 잔존을 확인

use crate::tools::background::registry::BackgroundRegistry;
use crate::tools::process_tree::ProcessTree;

use super::super::shell::long_sleep_command;
use super::super::support::{TempFolder, spawn_pipe_reader};

/// 백그라운드 목록에 트리를 등록하고 옮겼다는 표시 한 줄을 받는지 확인
#[tokio::test]
async fn registry_register_reports_transition_line() {
    let folder = TempFolder::new("bg-register");
    let mut registry = BackgroundRegistry::new();
    let mut tree = ProcessTree::start(&long_sleep_command("20"), folder.path())
        .expect("트리 시험 명령은 실행되어야 합니다");
    let (stdout, stderr) = tree.take_pipes();
    let text = registry.register(
        tree,
        stdout.map(spawn_pipe_reader),
        stderr.map(spawn_pipe_reader),
    );
    assert!(text.contains("백그라운드로 옮겼습니다"), "{text}");
    assert_eq!(text.lines().count(), 1, "표시 한 줄만: {text}");
    assert_eq!(registry.len(), 1, "등록된 명령이 목록에 남아야 합니다");
    assert!(!registry.is_empty(), "등록 직후는 비어 있지 않음");
}
