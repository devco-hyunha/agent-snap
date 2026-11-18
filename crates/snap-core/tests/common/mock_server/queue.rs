//! 왕복 여러 턴 시험용 스크립트 대기열
//!
//! '요청마다 앞 스크립트를 소모하는 대기열'만 담당. 꺼내 쓰는 기준은 `shared.rs`에 맞춤

use std::collections::VecDeque;
use std::sync::Arc;

use super::script::MockResponseScript;
use super::shared::Shared;

/// 스크립트 목록을 앞과 뒤로 갈라 기본 스크립트와 대기열로 나눔
///
/// 앞 스크립트부터 한 번씩 쓰고, 다 쓰면 마지막 스크립트를 그대로 씀
pub(super) fn split(
    scripts: Vec<MockResponseScript>,
) -> (MockResponseScript, VecDeque<MockResponseScript>) {
    let mut queue = VecDeque::from(scripts);
    let tail = queue
        .pop_back()
        .expect("왕복 스크립트는 하나 이상 있어야 합니다");
    (tail, queue)
}

/// 서버 공유 상태의 대기열에 남은 스크립트를 채움
///
/// 서버가 요청받기 전에 채워야 함. 안 채우면 첫 요청이 빈 기본 스크립트로 답하는 경쟁이 생김
pub(super) async fn fill(shared: &Arc<Shared>, queue: VecDeque<MockResponseScript>) {
    *shared.script_queue.lock().await = queue;
}
