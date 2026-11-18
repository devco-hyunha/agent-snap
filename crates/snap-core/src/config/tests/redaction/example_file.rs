//! 예시 파일 검사: `agent-snap.example.jsonc`가 검증을 통과하고 자리표시자만 담는지 확인
//!
//! 예시 파일은 설정 해석 지원부로 읽는 별도 책임의 시험

use super::super::support::parse_ok;

const EXAMPLE_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../agent-snap.example.jsonc"
));

/// 예시 파일을 읽으면 검증을 통과하고 엔드포인트·모델이 들어 있는지 확인
#[test]
fn example_file_passes_validation() {
    let config = parse_ok(EXAMPLE_CONFIG);

    assert_eq!(config.endpoints.len(), 2);
    assert!(config.models().count() >= 2);
}

/// 예시 파일에 자리표시자만 있고 실제 키 칸이나 알려진 로컬 주소가 없는지 확인
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
