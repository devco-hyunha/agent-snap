//! 2안 텍스트 프로토콜의 안내 글자를 만드는 자리입니다(03-4 spec AC 3·AC 4).
//!
//! 모델에게 돌려주는 짧은 오류·안내 문구의 정본을 이 한 파일에 둡니다. 문구를 고칠 때
//! 다른 파일까지 함께 찾지 않도록 한곳에만 모습니다(dev-principles 3절 DRY).
//! 기록 채움은 `record.rs`, messages 줄은 `messages.rs`가 맡습니다(관심사 분리).

use super::validate::CallProblem;
use crate::agent::registry::ToolRegistry;

/// 2안에서 JSON을 찾지 못했을 때 모델에게 돌려주는 짧은 오류 한 줄입니다(spec AC 3, 12번 9절 (a) 권고).
#[must_use]
pub const fn missing_json_notice() -> &'static str {
    "JSON을 찾지 못했어. 형식대로 다시 써줘"
}

/// 2안에서 여러 호출(2개 이상)이 왔을 때 첫 호출만 쓰고 돌려주는 안내입니다(범위 Out, spec 결과 전달 절).
#[must_use]
pub const fn multi_call_notice() -> &'static str {
    "한 번에 하나만 호출해줘"
}

/// 인자 형식이 어긋났을 때의 짧은 오류 한 줄입니다. `text_validate`가 만든 "필요한 것: …"을 씁니다(spec AC 4).
#[must_use]
pub fn argument_notice(reason: &str) -> String {
    format!("인자 형식이 어긋났어({reason})")
}

/// 등록에 없는 도구 이름에 대한 짧은 오류 한 줄입니다(spec AC 4, 사용 가능 목록을 함께 밝힘).
///
/// 이 한 자리에만 앞글자를 둡니다(리뷰 3차 2번 지적 보정). 정본을 한곳에 두어야 안내 문구를
/// 고칠 때 다른 파일까지 함께 고치는 중복이 사라집니다(dev-principles 3절 DRY).
#[must_use]
pub fn unregistered_notice(name: &str, registry: &ToolRegistry) -> String {
    format!("{} — `{name}`는 쓸 수 없어", unregistered_head(registry))
}

/// 검증 걸러짐 표기를 모델에 넘길 안내 글자로 바꾸는 자리입니다(spec AC 4, 글자 정본은 이 파일).
#[must_use]
pub fn problem_notice(name: &str, problem: &CallProblem, registry: &ToolRegistry) -> String {
    match problem {
        CallProblem::Unregistered => unregistered_notice(name, registry),
        CallProblem::Argument { reason } => argument_notice(reason),
    }
}

/// 사용 가능 목록까지 담은 미등록 안내 앞글자입니다(spec AC 4).
///
/// 도구가 하나도 없을 때의 대체 글자까지 한곳에서 정합니다.
fn unregistered_head(registry: &ToolRegistry) -> String {
    let usable = registry.names();
    let usable_text = if usable.is_empty() {
        "없음".to_owned()
    } else {
        usable.join(", ")
    };
    format!("등록되지 않은 도구야(사용 가능: {usable_text})")
}
