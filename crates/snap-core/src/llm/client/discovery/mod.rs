//! 자동 발견 자리. 대상 판단·연결 프로브는 로직 파일, 결과 타입은 타입 파일로 나눔.

mod outcome;
mod probe;

pub use outcome::DiscoveryOutcome;
pub use probe::default_base_url;
pub use probe::discover_from;
