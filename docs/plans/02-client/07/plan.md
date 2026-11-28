# 02-7 모델 선택과 모델 목록 조회: 상세 기획

- 상태: 완료 (T1~T5 구현·개별 리뷰 PASS, 작업 단위 전체 리뷰 r2 PASS. `feature/client-endpoint`를 `feature/client`에 병합. 기획 근거 조사는 [docs/research/02-client-models.md](../../../research/02-client-models.md))
- 상위 문서: [02-client.md](../../02-client.md)
- 선행 결정: 14번(전송 크레이트 reqwest), 15번(`reasoning` 변환), 29번(`context_window` 자동 채우기 미적용)

## 목적

설정에서 고른 모델을 클라이언트로 조립하는 창구를 만든다. 선택 키 `엔드포인트이름/모델이름`으로 엔드포인트와 모델을 찾아 02-6까지 갖춘 전송(재시도 포함)을 만들고, 같은 엔드포인트의 모델 목록을 OpenAI 호환 `/models`로 읽는다. 로컬 서버(Ollama·LM Studio)는 주소가 비어 있으면 기본 포트 한 번만 프로브해 채운다. 에이전트 루프(작업 03)는 조립된 클라이언트와 모델 id만 받는다.

## 리서치 및 설계 근거

- 상세: [docs/research/02-client-models.md](../../../research/02-client-models.md) — 서버 6종 `/v1/models` 응답 비교, llama.cpp 기본 포트 9931 변경, OpenCode 자동 발견(상시 프로브 기각) 기록.
- 백업 확인: `C:\Workspace\agent-snap-backup\docs\research\02-client.md`의 02-7 절(2026-11-05 재확인분). `data[].id`만 필수, Bearer 통일, LM Studio 고유 REST 미사용, `context_length` 자동 채우기 미적용(D29) 결론을 그대로 계승했다.
- 사용자 결정(2026-11-27): 자동 발견은 설정 우선 + 미설정 시 기본 포트 한 번만 프로브. 목록 조회는 OpenAI 호환 `/v1/models`만. 클라이언트 조립은 snap-core에 둔다.

## 범위(In)

- 선택 키 조립: `엔드포인트이름/모델이름` 문자열을 해석해 `Config`에서 엔드포인트와 모델을 찾고, `HttpTransport`(02-5)에 자격 키와 `http` 설정을 넣어 클라이언트로 만들고 모델 id와 함께 돌려준다. 재시도 래퍼(02-6)를 씌운 상태로 돌려준다.
- 모델 목록 조회: `{base_url}/models`를 GET으로 요청하고 최상위 `data` 배열에서 문자열 `id`만 모은다. `id`가 없거나 문자열이 아니면 형식 오류로 알린다. 모르는 필드는 버린다.
- 자동 발견: `kind`가 `ollama`·`lmstudio`이고 `base_url`이 비어 있으면 기본 주소(`http://127.0.0.1:11434/v1`, `http://127.0.0.1:1234/v1`)를 한 번 프로브해 reachable이면 채운다. 상시 폴링·캐시·mDNS는 두지 않는다.
- `/v1` 보정: `base_url`에 `/v1`이 이미 있으면 `/models`·`/chat/completions`를 그대로 붙이고, 없으면 `/v1/models`처럼 붙인다(이중 경로 방지).
- 목록 조회 오류·선택 키 실패 메시지는 02-5 가리기 규칙(키·base_url 호스트 제외)을 따른다.

## 비범위(Out)

- 목록으로 설정을 자동 채우기(설정 저장은 02-9가 맡고, 02-7은 읽기만).
- LM Studio 고유 REST `/api/v1/models`, Ollama 고유 `/api/tags`, OpenRouter 쪽 나누기·필드 해석.
- 상시 프로브·인벤토리 캐시·mDNS·`/health` 선행 확인(vLLM은 자동 발견 대상에서 제외).
- llama.cpp `multimodal`·`architecture` 능력 해석(YAGNI).
- 대화형 CLI 화면(작업 04).

## 선행 세부 작업

[02-6 재시도 적용](../06/plan.md) 완료(PR #11 병합 2026-11-27). 02-2의 `Config`·`EndpointConfig`·`ModelConfig`, 02-1의 자격 키 찾기를 쓴다.

## 리스크

- 자동 발견 프로브가 네트워크 시험을 부른다. 루프백 주소 한 번만 찔러 보고, 실패를 정상 처리로 구분해야 한다. 시험은 모의 서버 재료로 reachable/unreachable을 만든다.
- llama.cpp는 기본 포트가 9931로 바뀌어 버전마다 다르다. 자동 발견 대상에서 빼고 설정 입력을 정본으로 둔다.
- `base_url`에 `/v1`이 있는지 없는지 구분하는 규칙이 어긋나면 `/v1/v1` 이중 경로가 생긴다. 문자열 끝 비교 규칙을 한 곳에서만 구현한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 | 선택 키 해석(`엔드포인트이름/모델이름`)과 엔드포인트·모델 조회, 실패 오류(가리기 준수) | Config 타입 변경, 키·호스트 노출 | 선택 키 타입·해석 함수와 시험 | 게이트 3종 |
| T2 | 클라이언트 조립(전송+자격 키+http 설정+재시도 래퍼), 모델 id 전달 | reqwest 타입 노출, 판단 로직 변경 | 조립 함수와 모의 서버 시험 | 게이트 3종 |
| T3 | 모델 목록 조회(`{base_url}/models` GET, `data[].id`만, 형식 오류, 가리기) | 모르는 필드 해석, LM Studio REST 사용 | 목록 조회 함수·형식 오류 타입과 모의 서버 시험 | 게이트 3종 |
| T4 | 자동 발견(ollama·lmstudio 기본 주소 한 번 프로브, `/v1` 보정 규칙 한곳) | 상시 폴링·캐시·mDNS, llamacpp·vllm 대상화 | 발견 함수·상수와 시험 | 게이트 3종 |
| T5 | 미커밋 작업 파일 전수 구조 점검 표 작성, 위반 잔여 이관 | 코드 변경, 파일 삭제, 커밋·푸시 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/plans/02-client/07/plan.md` | 상세 기획 | 완료 |
| `docs/plans/02-client/07/spec.md` | 명세·AC | 완료 |
| `docs/plans/02-client/07/tasks.md` | 작업 계획·DoD | 완료 |
| `docs/research/02-client-models.md` | 02-7 조사 상세 | 완료 |
| `docs/tasks/02/07/T1-selection-key.md` | T1 위임 문서 | 완료 |
| `docs/tasks/02/07/T2-client-build.md` | T2 위임 문서 | 완료 |
| `docs/tasks/02/07/T2-fix-kind-fallback-test.md` | T2 폴백 시험 보정 | 완료 |
| `docs/tasks/02/07/T3-models-list.md` | T3 위임 문서 | 완료 |
| `docs/tasks/02/07/T3-fix-models-list.md` | T3 minor 3건 보정 | 완료 |
| `docs/tasks/02/07/T4-discovery.md` | T4 위임 문서 | 완료 |
| `docs/tasks/02/07/T4-fix-discovery-secret.md` | T4 주소 가리기 보정 | 완료 |
| `docs/tasks/02/07/T5-structure-check.md` | T5 위임 문서 | 완료 |
| `docs/tasks/02/07/unit-fix-discovery-window.md` | 작업 단위 리뷰 r1 지적 1 보정 | 완료 |

## 문서 영향

- `docs/research/02-client-models.md`: 신규 작성 완료. `docs/RESEARCH.md` 02 절에 서버 목록·자동 발견 행 반영(구현 후 적용 위치 갱신).
- `docs/DECISIONS.md`: 선택 키 조립 위치·자동 발견 범위·`/v1` 보정 규칙 결정을 구현 중 결정으로 추가.
- `docs/plans/02-client.md`·`docs/PLAN.md`: 02-7 완료 갱신(`feature/client` 병합 반영).
- 리뷰 기록: `docs/reviews/02/`에 T1~T5 개별 리뷰와 작업 단위 전체 리뷰.

## DoD

[spec.md](spec.md)의 AC를 모두 충족하고, 게이트 3종(Windows·WSL)을 통과하고, 조사 적용 위치가 갱신되고, 작업 단위 전체 리뷰가 PASS한 상태.
