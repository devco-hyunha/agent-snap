//! 02-5 시험 묶음. 종류·트레이트·오류 타입·설정 검증 연결·본문 조립·변환·병합·모의 서버 전송 책임을 각각의 파일로 둠.
//! 모의 서버 재료는 `mock_server` 시험과 같은 파일을 경로나열로 재사용. 이 시험이 쓰는 부분만 확인하므로 재료의 남은 부분은 죽은 코드로 남김.
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
#[path = "../mock_server/script.rs"]
mod script;
#[path = "../mock_server/server.rs"]
mod server;
#[path = "../mock_server/store.rs"]
mod store;
#[path = "../mock_server/types.rs"]
mod types;
#[path = "../mock_server/usage_stream.rs"]
mod usage_stream;

mod cancel;
mod connect_kinds;
mod connect_redact;
mod error;
mod extra_merge;
mod kind;
mod mock_request;
mod reasoning;
mod redact;
mod request;
mod request_body;
mod restore;
mod restore_timings;
mod setting_link;
mod stream_contract;
mod stream_echo;
mod stream_open;
mod stream_payload;
mod stream_redact;
mod stream_transport;
mod timeout;
mod timeout_connect;
mod timings;
mod traits;
mod usage;
