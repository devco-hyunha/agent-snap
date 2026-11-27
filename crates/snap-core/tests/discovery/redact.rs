//! 02-7 T4 발견 결과 가리기 시험. `Found`의 주소가 Debug에 원문으로 나가지 않는지, Display는 값을 유지하는지만 봄.

use snap_core::config::SecretString;
use snap_core::llm::DiscoveryOutcome;

/// 시험용 주소. 프로브가 쓰는 기본 주소 형태.
const TEST_BASE_URL: &str = "http://127.0.0.1:11434/v1";

/// `Found`를 시험 재료로 만듦.
fn found_outcome() -> DiscoveryOutcome {
    DiscoveryOutcome::Found {
        base_url: SecretString::new(TEST_BASE_URL.to_owned()),
    }
}

/// Debug 출력에 주소 원문이 나오지 않는다.
#[test]
fn debug_hides_found_address() {
    let output = format!("{:?}", found_outcome());

    assert!(
        !output.contains(TEST_BASE_URL),
        "Debug must not carry the discovered address"
    );
}

/// Display는 명세상 금지 대상이 아니므로 채운 주소 값을 그대로 낸다.
#[test]
fn display_keeps_found_address() {
    assert_eq!(found_outcome().to_string(), TEST_BASE_URL);
}
