//! 클라이언트 골격 자리. 트레이트·타입·전송 파일을 선언하고 재수출함.

mod auth;
mod body;
mod connect;
mod connect_kind;
mod effort;
mod endpoint;
mod error;
mod kind;
mod known_secrets;
mod open;
mod read;
mod reasoning;
mod receiver;
mod redact;
mod request;
mod status;
mod stream;
mod timings;
mod token_times;
mod traits;
mod transport;

pub use auth::auth_header;
pub use body::build_body;
pub use endpoint::completions_url;
pub use error::TransportError;
pub use kind::EndpointKind;
pub use receiver::RestoreReceiver;
pub use request::ChatMessage;
pub use request::ChatRequest;
pub use request::ReasoningEffort;
pub use stream::StreamOutcome;
pub use stream::StreamTimings;
pub use timings::TransportTimings;
pub use traits::LlmClient;
pub use traits::StreamReceiver;
pub use transport::HttpTransport;
