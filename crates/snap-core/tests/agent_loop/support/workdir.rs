//! 03-5 루프 시험의 임시 작업 폴더 재료입니다.
//!
//! 루프는 설정의 작업 폴더에서 `AGENTS.md`를 읽으므로, 시험이 만든 폴더는 시험이 끝나면
//! 스스로 지워야 합니다. 만들기·정리 순서는 단위 시험 쪽 `TempFolder`(src/tools/tests/support.rs)와
//! 같은 규칙을 따릅니다. 단위 시험 트리는 `#[cfg(test)]` 안이라 통합 시험이 그 경로를 볼 수 없어
//! 이 트리에 같은 성질의 재료를 두고, 루프 시험은 이 한 곳만 씁니다.

#![allow(dead_code, unreachable_pub)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 시험용 임시 작업 폴더. 시험이 끝나면 폴더와 안의 파일을 지웁니다.
pub struct LoopTempWorkdir {
    path: PathBuf,
}

impl LoopTempWorkdir {
    /// 프로세스 번호와 순번을 붙여 임시 작업 폴더를 만듭니다(서로 다른 시험이 같은 폴더를 쓰지 않게).
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

    /// 임시 작업 폴더 안에 텍스트 파일을 씁니다.
    pub fn write(&self, name: &str, text: &str) {
        std::fs::write(self.path.join(name), text.as_bytes())
            .expect("임시 작업 폴더에 지시 파일을 쓸 수 있어야 합니다");
    }
}

impl Drop for LoopTempWorkdir {
    fn drop(&mut self) {
        // 정리 실패는 시험 판정에 영향을 주지 않으므로 넘깁니다.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
