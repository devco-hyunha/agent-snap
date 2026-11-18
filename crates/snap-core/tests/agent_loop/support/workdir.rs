//! 루프 시험의 임시 작업 폴더 재료.
//!
//! 루프는 설정의 작업 폴더에서 `AGENTS.md`를 읽으므로, 시험이 만든 폴더는 끝나면 스스로 지워야 함.

#![allow(dead_code, unreachable_pub)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 시험용 임시 작업 폴더. 끝나면 폴더와 안의 파일을 지움.
pub struct LoopTempWorkdir {
    path: PathBuf,
}

impl LoopTempWorkdir {
    /// 프로세스 번호와 순번을 붙여 임시 작업 폴더를 생성(서로 다른 시험이 같은 폴더를 쓰지 않게).
    pub fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "agent-snap-loop-test-{}-{unique}-{label}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("임시 작업 폴더를 만들 수 있어야 합니다");
        Self { path }
    }

    /// 임시 작업 폴더 경로.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 임시 작업 폴더 안에 텍스트 파일을 씀.
    pub fn write(&self, name: &str, text: &str) {
        std::fs::write(self.path.join(name), text.as_bytes())
            .expect("임시 작업 폴더에 지시 파일을 쓸 수 있어야 합니다");
    }
}

impl Drop for LoopTempWorkdir {
    fn drop(&mut self) {
        // 정리 실패는 시험 판정에 영향을 주지 않으므로 넘김.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
