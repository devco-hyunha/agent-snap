//! 파일 도구 시험 공용 재료를 책임별 파일로 나눈 정본
//!
//! 책임별 정의를 한 자리에서 내보내 `support::` 이름 하나로 재사용
//! 제품 코드 항목은 같은 크레이트 경로(`crate::`)로만 봄

#![allow(dead_code, unreachable_pub)]

mod context;
mod pipes;
mod run_settings;
mod settings;
mod symlink;
mod temp_file_ops;
mod temp_folder;
mod temp_symlink_ops;
mod workdir;

pub use self::context::context_of;
pub use self::pipes::spawn_pipe_reader;
pub use self::run_settings::run_settings;
pub use self::settings::approval_limit;
pub use self::symlink::make_symlink;
pub use self::temp_folder::TempFolder;
pub use self::workdir::TempWorkdir;
