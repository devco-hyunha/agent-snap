//! 셸의 표준 출력·표준 오류를 읽는 태스크 자리
//!
//! 파이프가 막히면 읽기가 멈추므로 각 파이프를 별도 태스크로 읽음

use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::task::JoinHandle;

/// 표준 출력·표준 오류 읽기 태스크 생성
pub fn spawn_pipe_reader<R>(pipe: R) -> JoinHandle<String>
where
    R: AsyncRead + Send + Unpin + 'static,
{
    tokio::spawn(read_pipe_to_end(pipe))
}

/// 파이프를 끝까지 읽는 태스크 본문. 백그라운드 wake가 같은 정본을 씀
pub async fn read_pipe_to_end<R>(mut pipe: R) -> String
where
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::new();
    if pipe.read_to_end(&mut bytes).await.is_err() {
        return String::new();
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// 읽기 태스크 결과를 받음. 태스크가 죽으면 빈 문자열로 봄
pub async fn collect_pipe(task: Option<JoinHandle<String>>) -> String {
    match task {
        Some(task) => task.await.unwrap_or_default(),
        None => String::new(),
    }
}
