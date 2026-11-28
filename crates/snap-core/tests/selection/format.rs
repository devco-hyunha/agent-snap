//! T1 선택 키 시험: 형식 거부 책임. 구분자·빈 칸만 봄.

use snap_core::llm::SelectionFormatProblem;
use snap_core::llm::parse_selection_key;

#[test]
fn parses_two_part_key() {
    let key = parse_selection_key("local/qwen").expect("two part key parses");
    assert_eq!(key.endpoint, "local");
    assert_eq!(key.model, "qwen");
}

#[test]
fn trims_both_parts() {
    let key = parse_selection_key(" local / qwen ").expect("trim keeps parts");
    assert_eq!(key.endpoint, "local");
    assert_eq!(key.model, "qwen");
}

#[test]
fn keeps_extra_separator_in_model_part() {
    let key = parse_selection_key("local/a/b").expect("first separator splits");
    assert_eq!(key.endpoint, "local");
    assert_eq!(key.model, "a/b");
}

#[test]
fn rejects_key_without_separator() {
    let err = parse_selection_key("local").expect_err("separator is required");
    assert!(matches!(
        err,
        snap_core::llm::SelectionError::Format {
            problem: SelectionFormatProblem::MissingSeparator,
        }
    ));
}

#[test]
fn rejects_empty_endpoint_part() {
    let err = parse_selection_key(" /qwen").expect_err("endpoint part is empty");
    assert!(matches!(
        err,
        snap_core::llm::SelectionError::Format {
            problem: SelectionFormatProblem::EmptyPart,
        }
    ));
}

#[test]
fn rejects_empty_model_part() {
    let err = parse_selection_key("local/ ").expect_err("model part is empty");
    assert!(matches!(
        err,
        snap_core::llm::SelectionError::Format {
            problem: SelectionFormatProblem::EmptyPart,
        }
    ));
}
