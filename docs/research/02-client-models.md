# 02-7 모델 선택과 모델 목록 조회 조사 상세

- 인덱스: [docs/RESEARCH.md](../RESEARCH.md) 작업 02 절
- 역할: 조사 상세. 규범 정본이 아님. 정책은 `docs/DECISIONS.md`와 `docs/plans/02-client.md`.
- 확인 날짜: 2026-11-27. 링크는 리서치 서브와 메인이 직접 열어 확인한 것만 적었다.
- 백업 확인: `C:\Workspace\agent-snap-backup\docs\research\02-client.md`의 02-7 모델 목록 절(서버 6종, 2026-11-05 재확인분)을 열어 기존 정리와 신규 조사를 대조했다. 백업의 서버 6종 결론(`data[].id`만 필수, Bearer 통일, `context_length` 자동 채우기 미적용=D29)은 그대로 유효하고, 이번 조사에서 자동 발견 근거를 더했다.

## 서버별 `/v1/models` 응답 (백업 정본 요약, 2026-11-05 확인)

| 서버 | 경로 | 응답 핵심 | 인증 |
|---|---|---|---|
| OpenAI | `GET /v1/models` | `object:"list"`, `data[].id`·`created`·`owned_by`, `shutdown_date` 선택 | Bearer 키 |
| Ollama | `/v1/models`, `/v1/models/{model}` | `created`(마지막 수정), `owned_by`(기본 "library"). 로컬은 키를 보내도 무시 | 로컬 불요, 클라우드(ollama.com)는 https+키 |
| LM Studio | OpenAI 호환 `/v1/models` (필드 미문서). 고유 REST `/api/v1/models`는 `key`·`display_name`·`loaded_instances`로 형식 완전 다름 | 인증 기본 꺼짐, 켜면 Bearer(0.4.0+) |
| llama.cpp | `/v1/models` (+`/models`) | `id`가 기본 모델 파일 경로(`--alias`로 변경), `meta`는 로딩 중 `null`, `architecture`·`output_modalities`·`input_modalities`·`multimodal` 추가 필드. 단일 모델은 `data` 항목 하나, 라우터 모드는 로드된 모델 합침 | `--api-key` Bearer·`x-api-key` |
| vLLM | `/v1/models` (OpenAI 형식, 필드 미문서) | `--api-key`는 `/v1`·`/v2`·`/inference`만 검사 | 키 있으면 Bearer |
| OpenRouter | `GET /api/v1/models` | `data[]`에 `id`·`canonical_slug`·`name`·`pricing`·`context_length` 등 다수. `total_count`·`links`. 쪽 나누기 `offset`·`limit`(기본 500, 최대 1000), 기본 필터 `output_modalities=text` | Bearer 키 필수 |

- 공통 결론: 최상위 `data` 배열에서 문자열 `id`만 필수로 읽는다. `id`가 없거나 문자열이 아니면 형식 오류로 알린다. 모르는 필드는 버린다.
- 인증은 Bearer 헤더로 통일해 보내면 키가 필요한 서버와 무시하는 서버가 모두 동작한다(02-5 정책 그대로).
- LM Studio 고유 REST는 쓰지 않는다(형식 상이, OpenAI 호환만 사용).
- `context_length`로 설정을 자동 채우지 않는다(D29).

## llama.cpp 서버 README (2026-11-27 직접 확인)

- 링크: [llama.cpp tools/server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- 핵심: `--port` 기본값이 `9931`이다(README 193행 `default: 9931`, 462행 `127.0.0.1:9931`). 예전 문서에 있던 `8080` 기본값은 docker 예시에만 남고 기본값은 아니다.
- 핵심: `GET /v1/models` 항목은 `multimodal` 능력을 밝힌다(480행 부근 "Clients should check `/models` or `/v1/models` for the `multimodal` capability"). 클라이언트가 능력을 요구하지 않으므로 02-7은 `id`만 읽는다.
- 적용: 자동 발견 기본 포트 상수를 `9931`로 두는 근거. llama.cpp는 `kind`로 지정해도 `base_url`을 설정하지 않으면 자동 발견 대상에서 뺀다(포트가 버전마다 바뀌어 상수 고정 위험).

## OpenCode 프로바이더 문서 (2026-11-27 직접 확인)

- 링크: [OpenCode Providers](https://opencode.ai/docs/providers/)
- 핵심: Ollama는 `http://127.0.0.1:11434`를 프로브하고, LM Studio와 vLLM의 기본 엔드포인트는 `http://127.0.0.1:1234/v1`, `http://127.0.0.1:8000/v1`이다.
- 핵심: OpenCode는 발견을 주기적으로 갱신하고, 일시 장애 시 마지막 성공 목록을 유지한다. LM Studio는 `/api/v1/models`로, vLLM은 `/health` 확인 뒤 `/v1/models`로 발견한다.
- 채택하지 않음: 상시 주기 프로브. snap은 단일 사용자 로컬 에이전트라 상시 폴링·인벤토리 유지가 필요 없다. 설정이 정본이고, 목록은 참고용이다.
- 근거로만 사용: 기본 포트 값(11434, 1234)과 `/v1` 포함 baseURL 관례. OpenCode는 `baseURL`에 `/v1`을 포함하라고 안내한다(이중 `/v1/v1` 방지).

## 자동 발견 범위 판단 (사용자 결정 2026-11-27)

- 결정: 설정 우선 + 미설정 시 기본 포트 한 번만 프로브. 상시 폴링·mDNS 없음.
- 근거: OpenCode의 상시 프로브는 상시 켜진 서버를 전제로 한 설계다. snap은 설정 파일의 `endpoints`가 정본이고, 목록 조회는 설정 저장을 돕는 참고 기능이다.
- 대상: `kind`가 `ollama`(11434)·`lmstudio`(1234)이고 `base_url`이 비어 있는 경우만 기본 주소를 채운다. `llamacpp`·`vllm`은 기본 포트가 불안정하거나(9931로 변경, docker 예시 8080) 기본값이 없어 대상에서 뺀다.
- `base_url`에 `/v1`이 이미 있으면 `/models`를 그대로 붙이고, 없으면 `/v1/models`를 붙인다(이중 경로 방지).

## 미적용 기록

- OpenRouter 쪽 나누기(`offset`·`limit`): 미적용. 매개변수 없이 호출하면 전체를 돌려준다고 문서화됨(백업 2026-11-05 확인).
- OpenRouter 기본 필터(`output_modalities=text`): 미적용. 설정 `models` 수동 입력이 정본이고 목록은 참고용.
- mDNS·상시 프로브·인벤토리 캐시: 미적용(위 자동 발견 범위 판단).
- llama.cpp `multimodal`·`architecture` 능력 해석: 미적용(YAGNI. `id`만 읽음).
