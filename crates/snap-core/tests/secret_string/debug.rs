//! T1 비밀 값 래퍼 Debug 가리기 시험.
use snap_core::config::SecretString;
use snap_core::locale::config::SECRET_REDACTED;

#[test]
fn debug_shows_redacted_mark() {
    let secret = SecretString::new(String::from("primary-key"));

    let output = format!("{secret:?}");

    assert_eq!(output, format!("SecretString({SECRET_REDACTED})"));
}

#[test]
fn debug_hides_plain_value() {
    let secret = SecretString::new(String::from("primary-key"));

    let output = format!("{secret:?}");

    assert!(!output.contains("primary-key"));
}
