//! 03-8 execute 브리지 차단 시험입니다(AC 2 게이트 전 차단·spec 1차 목록).
//!
//! 결론: 1차 허용 목록 밖 이름과 쓰기·명령 이름이 JS에 걸리지 않는지 확인합니다.

use super::super::support::TempWorkdir;
use super::shared::run_execute;

/// 1차 허용 목록 밖 쓰기·명령 이름은 JS에 걸리지 않는다(spec.md 1차 목록).
#[tokio::test]
async fn write_and_command_names_are_not_exposed() {
    let workdir = TempWorkdir::new("bridge-readonly-list");
    let text = run_execute(
        &workdir,
        "JSON.stringify([typeof write_file, typeof edit_file, typeof run_command]);",
    )
    .await;
    assert!(
        text.contains("\"undefined\",\"undefined\",\"undefined\""),
        "미노출: {text}"
    );
}

/// AC 2: `run_tool` 창구도 1차 허용 목록 밖 이름은 같은 자리에서 막는다(승인 게이트 전 차단).
#[tokio::test]
async fn run_tool_blocks_names_outside_readonly_list() {
    let workdir = TempWorkdir::new("bridge-runtool-block");
    let text = run_execute(
        &workdir,
        "await run_tool('read_lines', JSON.stringify({path: 'x.txt'}));",
    )
    .await;
    assert!(text.contains("1차 허용 목록 밖"), "막힌 글자: {text}");
    assert!(
        !workdir.path().join("x.txt").exists(),
        "1차 목록 밖 이름은 실행되면 안 됩니다"
    );
}

/// AC 2 연장(라운드 3 minor 11): 매개 얅개가 순환 객체의 `JSON.stringify` 예외를 잡는다.
///
/// 순환 객체를 매개로 주면 엔진이 멈추지 않고 짧은 오류 글자로 끝나는지 봅니다.
#[tokio::test]
async fn circular_arg_fails_as_short_error_not_engine_halt() {
    let workdir = TempWorkdir::new("bridge-circular-arg");
    let text = run_execute(
        &workdir,
        "const loop = {}; loop.self = loop; await run_tool('read_file', loop);",
    )
    .await;
    assert!(
        text.starts_with("오류:") || text.contains("매개 JSON 변환 실패"),
        "짧은 오류로 끝나야 합니다: {text}"
    );
    assert!(
        text.chars().count() < 400,
        "짧은 글자: {}",
        text.chars().count()
    );
}
