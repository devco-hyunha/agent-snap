//! 백그라운드 목록 창구 시험: 복제본이 같은 목록을 가리키는지 확인

use std::sync::Arc;

use super::super::support::{TempFolder, context_of};

/// 백그라운드 목록 창구가 실행 판 복제본에서 같은 자리를 가리킴
#[test]
fn shared_registry_is_shared_by_context_copies() {
    let folder = TempFolder::new("bg-shared");
    let base = context_of(&folder);
    let copy = base.clone();
    assert!(
        Arc::ptr_eq(&base.background, &copy.background),
        "복제본은 같은 백그라운드 목록 창구를 가리켜야 합니다"
    );
}
