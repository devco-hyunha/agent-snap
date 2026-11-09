//! 왕복 여러 턴 시험용 스크립트 대기열 책임입니다.
//!
//! 서버 수명(`server`)은 빌려주지 않고, 이 파일은 '요청마다 앞 스크립트를 소모하는 대기열'만 다룹니다.
//! 대기열을 어떻게 꺼내 쓰는지(shared.rs)의 기준도 여기 정본에 맞춥니다.

use std::collections::VecDeque;
use std::sync::Arc;

use super::script::MockResponseScript;
use super::shared::Shared;

/// 스크립트 목록을 앞과 뒤로 갈라, 기본 스크립트와 대기열로 나눕니다.
///
/// 왕복 여러 턴 시험은 앞 스크립트부터 한 번씩 쓰고, 다 쓰면 마지막 스크립트를 그대로 씁니다.
/// 목록이 비면 요청을 한 번도 못 받았다는 뜻이므로 기본 스크립트를 정할 수 없습니다.
pub(super) fn split(
    scripts: Vec<MockResponseScript>,
) -> (MockResponseScript, VecDeque<MockResponseScript>) {
    let mut queue = VecDeque::from(scripts);
    let tail = queue
        .pop_back()
        .expect("왕복 스크립트는 하나 이상 있어야 합니다");
    (tail, queue)
}

/// 서버 공유 상태의 대기열에 남은 스크립트를 채웁니다.
///
/// 서버가 요청받기 전에 채워야 합니다. 띄운 뒤에 채우면 첫 요청이 대기열이 빈 기본 스크립트로
/// 답하는 짧은 경쟁이 생깁니다(`server.rs`의 `spawn_queue`가 이 순서를 지킵니다).
pub(super) async fn fill(shared: &Arc<Shared>, queue: VecDeque<MockResponseScript>) {
    *shared.script_queue.lock().await = queue;
}
