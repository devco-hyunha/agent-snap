//! T3 시험: `EndpointKind` 종류 책임. 6종 왕복과 목록 밖 거부만 봄.
use snap_core::llm::EndpointKind;

/// 설정 파일이 쓰는 이름 6종.
const NAMES: [(&str, EndpointKind); 6] = [
    ("openai", EndpointKind::Openai),
    ("openrouter", EndpointKind::Openrouter),
    ("ollama", EndpointKind::Ollama),
    ("vllm", EndpointKind::Vllm),
    ("llamacpp", EndpointKind::Llamacpp),
    ("lmstudio", EndpointKind::Lmstudio),
];

#[test]
fn names_list_covers_every_kind() {
    for (name, expected) in NAMES {
        let kind =
            EndpointKind::from_setting(name).unwrap_or_else(|| panic!("test accepts kind {name}"));

        assert_eq!(kind, expected, "name {name} maps to its kind");
        assert_eq!(kind.as_str(), name, "kind prints its name");
        assert_eq!(kind.to_string(), name, "display is the name");
    }
}

#[test]
fn names_array_has_six_entries() {
    assert_eq!(
        EndpointKind::NAMES.len(),
        NAMES.len(),
        "allowed names array has one entry per kind"
    );
}

#[test]
fn rejects_names_outside_list() {
    let outside = ["openAI", "OpenAI", "open ai", "", "openaicompat", "gpt"];
    for name in outside {
        assert_eq!(
            EndpointKind::from_setting(name),
            None,
            "name {name} is outside the allowed list"
        );
    }
}
