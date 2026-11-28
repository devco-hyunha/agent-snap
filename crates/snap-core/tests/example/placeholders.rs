//! T5 예시 파일 시험: 자리표시자 유지 책임. 키 칸과 키 값이 없는 것만 봄.
use std::path::PathBuf;

#[test]
fn example_file_keeps_placeholders_only() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("snap.example.jsonc");
    let text = std::fs::read_to_string(&path).expect("test reads example file");

    // 안내 주석의 칸 이름은 두고, 실제 키 칸과 키 값은 두지 않음.
    let has_key_field = text.contains(r#""api_key"#);
    let has_real_key = text.contains("sk-");
    assert!(!has_key_field, "example file has no key fields");
    assert!(!has_real_key, "example file has no key value");
}
