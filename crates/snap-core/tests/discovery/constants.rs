//! 02-7 T4 기본 주소 상수 시험. 상수 값과 종류별 매핑 한곳만 봄.

use snap_core::consts::defaults::DEFAULT_LMSTUDIO_BASE_URL;
use snap_core::consts::defaults::DEFAULT_OLLAMA_BASE_URL;
use snap_core::llm::EndpointKind;
use snap_core::llm::default_base_url;

/// Ollama 기본 주소 상수 값.
#[test]
fn ollama_default_address_constant() {
    assert_eq!(DEFAULT_OLLAMA_BASE_URL, "http://127.0.0.1:11434/v1");
}

/// LM Studio 기본 주소 상수 값.
#[test]
fn lmstudio_default_address_constant() {
    assert_eq!(DEFAULT_LMSTUDIO_BASE_URL, "http://127.0.0.1:1234/v1");
}

/// 종류별 기본 주소 매핑. 대상 종류는 상수 값, 그 밖은 없음.
#[test]
fn default_address_mapping_per_kind() {
    assert_eq!(
        default_base_url(EndpointKind::Ollama),
        Some(DEFAULT_OLLAMA_BASE_URL)
    );
    assert_eq!(
        default_base_url(EndpointKind::Lmstudio),
        Some(DEFAULT_LMSTUDIO_BASE_URL)
    );
    assert_eq!(default_base_url(EndpointKind::Llamacpp), None);
    assert_eq!(default_base_url(EndpointKind::Vllm), None);
}
