//! T5 예시 파일 시험: 검증 통과 책임. 예시 파일이 검증을 지나는 것만 봄.
use std::path::PathBuf;

use snap_core::config::config_from_str;

/// 저장소 루트 예시 파일 경로. 크레이트 폴더에서 한 위.
fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("snap.example.jsonc")
}

#[test]
fn example_file_passes_validation() {
    let path = example_path();
    let text = std::fs::read_to_string(&path).expect("test reads example file");

    let config = config_from_str(&text, &path).expect("test expects example file to pass");

    assert_eq!(config.language, "ko");
    assert_eq!(config.endpoints.len(), 2);
    assert_eq!(config.endpoints[0].models.len(), 1);
    assert_eq!(config.endpoints[1].models.len(), 1);
    assert_eq!(config.http.retries, 3);
}
