//! 자격 파일 읽기 자리. 오류·읽기·구조·조회를 책임별 파일로 나눔.

mod error;
mod lookup;
mod read;
mod shape;

pub use error::CredentialsError;
pub use lookup::find_api_key;
pub use read::credentials_from_str;
pub use read::read_credentials;
