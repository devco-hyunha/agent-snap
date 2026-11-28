//! 주석·공백을 보존하는 CST 편집 창구 자리.

use jsonc_parser::cst::CstObject;
use jsonc_parser::cst::CstRootNode;

use crate::jsonc::error::JsoncError;
use crate::jsonc::parse::parse_jsonc;
use crate::jsonc::parse::parse_options;

/// CST 편집 창구. 편집 함수 결과로 텍스트 반환.
/// 루트 노드 수명은 이 함수 안만: 바깥으로 내보내면 드롭 후 조상 약참조가 깨져 패닉.
/// 편집 결과는 해석 재통과 후에만 성공.
pub fn edit_jsonc<F>(text: &str, edit: F) -> Result<String, JsoncError>
where
    F: FnOnce(&CstObject),
{
    let root = CstRootNode::parse(text, &parse_options()).map_err(|e| JsoncError::Syntax {
        line: e.line_display(),
    })?;
    let Some(object) = root.object_value() else {
        return Err(JsoncError::RootNotObject);
    };
    edit(&object);
    let edited = root.to_string();
    drop(root);
    parse_jsonc(&edited)?;
    Ok(edited)
}
