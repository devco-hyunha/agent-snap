//! 시험용 임시 폴더의 생성·경로·정리 재료
//!
//! 폴더 한 개 책임. 파일 쓰기·링크 생성은 같은 계통 파일에서 담당

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 시험용 임시 폴더. 시험이 끝나면 폴더와 안의 파일을 제거
pub struct TempFolder {
    pub(super) path: PathBuf,
}

impl TempFolder {
    /// 프로세스 번호와 순번을 붙여 임시 폴더 생성(서로 다른 시험이 같은 폴더를 쓰지 않게)
    pub fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "agent-snap-tools-test-{}-{unique}-{label}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("임시 폴더를 만들 수 있어야 합니다");
        Self { path }
    }

    /// 임시 폴더 경로
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        // 정리 실패는 시험 판정에 영향을 주지 않으므로 무시
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
