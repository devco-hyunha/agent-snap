//! 도구가 열어야 하는 실행 판과 그 스레드 공유 상태만 두는 자리
//!
//! JS 엔진(브리지)가 실행 판을 여러 스레드에서 씀 → 복제하면 같은 상태를 가리킴
//!
//! 백그라운드 목록 창구는 복제본마다 새로 만들지 않고 실행 판이 조립한 공유 창구를 복제본이 함께 가리킴

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::tools::approval::request::ApproverBox;
use crate::tools::approval::settings::ApprovalSettings;
use crate::tools::background::shared::SharedBackgroundRegistry;
use crate::tools::command::settings::RunCommandSettings;

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
    /// 명령 실행 시간 상한. 설정 값(`agent` 섹션)으로 채움
    pub run_command: RunCommandSettings,
    /// 승인 대기 시간 상한. 설정 값(`agent` 섹션)으로 채움
    pub approval: ApprovalSettings,
    /// 백그라운드 확인 주기. 설정 값(`agent` 섹션)으로 채움
    pub wake_interval: Duration,
    /// 백그라운드 목록 창구. 실행 판 조립이 한 정의에서만 만들고 복제본이 같은 자리를 가리킴
    pub background: SharedBackgroundRegistry,
    /// 취소 토큰과 승인 표식을 묶은 창구. 복제본이 같은 자리
    pub(in crate::tools) shared: Arc<SharedToolState>,
}

impl ToolContext {
    /// 승인자·시간 상한·백그라운드 목록을 기본값으로 채운 실행 판 생성
    pub fn new(working_dir: impl Into<PathBuf>, tool_result_max_chars: usize) -> Self {
        Self {
            working_dir: working_dir.into(),
            tool_result_max_chars,
            approver: None,
            run_command: RunCommandSettings::with_defaults(),
            approval: ApprovalSettings::with_defaults(),
            wake_interval: crate::config::AgentSettings::default().wake_interval_seconds,
            background: crate::tools::background::shared::shared_background_registry(),
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
