//! T1 저장 골격 시험: 생성 책임. 폴더·파일이 없을 때 새로 만들어지는지만 봄(spec AC 7).
#[test]
fn creates_missing_folder_and_file() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let config_dir = base.path().join("snap");
    let path = config_dir.join("snap.jsonc");

    snap_core::jsonc::write_text_atomic(&path, "{ \"language\": \"ko\" }")
        .expect("test saves into missing folder");

    let saved = std::fs::read_to_string(&path).expect("test reads saved file");
    assert_eq!(saved, "{ \"language\": \"ko\" }");
}
