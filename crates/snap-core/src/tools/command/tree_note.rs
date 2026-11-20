//! 명령을 끊는 자리에서 트리 종료 결과를 결과 문구에 붙이는 자리

use crate::tools::process_tree::ProcessTree;

/// 명령을 끊는 자리에서 트리를 끝낸 결과를 결과 문구에 붙임
///
/// 트리가 이미 끝나면 종료 요청이 실패하지만 그 경우는 지장이 없음
#[must_use]
pub async fn append_tree_note(text: &str, tree: &mut ProcessTree) -> String {
    let text = text.to_owned();
    match tree.terminate().await {
        Ok(()) => text,
        Err(_) => format!("{text}(트리 종료 표기 실패)"),
    }
}
