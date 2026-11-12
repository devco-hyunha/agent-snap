//! `Tool` 트레이트 골격입니다. 03-1 spec.md가 서명 정본이고, 필드·메서드 이름을 바꾸지 않습니다.
//!
//! 등록 목록과 변환은 `agent/registry.rs` 한 곳에만 있고(DRY, 03-1 등록 규칙),
//! 이 파일은 03-3·03-6이 만들 도구 구현의 골격인 트레이트와 실행 판만 둡니다.
//! 승인 표식·취소 토큰을 다루는 메서드는 `approval_state.rs`에 따로 있습니다(03-6 R1 감사 분리).

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use tokio_util::sync::CancellationToken;

use super::approver::ApproverBox;
use super::run_command_settings::RunCommandSettings;

/// 도구가 열어야 하는 실행 판입니다.
///
/// 작업 폴더는 도구 경로 검사의 기준이고, `tool_result_max_chars`는 결과를 잘리는 상한이며,
/// `approver`는 03-6이 채우는 승인자이고 없으면 즉시 거부이고, `run_command`는 03-6이 쓰는
/// 승인 대기·실행 시간 상한입니다. 02 취소 토큰은 루프가 도구 실행 창구로 넘깁니다.
pub struct ToolContext {
    /// 도구 경로 검사가 허용할 기준 폴더입니다.
    pub working_dir: PathBuf,
    /// 도구 결과를 잘라 모델에 보낼 최대 글자 수입니다(잘림 정본은 03-2 `truncate.rs`).
    pub tool_result_max_chars: usize,
    /// 쓰기·명령 도구가 넘기는 승인자 자리입니다. 03-6이 채우고 없으면 즉시 거부입니다.
    pub approver: Option<ApproverBox>,
    /// 03-6가 쓰는 승인 대기·실행 시간 상한입니다. 설정 값(`agent` 섹션)으로 채웁니다.
    pub run_command: RunCommandSettings,
    /// 02가 만든 취소 토큰입니다. 03-6이 승인 대기·명령 중단(cancel) 자리에서 듣습니다.
    pub(super) cancel: Mutex<CancellationToken>,
    /// 승인 표식입니다. 승인자 응답이 승인한 호출 한 개에서만 참이 되고, 그 호출이 끝나면 거짓으로 돌아옵니다.
    pub(super) approved: AtomicBool,
}

impl ToolContext {
    /// 승인자·시간 상한을 기본값으로 채운 실행 판을 만듭니다.
    pub fn new(working_dir: impl Into<PathBuf>, tool_result_max_chars: usize) -> Self {
        Self {
            working_dir: working_dir.into(),
            tool_result_max_chars,
            approver: None,
            run_command: RunCommandSettings::with_defaults(),
            cancel: Mutex::new(CancellationToken::new()),
            approved: AtomicBool::new(false),
        }
    }
}

/// 도구가 돌려주는 실행 결과입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolOutcome {
    /// 결과가 문자열 하나입니다. 잘림은 루프(03-2)가 적용합니다.
    Done(String),
    /// 승인자에게 넘길 명령 요약입니다. 03-6·03-8이 채웁니다.
    ApprovalRequired {
        /// 승인 화면에 보여주는 한 줄 요약입니다.
        summary: String,
    },
    /// 모델에 그대로 보내는 한 줄 오류입니다(`오류: 이유` 형식).
    Error(String),
}

impl ToolOutcome {
    /// 오류 한 줄을 만듭니다. `오류: `를 앞에 붙여 한 줄로만 만듭니다(03-1 결과 규격).
    pub fn error(reason: &str) -> Self {
        Self::Error(format!("오류: {reason}"))
    }

    /// 모델에 보낼 결과 문자열을 만들어 줍니다.
    pub fn text(&self) -> &str {
        match self {
            Self::Done(text) | Self::Error(text) => text.as_str(),
            Self::ApprovalRequired { summary } => summary.as_str(),
        }
    }
}

/// 에이전트가 쓰는 도구가 지켜야 할 트레이트입니다(서명은 03-1 spec.md 정본).
pub trait Tool {
    /// 등록 이름입니다. 루프가 `tool_calls`의 이름과 같은지 비교합니다.
    fn name(&self) -> &str;

    /// 모델에게 보이는 한 줄 설명입니다.
    fn description(&self) -> &str;

    /// 인자 JSON Schema 객체입니다.
    fn parameters_schema(&self) -> serde_json::Value;

    /// 쓰기·명령을 하는 도구인지 알립니다. 03-8의 승인 구분 기준이고, 03-6의 승인 대상은
    /// `run_command` 한 개입니다(D22 5.3절·D49).
    fn executes_writes(&self) -> bool {
        false
    }

    /// 인자 JSON 원문을 받아 도구를 실행합니다. 03-2(루프)가 1안 경로에서 부릅니다.
    ///
    /// 03-1은 `call` 이름과 매개변수를 정본으로 두고 매개체 미래 타입은 03-2가 정합니다.
    /// 등록 목록이 `Box<dyn Tool>`로 트레이트 객체를 쓰려면 미래 타입을 상자로 고정해야 합니다.
    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a>;
}

/// 03-3·03-6의 구현체가 같은 미래 타입을 한곳에서 쓰게 하는 자리입니다(DRY).
pub type ToolCallFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ToolOutcome> + Send + 'a>>;
