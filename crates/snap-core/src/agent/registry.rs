//! 도구 등록 목록(`ToolRegistry`): 등록·유지·조회만 맡습니다(03-1 등록 규칙).
//!
//! OpenAI `tools` 배열 변환은 `agent/definitions.rs`가 맡습니다(책임 분리).
//! 등록 이름 중복 검사는 이 파일 한 곳(`find_duplicate_name`)에 있습니다.

use crate::tools::edit_file::EditFileTool;
use crate::tools::read_file::ReadFileTool;
use crate::tools::tool::Tool;
use crate::tools::write_file::WriteFileTool;

/// 루프가 쓸 도구 등록 목록입니다. 등록한 순서대로 줄을 세우고 같은 구현 객체를 가리킵니다.
#[derive(Default)]
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    /// 빈 등록 목록을 만듭니다.
    #[must_use]
    pub const fn new() -> Self {
        Self { tools: Vec::new() }
    }

    /// 파일 읽기·쓰기·편집 도구를 등록한 목록을 만듭니다(03-3 T1).
    ///
    /// 파일 도구를 모으는 자리를 한 곳으로 정했습니다. 조립 코드를 여러 곳에 두면 이름과
    /// 순서가 갈라지므로, 이 한 곳에서만 읽기 → 쓰기 → 편집 순서를 만듭니다(DRY, dev-principles 3절).
    /// 등록 순서는 읽기 → 쓰기 → 편집이며, `find_duplicate_name` 검사로 막혀 있습니다.
    ///
    /// ```
    /// use snap_core::agent::ToolRegistry;
    /// let registry = ToolRegistry::with_file_tools();
    /// assert!(registry.contains("read_file"));
    /// ```
    #[must_use]
    pub fn with_file_tools() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(ReadFileTool));
        registry.register(Box::new(WriteFileTool));
        registry.register(Box::new(EditFileTool));
        registry
    }

    /// 도구를 한 개 추가합니다.
    pub fn register(&mut self, tool: Box<dyn Tool + Send + Sync>) {
        self.tools.push(tool);
    }

    /// 등록 목록 전체를 그대로 돌려줍니다.
    #[must_use]
    pub fn tools(&self) -> &[Box<dyn Tool + Send + Sync>] {
        &self.tools
    }

    /// 등록된 이름을 등록 순서대로 돌려줍니다.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| tool.name().to_owned())
            .collect()
    }

    /// 등록 목록이 비었는지 확인합니다.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// 이름이 등록 목록에 있는지 확인합니다. 없으면 `도구 없음`으로 알립니다.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.tools.iter().any(|tool| tool.name() == name)
    }

    /// 서로 같은 이름을 가진 첫 자리를 이름과 등록 순번으로 돌려줍니다.
    ///
    /// 루프 시작 전에 확인하는 중복 검사 한 곳입니다. 같은 이름이 두 개면 `find`가 앞의 도구만
    /// 찾아 뒤의 도구는 항상 가려지므로, 첫 요청을 보내기 전에 막아야 합니다.
    #[must_use]
    pub fn find_duplicate_name(&self) -> Option<(String, usize)> {
        let names = self.names();
        names
            .iter()
            .enumerate()
            .find(|(index, name)| names[..*index].iter().any(|first| first == *name))
            .map(|(index, name)| (name.clone(), index))
    }

    /// 이름으로 등록한 도구를 찾습니다.
    pub fn find(&self, name: &str) -> Option<&(dyn Tool + Send + Sync)> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(Box::as_ref)
    }
}
