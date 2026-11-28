//! T1 저장 골격 시험: 교체 실패 책임. 실패할 때 원본과 폴더가 그대로인지만 봄.
//! 교체 실패 재료는 '경로 자리에 폴더': Windows·유닉스 양쪽에서 교체 거부됨.
#[test]
fn replace_failure_keeps_original_and_leaves_no_temp_file() {
    let base = tempfile::tempdir().expect("test creates temp folder");
    let target = base.path().join("snap.jsonc");
    std::fs::create_dir(&target).expect("test blocks replace by folder");
    let original = target.join("keep.txt");
    std::fs::write(&original, "original text").expect("test writes original file");

    let result = snap_core::jsonc::write_text_atomic(&target, "new text");

    assert!(matches!(
        result,
        Err(snap_core::jsonc::SaveError::Replace { .. })
    ));
    let kept = std::fs::read_to_string(&original).expect("test reads original file");
    assert_eq!(kept, "original text");
    let entries: Vec<std::path::PathBuf> = std::fs::read_dir(base.path())
        .expect("test reads folder")
        .map(|entry| entry.expect("test reads folder entry").path())
        .collect();
    assert_eq!(entries, vec![target]);
}

#[test]
fn path_without_folder_is_rejected() {
    let result = snap_core::jsonc::write_text_atomic(std::path::Path::new("snap.jsonc"), "{}");

    assert!(matches!(
        result,
        Err(snap_core::jsonc::SaveError::NoFolder { .. })
    ));
}
