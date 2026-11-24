//! T3 검증 시험: `base_url` 값 책임. 허용 접두부 거부와 대문자 접두부 허용만 봄.
use crate::material::HOST;
use crate::material::endpoints;
use crate::material::parse;
use crate::material::reject_message;

const ENDPOINT: &str = "primary";

#[test]
fn rejects_base_url_without_allowed_scheme() {
    let urls = [
        "",
        "  https://secret-host.invalid",
        "https://secret-host.invalid/",
        "ftp://secret-host.invalid",
        "https:/secret-host.invalid",
    ];
    for url in urls {
        let text = endpoints(
            &r#"[{ "name": "primary", "base_url": "BASEURL", "kind": "ollama" }]"#
                .replace("BASEURL", url),
        );

        let message = reject_message(&text);

        assert!(
            message.contains(ENDPOINT),
            "message names endpoint for {url:?}: {message}"
        );
        assert!(
            !message.contains(HOST),
            "message hides address for {url:?}: {message}"
        );
    }
}

#[test]
fn accepts_uppercase_scheme_and_v1_path() {
    let text = endpoints(
        r#"[{ "name": "primary", "base_url": "HTTPS://secret-host.invalid/v1", "kind": "openai" }]"#,
    );

    let config = parse(&text).expect("test accepts uppercase scheme and v1 path");

    assert_eq!(config.endpoints.len(), 1);
}
