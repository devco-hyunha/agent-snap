//! 03-7 프로세스 트리 종료 시험(AC 1·AC 2·AC 4, spec 동작 규칙 1~5).
//!
//! 결론: 시험은 "자식 잔존 없음"을 마커 파일로 봅니다. 손자가 3초 뒤에 마커를 쓰게 하고,
//! 우리는 트리를 먼저 끝낸 뒤 5초를 더 기다려 마커가 없는지 확인한다.
//! 마커가 없다는 것은 손자가 중간에 끝나 트리가 묶음째 끝났다는 뜻이다(AC 1·AC 2).
//! 그룹 밖으로 나가는 경로는 시그널이 닿지 않음을 시험으로 표시한다(동작 규칙 5).

use std::path::Path;
use std::time::Duration;

use crate::tools::process_tree::ProcessTree;

use super::support::TempFolder;

/// 셸이 백그라운드로 넘긴 `setsid` 자식이 새 세션을 만들 시간을 들이는 상수다.
#[cfg(unix)]
const SPAWN_GRACE: Duration = Duration::from_secs(1);

/// 손자가 3초 뒤에 마커를 쓰는 2단 명령(자식의 자식까지 확인, AC 1).
///
/// Windows는 `start`로 만든 자식의 `ping`이 자식의 자식이고, 그 밖의 운영체제는
/// 백그라운드 `sleep`과 그 뒤에 쓰는 `echo`가 순서대로 이어진 2단 명령이다.
#[cfg(windows)]
fn grandchild_marker_command(marker: &str) -> String {
    format!(r#"start /b cmd /c "ping -n 4 -w 1000 127.0.0.1 >nul && echo leak > {marker}""#)
}

#[cfg(not(windows))]
fn grandchild_marker_command(marker: &str) -> String {
    format!("sleep 3; echo leak > {marker}")
}

/// 그룹 밖으로 나가는 경로 시험 재료(Unix만, `setsid`).
#[cfg(unix)]
fn outside_group_command(marker: &str) -> String {
    format!("setsid sh -c 'sleep 3; echo leak > {marker}' &")
}

/// deadline 만료·cancel 자리(`terminate`) 뒤에 손자를 포함한 트리가 남지 않는지 확인한다(AC 1).
#[tokio::test]
async fn terminate_leaves_no_grandchild() {
    let folder = TempFolder::new("tree-terminate");
    let marker = folder.path().join("leak.txt");
    let mut tree = ProcessTree::start(&grandchild_marker_command("leak.txt"), folder.path())
        .expect("트리 시험 명령은 실행되어야 합니다");
    tree.terminate().await.expect("트리 종료는 성공해야 합니다");
    assert_no_marker_for(&marker, Duration::from_secs(5)).await;
}

/// 프로그램 종료(판 드롭)에 Job Object·그룹이 닫혀 손자가 끝나 있다(AC 2).
#[tokio::test]
async fn drop_closes_tree_and_ends_grandchild() {
    let folder = TempFolder::new("tree-drop");
    let marker = folder.path().join("leak.txt");
    {
        let tree = ProcessTree::start(&grandchild_marker_command("leak.txt"), folder.path())
            .expect("트리 시험 명령은 실행되어야 합니다");
        drop(tree);
    }
    assert_no_marker_for(&marker, Duration::from_secs(5)).await;
}

/// 그룹 밖으로 나가는 경로가 시그널의 범위 밖임을 표시한다(동작 규칙 5, AC 3의 Unix 게이트).
///
/// `setsid`로 새 세션을 만든 자식은 우리 그룹 시그널이 닿지 않는다. 이 경로를 우리가
/// 책임지는 범위가 아니라는 표시이고, Windows는 Job Object가 기본으로 묶음 유지라 이 경로가 없다.
///
/// 셸이 백그라운드로 넘긴 `setsid` 자식이 새 세션을 만드는 데 걸리는 시간이 있다. 셸을
/// 종료하기 전에 그 시간을 먼저 들여야 시험이 확인하려는 상태(그룹 밖 자식)에 도달한다.
#[cfg(unix)]
#[tokio::test]
async fn group_outside_path_is_marked_not_killed() {
    let folder = TempFolder::new("tree-breakaway");
    let marker = folder.path().join("leak.txt");
    let mut tree = ProcessTree::start(&outside_group_command("leak.txt"), folder.path())
        .expect("트리 시험 명령은 실행되어야 합니다");
    tokio::time::sleep(SPAWN_GRACE).await;
    tree.terminate().await.expect("트리 종료는 성공해야 합니다");
    tokio::time::sleep(Duration::from_secs(5)).await;
    assert!(
        marker.exists(),
        "그룹 밖으로 나간 경로는 시그널이 닿지 않아 마커가 생겨야 합니다"
    );
}

/// 트리를 끝낸 시각에서 관찰 시간을 더 두고 마커 유무를 판정하는 한 자리다.
async fn assert_no_marker_for(marker: &Path, observe: Duration) {
    tokio::time::sleep(observe).await;
    assert!(
        !marker.exists(),
        "손자까지 트리가 끝나 마커가 생기면 안 됩니다: {}",
        marker.display()
    );
}
