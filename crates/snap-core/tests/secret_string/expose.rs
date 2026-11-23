//! T1 expose 값 접근 시험.
use snap_core::config::SecretString;

#[test]
fn expose_returns_wrapped_value() {
    let secret = SecretString::new(String::from("primary-key"));

    assert_eq!(secret.expose(), "primary-key");
}
