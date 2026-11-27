//! T3 시험: `EndpointKind`와 설정 검증의 연결 책임. 허용값 한 정의가 이어지는지 봄.
use std::path::Path;

use snap_core::config::config_from_str;
use snap_core::llm::EndpointKind;

/// 시험용 설정 파일 경로 표시.
const PATH: &str = "dir/snap.jsonc";

/// 엔드포인트 하나를 담은 설정 텍스트.
fn config_with_kind(kind: &str) -> String {
    format!(
        r#"{{ "endpoints": [{{ "name": "ep", "base_url": "https://host.invalid", "kind": "{kind}" }}] }}"#
    )
}

#[test]
fn every_kind_name_passes_config_validation() {
    for name in EndpointKind::NAMES {
        let text = config_with_kind(name);
        let parsed = config_from_str(&text, Path::new(PATH));

        assert!(parsed.is_ok(), "config accepts kind {name}");
    }
}

#[test]
fn kind_outside_list_is_rejected() {
    let text = config_with_kind("unknownserver");
    let parsed = config_from_str(&text, Path::new(PATH));

    assert!(parsed.is_err(), "config rejects kind outside the list");
}
