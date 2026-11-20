//! 명령 결과 텍스트를 도구 결과 문구로 조립하는 자리
//!
//! 잘림은 에이전트 루프가 적용하므로 이 자리에서는 조립만 함

/// 종료 코드·표준 출력·표준 오류를 도구 결과 텍스트로 만듦
#[must_use]
pub fn command_result_text(stdout_text: &str, stderr_text: &str, code: Option<i32>) -> String {
    let code_text = code.map_or("알 수 없음".to_owned(), |code| code.to_string());
    let mut text = format!("종료 코드: {code_text}");
    if !stdout_text.is_empty() {
        text.push_str("\n표준 출력:\n");
        text.push_str(stdout_text);
    }
    if !stderr_text.is_empty() {
        text.push_str("\n표준 오류:\n");
        text.push_str(stderr_text);
    }
    text
}
