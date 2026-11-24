//! T2 설정 읽기 시험: 파일 읽기 책임. 폴더 안 설정 파일을 읽는 것만 봄.
#[test]
fn reads_config_file_from_folder() {
    let dir = tempfile::tempdir().expect("test creates temp folder");
    let path = dir.path().join("snap.jsonc");
    std::fs::write(&path, r#"{ "language": "ko" }"#).expect("test writes config file");

    let config = snap_core::config::read_config(dir.path()).expect("test reads config file");

    assert_eq!(config.language, "ko");
    assert!(config.endpoints.is_empty());
}
