//! 읽기·쓰기·편집 시험이 쓰는 작업 폴더 재료
//!
//! 임시 폴더를 작업 폴더로 쓰고 같은 조립을 다시 적지 않게 한 정의로 둠

use std::path::Path;

use super::context::context_of;
use super::temp_folder::TempFolder;
use crate::tools::tool::ToolContext;

/// 읽기·쓰기·편집 시험이 같은 조립을 다시 적지 않게 쓰는 작업 폴더 재료
pub struct TempWorkdir(TempFolder);

impl TempWorkdir {
    /// 임시 폴더를 만들어 작업 폴더로 쓰는 타입
    pub fn new(label: &str) -> Self {
        Self(TempFolder::new(label))
    }

    /// 작업 폴더 경로
    pub fn path(&self) -> &Path {
        self.0.path()
    }

    /// 작업 폴더 안에 파일을 기록
    pub fn write(&self, name: &str, text: &str) {
        self.0.write_file(name, text);
    }

    /// 이 작업 폴더를 기준으로 한 실행 판 생성
    pub fn context(&self) -> ToolContext {
        context_of(&self.0)
    }
}
