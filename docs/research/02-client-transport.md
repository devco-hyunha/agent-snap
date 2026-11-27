# 작업 02-5 전송·취소·모의 서버 조사 상세

- 인덱스: [docs/RESEARCH.md](../RESEARCH.md) 작업 02 절
- 역할: 조사 상세. 규범 정본이 아님. 정책은 `docs/DECISIONS.md`가 정본.
- 확인 날짜: 2026-11-26. 각 링크를 그 날짜에 직접 열었다.

### reqwest ClientBuilder

- 링크: [reqwest ClientBuilder (docs.rs)](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
- 핵심: `connect_timeout`은 연결 단계에만 걸린다. `timeout`은 요청 전체에 걸리고, `read_timeout`은 읽기마다 적용되어 성공한 읽기 뒤 다시 센다.
- 적용: 02-5. 연결 시간만 라이브러리에 맡기고, 헤더 대기와 스트림 유휴는 따로 잰다. `timeout`·`read_timeout`은 두 구간을 구분하지 못해 쓰지 않는다. T6 코드: `crates/snap-core/src/llm/client/open.rs`의 `build_client`(연결 시간만 실음)·`open_stream`(헤더 대기), `crates/snap-core/src/llm/client/read.rs`의 `read_stream`(스트림 유휴).
- 관련 작업: 02-5

### reqwest 라이선스

- 링크: [crates.io reqwest 0.13.5](https://crates.io/api/v1/crates/reqwest/0.13.5), [reqwest LICENSE-MIT](https://raw.githubusercontent.com/seanmonstar/reqwest/master/LICENSE-MIT), [LICENSE-APACHE](https://raw.githubusercontent.com/seanmonstar/reqwest/master/LICENSE-APACHE)
- 핵심: 0.13.5 라이선스 표기는 MIT OR Apache-2.0. 기본 TLS는 rustls.
- 적용: 02-5 T1. 결정 14번에 버전·features와 함께 기록.
- 관련 작업: 02-5

### tokio / tokio-util / axum / http-body 라이선스

- 링크: [crates.io tokio 1.54.1](https://crates.io/api/v1/crates/tokio/1.54.1), [crates.io tokio-util 0.7.20](https://crates.io/api/v1/crates/tokio-util/0.7.20), [crates.io axum 0.8.9](https://crates.io/api/v1/crates/axum/0.8.9), [crates.io http-body 1.1.0](https://crates.io/api/v1/crates/http-body/1.1.0), [tokio LICENSE](https://raw.githubusercontent.com/tokio-rs/tokio/master/LICENSE), [tokio-util LICENSE](https://raw.githubusercontent.com/tokio-rs/tokio/master/tokio-util/LICENSE), [axum LICENSE](https://raw.githubusercontent.com/tokio-rs/axum/main/LICENSE), [http-body LICENSE](https://raw.githubusercontent.com/hyperium/http-body/master/LICENSE)
- 핵심: tokio 1.54.1, tokio-util 0.7.20, axum 0.8.9, http-body 1.1.0은 모두 라이선스 표기가 MIT이고 포장 LICENSE 전문도 MIT다.
- 적용: 02-5 T1. 제품에는 tokio의 `macros`·`time`만 켜고, 시험용 rt-multi-thread·sync·net과 axum·http-body는 dev-dependencies로 분리한다. tokio-util의 `sync`은 features 목록에 없는 기본 모듈이라 features를 적지 않는다(T1 실측으로 확인).
- 관련 작업: 02-5

### T1 라이선스 실측 (2026-11-26, T1 구현)

- 링크: [crates.io reqwest 0.13.5](https://crates.io/api/v1/crates/reqwest/0.13.5), [crates.io tokio 1.54.1](https://crates.io/api/v1/crates/tokio/1.54.1), [crates.io tokio-util 0.7.20](https://crates.io/api/v1/crates/tokio-util/0.7.20), [crates.io axum 0.8.9](https://crates.io/api/v1/crates/axum/0.8.9), [crates.io http-body 1.1.0](https://crates.io/api/v1/crates/http-body/1.1.0), [axum LICENSE](https://raw.githubusercontent.com/tokio-rs/axum/main/LICENSE)
- 핵심: crates.io API의 `license` 필드는 reqwest 0.13.5가 `MIT OR Apache-2.0`, tokio 1.54.1·tokio-util 0.7.20·axum 0.8.9·http-body 1.1.0이 `MIT`. 포장 안 `LICENSE`(reqwest는 `LICENSE-APACHE`·`LICENSE-MIT`) 전문도 같은 라이선스이고, `reqwest` README는 "Licensed under either of"로 이중 라이선스다.
- 핵심: `cargo deny check`를 대상 OS 없이 돌리면 `rustls-platform-verifier` 0.7.1의 wasm32 전용 의존인 `webpki-root-certs` 1.0.12(CDLA-Permissive-2.0)가 거부된다. Windows·Linux 트리에 `cargo tree --target`로 `-t x86_64-pc-windows-msvc -t x86_64-unknown-linux-gnu`를 붙이면 같은 검사가 통과한다.
- 적용: 02-5 T1. `crates/snap-core/Cargo.toml`에 버전·features로 반영하고 결정 14번에 기록.
- 관련 작업: 02-5 T1

### tokio-util CancellationToken

- 링크: [CancellationToken (docs.rs)](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
- 핵심: 복제본끼리 취소 상태를 공유하고, `cancelled()`는 cancel-safe라 `select!`에 쓸 수 있다. `child_token()`은 부모 취소를 따르고 자식 취소는 부모에 전파되지 않는다.
- 적용: 02-5. 스트림 도중 취소를 `select!`로 받아 연결을 정리한다. T6 코드: `crates/snap-core/src/llm/client/read.rs`의 `read_stream`·`crates/snap-core/src/llm/client/open.rs`의 `open_stream`(헤더 대기 중 취소), 연결 정리는 `crates/snap-core/src/llm/client/transport.rs`의 `stream`에서 응답 값을 버려 한다.
- 관련 작업: 02-5

### Ollama OpenAI compatibility

- 링크: [Ollama OpenAI compatibility](https://docs.ollama.com/api/openai-compatibility)
- 핵심: `/v1/chat/completions`에서 스트림과 `tools`를 지원한다. `tool_choice`, `logit_bias`, `n`은 지원하지 않는다. reasoning은 `reasoning_effort`로 조절한다. 로컬 서버는 인증 헤더를 요구하지 않고 키를 보내도 무시한다.
- 적용: 02-5. `tool_choice`·`n`·`stream_options`를 요청에 넣지 않는다. `kind`가 `ollama`일 때 `reasoning_effort`로 변환한다. 키가 없으면 Authorization 헤더를 보내지 않는다. T4 코드: `crates/snap-core/src/llm/client/body.rs`의 `build_body`, `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`, `crates/snap-core/src/llm/client/auth.rs`의 `auth_header`.
- 관련 작업: 02-5

### OpenRouter Streaming

- 링크: [OpenRouter Streaming](https://openrouter.ai/docs/api/reference/streaming)
- 핵심: 연결 유지를 위해 `: OPENROUTER PROCESSING` 주석 줄을 보낸다. 헤더 이후 오류는 HTTP 200 안에서 최상위 `error` 객체와 `finish_reason: "error"`를 가진 data 청크로 온다. 연결을 끊으면 이를 지원하는 공급자에서는 생성과 과금이 멈춘다. usage 청크는 빈 `delta`로 `finish_reason`만 되풀이한다.
- 적용: 02-5. 취소 시 연결 정리 근거. 02-3이 `error` 청크는 이미 스트림 오류로 끝낸다. T6 코드: `crates/snap-core/src/llm/client/transport.rs`의 `stream`이 읽기 종료 후 응답 값을 버려 연결을 정리한다.
- 적용: 02-5 T8. 최상위 `usage` 청크의 사용량은 02-3 정본인 `crates/snap-core/src/llm/delta/types.rs`의 `Delta.usage` 한곳에만 담고, `crates/snap-core/src/llm/client/receiver.rs`의 `finish`가 그 값을 `StreamOutcome.delta`에 그대로 실어 전달한다. 전송 쪽에 사용량 타입을 새로 정의하지 않는다. 시험: `crates/snap-core/tests/client/usage.rs`, 재료: `crates/snap-core/tests/mock_server/usage_stream.rs`.
- 관련 작업: 02-5

### OpenRouter Reasoning Tokens

- 링크: [OpenRouter Reasoning Tokens](https://openrouter.ai/docs/use-cases/reasoning-tokens)
- 핵심: 요청 쪽 생각 강도는 `reasoning: { "effort": … }` 형태다. 평탄한 `reasoning_effort`가 아니다.
- 적용: 02-5. `kind`가 `openrouter`일 때 중첩 객체로 변환한다. T4 코드: `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`(결정 15번).
- 관련 작업: 02-5

### LM Studio OpenAI Compatibility

- 링크: [LM Studio OpenAI compat](https://lmstudio.ai/docs/developer/openai-compat), [LM Studio Chat Completions](https://lmstudio.ai/docs/developer/openai-compat/chat-completions)
- 핵심: `/v1/chat/completions`를 지원하고 `stream` 매개변수를 받는다. 문서에 thinking(reasoning) **요청** 필드 설명이 없다.
- 적용: 02-5. `kind`가 `lmstudio`일 때는 공통 thinking 요청 필드를 보내지 않고 `extra`만 합친다. T4 코드: `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`, `crates/snap-core/src/llm/client/body.rs`의 `attach_extra`(결정 15번).
- 관련 작업: 02-5

### vLLM Reasoning Outputs

- 링크: [vLLM Reasoning Outputs](https://docs.vllm.ai/en/latest/features/reasoning_outputs/)
- 핵심: 생각 출력 필드는 `reasoning`(스트림은 `delta.reasoning`)이고 옛 이름이 `reasoning_content`다. 요청 쪽 조절은 `reasoning_effort`를 쓴다.
- 적용: 02-5. `kind`가 `vllm`일 때 `reasoning_effort`로 변환한다. T4 코드: `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`(결정 15번).
- 관련 작업: 02-5

### llama.cpp server README

- 링크: [llama.cpp tools/server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- 핵심: 스트림은 SSE이고, 생각 강도는 `reasoning_effort`로 조절한다. 기본 base URL은 `http://localhost:8080/v1`이다.
- 적용: 02-5. `kind`가 `llamacpp`일 때 `reasoning_effort`로 변환한다. T4 코드: `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`(결정 15번).
- 관련 작업: 02-5

### OpenAI Chat Completions 참조

- 링크: [OpenAI Chat Completions](https://developers.openai.com/api/reference/resources/chat)
- 핵심: 대화 요청은 `model`, `messages`, `stream`, `tools`, 샘플링 항목을 받고, 생각 강도는 `reasoning_effort`를 쓴다.
- 적용: 02-5. 요청 본문 골격과 `kind`가 `openai`일 때의 변환 표 근거. T4 코드: `crates/snap-core/src/llm/client/body.rs`의 `build_body`, `crates/snap-core/src/llm/client/reasoning.rs`의 `attach_reasoning`(결정 15번).
- 관련 작업: 02-5

### DistServe (arXiv 2401.09670)

- 링크: [DistServe](https://arxiv.org/abs/2401.09670)
- 핵심: TTFT는 요청 시작부터 첫 토큰까지의 지연, TPOT는 출력 토큰 하나당 시간으로 정의한다.
- 적용: 02-5. 요청 시작·첫 토큰·마지막 토큰 원시 시각을 그대로 돌려준다. 서버 구조 내용은 적용하지 않는다. T8 코드: `crates/snap-core/src/llm/client/stream.rs`의 `StreamTimings`가 세 필드 정본, `crates/snap-core/src/llm/client/token_times.rs`의 `TokenTimes::record`가 합친 응답이 늘어난 시점에만 채우고 `TokenTimes::timings`가 묶음(T10에서 `receiver.rs`에서 분리). 시각 확인 시험은 `crates/snap-core/tests/client/restore_timings.rs`·`timings.rs`.
- 관련 작업: 02-5

### Etalon (arXiv 2407.07000)

- 링크: [Etalon](https://arxiv.org/abs/2407.07000)
- 핵심: TTFT·TBT·TPOT 같은 기존 지표가 실시간 사용 경험을 충분히 담지 못한다고 본다.
- 적용: 02-5. 평균으로 줄이지 않고 요청별 원시 시각만 돌려준다. 지표 계산은 만들지 않는다. T8 코드: `crates/snap-core/src/llm/client/stream.rs`의 `StreamTimings`가 원시 시각 셋만 담고, 평균 계산은 어디에도 두지 않는다. 시험: `crates/snap-core/tests/client/timings.rs`의 `timings_keep_request_start_before_first_and_last_token`.
- 관련 작업: 02-5

### When Is the Same Model Not the Same Service? (arXiv 2605.02821)

- 링크: [arXiv 2605.02821](https://arxiv.org/abs/2605.02821)
- 핵심: 호스팅 LLM API는 모델 이름이 아니라 제공자별 서비스로 측정해야 하고, 지연과 처리량은 제공자·시점에 따라 크게 다르다.
- 적용: 02-5. 요청별 원시 시각을 돌려줘 작업 06에서 엔드포인트별 분포를 볼 수 있게 한다. T8 코드: `crates/snap-core/src/llm/client/token_times.rs`의 `TokenTimes::timings`가 요청 시작·첫 토큰·마지막 토큰을 `StreamOutcome.timings`로 그대로 전달한다(T10에서 `receiver.rs`에서 분리). 시험: `crates/snap-core/tests/client/timings.rs`의 `stream_without_tokens_leaves_first_and_last_empty`.
- 관련 작업: 02-5
