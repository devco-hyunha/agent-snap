//! T2 설정 거부 시험: 파일 없음 책임. 설정 파일이 없을 때의 읽기 실패만 봄.
#[test]
fn rejects_missing_config_file() {
    let dir = tempfile::tempdir().expect("test creates temp folder");

    let err = snap_core::config::read_config(dir.path()).expect_err("test expects missing file");

    assert!(matches!(
        err,
        snap_core::config::ConfigError::ReadFailed { .. }
    ));
}
