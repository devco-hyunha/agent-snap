//! 03-8 execute 브리지가 쓰는 글자 보조(줄이기·한 줄로 바꾸기)와 요약 조립만 둡니다.
//!
//! 결론: 기록 줄과 승인 화면 요약은 같은 글자 규칙(앞부분만 잘라 표시)을 따르므로,
//! 이 한 파일의 보조만 반복해 씁니다. 자르는 기준을 한 곳에서만 고치면 기록 줄 길이가
//! 함께 맞아들어갑니다(dev-principles 3절 DRY·2절 SRP).

use crate::tools::tool::ToolOutcome;

/// 기록 줄 한 줄 규칙(줄바꿈 금지)에 맞추는 자리입니다.
pub(in crate::tools) fn one_line(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
}

/// 글자 수로 앞부분만 자릅니다(바이트가 아니라 문자 수로 셈).
///
/// 03-2 잘림 정본(`agent::record::truncate::truncate_tool_result`)은 도구 결과를 모델에
/// 보낼 때 쓰는 자리이고 표식을 붙입니다. 이 자리는 기록 줄과 승인 화면 요약처럼
/// 한 줄에 담는 부분만 자르는 자리라 표식이 필요 없습니다.
///
/// 03-2 잘림과 같은 성질(문자 수로 앞만 자르기)이라 이 자리에만 쓰는 보조를 다시
/// 만들지 않으려 재사용 사다리 1단계를 검색했습니다(dev-principles 3절, 보정 3).
/// 검색 결과는 라운드 보고에 적었고, 표식을 붙이는 03-2 정본을 기록 줄에 쓰면
/// `[execute][i/total]` 형식에 `…[truncated]`가 섞여 spec AC 3의 기록 형식이 깨집니다.
/// 그래서 표식 없는 글자 단위 자르기가 필요해 이 보조를 두었습니다(보정 3, 라운드 3 minor 8).
pub(in crate::tools) fn truncate_chars(text: &str, max_chars: usize) -> String {
    text.chars().take(max_chars).collect()
}

/// 호출이 부른 도구 결과를 호출이 쓸 글자로 넘기는 자리입니다.
///
/// 03-1 결과 규격대로 도구는 결과를 자르지 않으므로, 잘림 정본은 03-2(루프)를 씁니다.
/// 기록 칸에 담는 요약만 이 자리에서 200자로 줄입니다.
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

/// 호출 전체 확인 1승인의 화면 요약을 만드는 자리입니다(AC 5: 승인 로그에 남는 요약).
#[must_use]
pub(in crate::tools) fn script_approval_summary(source: &str) -> String {
    format!(
        "execute 호출(스크립트 {}자): {}",
        source.chars().count(),
        truncate_chars(source, 100)
    )
}

/// 쓰기·명령 호출의 승인 화면 요약을 만드는 한 자리입니다(AC 5: 승인 로그에 남는 요약).
#[must_use]
pub(in crate::tools) fn write_summary(name: &str, args_json: &str) -> String {
    format!("execute 안 {name} 호출: {}", truncate_chars(args_json, 100))
}
