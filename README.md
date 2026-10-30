# agent-snap

로컬 LLM용 독립 코딩 에이전트입니다. OpenAI 호환 엔드포인트 하나에 연결해 실제 코딩 작업을 수행합니다.
에이전트를 먼저 완성하고, 이후 스트림 경로에 감시·교정 지점과 작은 판정 모델을 얹을 계획입니다.
Rust 워크스페이스(`snap-core` 라이브러리, `snap-cli` 실행 파일)로 구성됩니다.

상태: 개발 중

검사: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
