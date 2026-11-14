# 조사 기록 (인덱스)

상세 본문은 [`docs/research/`](research/) 개별 문서에 두고, 이 파일에는 적용 위치·미적용 이유·**상태**만 남깁니다. 링크는 직접 열어 확인한 것만 적습니다. 규범 정책은 [`docs/DECISIONS.md`](DECISIONS.md)와 [`docs/decisions/`](decisions/) 정본을 따릅니다.

## 기록 형식

- 제목 / 상세 링크 / 확인 날짜 / 핵심(한 줄) / 적용 위치 또는 미적용 / **상태** / 관련 작업
- 상태 값 예: 조사됨 | 기획 반영 | 적용(코드 경로) | 미적용 | 보류 | D22에 반영

## 기획 단계 조사

| 항목 | 상세 | 확인 | 핵심 | 적용·상태 | 관련 |
|---|---|---|---|---|---|
| 기획 매트릭스 | [research/2026-10-28-planning.md](research/2026-10-28-planning.md) | 2026-10-28 | arXiv 등 기획 단계 후보·적용 작업 표 | **기획 반영** (작업 01~08 후보) | 01~08 |

## 작업 01

상세: [research/01-foundation.md](research/01-foundation.md)

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| Harness² | [01-foundation.md#harness²](research/01-foundation.md#harness²) | 2026-10-28 | 프롬프트·도구·감시 규칙을 설정 파일로 분리해 git 이력 관리 | `prompts/`, `rules/` (내용은 03·05에서 채움) | 기획 반영 | 01 |

## 작업 02

상세: [research/02-client.md](research/02-client.md) — `platform.openai.com`은 403이라 미기록, 모델 목록·SSE 구간은 `developers.openai.com` 등 대체 링크 사용(상세 서두·중간 주석 참고).

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| RFC 9110 Retry-After | [02-client.md#rfc-9110-http-semantics-1023-retry-after](research/02-client.md#rfc-9110-http-semantics-1023-retry-after) | 2026-11-02 | delay-seconds·HTTP-date, 503·3xx 예 | 02-4: `llm/retry/after.rs`·`decide.rs`(두 형식·5xx 헤더) | 적용(코드 경로) | 02 |
| RFC 6585 429 | [02-client.md#rfc-6585-4절-429-too-many-requests](research/02-client.md#rfc-6585-4절-429-too-many-requests) | 2026-11-02 | 429·`Retry-After` | 02-4: `llm/retry`(classify·after·decide); 02-6: `llm/transport/stream_chat.rs` `stream_chat` 재시도 고리의 대기·재요청, `llm/retry/retry_loop.rs` `judge_once`의 판단 반복, `llm/retry/retry_map.rs` `retry_input` 매핑; 확인은 `llm/tests/transport_unit/retry_status.rs`(AC 1~3)·`retry_run.rs` | 적용(코드 경로) | 02 |
| WHATWG SSE | [02-client.md#whatwg-html-standard-server-sent-events](research/02-client.md#whatwg-html-standard-server-sent-events) | 2026-11-02 | CRLF/LF/CR, 주석·data 결합·BOM | 02-3: `llm/client/sse.rs` `SseParser`(D12 finish) | 적용(코드 경로) | 02 |
| llama.cpp server README | [02-client.md#llamacpp-server-readme](research/02-client.md#llamacpp-server-readme) | 2026-11-02 | SSE, `reasoning_content`, 503 | 02-3: `llm/codec/chunk.rs` `reasoning_content` 스키마·`llm/codec/delta.rs` 누적; 02-4(503); `<think>` 분리는 안 함(05) | 적용(코드 경로) | 02 |
| Ollama OpenAI compatibility | [02-client.md#ollama-openai-compatibility](research/02-client.md#ollama-openai-compatibility) | 2026-10-28 | 스트림·tools; `reasoning_effort` | 확인: `llm/request_body.rs` `build_chat_completions_body`(도구 실재 시 `tools`만, `tool_choice`/`n`/`stream_options` 미전송)·`insert_reasoning`(`reasoning`→`reasoning_effort`)·`reasoning_wire_value`(`off`→`none`, D15); 확인은 `crates/snap-core/tests/request.rs` | 적용(코드 경로) | 02 |
| vLLM Reasoning Outputs | [02-client.md#vllm-reasoning-outputs](research/02-client.md#vllm-reasoning-outputs) | 2026-11-02 | `reasoning` vs `reasoning_content` | 02-3: `llm/codec/chunk.rs` 두 이름 읽기·`llm/codec/delta.rs` `StreamEvent::Reasoning`; 02-5: `llm/request_body.rs` `insert_reasoning`(`reasoning_effort`) | 적용(코드 경로) | 02 |
| LM Studio OpenAI compat | [02-client.md#lm-studio-openai-compatibility-endpoints](research/02-client.md#lm-studio-openai-compatibility-endpoints) | 2026-11-02 | `/v1/chat/completions`, thinking 요청 필드 문서 없음 | 02-3: 응답 두 이름·미지 필드; 02-5: `llm/request_body.rs` `insert_reasoning`의 `LmStudio` 자리(공통 `reasoning` 미전송)·`merge_extra`(`extra`만, D15) | 적용(코드 경로) | 02 |
| OpenRouter Streaming | [02-client.md#openrouter-streaming](research/02-client.md#openrouter-streaming) | 2026-11-02 | 주석 줄, HTTP 200 중 `error` 청크 | 02-3: `SseParser` 주석 무시·`llm/codec/chunk.rs` 최상위 `error`·`llm/error/redact.rs` 가림; 02-5(취소 시 연결 정리) | 적용(코드 경로) | 02 |
| OpenRouter Reasoning Tokens | [02-client.md#openrouter-reasoning-tokens](research/02-client.md#openrouter-reasoning-tokens) | 2026-11-02 | `reasoning_details`·요청 `reasoning.effort` | 02-3: 응답 `reasoning` 이벤트; 02-5: `llm/request_body.rs` `insert_reasoning`의 `OpenRouter` 자리(`reasoning: { effort }`, D15) | 적용(코드 경로) | 02 |
| kind별 reasoning 요청 변환 | [02-client.md#kind별-reasoning-요청-필드-변환-02-5](research/02-client.md#kind별-reasoning-요청-필드-변환-02-5) | 2026-11-03 | 6 kind 표·`off`→`none`·lmstudio 미전송 | 02-5: `llm/request_body.rs` `insert_reasoning`·`reasoning_wire_value`(`off`→`none`), 확인은 `crates/snap-core/tests/request.rs` | 적용(코드 경로) | 02 |
| OpenRouter Rate Limits | [02-client.md#openrouter-api-rate-limits](research/02-client.md#openrouter-api-rate-limits) | 2026-11-02 | 429·`X-RateLimit-*` | 02-4: `llm/retry`·`decide_retry`; 02-6: `llm/transport/send_once.rs`가 `Retry-After` 헤더를 읽어 판단에 넘김(헤더 우선 대기), 확인은 `llm/tests/transport_unit/retry_status.rs`; `X-RateLimit-Reset` 미사용; 긴 대기는 포기; **402 미적용(D26)** | 적용(코드 경로) | 02 |
| tokio-util CancellationToken | [02-client.md#tokio-util-cancellationtoken](research/02-client.md#tokio-util-cancellationtoken) | 2026-10-28 | cancel-safe `select!` | 확인: `llm/client/mod.rs` `CancellationToken`(재-export)·`llm/transport/send_once.rs` `wait_for_headers`/`llm/transport/stream_state.rs` `read_one_chunk`의 `select!`; 확인은 `crates/snap-core/tests/transport/cancel.rs`·`timeouts.rs` 취소·시간 초과 테스트; 02-6: `llm/transport/stream_chat.rs` `sleep_before_retry`의 재시도 대기 취소 경합, 확인은 `crates/snap-core/tests/transport/cancel.rs` `cancel_during_retry_wait_ends_immediately`(AC 4) | 적용(코드 경로) | 02 |
| reqwest ClientBuilder | [02-client.md#reqwest-clientbuilder](research/02-client.md#reqwest-clientbuilder) | 2026-10-28 | connect vs read vs total timeout | 02-5: `llm/transport/client.rs` `OpenAiCompatClient::new`의 `connect_timeout`(연결만 reqwest)·`llm/transport/send_once.rs` `wait_for_headers`(헤더 대기)·`llm/transport/stream_state.rs` `read_one_chunk`(유휴)를 따로 잰다; 확인은 `crates/snap-core/tests/transport/timeouts.rs` | 적용(코드 경로) | 02 |
| OpenAI List models | [02-client.md#openai-api-참조-모델-목록](research/02-client.md#openai-api-참조-모델-목록) | 2026-11-05 | `data[].id` 중심 | 02-7: `llm/transport/fetch_models.rs`(1회 조회, 재시도 없음) + `llm/codec/models_list.rs`(`data[].id`만 해석); 확인은 `crates/snap-core/tests/selection/models_list/ids_read.rs` | 적용(코드 경로) | 02 |
| llama.cpp 모델 목록 | [02-client.md#llamacpp-server-readme-모델-목록](research/02-client.md#llamacpp-server-readme-모델-목록) | 2026-11-05 | `GET /v1/models`, `id`가 경로일 수 있음 | 02-7: `llm/codec/models_list.rs`가 `meta`가 `null`이어도 무시(모르는 필드 허용), base_url에 경로만 바꿔 붙임(`.../v1/v1` 금지); 확인은 `crates/snap-core/tests/selection/models_list/ids_read.rs`의 `null` 무시·이중 결합 없음 시험. 02-2: `config/selection.rs`가 id를 모델 이름과 따로 다룸 | 적용(코드 경로) | 02 |
| Ollama 모델 목록 | [02-client.md#ollama-openai-호환-문서-모델-목록](research/02-client.md#ollama-openai-호환-문서-모델-목록) | 2026-10-28 | 로컬 키 무시 | 02-1·02-2, 02-5: `llm/transport/send_once.rs`의 `Authorization: Bearer` 조립에서 `api_key`가 없으면 헤더 생략(확인 `crates/snap-core/tests/request.rs`), 02-7 | 적용(코드 경로) | 02 |
| llama.cpp base URL | [02-client.md#llamacpp-서버-readme-http-serve와-기본-base-url](research/02-client.md#llamacpp-서버-readme-http-serve와-기본-base-url) | 2026-11-01 | 기본 `http://localhost:8080/v1` | 02-2: `validate.rs` `is_valid_base_url`, `parse.rs` `read_endpoint` | 적용(코드 경로) | 02 |
| Ollama 인증 | [02-client.md#ollama-openai-compatibility-인증](research/02-client.md#ollama-openai-compatibility-인증) | 2026-11-01 | 로컬 무키·클라우드 HTTPS+키 | 02-2: `is_valid_base_url`, `keys.rs` `EndpointConfig::api_key` | 적용(코드 경로) | 02 |
| vLLM OpenAI 서버 | [02-client.md#vllm-openai-호환-서버](research/02-client.md#vllm-openai-호환-서버) | 2026-10-28 | `/v1/models`, `--api-key` 경로 제한 | 02-5: `llm/transport/send_once.rs`의 `Bearer` 헤더(키 있을 때만, 헤더 값은 sensitive로 가림), 02-7: `data[].id`만 필수 | 적용(코드 경로) | 02 |
| LM Studio 모델·인증 | [02-client.md#lm-studio-모델-목록과-인증](research/02-client.md#lm-studio-모델-목록과-인증) | 2026-11-05 | OpenAI `/v1/models`만; 고유 REST 미사용 | 02-7: `llm/transport/fetch_models.rs`가 OpenAI 호환 경로만 쓰고, `data` 배열이 아니면 `llm/error/models_format.rs`의 형식 오류로 알림; 확인은 `crates/snap-core/tests/selection/models_list/format_error.rs` | 적용(코드 경로) | 02 |
| OpenRouter 모델 목록 | [02-client.md#openrouter-모델-목록](research/02-client.md#openrouter-모델-목록) | 2026-11-05 | Bearer·풍부한 `data[]` | 02-7: `llm/codec/models_list.rs`가 `id`만 읽고 나머지는 버림; **쪽 나누기 미적용**(무파라미터=전체라 문서화), **기본 필터(`output_modalities=text`) 미적용**(사유는 research 02-client 절) | 적용(코드 경로) | 02 |
| 모델 목록 서버별 정리 | [02-client.md#모델-목록-서버별-정리](research/02-client.md#모델-목록-서버별-정리) | 2026-11-05 | 공통 필수는 `data[].id`, Bearer 통일 | 02-7: `llm/codec/models_list.rs`가 `data[].id`만 필수 해석, `llm/transport/send_headers.rs`가 Bearer 통일(키 없으면 헤더 생략); 확인은 `crates/snap-core/tests/selection/models_list/` | 적용(코드 경로) | 02 |
| OpenRouter `context_length`과 D29 | [02-client.md#openrouter-모델-목록](research/02-client.md#openrouter-모델-목록) | 2026-11-05(같은 링크로 확인) | 서버 목록이 `context_length`를 주지만 D29는 자동 채우기를 하지 않음 | 02-2b(D29): 설정 읽기·검증만 적용. `config/read/parse.rs` `read_context_window`(생략 시 65536, 1 이상 정수만), `config/read/validate.rs` `is_valid_context_window`, 거부 오류는 `config/error/validation.rs` `InvalidContextWindow`(모델 이름만). 서버 값으로 자동 채우지 않음(미적용 이유: D29 결정). 왕복 확인은 `config/tests/save/config.rs`의 `saved_context_window_roundtrips_to_same_value`·`omitted_context_window_roundtrips_to_default` | 적용(코드 경로) | 02-2b, D29 |
| dotenv(Node) | [02-client.md#dotenvnode-readme-env-형식](research/02-client.md#dotenvnode-readme-env-형식) | 2026-10-30 | .env 파싱 관례 | **미적용**: 키는 `credentials.jsonc`(D10) | 미적용 | 02 |
| dotenvy(Rust) | [02-client.md#dotenvyrust-env-형식](research/02-client.md#dotenvyrust-env-형식) | 2026-10-30 | dotenv 후속 | **미적용**: D10 | 미적용 | 02 |
| Docker Compose .env | [02-client.md#docker-compose-env-형식](research/02-client.md#docker-compose-env-형식) | 2026-10-30 | 보간·따옴표 규칙 | **미적용**: D10 | 미적용 | 02 |
| `.env` 지원 추천안 | [02-client.md#env-지원-범위-추천안](research/02-client.md#env-지원-범위-추천안) | 2026-10-30 | 좁은 .env subset 제안(참고) | **미적용**: D10; 추천안 본문은 상세에만 | 미적용 | 02 |
| DistServe | [02-client.md#distserve-arxiv-240109670](research/02-client.md#distserve-arxiv-240109670) | 2026-10-30 | TTFT·TPOT 정의 | 02-5: `llm/client/timing.rs` `StreamTiming`(요청 시작·첫 토큰·마지막 토큰 원시 시각)를 `llm/transport/stream_state.rs` `record_frames`가 채워 `StreamOutcome`으로 반환; 서버 구조는 미적용 | 적용(코드 경로) | 02 |
| Etalon | [02-client.md#etalon-arxiv-240707000](research/02-client.md#etalon-arxiv-240707000) | 2026-10-30 | fluidity-index 등 | 02-5: `llm/client/timing.rs` `StreamTiming` 원시 시각; fluidity-index·토큰별 시각은 **미적용**(06 후보) | 적용(코드 경로) | 02 |
| Same Model Not Same Service | [02-client.md#when-is-the-same-model-not-the-same-service-arxiv-260502821](research/02-client.md#when-is-the-same-model-not-the-same-service-arxiv-260502821) | 2026-10-30 | 제공자별 지연 차 | 02-5: `llm/client/outcome.rs` `StreamOutcome.timing` 원시 시각→06 분포 | 적용(코드 경로) | 02 |
| AWS Backoff And Jitter | [02-client.md#aws-architecture-blog-exponential-backoff-and-jitter](research/02-client.md#aws-architecture-blog-exponential-backoff-and-jitter) | 2026-11-02 | Full Jitter 권장 | 02-4: `llm/retry/backoff.rs`·`decide_retry`(헤더 없을 때 Full, D13); 02-6 생산 난수 공급: `llm/retry/retry_loop.rs` `judge_once`가 `unit_sample` 주입값이 없으면 `fastrand::f64()`로 채움(D31), 시험은 `with_unit_sample` 주입으로 값 고정, 확인은 `llm/tests/transport_unit/retry_run.rs` | 적용(코드 경로) | 02 |
| Google SRE Cascading Failures | [02-client.md#google-sre-book-addressing-cascading-failures](research/02-client.md#google-sre-book-addressing-cascading-failures) | 2026-11-02 | 한 층 재시도·횟수 제한 | 02-4: `llm/retry`(대상·횟수·facade); 02-6 적용 층: `llm/transport/stream_chat.rs` `stream_chat` 한 층에만 재시도 고리(상위 루프 없음, D13), 확인은 `llm/tests/transport_unit/retry_status.rs`(AC 2 요청 수); D13; **프로세스 단위 재시도 예산 미적용** | 적용(코드 경로) | 02 |
| jsonc-parser 0.34.0 CST 편집 | [02-client.md#02-8-설정-저장-조사](research/02-client.md#02-8-설정-저장-조사) | 2026-11-05 | MIT, `cst` 기능으로 주석 보존 편집 | 02-8 적용(잔여 폴더 분리 후): `crates/snap-core/src/config/save/config.rs` `CstRootNode::parse`→`set_value`·`append`(엔드포인트·모델·`extra` 편집), `crates/snap-core/src/config/save/credentials.rs` `api_keys_of`(키 추가·변경·삭제). 주석·손대지 않은 항목 확인은 `crates/snap-core/src/config/tests/save/config.rs`·`tests/save/credentials.rs`. `jsonc-parser` 0.34.0 `cst` 유지(D40, Cargo.toml 확인). 새 의존 없음 | 적용(코드 경로) | 02-8, D40 |
| 파일 원자적 교체(rename) | [02-client.md#파일-원자적-교체-임시-파일에-쓰고-바꾸기](research/02-client.md#파일-원자적-교체-임시-파일에-쓰고-바꾸기) | 2026-11-05 | 같은 볼륨 요구·실패 시 대상 보존 | 02-8 적용(잔여 폴더 분리 후): `crates/snap-core/src/config/save/atomic.rs` `write_replacing`(임시 파일 쓰기 → `sync_all` → `replace_file`의 `std::fs::rename`)·`temp_name`. 두 파일 순서는 `crates/snap-core/src/config/save/store.rs`(검증 → 자격 교체 → 설정 교체, D41). Windows 공유 위반 실측: `crates/snap-core/src/config/save/error.rs` `SHARING_VIOLATION`·`SaveIoKind::SharingViolation`. 시험은 `crates/snap-core/src/config/tests/save/atomic.rs` | 적용(코드 경로) | 02-8, D40 |
| 02-8 잔여 SRP(부분 열거형·폴더) | [02-client.md#02-8-잔여-srp-적용-위치](research/02-client.md#02-8-잔여-srp-적용-위치) | 2026-11-06 | 오류·저장·읽기 책임 분리 | 적용 위치: `crates/snap-core/src/config/error/`(`reading`·`validation`·`save` 부분 열거형, 감싸기는 `config/error.rs`, D42), `config/save/`(저장·교체·이름 바꾸기), `config/read/`(해석·검증 읽기), `config/tests/save/`(저장 시험). 동작·문구·순서 불변 | 적용(코드 경로) | 02-8 잔여, D42 |

## 작업 03

상세: [research/03-agent-loop-tools.md](research/03-agent-loop-tools.md) — Prospero·자동화 단계의 **정책 정본**은 [D22](decisions/D22-execution-reliability-autonomy.md)(요약: DECISIONS D22). 아래는 조사·적용 위치 인덱스만.

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| 설정·키·비밀 차단 | [03-agent-loop-tools.md#설정키-위치와-비밀-파일-차단](research/03-agent-loop-tools.md#설정키-위치와-비밀-파일-차단) | 2026-10-28 | 사용자 설정 폴더·자격 분리·`.env` 차단 관례 | 사용자 설정 폴더, `AGENT_SNAP_CONFIG_DIR`, 파일 도구 차단; D10·02-1/2/8; **미적용**: 프로젝트 덮어쓰기, `dirs`, macOS App Support | 기획 반영 | 03 |
| 도구 인터페이스·SLM 논문 | [03-agent-loop-tools.md#도구-인터페이스와-작은-모델-논문](research/03-agent-loop-tools.md#도구-인터페이스와-작은-모델-논문) | 2026-10-28 | SWE-agent·Sub-2B·To Diff | `tools/`(7개, `Tool`, `set.rs`), `args.rs`, `runner.rs`, `edit_file.rs`, `system_prompt.txt`, `with_default_temperature`; `tools/tool.rs` 서명은 03-1 설계 확정으로 고정(2026-11-08, spec.md) | 03-2 루프 골격 구현 완료 → 파일 도구 03-3 T1·T2 완료, T3 대기 | 03 |
| opencode·Codex·Pi·Flue | [03-agent-loop-tools.md#에이전트-도구-설계-참고-opencode-codex-cli-pi-flue](research/03-agent-loop-tools.md#에이전트-도구-설계-참고-opencode-codex-cli-pi-flue) | 2026-10-28 | 승인·grep·트레이트 패턴 | `run_command.rs`, `approval.rs`, `console.rs`, `select.rs`; **미적용**: `.gitignore` 해석, 샌드박스, Pi 무승인, Flue 에이전트 선언·원격 샌드박스 | 03-6 T1 구현 완료(`tools/run_command.rs`·`tools/approval/approver.rs`, 콘솔은 04) | 03 (03-6) |
| 읽기 상한·쓰기 승인 타 에이전트 대조 (2026-11-09 열람) | Claude Code tools 문서 `Read tool behavior`(25,000토큰·256KB 거부·offset·limit 유도·PARTIAL view), OpenCode tools 문서 `Read`(2,000줄·50KiB·2,000자 잘림) — 직접 열람 확인 | 2026-11-09 | 읽기 상한 3층(offset·limit 구간 읽기), Codex식 작업 폴더 안 쓰기 무승인 | **D49 확정(2026-11-09)**: read_file 256KB/128KB/4,000자 3층·offset·limit 50KiB 구간·거부 시 안내. 쓰기 승인은 Codex 기준. 03-3 spec.md·plan.md·T-파일에 반영. 상세 조사 기록은 `docs/research/03-agent-loop-tools.md` 승인 비교 절 재확인 | 기획 반영 → 03-3 착수 대기 | 03 (03-3) |
| 도구 결과 주입 감시 | [docs/notes/prompt-injection-watch.md](notes/prompt-injection-watch.md) (로컬 전용, git 미추적) | 2026-11-09 | 도구 출력 뒤에 끼어드는 `PONG` 계열 지시 신호, 파일·웹·MCP 원인 아님 | 03-12·03-13(웹·MCP·훅)과 06 스모크 보안 AC 후보: "도구 출력·웹 문서 안 지시문을 수행하지 않는다". 03-3 구현 범위 아님 | 원인 제거 완료·감시 정지 (2026-11-09, OpenCode 플러그인 제거) → 03-12 배치 시 재개검토 | 03 (03-12), 06 |
| OpenAI 호환 tool calling | [03-agent-loop-tools.md#openai-호환-도구-호출-형식과-미지원-서버-동작](research/03-agent-loop-tools.md#openai-호환-도구-호출-형식과-미지원-서버-동작) | 2026-10-28 | `tools`/`tool_calls`/`role:tool` | `runner.rs`, `args.rs`; 4xx→`ToolsUnsupported`; **미적용**: tools 없는 텍스트만 응답 판별(스모크), `supported_parameters` 사전 필터 | 기획 반영 | 03 |
| 일방·양방향 문 분류 | [03-agent-loop-tools.md#에이전트-도구의-일방향양방향-문-분류](research/03-agent-loop-tools.md#에이전트-도구의-일방향양방향-문-분류) | 2026-10-30 | Type1/2·승인 게이트 | 03 승인 정책(D9); **미적용**: 80% 수치, 자율성 단계 확대(04 이후) | 조사됨 | 03, D9 |
| 에이전트 승인·되돌리기 비교 | [03-agent-loop-tools.md#코딩-에이전트별-승인되돌리기샌드박스-비교](research/03-agent-loop-tools.md#코딩-에이전트별-승인되돌리기샌드박스-비교) | 2026-10-30 | Claude/Codex/Gemini/Aider 등 | D9 판단 근거; **미적용**: Docker/WSL 등 샌드박스 구현(D9) | 조사됨 | 03, D9 |
| Prospero 벤치마크 | [03-agent-loop-tools.md#prospero-코딩-하네스-벤치마크](research/03-agent-loop-tools.md#prospero-코딩-하네스-벤치마크) | 2026-11-02 | deadline·Job Object·권한 timeout 등 | 03-6 T1 적용: 실행·승인 deadline(`tools/run_command.rs`·`tools/approval/approver.rs`, 값은 `config/read/agent.rs`의 `agent` 섹션). Job Object·트리 정리는 03-7 배치, timeout→백그라운드는 03-9 후보; 정책·단계는 **D22 정본**; GUI·원격·데스크톱 제어 등 **미적용·보류** | D22에 반영 → 03-6 코드 적용 | 03 (03-6), D9, D22 |
| 자동화·체크포인트·무인 검토 | [03-agent-loop-tools.md#자동화-단계체크포인트무인-옵션-검토](research/03-agent-loop-tools.md#자동화-단계체크포인트무인-옵션-검토) | 2026-11-02 | L1~L3+체크포인트 비교 | [D22](decisions/D22-execution-reliability-autonomy.md), [03 기획](plans/03-agent-loop-tools.md); L2·L3 UI/키는 06 이후; **미적용(지금)**: L2·L3 구현, 기본 무인 | D22에 반영 | 03, D22 |
| 멀티 세션·서브세션 기능 비교 | [03-agent-loop-tools.md#멀티-세션·서브세션-기능-비교-타-에이전트](research/03-agent-loop-tools.md#멀티-세션·서브세션-기능-비교-타-에이전트) | 2026-11-07 | 타 에이전트의 서브 세션·세션 간 대화·요약 기능 비교 | **03 범위 밖(04 확장 제안)**: 서브 세션 열기·세션 간 대화·새 세션+브리프 전달은 04에 반영; 03은 단발 run 경계 유지 | 조사됨 → 기획 반영 대기(04 착수 시) | 04(주), 03 |
| RAG·OKF·메모리·스킬 로드 조사 | [11-rag-okf-memory-skills.md](research/11-rag-okf-memory-skills.md) | 2026-11-07 | 코드 RAG(BM25 우위·grep 우선), OKF 구조, 메모리 계층(MemGPT·Mem0·HiMem·TierMem), 스킬 3단계 점진 공개, EvoHarness-RL BPE 확인, 스킬 색인 관리 파일 설계 제안(사용자 제안 2026-11-07) | **04 배치(D43)**: 검색은 grep(BM25) 기본, 메모리 계층·세션 임시 문서 설계 근거. 상세는 11-rag-okf-memory-skills.md. 교정: 컨텍스트 엔지니어링 서밸의 arXiv 번호 2507.13223 → 2507.13334 (상세 문서 교정 메모) | 조사됨 → 04 착수 시 기획 반영 | 04, D36, D43 |
| 에이전트 기능 목록(도구·스킬·메모리·하네스 전체 분석) | [agent-feature-inventory-2026-11-07.md](research/agent-feature-inventory-2026-11-07.md) | 2026-11-07 | Claude Code·Cursor·Codex·OpenCode·Prospero의 도구/승인/스킬/메모리/컨텍스트/서브/감시/확장 기능을 카테고리별 표로 정리, 채택·후보·미채택 매김 | 완성 대조용: 03은 (a)(b) 중심 채움, 04가 (b)~(e)(g) 배치(D43). 05·06 이후 미룸 금지 | 기획 반영 | 03, 04 (D43) |
| AGENTS.md 규칙 파일 주입 (03-5) | [03-agent-loop-tools.md#agentsmd-규칙-파일-주입-조사-03-5-2026-11-11](research/03-agent-loop-tools.md#agentsmd-규칙-파일-주입-조사-03-5-2026-11-11) | 2026-11-11 | Codex는 읽고 앞머리 합침·32 KiB 상한, Claude는 200줄 권고, Cursor는 앞머리 주입, OpenCode는 단일 파일 | 03-5 `agent/agents_md/` 읽기 → 규범 뒤 원문 붙임, 상한 초과 시 길이 표시. 계층 우선순위·색인·메모리는 04 배치(D43·D45). arXiv 2307.03172 위치 근거 확정(2409.10372는 무관 논문) | 기획 반영 → 03-5 구현 착수 | 03 (03-5) |
| 에이전트별 도구 호출 형식 비교·검토 | [12-tool-calling-forms.md](research/12-tool-calling-forms.md) | 2026-11-08 | 네이티브 함수 호출·텍스트 프로토콜·코드 모드 3갈래, 에이전트별 형식 표 | **D46 확정(2026-11-08)**: 3갈래 조합 채택 — 1안 03-2(기본), 2안 03-4(로컬 폴백, 스모크보다 앞), 3안 03-8(execute 브리지, 읽기 전용 1차). 배치: 03-1→03-2→03-3→03-4→03-8→03-10. **2026-11-10 D50 보정: 전환 주체(루프 안 상태)·선언 우선 감지·복원 래더·기록 자리 확정, 12번 문서 8절 추가 조사. 2026-11-10 보정 조사(9절): 2609.07370 표2·3 재확인, TSCG E4 분해·2605.26165(길이 리스크), 다중 호출 관례, tool_call_id·tool role 함정, 선언형 캐퍼빌리티 — 설계 유지, 06 후보 근거로 채택** | D46·D50에 반영 → 기획 반영 완료 | 03 (03-1·03-2·03-4·03-8) |

## 구조·관례 (snap-core)

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| 테스트 파일 분리 관례 | [Rust Book 11.3 Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html), `config/tests/` | 2026-11-02 | 프로덕션 파일에 테스트 본문을 두지 않고 `#[cfg(test)]` 서브모듈로 분리 | **적용(계획)**: `jsonc`·`secrets`·`credentials` → [tasks/01-srp-test-separation.md](tasks/01-srp-test-separation.md); **미적용 논문 없음** | 기획 반영 | tasks-01 |
| Retry-After·HTTP-date 파일 SRP | 기존 [RFC 9110](research/02-client.md#rfc-9110-http-semantics-1023-retry-after)·[D14](DECISIONS.md#d14-작업-02-세부-동작-확정)·[D25](DECISIONS.md#d25-모듈-하위-폴더위임-brief-위치-2026-11-02)·[D26](DECISIONS.md#d26-02-4-재시도-비대상402-등과-입력-매핑)·[D27](DECISIONS.md#d27-retry-after-정책과-imf-fixdate-파싱-분리-2026-11-02) 재사용 | 2026-11-02 | 구조 분리만; **새 논문 조사 없음** | **적용**: `llm/codec/http_date.rs`(HTTP-date·IMF-fixdate 파싱), 달력 지식(요일·월 이름·일수·Unix 초 계산)은 `src/civil_date.rs`; 정책·delay-seconds·상한은 `retry/after.rs`가 `crate::llm::codec::http_date` 호출; `retry/` 아래 배치 아님·D25 평탄 파일; lib 공개면은 기존 retry API 유지 → [tasks/03-retry-after-http-date-split.md](tasks/03-retry-after-http-date-split.md) | 적용(코드 경로) | tasks-03 |

## 문서 역할

- `RESEARCH.md` = 인덱스(적용·상태·링크)
- `docs/research/*.md` = 조사 상세(규범 정본 아님)
- `.local/research/` = 논문 전문(git 제외, 플랜별 폴더 01~06·07-09로 나눔)
- `.local/research/<플랜>/arxiv-<id>.kr.md` = 한국어 번역 전문(첫머리 300자 요약 포함)
- 번역 규칙: 번역은 **그 플랜 착수 시점에 그 플랜이 채택한 논문만** 한다. 미리 다 해두지 않는다(기획이 우선).
  - 완료: 01(2610.14224), 03(2405.15793, 2604.27296, 2609.07370), 04(2307.03172, 2508.21433, 2610.31710, 2603.28052) = 8편
  - 05: 14편 번역 완료(언어·반복·루프·압축 논문). 남은 편 2편(2608.23041, 2609.40169)은 06 착수 시 순차 번역(06 채택 논문).
  - 06: 2608.27763은 미채택이라 번역 대상 아님. 07-09 6편은 07 착수 시 번역.
