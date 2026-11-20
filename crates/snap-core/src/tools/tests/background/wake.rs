//! 백그라운드 wake 시험: 끝난 명령은 목록에서 빠지고 실행 중 명령은 목록에 다시 남음 확인

use std::time::Duration;

use crate::tools::background::registry::BackgroundRegistry;
use crate::tools::process_tree::ProcessTree;

use super::super::shell::{echo_command, long_sleep_command};
use super::super::support::{TempFolder, spawn_pipe_reader};

/// wake 주기 안에 트리가 끝나면 목록에서 빠지고, 끝나지 않은 명령은 목록 맨 앞에 다시 남음
#[tokio::test]
async fn wake_remends_finished_command_and_keeps_running_one() {
    let folder = TempFolder::new("bg-wake");
    let mut registry = BackgroundRegistry::new();
    let mut short = ProcessTree::start(&echo_command("done"), folder.path())
        .expect("트리 시험 명령은 실행되어야 합니다");
    let (short_stdout, short_stderr) = short.take_pipes();
    let _ = registry.register(
        short,
        short_stdout.map(spawn_pipe_reader),
        short_stderr.map(spawn_pipe_reader),
    );
    let mut running = ProcessTree::start(&long_sleep_command("20"), folder.path())
        .expect("트리 시험 명령은 실행되어야 합니다");
    let (running_stdout, running_stderr) = running.take_pipes();
    let _ = registry.register(
        running,
        running_stdout.map(spawn_pipe_reader),
        running_stderr.map(spawn_pipe_reader),
    );

    let mut command = registry
        .take_for_wake()
        .expect("등록된 명령을 꺼내야 합니다");
    let outcome = command.wake(Duration::from_millis(2_000)).await;
    assert!(outcome.contains("완료: 종료 코드"), "{outcome}");
    assert!(command.is_finished(), "트리가 끝난 명령은 끝난 상태");
    let _ = command.release().await;

    let mut running_command = registry.take_for_wake().expect("둘째 명령을 꺼내야 합니다");
    let running_outcome = running_command.wake(Duration::from_millis(200)).await;
    assert!(running_outcome.contains("실행 중"), "{running_outcome}");
    assert!(
        !running_command.is_finished(),
        "긴 대기 명령은 아직 실행 중"
    );
    registry.reinsert(running_command);
    assert_eq!(
        registry.len(),
        1,
        "실행 중 명령이 목록에 다시 남아야 합니다"
    );
    assert!(registry.has_next(), "wake 대상이 남아 있어야 합니다");
}
