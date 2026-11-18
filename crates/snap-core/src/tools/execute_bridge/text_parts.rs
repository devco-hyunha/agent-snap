//! execute 브리지가 쓰는 글자 보조(줄이기·한 줄로 바꾸기)와 요약 조립.
//!
//! 기록 줄과 승인 화면 요약이 같은 글자 규칙(앞부분만 잘라 표시)을 따르므로 자르는 기준을 한 곳에서만 관리.

use crate::tools::tool::ToolOutcome;

/// 기록 줄 한 줄 규칙(줄바꿈 금지)에 맞춤.
pub(in crate::tools) fn one_line(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
}

/// 글자 수로 앞부분만 자름.
///
/// 표식(`…[truncated]`)을 붙이는 잘림 정본은 기록 줄 형식을 깨므로 사용하지 않음 → 표식 없는 글자 단위 자르기가 필요.
pub(in crate::tools) fn truncate_chars(text: &str, max_chars: usize) -> String {
    text.chars().take(max_chars).collect()
}

/// 호출이 부른 도구 결과를 호출이 쓸 글자로 변환.
#[must_use]
pub(in crate::tools) fn outcome_text(outcome: &ToolOutcome) -> String {
    match outcome {
        ToolOutcome::Done(text) | ToolOutcome::Error(text) => text.clone(),
        ToolOutcome::ApprovalRequired { summary } => {
            ToolOutcome::error(&format!("승인 필요: {summary}"))
                .text()
                .to_owned()
        }
    }
}

/// 호출 전체 확인 승인의 화면 요약 생성.
#[must_use]
pub(in crate::tools) fn script_approval_summary(source: &str) -> String {
    format!(
        "execute 호출(스크립트 {}자): {}",
        source.chars().count(),
        truncate_chars(source, 100)
    )
}

/// 쓰기·명령 호출의 승인 화면 요약 생성.
#[must_use]
pub(in crate::tools) fn write_summary(name: &str, args_json: &str) -> String {
    format!("execute 안 {name} 호출: {}", truncate_chars(args_json, 100))
}
