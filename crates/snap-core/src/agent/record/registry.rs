//! 도구 등록 목록(`ToolRegistry`): 등록·유지·조회만 담당
//!
//! 중복 이름 검사는 `find_duplicate_name` 한 곳에 정본으로 둠

use crate::tools::edit_file::EditFileTool;
use crate::tools::execute::ExecuteTool;
use crate::tools::glob::GlobTool;
use crate::tools::grep::GrepTool;
use crate::tools::list_dir::ListDirTool;
use crate::tools::read_file::ReadFileTool;
use crate::tools::run_command::RunCommandTool;
use crate::tools::tool::Tool;
use crate::tools::write_file::WriteFileTool;

/// 루프가 쓸 도구 등록 목록. 등록 순서대로 줄을 세우고 같은 구현 객체를 가리킴
#[derive(Default)]
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    /// 빈 등록 목록 생성
    #[must_use]
    pub const fn new() -> Self {
        Self { tools: Vec::new() }
    }

    /// 파일 읽기·쓰기·편집·목록·검색 도구와 `run_command`·`execute` 도구를 등록한 목록 생성
    ///
    /// 조립 코드를 여러 곳에 두면 이름과 순서가 갈라지므로 이 한 곳에서만
    #[must_use]
    pub fn with_agent_tools() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(ReadFileTool));
        registry.register(Box::new(WriteFileTool));
        registry.register(Box::new(EditFileTool));
        registry.register(Box::new(ListDirTool));
        registry.register(Box::new(GlobTool));
        registry.register(Box::new(GrepTool));
        registry.register(Box::new(RunCommandTool));
        registry.register(Box::new(ExecuteTool));
        registry
    }

    /// 도구 한 개 추가
    pub fn register(&mut self, tool: Box<dyn Tool + Send + Sync>) {
        self.tools.push(tool);
    }

    /// 등록 목록 전체를 그대로 반환
    #[must_use]
    pub fn tools(&self) -> &[Box<dyn Tool + Send + Sync>] {
        &self.tools
    }

    /// 등록된 이름을 등록 순서대로 반환
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| tool.name().to_owned())
            .collect()
    }

    /// 이름의 등록 여부 확인. 없으면 `도구 없음`으로 알림
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.tools.iter().any(|tool| tool.name() == name)
    }

    /// 서로 같은 이름을 가진 첫 자리를 이름과 등록 순번으로 반환
    ///
    /// 같은 이름이 있으면 `find`가 앞의 도구만 찾아 뒤의 도구가 가려지므로 첫 요청 전에 막는 검사
    #[must_use]
    pub fn find_duplicate_name(&self) -> Option<(String, usize)> {
        let names = self.names();
        names
            .iter()
            .enumerate()
            .find(|(index, name)| names[..*index].iter().any(|first| first == *name))
            .map(|(index, name)| (name.clone(), index))
    }

    /// 이름으로 등록한 도구 탐색
    pub fn find(&self, name: &str) -> Option<&(dyn Tool + Send + Sync)> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(Box::as_ref)
    }
}
