//! 02-7 T3 모델 목록 시험 묶음. 재료·정상 id 추출·형식 오류·HTTP 오류·경로를 책임별 파일로 나눔.
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

mod auth;
mod body_read;
mod format;
mod ids;
mod materials;
mod path;
mod status;
