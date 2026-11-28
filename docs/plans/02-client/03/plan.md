# 02-3 SSE 파싱과 델타 복원: 상세 기획

- 상태: 완료 (승인 2026-11-24. T1~T6 완료·리뷰 PASS 2026-11-25. T8 구조 보정 완료·4차 리뷰 PASS 2026-11-25. T7 구조 점검 완료·8차 Fast-track PASS 2026-11-25. 잔여 없음. PR #6으로 `feature/client` 병합)
- 상위 문서: [02-client.md](../../02-client.md)

## 목적

네트워크 없이 바이트 조각을 이벤트로 바꾸고 하나의 응답으로 합친다. 전송([02-5](../05/plan.md), 예정)은 이 결과를 그대로 쓰므로, 서버별 스트림 차이를 이 세부 작업에서 모두 흡수한다.

## 리서치 및 설계 근거

- 상세: [docs/research/02-client-sse.md](../../../research/02-client-sse.md) (2026-11-24 링크 직접 확인)
- WHATWG Server-sent events: 줄 끝 세 종류, `:` 주석 줄 무시, `data` 여러 줄 개행 결합, 빈 줄에서 이벤트 내보냄, UTF-8, 맨 앞 BOM 하나 버림. 명세는 파일 끝의 미완 이벤트를 버리지만 이 작업은 내보낸다(결정 9번).
- llama.cpp server README: 스트림은 SSE, 생각은 `reasoning_content`(`--reasoning-format deepseek`), 로딩 중 503.
- vLLM Reasoning Outputs: 생각 필드는 `reasoning`(스트림은 `delta.reasoning`), 예전 이름이 `reasoning_content`. 한쪽만 읽으면 빈 값을 읽는다는 경고.
- LM Studio OpenAI Compatibility: `stream` 지원, reasoning 요청 필드 문서 없음.
- OpenRouter Streaming: `: OPENROUTER PROCESSING` 주석 줄, 헤더 이후 오류는 HTTP 200 안 최상위 `error` 청크로 오고 그 뒤 스트림이 끝난다. usage 청크가 빈 `delta`로 `finish_reason`을 되풀이해 마지막 `finish_reason`이 두 번 나타난다(회계 프레임).
- OpenRouter Reasoning Tokens: 스트림 생각은 `delta.reasoning_details` 배열, 요청 쪽 `reasoning`·`reasoning_content`는 별칭.
- 확인된 사실(02-client.md): keep-alive 주석 줄과 reasoning 필드가 오고, CRLF와 청크 경계에서 잘린 줄이 있다.

## 범위(In)

- SSE 줄 처리: 줄 끝 CRLF·LF·CR, `:`로 시작하는 주석 줄(`: keep-alive`, `: OPENROUTER PROCESSING`), 여러 `data` 줄 결합, 맨 앞 BOM, 조각 경계에서 잘린 줄과 멀티바이트 문자, `[DONE]`, 빈 줄 없이 끝난 마지막 이벤트(결정 9번).
- 청크 JSON 해석: content, 생각(`reasoning_content`와 `reasoning` 두 이름 모두), tool_calls를 index별로 누적(서로 다른 index의 델타가 번갈아 오는 경우 포함).
- 모르는 델타 필드(예: `reasoning_details`)는 원문 JSON으로 보존한다.
- 최상위 `error` 청크는 스트림 오류로 끝낸다.
- usage는 사용량으로 읽는다. usage 청크의 `finish_reason` 되풀이는 내용 중복을 만들지 않는다.
- 빈 줄 없이 끝난 마지막 이벤트를 내보낸다. 잘린 JSON이면 형식 오류로 끝난다.

## 비범위(Out)

- HTTP 요청과 응답 헤더 처리, 요청 시작·첫 토큰·마지막 토큰 시각 기록: [02-5](../05/plan.md).
- `<think>` 태그를 content에서 떼어 내는 일(서버 설정으로 해결하고 감시는 작업 05).
- OpenRouter `reasoning_details` 배열의 구조 해석(모르는 필드로 원문 보존만 한다. 필요해질 때 해석한다).
- 토큰마다 도착 시각 기록.
- OpenRouter `response.failed` 형식(Responses API라 이 계획의 비범위).

## 선행 세부 작업

[02-2 설정 읽기와 검증](../02/tasks.md) 완료. 이 세부 작업은 설정 값을 직접 쓰지 않지만 작업 순서상 앞선다(02-client.md의 선행 표).

## 리스크

- 서버마다 스트림 형식이 다르다. 모르는 델타 필드를 보존하고 생각은 두 이름을 모두 받는다.
- 바이트 조각 경계가 멀티바이트 문자나 CRLF 중간에 걸리면 결과가 달라질 수 있다. 한 바이트씩 나눠 넣는 테스트로 막는다.
- 빈 줄 없이 끝난 마지막 이벤트를 받아들이므로, 잘린 JSON은 형식 오류로 끝나야 한다. 이 경계를 테스트로 확인한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| 1 | SSE 줄 처리(줄 끝 3종·주석·data 결합·BOM·조각 경계 잘림·`[DONE]`·빈 줄 없는 마지막 이벤트) | HTTP 전송 코드, 시각 기록 | `llm/` 줄 처리 모듈과 단위 테스트 | 게이트 3종 |
| 2 | 조각 경계 불변성 시험(한 바이트씩 나눠 넣기 포함) | 제품 코드 추가 중복 | 경계 불변성 단위 테스트 | 게이트 3종 |
| 3 | 청크 JSON 델타 복원(content·생각 두 이름·tool_calls index별 누적·교차 델타) | 전송 계층 수정 | 델타 복원 모듈과 단위 테스트 | 게이트 3종 |
| 4 | 모르는 필드 원문 보존, `error` 청크 스트림 오류, 잘못된 JSON·잘린 마지막 이벤트 형식 오류 | 오류에 키·주소 담기 | 오류 타입과 단위 테스트 | 게이트 3종 |
| 5 | usage 읽기(있으면 값, 없으면 없음), `finish_reason` 되풀이 내용 중복 방지 | 토큰별 시각 기록 | usage 타입과 단위 테스트 | 게이트 3종 |
| 6 | `docs/RESEARCH.md` 02 절의 02-3 적용 위치 갱신, 상태 줄 갱신 | 계획 문서 재설계 | 문서 연쇄 반영 목록 | 게이트 3종 |
| 7 | 미커밋 작업 파일 전체 구조 점검 표 작성, 위반 잔여 이관 | 코드 변경·커밋 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |
| 8 | T7 위반 5건 이동·분리 보정, 타입 분리 2건 보정 | 동작·시그니처 변경, 커밋 | 평면화·책임 분리 파일, `split_error.rs`·`delta/chunk_types.rs` | 게이트 3종 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/plans/02-client/03/plan.md` | 상세 기획 | 완료 |
| `docs/plans/02-client/03/spec.md` | 명세·AC | 완료 |
| `docs/plans/02-client/03/tasks.md` | 작업 계획·DoD | 진행 |
| `docs/tasks/02/03/T1-sse-lines.md` | T1 위임 문서 | 완료 |
| `docs/tasks/02/03/T2-bounds.md` | T2 위임 문서 | 완료 |
| `docs/tasks/02/03/T3-delta.md` | T3 위임 문서 | 완료 |
| `docs/tasks/02/03/T4-errors.md` | T4 위임 문서 | 완료 |
| `docs/tasks/02/03/T5-usage.md` | T5 위임 문서 | 완료 |
| `docs/tasks/02/03/T6-docs.md` | T6 위임 문서 | 완료 |
| `docs/tasks/02/03/T7-structure-check.md` | T7 위임 문서 (미커밋 작업 파일 전수 구조 점검) | 완료 |
| `docs/tasks/02/03/T8-structure-fix.md` | T8 위임 문서 (T7 위반 5건 보정) | 완료 |
