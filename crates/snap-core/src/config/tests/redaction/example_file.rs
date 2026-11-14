//! 예시 파일 검사(명세 AC 5 3번째 줄): `agent-snap.example.jsonc`의 형태를 확인합니다.
//!
//! 가리기 확인과 다른 책임이라 따로 분리했습니다. 예시 파일은 설정 해석 지원부(`config_text.rs`)
//! 로 읽고, 자리표시자만 쓰이는지 봅니다.

use super::super::support::parse_ok;

const EXAMPLE_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../agent-snap.example.jsonc"
));

/// AC 5(3): `agent-snap.example.jsonc`를 읽으면 검증을 통과하고 모델이 들어 있다.
#[test]
fn example_file_passes_validation() {
    let config = parse_ok(EXAMPLE_CONFIG);

    assert_eq!(config.endpoints.len(), 2);
    assert!(config.models().count() >= 2);
}

/// AC 5(3): 예시 파일에는 자리표시자만 있고 실제 키 칸과 알려진 로컬 주소가 없다.
#[test]
fn example_file_contains_placeholders_only() {
    let config = parse_ok(EXAMPLE_CONFIG);

    for endpoint in &config.endpoints {
        assert!(endpoint.name.starts_with('<'), "자리표시자가 아닙니다");
        assert!(endpoint.base_url.reveal().contains("<엔드포인트 호스트"));
    }
    for forbidden in ["localhost", "127.0.0.1", "api_key\"", "sk-"] {
        assert!(
            !EXAMPLE_CONFIG.contains(forbidden),
            "예시에 `{forbidden}`가 있습니다"
        );
    }
}
