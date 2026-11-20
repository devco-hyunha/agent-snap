//! 백그라운드 상태·wake 결과를 사람 눈에 보이는 텍스트로 만듦
//!
//! 잘림은 에이전트 루프가 적용하므로 이 자리에서는 조립만 함

use std::fmt::Write;

/// 백그라운드로 옮겼다는 표시 한 줄
#[must_use]
pub fn background_transition_line(pid: Option<u32>) -> String {
    format!("{}백그라운드로 옮겼습니다", background_status_prefix(pid))
}

/// wake 결과를 도구 결과 텍스트로 만듦
#[must_use]
pub fn background_result_text(
    pid: Option<u32>,
    stdout_text: &str,
    stderr_text: &str,
    code: Option<i32>,
) -> String {
    let state = code.map_or("실행 중".to_owned(), |code| {
        format!("완료: 종료 코드 {code}")
    });
    let mut text = background_status_prefix(pid);
    let _ = write!(text, "{state}");
    if !stdout_text.is_empty() {
        let _ = write!(text, "\n표준 출력:\n{stdout_text}");
    }
    if !stderr_text.is_empty() {
        let _ = write!(text, "\n표준 오류:\n{stderr_text}");
    }
    text
}

/// 백그라운드 상태 표시 앞부분
fn background_status_prefix(pid: Option<u32>) -> String {
    pid.map_or("백그라운드: ".to_owned(), |pid| {
        format!("백그라운드 pid {pid}: ")
    })
}
