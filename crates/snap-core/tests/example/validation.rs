//! 예시 파일 시험: 검증 통과 책임. 예시 파일이 검증을 지나는 것만 봄.
use snap_core::config::config_from_str;

use crate::paths::example_path;

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
