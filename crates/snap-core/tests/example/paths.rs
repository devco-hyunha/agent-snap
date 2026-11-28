//! 예시 파일 시험 경로 헬퍼 자리. 예시 파일 경로를 한곳에서 공유함.
use std::path::PathBuf;

/// 저장소 루트 예시 파일 경로. 크레이트 폴더에서 한 위.
pub(super) fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("snap.example.jsonc")
}
