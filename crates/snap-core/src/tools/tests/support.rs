//! 파일 도구 시험 공용 재료: 임시 폴더를 만들고, 작업 폴더를 기준으로 한 실행 판을 만든다.
//!
//! 임시 폴더는 시험이 끝나면 이 파일의 정리 장치가 스스로 지웁니다.
//! 제품 코드 항목은 같은 크레이트 경로(`crate::`)로만 봅니다. 개발 의존과 붙은 사본이
//! 따로 놀아 타입 두 벌이 생기는 일을 막습니다.

#![allow(dead_code, unreachable_pub)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::agent::DEFAULT_TOOL_RESULT_MAX_CHARS;
use crate::tools::tool::ToolContext;

/// 시험용 임시 폴더. 시험이 끝나면 폴더와 안의 파일을 지웁니다.
pub struct TempFolder {
    path: PathBuf,
}

impl TempFolder {
    /// 프로세스 번호와 순번을 붙여 임시 폴더를 만듭니다(서로 다른 시험이 같은 폴더를 쓰지 않게).
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

    /// 임시 폴더 경로.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 임시 폴더 아래에 텍스트 파일을 쓰고 그 경로를 돌려줍니다(부모 폴더도 함께 만듭니다).
    pub fn write_file(&self, name: &str, text: &str) -> PathBuf {
        self.write_bytes(name, text.as_bytes())
    }

    /// 임시 폴더 아래에 바이트 그대로 파일을 쓰고 그 경로를 돌려줍니다.
    pub fn write_bytes(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let target = self.path.join(name);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).expect("임시 폴더 아래 폴더를 만들 수 있어야 합니다");
        }
        std::fs::write(&target, bytes).expect("임시 파일을 쓸 수 있어야 합니다");
        target
    }

    /// 임시 폴더 아래 파일을 읽어 돌려줍니다.
    pub fn read_file(&self, name: &str) -> String {
        std::fs::read_to_string(self.path.join(name))
            .unwrap_or_else(|error| panic!("임시 파일을 읽지 못했습니다: {error}"))
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        // 정리 실패는 시험 판정에 영향을 주지 않으므로 넘깁니다.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// 읽기·쓰기·편집 시험이 같은 조립을 다시 적지 않게 쓰는 작업 폴더 재료입니다.
pub struct TempWorkdir(TempFolder);

impl TempWorkdir {
    /// 임시 폴더를 만들고 작업 폴더 이름만 붙여 돌려줍니다.
    pub fn new(label: &str) -> Self {
        Self(TempFolder::new(label))
    }

    /// 작업 폴더 경로.
    pub fn path(&self) -> &Path {
        self.0.path()
    }

    /// 작업 폴더 안에 파일을 씁니다.
    pub fn write(&self, name: &str, text: &str) {
        self.0.write_file(name, text);
    }

    /// 이 작업 폴더를 기준으로 한 실행 판을 만듭니다.
    pub fn context(&self) -> ToolContext {
        context_of(&self.0)
    }
}

/// 임시 폴더를 작업 폴더로 정한 실행 판을 만듭니다. 잘림 상한은 03-2 정본 값을 씁니다.
#[must_use]
pub fn context_of(folder: &TempFolder) -> ToolContext {
    ToolContext::new(folder.path(), DEFAULT_TOOL_RESULT_MAX_CHARS)
}
