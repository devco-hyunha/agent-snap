//! 테스트 트리 공유 지원 파일의 루트
//!
//! 단위·통합 타깃 양쪽의 루트로 포함됨. 모의 서버는 시험 전용 지원 코드
//! 정본은 `script`·`server`·`handler`·`body_stream` 파일이고 여기서 내보냄. 타깃마다 쓰는 항목이 달라 미사용 경고를 허용

// 모의 서버 트리는 시험 파일 전용 지원 코드.
// 단위 트리(라이브러리 안)에서는 `pub(crate)`로 크레이트 안에서만,
// 바깥 시험(통합 타깃)에서는 시험 파일들이 타깃 경로로 접근.
#[cfg(test)]
#[allow(unused_imports, unreachable_pub, dead_code)]
pub(crate) mod mock_server {
    pub mod script;
    pub mod server;

    mod body_stream;
    mod chat_handler;
    mod disconnect;
    mod guard;
    mod models_handler;
    mod models_response;
    mod queue;
    pub mod record;
    mod response;
    #[cfg(test)]
    mod self_tests;
    mod shared;

    pub use record::RecordedRequest;
    pub use script::{MockBodyPart, MockResponseScript};
    pub use server::MockServer;
}

#[cfg(not(test))]
#[allow(unused_imports, unreachable_pub, dead_code)]
mod mock_server {
    pub mod script;
    pub mod server;

    mod body_stream;
    mod chat_handler;
    mod disconnect;
    mod guard;
    mod models_handler;
    mod models_response;
    mod queue;
    pub mod record;
    mod response;
    #[cfg(test)]
    mod self_tests;
    mod shared;

    pub use record::RecordedRequest;
    pub use script::{MockBodyPart, MockResponseScript};
    pub use server::MockServer;
}

pub mod support;
