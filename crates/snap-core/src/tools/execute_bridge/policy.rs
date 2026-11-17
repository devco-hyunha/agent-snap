//! 03-8 execute 브리지의 정책 값(허용 목록·승인 대상·길이·요약 상한)만 모는 자리입니다.
//!
//! 결론: 목록과 상한은 코드 전체가 아니라 이 한 파일에만 둡니다. 승인이 어디에 걸리는지,
//! 코드가 얼마나 길어야 하는지를 한 자리에서 확인해야 리뷰와 설정 변경이 쉽기 때문입니다
//! (dev-principles 2절 SRP, spec AC 2·AC 4).
//!
//! 값은 건드리지 않습니다. 1차 허용 목록 4종과 승인 대상 3종, 코드 길이 상한, 요약 상한은
//! spec.md가 정한 값이고, 이 파일의 이름을 가리키는 곳만 고쳐 씁니다.

/// execute 결과 요약의 상한 글자 수입니다(spec AC 4, 03-2 `truncate_tool_result` 정본).
pub const SCRIPT_SUMMARY_MAX_CHARS: usize = 4_000;

/// JS 코드가 담기는 글자 수의 허용 상한(바이트)입니다. 승인 없이 오래 도는 코드를 사전에 막습니다.
pub const SCRIPT_MAX_BYTES: usize = 64_000;

/// 03-8 1차 허용 목록(읽기 4개)입니다(spec.md 19행. 이름을 한 곳에 둡니다).
pub const READONLY_TOOLS: [&str; 4] = ["read_file", "grep", "glob", "list_dir"];

/// 03-8 1차에 승인 창구로 여는 이름입니다(spec AC 2·AC 5. L1 승인 대상과 승인을
/// 기다리는 쓰기를 같은 자리에 두기 위해 이름을 한 곳에 둡니다).
pub const GATED_TOOLS: [&str; 3] = ["write_file", "edit_file", "run_command"];

/// 워치독이 결과를 받는 대기 시간 여유(초, 시간 상한에 더합니다)입니다.
pub const WATCHDOG_EXTRA_SECS: u64 = 1;

/// 엔진이 남긴 future를 마저 굴릴 때 주는 여유 시간(초)입니다.
pub const WATCHDOG_GRACE_SECS: u64 = 2;

/// 프로미스를 기다릴 때 채우는 최대 겹 수입니다. 봉지 겹이 계속되는 코드를 막습니다.
/// 정본은 이 한 자리이고, JS 얅개는 이 숫자를 조립해 씁니다(dev-principles 3절 DRY, 보정 3).
pub const MAX_PROMISE_ROUNDS: usize = 16;

/// QuickJS가 코드의 결승 값과 `Async` 창구 함수값을 담는 자리 이름입니다(AC 4).
///
/// QuickJS는 봉지라 부르는 `{value: ...}` 자리에 값을 담아 넘깁니다. 값이 프로미스로
/// 남으면 요약이 `{}`가 되므로, 이 칸을 열어 안의 값을 꺼냅니다. JS 쪽 얅개(`js_bindings.rs`)
/// 도 같은 칸 이름을 쓰므로, 이 한 정의가 판정의 정본입니다(보정 3: DRY).
pub(in crate::tools) const SCRIPT_VALUE_KEY: &str = "value";

/// JS에 다는 창구 얅개의 JS 이름입니다(한 자리만 두어 4개와 `run_tool`이 같은 조립을 재사용).
pub(in crate::tools) const JS_WRAPPER_NAME: &str = "__snapTool";

/// 도구 매개를 JSON 글자로 바꾸는 얅개의 JS 이름입니다(AC 1의 매개 규격 한 자리, DRY).
pub(in crate::tools) const JS_ARG_NAME: &str = "__snapArg";

/// `Async` 창구를 붙인 원시 함수의 JS 이름 접두어입니다(코드가 부르는 이름은 1차 목록 그대로).
pub(in crate::tools) const JS_RAW_PREFIX: &str = "__snapRaw_";

/// `run_tool` 원시 함수의 JS 이름입니다.
pub(in crate::tools) const JS_RAW_RUN_TOOL_NAME: &str = "__snapRawRunTool";

/// JS가 도구를 불러 쓰는 이름 상수입니다(1차 목록 이름과 같은 자리).
pub(in crate::tools) const JS_RUN_TOOL_NAME: &str = "run_tool";
