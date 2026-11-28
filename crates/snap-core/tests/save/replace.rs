//! T1 저장 골격 시험: 교체 성공 책임. 원자 교체가 내용만 바꾸는지만 봄.
#[test]
fn replaces_original_file_content() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let path = base.path().join("snap.jsonc");
    std::fs::write(&path, "{ \"language\": \"ko\" }").expect("test writes original file");

    snap_core::jsonc::write_text_atomic(&path, "{ \"language\": \"en\" }")
        .expect("test saves over original file");

    let saved = std::fs::read_to_string(&path).expect("test reads saved file");
    assert_eq!(saved, "{ \"language\": \"en\" }");
}

#[test]
fn leaves_no_extra_file_in_folder() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let path = base.path().join("snap.jsonc");
    std::fs::write(&path, "{ \"language\": \"ko\" }").expect("test writes original file");

    snap_core::jsonc::write_text_atomic(&path, "{ \"language\": \"en\" }")
        .expect("test saves over original file");

    let entries: Vec<std::path::PathBuf> = std::fs::read_dir(base.path())
        .expect("test reads folder")
        .map(|entry| entry.expect("test reads folder entry").path())
        .collect();
    assert_eq!(entries, vec![path]);
}
