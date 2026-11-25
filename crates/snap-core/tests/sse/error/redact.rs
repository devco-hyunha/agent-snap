//! 02-3 T4 오류 요약 가리기 시험. 키·토큰·주소성 값이 가렸음 표시로 바뀌는 표시만 봄.

use snap_core::llm::redact_summary;

const SECRET_MARK: &str = "sk-secretvalue123";
const HOST_MARK: &str = "https://api.example-host.test/v1";
const SCHEMELESS_HOST_MARK: &str = "api.example-host.test/v1";
const BASE64URL_MARK: &str = "Ab12-CD34_EF56-GH78_IJ01-KL23-MN45-OP67";
const LETTERS_ONLY_MARK: &str = "AbCdEfGhIjKlMnOpQrSxUvWyZaBcDeFg";

#[test]
fn key_pair_value_is_replaced_in_summary() {
    let summary = redact_summary(&format!("quota key={SECRET_MARK} exceeded"));

    let shown = summary.expose();
    assert!(
        !shown.contains(SECRET_MARK),
        "key value must be hidden: {shown}"
    );
    assert!(
        shown.contains("quota") && shown.contains("exceeded"),
        "plain words stay: {shown}"
    );
}

#[test]
fn standalone_key_token_is_replaced_in_summary() {
    let summary = redact_summary(&format!("Invalid API key: {SECRET_MARK}"));

    let shown = summary.expose();
    assert!(
        !shown.contains(SECRET_MARK),
        "standalone key token must be hidden: {shown}"
    );
    assert!(
        shown.contains("Invalid") && shown.contains("API"),
        "plain words stay: {shown}"
    );
}

#[test]
fn long_opaque_token_is_replaced_in_summary() {
    let opaque = format!("{}9{}", "a".repeat(31), "bca7f2e1d0c9b8a6");
    let summary = redact_summary(&format!("bearer {opaque} rejected"));

    let shown = summary.expose();
    assert!(
        !shown.contains(&opaque),
        "opaque token must be hidden: {shown}"
    );
    assert!(
        shown.contains("bearer") && shown.contains("rejected"),
        "plain words stay: {shown}"
    );
}

#[test]
fn address_is_replaced_in_summary() {
    let summary = redact_summary(&format!("base {HOST_MARK} rejected"));

    let shown = summary.expose();
    assert!(
        !shown.contains("example-host"),
        "host must be hidden: {shown}"
    );
}

#[test]
fn summary_debug_hides_value() {
    let summary = redact_summary(&format!("key={SECRET_MARK}"));

    let shown = format!("{summary:?}");
    assert!(
        !shown.contains(SECRET_MARK),
        "debug must not carry the value: {shown}"
    );
}

#[test]
fn key_inside_delimiter_token_is_replaced_in_summary() {
    let forms = [
        format!("Bearer:{SECRET_MARK}"),
        format!("Bearer={SECRET_MARK}"),
        format!("({SECRET_MARK})"),
        format!(r#""detail":"{SECRET_MARK}""#),
    ];

    for form in forms {
        let summary = redact_summary(&format!("quota {form} exceeded"));
        let shown = summary.expose();
        assert!(
            !shown.contains(SECRET_MARK),
            "key inside delimiters must be hidden: {shown}"
        );
        assert!(
            shown.contains("quota") && shown.contains("exceeded"),
            "plain words stay: {shown}"
        );
    }
}

#[test]
fn base64url_key_token_is_replaced_in_summary() {
    let summary = redact_summary(&format!("bearer {BASE64URL_MARK} rejected"));
    let shown = summary.expose();

    assert!(
        !shown.contains(BASE64URL_MARK),
        "base64url key token must be hidden: {shown}"
    );
    assert!(
        shown.contains("bearer") && shown.contains("rejected"),
        "plain words stay: {shown}"
    );
}

#[test]
fn base_url_and_bearing_names_hide_their_value() {
    let forms = [
        format!("quota base_url={LETTERS_ONLY_MARK} exceeded"),
        format!("quota bearer={LETTERS_ONLY_MARK} exceeded"),
        format!("quota base_url={SECRET_MARK} exceeded"),
    ];

    for form in forms {
        let summary = redact_summary(&form);
        let shown = summary.expose();
        assert!(
            !shown.contains(LETTERS_ONLY_MARK) && !shown.contains(SECRET_MARK),
            "bearing name value must be hidden: {shown}"
        );
        assert!(
            shown.contains("quota") && shown.contains("exceeded"),
            "plain words stay: {shown}"
        );
    }
}

#[test]
fn schemeless_host_is_replaced_in_summary() {
    let summary = redact_summary(&format!("quota {SCHEMELESS_HOST_MARK} exceeded"));
    let shown = summary.expose();

    assert!(
        !shown.contains("example-host"),
        "schemeless host must be hidden: {shown}"
    );
    assert!(
        shown.contains("quota") && shown.contains("exceeded"),
        "plain words stay: {shown}"
    );
}

#[test]
fn letters_only_long_token_is_replaced_in_summary() {
    let summary = redact_summary(&format!("bearer {LETTERS_ONLY_MARK} rejected"));
    let shown = summary.expose();

    assert!(
        !shown.contains(LETTERS_ONLY_MARK),
        "letters-only long token must be hidden: {shown}"
    );
    assert!(
        shown.contains("bearer") && shown.contains("rejected"),
        "plain words stay: {shown}"
    );
}

#[test]
fn error_without_message_fallback_hides_key() {
    let payload = format!(r#"{{"error": {{"detail": "{SECRET_MARK}"}}}}"#);
    #[allow(clippy::unwrap_used)]
    let shown = super::stream_error_from(&payload).to_string();

    assert!(
        !shown.contains(SECRET_MARK),
        "fallback must not carry the key: {shown}"
    );
}
