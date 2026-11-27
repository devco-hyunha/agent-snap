//! 클라이언트 조립 자리. 조립 결과 타입과 조립 함수를 책임별 파일로 나눔.

mod assemble;
mod bundle;

pub use assemble::assemble_client;
pub use bundle::ClientBundle;
