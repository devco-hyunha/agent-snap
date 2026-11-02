use super::SecretValue;

/// AC 5: 비밀 값 래퍼의 Debug·Display에 키 값이 없다.
#[test]
fn secret_value_debug_and_display_hide_the_value() {
    let secret = SecretValue::new("super-secret-key-1234");
    let debug_text = format!("{secret:?}");
    let display_text = format!("{secret}");
    assert!(!debug_text.contains("super-secret-key-1234"));
    assert!(!display_text.contains("super-secret-key-1234"));
    assert!(debug_text.contains("가림"));
    assert!(display_text.contains("가림"));
}

/// AC 5: 원문은 명시적 함수(reveal)로만 나온다.
#[test]
fn secret_value_reveals_only_through_explicit_call() {
    let secret = SecretValue::new("super-secret-key-1234");
    assert_eq!(secret.reveal(), "super-secret-key-1234");
}

/// 값으로 비교할 수 있다(엔드포인트 키 비교 경로).
#[test]
fn secret_value_equality_compares_the_wrapped_value() {
    let a = SecretValue::new("alpha-key");
    let b = SecretValue::new("alpha-key");
    let c = SecretValue::new("beta-key");
    assert_eq!(a, b);
    assert_ne!(a, c);
}
