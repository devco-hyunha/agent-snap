//! 가리기 확인 도움: 가리기 대상 값과 오류 Display·Debug 확인 순서를 한 곳에 모아 둡니다.
//!
//! 호스트와 키 값은 가짜 표지만 쓰고, 확인 순서는 여러 시험 파일에서 다시 적지 않게 이 곳에서 부릅니다.

use crate::config::ConfigError;

/// 설정 파일 쪽 가리기 검사용 가짜 호스트입니다. `base_url`이 들어간 검사의 가리기 대상으로만
/// 씁니다(자격 파일 저장은 주소를 다루지 않으므로 파일 이름·키 값 검사에 쓰지 않습니다).
pub(crate) const HOST: &str = "leak-check-host.invalid";
/// 가리기 검사용 가짜 키 값입니다. 오류와 파일에서 키 값이 사라지는지 보는 검사에 씁니다.
pub(crate) const KEY_VALUE: &str = "leak-key-value-9";

/// 오류의 Display·Debug에 지정한 값들이 없는지 확인합니다. 가리기 확인 순서를 여러 파일에
/// 나눠 적지 않게 하려는 자리입니다.
pub(crate) fn assert_error_hides(label: &str, error: &ConfigError, secrets: &[&str]) {
    for (kind, text) in [
        ("Display", error.to_string()),
        ("Debug", format!("{error:?}")),
    ] {
        for secret in secrets {
            assert!(
                !text.contains(secret),
                "{label}: {kind}에 `{secret}`가 있습니다: {text}"
            );
        }
    }
}

/// 오류의 Display·Debug에 호스트와 키 값이 없는지 확인합니다.
pub(crate) fn assert_error_hides_secrets(label: &str, error: &ConfigError) {
    assert_error_hides(label, error, &[HOST, KEY_VALUE]);
}
