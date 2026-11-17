//! 03-8 execute 브리지 길이 상한 시험입니다(AC 4 잘림·AC 6 길이 차단).
//!
//! 결론: 결과 요약 잘림과 코드 길이 차단만 봅니다. 기록·승인은 각 책임 파일이 봅니다.

use super::super::support::TempWorkdir;
use super::shared::run_execute;

/// AC 4 연장: 결과가 4,000자를 넘으면 03-2 잘림 표시로 자릅니다.
#[tokio::test]
async fn long_script_result_is_truncated_summary() {
    let workdir = TempWorkdir::new("bridge-truncate");
    let text = run_execute(
        &workdir,
        "const big = []; for (let i = 0; i < 700; i++) big.push('가나다라'); JSON.stringify(big);",
    )
    .await;
    let summary_line = text.split('\n').next().unwrap_or_default().to_owned();
    assert!(
        summary_line.chars().count() <= 4_000,
        "요약 상한: {}",
        summary_line.chars().count()
    );
    assert!(
        summary_line.contains("[truncated]"),
        "잘림 표시: {summary_line}"
    );
}

/// AC 6: 코드 길이 상한을 넘으면 엔진을 켜지 않고 짧은 오류만 돌려준다.
#[tokio::test]
async fn script_over_length_cap_is_refused() {
    let workdir = TempWorkdir::new("bridge-length-cap");
    let long_script = format!("const s = '{}'; s;", "가".repeat(70_000));
    let text = run_execute(&workdir, &long_script).await;
    assert!(text.contains("코드 길이 초과"), "길이 차단: {text}");
}
