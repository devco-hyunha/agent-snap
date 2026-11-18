//! execute 브리지의 정책 값(허용 목록·승인 대상·길이·요약 상한)을 한 파일에 모음.
//!
//! 승인이 어디에 걸리는지와 코드 길이 기준을 한 자리에서 확인해야 리뷰·설정 변경이 쉬움.

/// execute 결과 요약의 상한 글자 수.
pub const SCRIPT_SUMMARY_MAX_CHARS: usize = 4_000;

/// JS 코드가 담기는 글자 수의 허용 상한(바이트). 승인 없이 오래 도는 코드를 사전에 차단.
pub const SCRIPT_MAX_BYTES: usize = 64_000;

/// 1차 허용 목록(읽기 4개).
pub const READONLY_TOOLS: [&str; 4] = ["read_file", "grep", "glob", "list_dir"];

/// 1차에 승인 창구로 여는 이름.
pub const GATED_TOOLS: [&str; 3] = ["write_file", "edit_file", "run_command"];

/// 워치독이 결과를 받는 대기 시간 여유(초, 시간 상한에 더함).
pub const WATCHDOG_EXTRA_SECS: u64 = 1;

/// 엔진이 남긴 future를 마저 굴릴 때 주는 여유 시간(초).
pub const WATCHDOG_GRACE_SECS: u64 = 2;

/// 프로미스를 기다릴 때 채우는 최대 겹 수. 봉지 겹이 계속되는 코드를 미리 차단.
/// JS 얅개는 이 숫자를 조립해 사용 → 상한 숫자 중복 없음.
pub const MAX_PROMISE_ROUNDS: usize = 16;

/// QuickJS가 코드의 결승 값과 `Async` 창구 함수값을 담는 자리 이름.
///
/// 값이 프로미스로 남으면 요약이 `{}`가 되므로 이 칸을 열어 안의 값을 꺼냄. JS 얅개도 같은 칸 이름을 사용 → 이 한 정의가 판정 정본.
pub(in crate::tools) const SCRIPT_VALUE_KEY: &str = "value";

/// JS에 다는 창구 얅개의 JS 이름. 4개와 `run_tool`이 같은 조립을 재사용.
pub(in crate::tools) const JS_WRAPPER_NAME: &str = "__snapTool";

/// 도구 매개를 JSON 글자로 바꾸는 얅개의 JS 이름.
pub(in crate::tools) const JS_ARG_NAME: &str = "__snapArg";

/// `Async` 창구를 붙인 원시 함수의 JS 이름 접두어.
pub(in crate::tools) const JS_RAW_PREFIX: &str = "__snapRaw_";

/// `run_tool` 원시 함수의 JS 이름.
pub(in crate::tools) const JS_RAW_RUN_TOOL_NAME: &str = "__snapRawRunTool";

/// JS가 도구를 불러 쓰는 이름 상수.
pub(in crate::tools) const JS_RUN_TOOL_NAME: &str = "run_tool";
