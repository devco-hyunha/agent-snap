//! OpenAI 호환 HTTP 스트리밍 전송입니다.
//!
//! `reqwest`는 `llm/transport/` 안의 파일에서만 씁니다. `snap-core` 공개면으로
//! 새지 않게 둡니다.

mod client;
mod client_debug;
mod client_settings;
pub(crate) mod connect_classify;
mod fetch_models;
mod open_stream;
pub(crate) mod os_error_codes;
mod send_headers;
mod send_once;
mod status_error;
mod stream_chat;
mod stream_state;

pub use client::OpenAiCompatClient;
