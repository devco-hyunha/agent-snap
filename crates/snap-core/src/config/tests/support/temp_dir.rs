//! 임시 폴더 도움: 테스트가 끝나면 지워지는 폴더와 그 안의 파일 읽기·쓰기·확인.
//!
//! 저장이 파일을 옮기는 시험들이 같은 폴더 준비를 다시 적지 않게 이 곳에서 부릅니다.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 테스트가 끝나면 지워지는 임시 폴더입니다.
pub(crate) struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// 이 프로세스 안에서 겹치지 않는 임시 폴더를 만듭니다.
    pub(crate) fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "agent-snap-test-{}-{unique}-{label}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|error| panic!("임시 폴더를 만들지 못했습니다: {error}"));
        Self { path }
    }

    /// 폴더 경로입니다.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// 폴더 안의 텍스트 파일을 읽어 돌려줍니다.
    pub(crate) fn read(&self, file: &str) -> String {
        std::fs::read_to_string(self.path.join(file))
            .unwrap_or_else(|error| panic!("임시 파일을 읽지 못했습니다: {error}"))
    }

    /// 폴더 안에 파일을 씁니다.
    pub(crate) fn write(&self, file: &str, text: &str) {
        std::fs::write(self.path.join(file), text)
            .unwrap_or_else(|error| panic!("임시 파일을 쓰지 못했습니다: {error}"));
    }

    /// 폴더에 남은 `.tmp` 파일이 하나도 없는지 확인합니다.
    pub(crate) fn assert_no_temp_files(&self) {
        let leftovers: Vec<String> = std::fs::read_dir(self.path())
            .expect("임시 폴더를 읽을 수 있어야 합니다")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".tmp"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "임시 파일이 남으면 안 됩니다: {leftovers:?}"
        );
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // 정리 실패는 테스트 결과에 영향이 없으므로 무시합니다.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
