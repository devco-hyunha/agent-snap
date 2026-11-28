//! T2 설정 값 가리기 시험. Debug 출력에 주소가 나오지 않는 책임을 둠.
use std::path::Path;

use snap_core::config::config_from_str;

const PATH: &str = "dir/snap.jsonc";

#[test]
fn keeps_base_url_secret_in_debug() {
    let text = r#"{ "endpoints": [{ "name": "primary", "base_url": "https://secret-host.invalid/v1", "kind": "ollama" }] }"#;

    let config = config_from_str(text, Path::new(PATH)).expect("test reads endpoint config");

    let shown = format!("{:?}", config);

    assert!(
        !shown.contains("secret-host") && !shown.contains("/v1"),
        "debug output hides the address"
    );
}
