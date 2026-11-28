//! 02-7 T4 대상 아닌 경우 시험. 대상 종류가 아닌 경우·주소가 채워진 경우의 발견 실패만 봄.

use snap_core::llm::EndpointKind;

use crate::materials::discover_with_default;

/// `llamacpp`·`vllm`·`openai`는 자동 발견 대상이 아니다.
#[tokio::test]
async fn non_target_kinds_are_not_discovered() {
    for kind in [
        EndpointKind::Llamacpp,
        EndpointKind::Vllm,
        EndpointKind::Openai,
        EndpointKind::Openrouter,
    ] {
        let outcome = discover_with_default(kind, "", "http://127.0.0.1:11434/v1").await;
        assert!(
            outcome.is_not_found(),
            "kind {kind} is not a discovery target"
        );
    }
}

/// `base_url`이 채워져 있으면 프로브하지 않고 발견 실패로 본다. 설정 입력이 정본이다.
#[tokio::test]
async fn filled_base_url_is_not_probed() {
    for kind in [EndpointKind::Ollama, EndpointKind::Lmstudio] {
        let outcome = discover_with_default(kind, "http://127.0.0.1:9999/v1", "").await;
        assert!(
            outcome.is_not_found(),
            "kind {kind} keeps the configured address"
        );
    }
}
