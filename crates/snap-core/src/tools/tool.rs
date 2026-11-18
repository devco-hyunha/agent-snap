//! `Tool` 트레이트 골격. 필드·메서드 이름은 고정
//!
//! JS 엔진(브리지)가 실행 판을 여러 스레드에서 씀 → 복제하면 같은 상태를 가리킴

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use tokio_util::sync::CancellationToken;

use super::approver::ApproverBox;
use super::run_command_settings::RunCommandSettings;

/// 도구가 열어야 하는 실행 판
///
/// `approver`는 승인자 자리이고 없으면 즉시 거부. 복제본은 같은 상태를 가리킴
#[derive(Clone)]
pub struct ToolContext {
    /// 도구 경로 검사가 허용할 기준 폴더
    pub working_dir: PathBuf,
    /// 도구 결과를 잘라 모델에 보낼 최대 글자 수
    pub tool_result_max_chars: usize,
    /// 쓰기·명령 도구가 넘기는 승인자 자리. 없으면 즉시 거부
    pub approver: Option<ApproverBox>,
    /// 승인 대기·실행 시간 상한. 설정 값(`agent` 섹션)으로 채움
    pub run_command: RunCommandSettings,
    /// 취소 토큰과 승인 표식을 묶은 창구. 복제본이 같은 자리
    pub(super) shared: Arc<SharedToolState>,
}

impl ToolContext {
    /// 승인자·시간 상한을 기본값으로 채운 실행 판 생성
    pub fn new(working_dir: impl Into<PathBuf>, tool_result_max_chars: usize) -> Self {
        Self {
            working_dir: working_dir.into(),
            tool_result_max_chars,
            approver: None,
            run_command: RunCommandSettings::with_defaults(),
            shared: Arc::new(SharedToolState {
                cancel: Mutex::new(CancellationToken::new()),
                approved: AtomicBool::new(false),
            }),
        }
    }
}

/// 실행 판이 스레드 여러 개에서 나누는 상태(취소 토큰·승인 표식)
#[derive(Default)]
pub(in crate::tools) struct SharedToolState {
    /// 취소 토큰. 승인 대기·cancel 자리에서 확인
    pub(in crate::tools) cancel: Mutex<CancellationToken>,
    /// 승인 표식. 승인자 응답이 승인한 호출 한 개에서만 참, 호출이 끝나면 거짓
    pub(in crate::tools) approved: AtomicBool,
}

/// 도구가 돌려주는 실행 결과
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolOutcome {
    /// 결과가 문자열 하나. 잘림은 루프가 적용
    Done(String),
    /// 승인자에게 넘길 명령 요약
    ApprovalRequired {
        /// 승인 화면에 보여주는 한 줄 요약
        summary: String,
    },
    /// 모델에 그대로 보내는 한 줄 오류(`오류: 이유` 형식)
    Error(String),
}

impl ToolOutcome {
    /// 오류 한 줄 생성. `오류: `를 앞에 붙임
    pub fn error(reason: &str) -> Self {
        Self::Error(format!("오류: {reason}"))
    }

    /// 모델에 보낼 결과 문자열 반환
    pub fn text(&self) -> &str {
        match self {
            Self::Done(text) | Self::Error(text) => text.as_str(),
            Self::ApprovalRequired { summary } => summary.as_str(),
        }
    }
}

/// 에이전트가 쓰는 도구가 지켜야 할 트레이트
pub trait Tool {
    /// 등록 이름. 루프가 `tool_calls`의 이름과 같은지 비교
    fn name(&self) -> &str;

    /// 모델에게 보이는 한 줄 설명
    fn description(&self) -> &str;

    /// 인자 JSON Schema 객체
    fn parameters_schema(&self) -> serde_json::Value;

    /// 쓰기·명령을 하는 도구인지 알림. 승인 구분 기준
    fn executes_writes(&self) -> bool {
        false
    }

    /// 인자 JSON 원문을 받아 도구를 실행
    ///
    /// 등록 목록이 `Box<dyn Tool>`로 트레이트 객체를 쓰려면 미래 타입을 상자로 고정해야 함
    fn call<'a>(&'a self, args_json: &'a str, ctx: &'a ToolContext) -> ToolCallFuture<'a>;
}

/// 구현체가 같은 미래 타입을 한곳에서 쓰게 하는 자리
pub type ToolCallFuture<'a> =
    std::pin::Pin<Box<dyn std::future::Future<Output = ToolOutcome> + Send + 'a>>;
