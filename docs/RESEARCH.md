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
| RFC 9110 Retry-After | [02-client.md#rfc-9110-http-semantics-1023-retry-after](research/02-client.md#rfc-9110-http-semantics-1023-retry-after) | 2026-10-28 | delay-seconds·HTTP-date, 503·3xx 예 | 02-4: 두 형식 해석, 5xx도 헤더 따름 | 기획 반영 | 02 |
| RFC 6585 429 | [02-client.md#rfc-6585-4절-429-too-many-requests](research/02-client.md#rfc-6585-4절-429-too-many-requests) | 2026-10-28 | 429·`Retry-After` | 02-4, 02-6; 대기 상한 초과 시 즉시 오류 | 기획 반영 | 02 |
| WHATWG SSE | [02-client.md#whatwg-html-standard-server-sent-events](research/02-client.md#whatwg-html-standard-server-sent-events) | 2026-10-28 | CRLF/LF/CR, 주석·data 결합·BOM | 02-3; 파일 끝 미완 이벤트는 내보냄(D12) | 기획 반영 | 02 |
| llama.cpp server README | [02-client.md#llamacpp-server-readme](research/02-client.md#llamacpp-server-readme) | 2026-10-28 | SSE, `reasoning_content`, 503 | 02-3, 02-4; `<think>` 태그 분리는 안 함(05) | 기획 반영 | 02 |
| Ollama OpenAI compatibility | [02-client.md#ollama-openai-compatibility](research/02-client.md#ollama-openai-compatibility) | 2026-10-28 | 스트림·tools; `reasoning_effort` | 02-5: `tool_choice`/`n`/`stream_options` 미전송; `reasoning`→`reasoning_effort`(D15) | 기획 반영 | 02 |
| vLLM Reasoning Outputs | [02-client.md#vllm-reasoning-outputs](research/02-client.md#vllm-reasoning-outputs) | 2026-10-28 | `reasoning` vs `reasoning_content` | 02-3: 둘 다 reasoning 이벤트 | 기획 반영 | 02 |
| LM Studio OpenAI compat | [02-client.md#lm-studio-openai-compatibility-endpoints](research/02-client.md#lm-studio-openai-compatibility-endpoints) | 2026-10-28 | `/v1/chat/completions`, reasoning 문서 없음 | 02-3: 두 이름 처리 또는 미지 필드 보존 | 기획 반영 | 02 |
| OpenRouter Streaming | [02-client.md#openrouter-streaming](research/02-client.md#openrouter-streaming) | 2026-10-28 | 주석 줄, HTTP 200 중 `error` 청크 | 02-3, 02-5(취소 시 연결 정리) | 기획 반영 | 02 |
| OpenRouter Reasoning Tokens | [02-client.md#openrouter-reasoning-tokens](research/02-client.md#openrouter-reasoning-tokens) | 2026-10-28 | `reasoning_details`·`reasoning` | 02-3: `reasoning`만 이벤트, details는 원문 보존(YAGNI) | 기획 반영 | 02 |
| OpenRouter Rate Limits | [02-client.md#openrouter-api-rate-limits](research/02-client.md#openrouter-api-rate-limits) | 2026-10-28 | 429·`X-RateLimit-*` | 02-4, 02-6; `X-RateLimit-Reset` 미사용; 긴 대기는 포기 | 기획 반영 | 02 |
| tokio-util CancellationToken | [02-client.md#tokio-util-cancellationtoken](research/02-client.md#tokio-util-cancellationtoken) | 2026-10-28 | cancel-safe `select!` | 02-5, 02-6 | 기획 반영 | 02 |
| reqwest ClientBuilder | [02-client.md#reqwest-clientbuilder](research/02-client.md#reqwest-clientbuilder) | 2026-10-28 | connect vs read vs total timeout | 02-5: connect만 reqwest; header/idle은 별도 | 기획 반영 | 02 |
| OpenAI List models | [02-client.md#openai-api-참조-모델-목록](research/02-client.md#openai-api-참조-모델-목록) | 2026-10-28 | `data[].id` 중심 | 02-7 | 기획 반영 | 02 |
| llama.cpp 모델 목록 | [02-client.md#llamacpp-server-readme-모델-목록](research/02-client.md#llamacpp-server-readme-모델-목록) | 2026-10-28 | `GET /v1/models`, `id`가 경로일 수 있음 | 02-7, 02-2(id·이름 분리) | 기획 반영 | 02 |
| Ollama 모델 목록 | [02-client.md#ollama-openai-호환-문서-모델-목록](research/02-client.md#ollama-openai-호환-문서-모델-목록) | 2026-10-28 | 로컬 키 무시 | 02-1·02-2, 02-5(무키 시 헤더 생략), 02-7 | 기획 반영 | 02 |
| llama.cpp base URL | [02-client.md#llamacpp-서버-readme-http-serve와-기본-base-url](research/02-client.md#llamacpp-서버-readme-http-serve와-기본-base-url) | 2026-11-01 | 기본 `http://localhost:8080/v1` | 02-2: `validate.rs` `is_valid_base_url`, `parse.rs` `read_endpoint` | 적용(코드 경로) | 02 |
| Ollama 인증 | [02-client.md#ollama-openai-compatibility-인증](research/02-client.md#ollama-openai-compatibility-인증) | 2026-11-01 | 로컬 무키·클라우드 HTTPS+키 | 02-2: `is_valid_base_url`, `keys.rs` `EndpointConfig::api_key` | 적용(코드 경로) | 02 |
| vLLM OpenAI 서버 | [02-client.md#vllm-openai-호환-서버](research/02-client.md#vllm-openai-호환-서버) | 2026-10-28 | `/v1/models`, `--api-key` 경로 제한 | 02-5, 02-7: Bearer 항상, `data[].id`만 필수 | 기획 반영 | 02 |
| LM Studio 모델·인증 | [02-client.md#lm-studio-모델-목록과-인증](research/02-client.md#lm-studio-모델-목록과-인증) | 2026-10-28 | OpenAI `/v1/models`만; 고유 REST 미사용 | 02-7: `data` 배열 아니면 오류 | 기획 반영 | 02 |
| OpenRouter 모델 목록 | [02-client.md#openrouter-모델-목록](research/02-client.md#openrouter-모델-목록) | 2026-10-28 | Bearer·풍부한 `data[]` | 02-7: `id`만; **쪽 나누기 미적용**(무파라미터=전체라 문서화) | 기획 반영 | 02 |
| 모델 목록 서버별 정리 | [02-client.md#모델-목록-서버별-정리](research/02-client.md#모델-목록-서버별-정리) | 2026-10-28 | 공통 필수는 `data[].id`, Bearer 통일 | 02-7 | 기획 반영 | 02 |
| dotenv(Node) | [02-client.md#dotenvnode-readme-env-형식](research/02-client.md#dotenvnode-readme-env-형식) | 2026-10-30 | .env 파싱 관례 | **미적용**: 키는 `credentials.jsonc`(D10) | 미적용 | 02 |
| dotenvy(Rust) | [02-client.md#dotenvyrust-env-형식](research/02-client.md#dotenvyrust-env-형식) | 2026-10-30 | dotenv 후속 | **미적용**: D10 | 미적용 | 02 |
| Docker Compose .env | [02-client.md#docker-compose-env-형식](research/02-client.md#docker-compose-env-형식) | 2026-10-30 | 보간·따옴표 규칙 | **미적용**: D10 | 미적용 | 02 |
| `.env` 지원 추천안 | [02-client.md#env-지원-범위-추천안](research/02-client.md#env-지원-범위-추천안) | 2026-10-30 | 좁은 .env subset 제안(참고) | **미적용**: D10; 추천안 본문은 상세에만 | 미적용 | 02 |
| DistServe | [02-client.md#distserve-arxiv-240109670](research/02-client.md#distserve-arxiv-240109670) | 2026-10-30 | TTFT·TPOT 정의 | 02-5 시각 반환; 서버 구조는 미적용 | 기획 반영 | 02 |
| Etalon | [02-client.md#etalon-arxiv-240707000](research/02-client.md#etalon-arxiv-240707000) | 2026-10-30 | fluidity-index 등 | 02-5 원시 시각; fluidity-index·토큰별 시각은 **미적용**(06 후보) | 기획 반영 | 02 |
| Same Model Not Same Service | [02-client.md#when-is-the-same-model-not-the-same-service-arxiv-260502821](research/02-client.md#when-is-the-same-model-not-the-same-service-arxiv-260502821) | 2026-10-30 | 제공자별 지연 차 | 02-5 원시 시각→06 분포 | 기획 반영 | 02 |
| AWS Backoff And Jitter | [02-client.md#aws-architecture-blog-exponential-backoff-and-jitter](research/02-client.md#aws-architecture-blog-exponential-backoff-and-jitter) | 2026-10-30 | Full Jitter 권장 | 02-4, D13: `Retry-After` 없을 때 Full | 기획 반영 | 02 |
| Google SRE Cascading Failures | [02-client.md#google-sre-book-addressing-cascading-failures](research/02-client.md#google-sre-book-addressing-cascading-failures) | 2026-10-30 | 한 층 재시도·횟수 제한 | 02-4, 02-6, D13; **프로세스 단위 재시도 예산 미적용**(단일 사용자) | 기획 반영 | 02 |

## 작업 03

상세: [research/03-agent-loop-tools.md](research/03-agent-loop-tools.md) — Prospero·자동화 단계의 **정책 정본**은 [D22](decisions/D22-execution-reliability-autonomy.md)(요약: DECISIONS D22). 아래는 조사·적용 위치 인덱스만.

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| 설정·키·비밀 차단 | [03-agent-loop-tools.md#설정키-위치와-비밀-파일-차단](research/03-agent-loop-tools.md#설정키-위치와-비밀-파일-차단) | 2026-10-28 | 사용자 설정 폴더·자격 분리·`.env` 차단 관례 | 사용자 설정 폴더, `AGENT_SNAP_CONFIG_DIR`, 파일 도구 차단; D10·02-1/2/8; **미적용**: 프로젝트 덮어쓰기, `dirs`, macOS App Support | 기획 반영 | 03 |
| 도구 인터페이스·SLM 논문 | [03-agent-loop-tools.md#도구-인터페이스와-작은-모델-논문](research/03-agent-loop-tools.md#도구-인터페이스와-작은-모델-논문) | 2026-10-28 | SWE-agent·Sub-2B·To Diff | `tools/`(7개, `Tool`, `set.rs`), `args.rs`, `runner.rs`, `edit_file.rs`, `system_prompt.txt`, `with_default_temperature` | 기획 반영 | 03 |
| opencode·Codex·Pi·Flue | [03-agent-loop-tools.md#에이전트-도구-설계-참고-opencode-codex-cli-pi-flue](research/03-agent-loop-tools.md#에이전트-도구-설계-참고-opencode-codex-cli-pi-flue) | 2026-10-28 | 승인·grep·트레이트 패턴 | `run_command.rs`, `approval.rs`, `console.rs`, `select.rs`; **미적용**: `.gitignore` 해석, 샌드박스, Pi 무승인, Flue 에이전트 선언·원격 샌드박스 | 기획 반영 | 03 |
| OpenAI 호환 tool calling | [03-agent-loop-tools.md#openai-호환-도구-호출-형식과-미지원-서버-동작](research/03-agent-loop-tools.md#openai-호환-도구-호출-형식과-미지원-서버-동작) | 2026-10-28 | `tools`/`tool_calls`/`role:tool` | `runner.rs`, `args.rs`; 4xx→`ToolsUnsupported`; **미적용**: tools 없는 텍스트만 응답 판별(스모크), `supported_parameters` 사전 필터 | 기획 반영 | 03 |
| 일방·양방향 문 분류 | [03-agent-loop-tools.md#에이전트-도구의-일방향양방향-문-분류](research/03-agent-loop-tools.md#에이전트-도구의-일방향양방향-문-분류) | 2026-10-30 | Type1/2·승인 게이트 | 03 승인 정책(D9); **미적용**: 80% 수치, 자율성 단계 확대(04 이후) | 조사됨 | 03, D9 |
| 에이전트 승인·되돌리기 비교 | [03-agent-loop-tools.md#코딩-에이전트별-승인되돌리기샌드박스-비교](research/03-agent-loop-tools.md#코딩-에이전트별-승인되돌리기샌드박스-비교) | 2026-10-30 | Claude/Codex/Gemini/Aider 등 | D9 판단 근거; **미적용**: Docker/WSL 등 샌드박스 구현(D9) | 조사됨 | 03, D9 |
| Prospero 벤치마크 | [03-agent-loop-tools.md#prospero-코딩-하네스-벤치마크](research/03-agent-loop-tools.md#prospero-코딩-하네스-벤치마크) | 2026-11-02 | deadline·Job Object·권한 timeout 등 | 03·D9 재검토 참고; 정책·단계는 **D22 정본**; GUI·원격·데스크톱 제어 등 **미적용·보류** | D22에 반영 | 03, D9, D22 |
| 자동화·체크포인트·무인 검토 | [03-agent-loop-tools.md#자동화-단계체크포인트무인-옵션-검토](research/03-agent-loop-tools.md#자동화-단계체크포인트무인-옵션-검토) | 2026-11-02 | L1~L3+체크포인트 비교 | [D22](decisions/D22-execution-reliability-autonomy.md), [03 기획](plans/03-agent-loop-tools.md); L2·L3 UI/키는 06 이후; **미적용(지금)**: L2·L3 구현, 기본 무인 | D22에 반영 | 03, D22 |

## 구조·관례 (snap-core)

| 제목 | 상세 | 확인 | 핵심(한 줄) | 적용 위치 또는 미적용 | **상태** | 관련 |
|---|---|---|---|---|---|---|
| 테스트 파일 분리 관례 | [Rust Book 11.3 Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html), `config/tests/` | 2026-11-02 | 프로덕션 파일에 테스트 본문을 두지 않고 `#[cfg(test)]` 서브모듈로 분리 | **적용(계획)**: `jsonc`·`secrets`·`credentials` → [tasks/01-srp-test-separation.md](tasks/01-srp-test-separation.md); **미적용 논문 없음** | 기획 반영 | tasks-01 |

## 문서 역할

- `RESEARCH.md` = 인덱스(적용·상태·링크)
- `docs/research/*.md` = 조사 상세(규범 정본 아님)
- `.local/research/` = 논문 전문(git 제외, 본 작업에서 미변경)
