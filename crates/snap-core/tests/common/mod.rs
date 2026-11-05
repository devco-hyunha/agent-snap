//! 테스트 트리가 함께 쓰는 지원 파일들의 루트입니다.
//!
//! 이 파일은 단위 타깃과 통합 타깃 양쪽의 루트 파일로 포함됩니다.
//! 경로는 `script`·`server`·`handler`·`body_stream` 파일이 정본이고, 한 자리에서 쓰게 여기서 다시 내보냅니다.
//! 모의 서버 자체 시험은 이 트리 안에서만 돕는 항목을 쓰기 때문에 이 트리에 둡니다.
//! 시험 타깃마다 쓰는 항목이 부분적으로 다르므로 Unused·미사용 경고를 그 자리에서 허용합니다.

// 모의 서버 트리는 시험 파일들 안에서만 쓰이는 지원 코드입니다.
// 단위 트리(라이브러리 안)에서는 `pub(crate)`로 크레이트 안에서만,
// 바깥 시험(통합 타깃)에서는 시험 파일들이 타깃 경로로 접근합니다.
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
