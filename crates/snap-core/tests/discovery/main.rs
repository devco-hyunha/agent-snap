//! 02-7 T4 자동 발견 시험 묶음. 채움 성공·연결 실패·대상 아닌 경우·기본 주소 상수를 책임별 파일로 나눔.
//! 모의 서버 재료는 `mock_server` 시험과 같은 파일을 경로나열로 재사용.
#![allow(dead_code)]

#[path = "../mock_server/body.rs"]
mod body;
#[path = "../mock_server/connections.rs"]
mod connections;
#[path = "../mock_server/fixtures.rs"]
mod fixtures;
#[path = "../mock_server/recorder.rs"]
mod recorder;
#[path = "../mock_server/response.rs"]
mod response;
#[path = "../mock_server/router.rs"]
mod router;
#[path = "../mock_server/script.rs"]
mod script;
#[path = "../mock_server/server.rs"]
mod server;
#[path = "../mock_server/state.rs"]
mod state;
#[path = "../mock_server/stop_frame.rs"]
mod stop_frame;
#[path = "../mock_server/store.rs"]
mod store;
#[path = "../mock_server/stream_step.rs"]
mod stream_step;
#[path = "../mock_server/types.rs"]
mod types;
#[path = "../mock_server/usage_stream.rs"]
mod usage_stream;

mod constants;
mod fail;
mod fill;
mod list;
mod materials;
mod redact;
mod target;
