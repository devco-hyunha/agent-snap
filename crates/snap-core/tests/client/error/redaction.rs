//! 02-6 전송 오류 가리기 시험. Display·Debug에서 키·호스트·헤더 값이 은닉되는지만 봄.
//! 시험 본문과 단정값은 `tests/client/error.rs`에서 그대로 옮김.

use snap_core::llm::TransportError;

use crate::error::materials::all_errors;
use crate::error::materials::connect_kinds;
use crate::error::materials::header;
use crate::error::materials::summary;

/// 시험용 호스트.
const HOST: &str = "secret-host.invalid";

/// 시험용 키.
const KEY: &str = "sk-secretvalue123";

#[test]
fn every_error_text_hides_key_and_host() {
    let errors = all_errors();
    for error in errors {
        let display = error.to_string();
        let debug = format!("{error:?}");
        for secret in [KEY, HOST] {
            assert!(
                !display.contains(secret) && !debug.contains(secret),
                "error text hides secret: {display}"
            );
        }
    }
}

#[test]
fn connect_variants_all_print() {
    for kind in connect_kinds() {
        let error = TransportError::connect(kind, 10.0);
        let display = error.to_string();

        assert!(
            !display.contains("base_url") && !display.contains(HOST),
            "connect display names kind only: {display}"
        );
    }
}

#[test]
fn status_debug_hides_retry_after_header_value() {
    let error = TransportError::status(503, summary(), Some(header("123")));

    let debug = format!("{error:?}");
    assert!(
        !debug.contains("123") && !debug.contains("Retry-After"),
        "debug carries no header text: {debug}"
    );
}
