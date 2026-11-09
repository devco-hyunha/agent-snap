//! 03-2 등록 검사 시험: 등록 이름 중복 검사 한 자리와 `tools` 배열 변환(AC 1 등록 규칙).

use snap_core::agent::registry_to_definitions;

use crate::support::{registry_with_fixed, registry_with_fixed_twice};

/// 등록 규칙 단위 확인: 같은 이름이 두 개면 앞의 도구만 쓰고 뒤는 가려지므로, 검사 자리에서 막아야 합니다.
#[test]
fn duplicate_registered_name_is_found() {
    let registry = registry_with_fixed_twice("echo", "ok");
    let (name, position) = registry
        .find_duplicate_name()
        .expect("중복 이름을 찾아야 합니다");
    assert_eq!(name, "echo");
    assert_eq!(position, 1, "뒤에 온 순번을 가리켜야 합니다");
}

/// 등록 규칙 단위 확인: 이름이 서로 다면 중복이 없습니다.
#[test]
fn distinct_registered_names_have_no_duplicate() {
    let mut registry = registry_with_fixed("read", "ok");
    registry.register(Box::new(FixedNamedTool("write")));
    assert!(registry.find_duplicate_name().is_none());
}

/// 등록 변환 확인: 등록한 순서대로 OpenAI `tools` 배열 자리가 만들어집니다(AC 1 앞부분).
#[test]
fn registry_to_definitions_keeps_registration_order() {
    let mut registry = registry_with_fixed("read", "ok");
    registry.register(Box::new(FixedNamedTool("write")));
    let definitions = registry_to_definitions(&registry);
    let names: Vec<&str> = definitions.iter().map(|def| def.name.as_str()).collect();
    assert_eq!(names, ["read", "write"]);
}

/// 03-3 T1·T2 등록 확인: 파일 도구를 한 자리에서 조립하면 등록한 파일 도구가 순서대로
/// 등록됩니다.
#[test]
fn file_tools_registry_has_the_file_tools() {
    let registry = snap_core::agent::ToolRegistry::with_file_tools();
    let names: Vec<&str> = registry.tools().iter().map(|tool| tool.name()).collect();
    assert_eq!(
        names,
        [
            "read_file",
            "write_file",
            "edit_file",
            "list_dir",
            "glob",
            "grep"
        ]
    );
    assert!(registry.find_duplicate_name().is_none());
    for name in [
        "read_file",
        "write_file",
        "edit_file",
        "list_dir",
        "glob",
        "grep",
    ] {
        assert!(registry.contains(name), "{name}이 등록되어 있어야 합니다");
    }
}

/// 이름만 채운 최소 도구입니다. 변환·중복 검사 확인에 씁니다.
struct FixedNamedTool(&'static str);

impl snap_core::tools::tool::Tool for FixedNamedTool {
    fn name(&self) -> &str {
        self.0
    }

    fn description(&self) -> &str {
        "이름만 채운 최소 도구"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type":"object","properties":{}})
    }

    fn call<'a>(
        &'a self,
        _args_json: &'a str,
        _ctx: &'a snap_core::tools::tool::ToolContext,
    ) -> snap_core::tools::tool::ToolCallFuture<'a> {
        unimplemented!("등록 검사 시험이므로 실행하지 않습니다")
    }
}
