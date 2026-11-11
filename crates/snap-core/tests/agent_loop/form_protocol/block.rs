use snap_core::agent::{RecoverOutcome, recover_text_call, text_protocol_block};

use crate::support::{FixedTool, registry_with_list_dir};

/// AC 2: 2안 지시 블록에 등록 도구 이름·인자 형식 글자가 있고, 20개 초과 안내가 붙는다.
#[test]
fn text_protocol_block_lists_tool_names_and_arguments() {
    let block = text_protocol_block(&registry_with_list_dir());
    assert!(block.contains("list_dir"), "도구 이름이 없습니다: {block}");
    assert!(block.contains("path"), "인자 형식 글자가 없습니다: {block}");
    assert!(
        block.contains("JSON"),
        "지시문에 JSON 글자가 없습니다: {block}"
    );
    let mut many = registry_with_list_dir();
    for index in 0..21 {
        many.register(Box::new(FixedTool {
            name: format!("tool_{index}"),
            output: "ok".to_owned(),
        }));
    }
    assert!(
        text_protocol_block(&many).contains("한 번에 노출하는 도구"),
        "20개 초과 안내가 없습니다"
    );
}

/// AC 3: Markdown 둘레·접두사·잘린 JSON 복원 3종과 모호한 객체 거부를 unit로 봅니다.
#[test]
fn text_recover_ladder_handles_three_shapes() {
    let markdown = "```json\n{\"name\":\"list_dir\",\"arguments\":{\"path\":\"docs\"}}\n```";
    let RecoverOutcome::Found { call, .. } = recover_text_call(markdown) else {
        panic!("Markdown 둘레를 복원해야 합니다");
    };
    assert_eq!(call.name, "list_dir");
    assert_eq!(call.arguments, r#"{"path":"docs"}"#);

    let with_prefix = "먼저 설명 글자 {\"name\":\"list_dir\",\"arguments\":{\"path\":\"docs\"}}";
    assert!(matches!(
        recover_text_call(with_prefix),
        RecoverOutcome::Found { .. }
    ));

    let cut = "```json\n{\"name\":\"list_dir\",\"arguments\":{\"path\":\"docs\"}";
    assert!(
        matches!(recover_text_call(cut), RecoverOutcome::Found { .. }),
        "잘린 JSON은 닫힘 기호 1회 복구로 복원해야 합니다"
    );

    assert!(matches!(
        recover_text_call("{\"foo\":1}"),
        RecoverOutcome::Unparsable
    ));
    assert!(matches!(
        recover_text_call("JSON 없음"),
        RecoverOutcome::Missing
    ));
}
