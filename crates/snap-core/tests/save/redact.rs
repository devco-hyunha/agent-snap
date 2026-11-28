//! T1 저장 골격 시험: 가리기 책임. 저장 오류 문구에 키·주소·파일 내용이 안 나오는지만 봄.
#[test]
fn save_error_text_has_no_key_value() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let target = base.path().join("credentials.jsonc");
    std::fs::create_dir(&target).expect("test blocks replace by folder");
    let secret_file = target.join("keep.txt");
    std::fs::write(&secret_file, "SECRET-VALUE-9137").expect("test writes secret file");

    let error = snap_core::jsonc::write_text_atomic(&target, "new text")
        .expect_err("test expects replace failure");

    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(!display.contains("SECRET-VALUE-9137"));
    assert!(!debug.contains("SECRET-VALUE-9137"));
    assert!(display.contains(&target.display().to_string()));
}
