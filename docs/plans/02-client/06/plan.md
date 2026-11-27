# 02-6 재시도 적용: 상세 기획

- 상태: 완료 (T1~T7 구현·리뷰 PASS. 기획 근거로 쓴 조사는 [docs/research/02-client-retry-apply.md](../../../research/02-client-retry-apply.md))
- 상위 문서: [02-client.md](../../02-client.md)
- 선행 결정: 10번(한 층 재시도·Full Jitter), 12번(비대상·매핑·판정 우선순위), 14번(전송 크레이트), 15번(`reasoning` 변환)

## 목적

02-5가 연 스트림에 재시도를 적용한다. 02-4가 만든 판단(재시도 여부·대기 시간)을 그대로 따라, 헤더를 받기 전 실패에서만 다시 요청한다. 스트림이 시작된 뒤 실패는 재시도하지 않고 그대로 돌려준다. 에이전트 루프(작업 03)는 02-5와 같은 `LlmClient` 트레이트만 보고, 재시도가 안 쪽에 있는지 알지 못한다.

## 리서치 및 설계 근거

- 상세(신규): [docs/research/02-client-retry-apply.md](../../../research/02-client-retry-apply.md) — 2026-11-27 조사 서브가 5종 링크를 직접 열어 원문 문구를 확인했다.
- OpenAI Rate limits guide: "For streaming requests, these HTTP error responses apply before the stream starts. An error after streaming begins can arrive as a stream event; don't automatically replay a request after consuming output". → 재시도 지점을 헤더 수신 전으로 한정하는 1차 규칙.
- reqwest-retry·reqwest-middleware: 재시도 판정은 "request finished without interruption"까지 온 응답만 보고, 스트리밍 본문 요청은 미들웨어가 거부한다. → 크레이트는 도입하지 않고 경계 근거로만 쓴다(결정 14번과 충돌).
- openai-python·async-openai: 재시도는 헤더·본문 머리까지 완료된 응답과 재생 가능한 요청 경계에서만 한다. 스트림 안에서는 재시도 없이 오류만 던진다.
- tokio: `tokio::time::sleep`은 future를 드롭해 취소하고, `CancellationToken::cancelled()`는 cancel-safe다. `select!`의 `biased;`로 취소 순서를 앞세운다.
- 02-4 리서치의 "다시 볼 점" 2건을 이번 기획에서 판정했다. 402 in-flight 재시도는 비재시도 유지, 408은 헤더 대기 초과로 나가므로 비재시도 유지. 근거: [docs/research/02-client-retry.md](../../../research/02-client-retry.md) 8항·408 절.
- 백업 확인: `.local/research/02-client-retry/`(02-4 논문 8편 원문·번역)를 열어 02-6 신규 쟁점(스트림 경계·재시도 지점·백오프 취소)과 겹치는 항목이 없는지 확인했다. 개인·에이전트 저장소는 비어 있어 확인할 백업이 없었다.

## 범위(In)

- 재시도 래퍼: `LlmClient` 트레이트를 구현하는 래퍼 타입을 `llm/client/`에 둔다. 안쪽 클라이언트를 감싸고, 같은 트레이트로 스트림을 연다.
- 재시도 지점 규칙: `open` 단계(헤더 수신 전) 실패에서만 02-4 판단을 따라 다시 연다. 스트림이 시작된 뒤 실패는 판단에 넣지 않고 그대로 돌려준다.
- 전송 오류 → 02-4 입력 매핑: 02-4 spec의 매핑 표를 코드 한 곳에서 구현한다.
- `Retry-After` 전달: 02-5의 상태 오류가 서버 헤더 값을 02-4 판단에 넘길 수 있게 담는다.
- 백오프 대기: 02-4가 준 대기 시간만큼 기다리되, 같은 취소 토큰과 `select!`(biased)로 경합한다. 취소되면 대기하지 않고 취소 오류로 끝난다.
- 시도 횟수 셈: 첫 요청을 1로 센다.
- 상용 난수원: 02-4가 요구하는 `RandomSource` 구현을 제품 코드에 둔다(새 난수 크레이트 의존 없이).
- 설정 변환: 02-2의 `http` 값(f64)을 02-4의 `RetrySettings`(Duration·정수)로 바꾼다.

## 비범위(Out)

- 재시도 판단 규칙 자체(화이트/블랙리스트·Full Jitter·`Retry-After` 해석): 02-4에 이미 있다. 이 세부 작업은 판단을 고치지 않고 부르기만 한다.
- 스트림 시작 후 재시도(스트림 유휴 초과·오류 청크·취소 재생): 리서치 근거로 두지 않는다.
- 402 in-flight 재시도: 02-4 결정 12번대로 비재시도 유지. 공급자 오류 코드 구분 재료를 만들지 않는다.
- 재시도 총 시간 상한·프로세스 단위 재시도 예산: 두지 않는다(단일 사용자 에이전트).
- 선택 키로 클라이언트 만들기·모델 목록 조회: [02-7](../07/plan.md).
- 재시도 이벤트 기록·지표 계산.

## 선행 세부 작업

[02-5 클라이언트 트레이트와 스트리밍 전송](../05/plan.md) 완료(PR #9 병합 2026-11-27), [02-4 재시도 판단](../04/plan.md) 완료(PR #7 병합 2026-11-25). 02-2의 `http` 설정 값을 쓴다.

## 리스크

- 02-5의 상태 오류가 `Retry-After` 헤더를 담지 않는다. 헤더 값을 담도록 확장해야 판단이 동작한다. 확장 시 오류 Display·Debug에 헤더 원문만 들고 키·base_url 호스트는 담지 않는다.
- 재시도 래퍼가 스트림을 여러 번 열면서 모의 서버 시험이 길어진다. 시험용 시간 값은 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 확인한다.
- 상용 난수원을 제품 코드에 두면 시험과 다른 값이 나온다. 시험은 값이 고정된 난수원을 주입하고, 상용 구현은 균등 분포 시험으로만 확인한다.
- POST chat completions는 멱등 키가 없어 재시도 시 과금 중복이 생길 수 있다. 02-4 결정 12번이 이미 비대상 범위를 좁혀 둔 이유를 그대로 따른다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 | 02-5의 상태 오류에 `Retry-After` 헤더 값을 담는 확장, 결정 기록 | 판단 로직 변경, 키·base_url 담기 | `TransportError::Status` 확장, `docs/DECISIONS.md` 갱신 | 게이트 3종 |
| T2 | 전송 오류 → 02-4 입력 매핑 함수(02-4 spec 매핑 표대로) | 판단 규칙 재정의, 매핑 표 문서 두곳화 | 매핑 함수와 매핑 시험 | 게이트 3종 |
| T3 | 재시도 래퍼 골격(`LlmClient` 구현, 시도 횟수 셈, open 단계 재시도) | 스트림 시작 후 재생, reqwest 타입 노출 | 래퍼 타입과 재시도 시험 | 게이트 3종 |
| T4 | 백오프 대기와 취소 경합(`select!` biased, 취소 시 대기 없이 종료) | sleep만 쓰는 취소 불가 대기 | 대기 코드와 취소 시험 | 게이트 3종 |
| T5 | 상용 난수원 구현(균등 분포), 설정 → `RetrySettings` 변환 | 새 난수 크레이트 의존, 설정 타입 의존 | 난수원·변환 함수와 시험 | 게이트 3종 |
| T6 | 미커밋 작업 파일 전수 구조 점검 표 작성, 위반 잔여 이관 | 코드 변경, 파일 삭제, 커밋·푸시 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |
| T7 | T6 잔여 해소 | 동작 변경, 파일 삭제, 커밋·푸시 | 분리·이동 파일, 잔여 해소 기록 | 게이트 3종 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/plans/02-client/06/plan.md` | 상세 기획 | 완료 |
| `docs/plans/02-client/06/spec.md` | 명세·AC | 완료 |
| `docs/plans/02-client/06/tasks.md` | 작업 계획·DoD | 완료 |
| `docs/research/02-client-retry-apply.md` | 02-6 조사 상세 | 완료 |
| `docs/tasks/02/06/T1-retry-after-header.md` | T1 위임 문서 | 완료 |
| `docs/tasks/02/06/T2-failure-mapping.md` | T2 위임 문서 | 완료 |
| `docs/tasks/02/06/T3-retry-wrapper.md` | T3 위임 문서 | 완료 |
| `docs/tasks/02/06/T3-fix-stream-reset.md` | T3 r1 critical 보정 문서 | 완료 |
| `docs/tasks/02/06/T4-backoff-cancel.md` | T4 위임 문서 | 완료 |
| `docs/tasks/02/06/T4-fix-docs-const.md` | T4 r1 minor 2건 보정 문서 | 완료 |
| `docs/tasks/02/06/T5-random-settings.md` | T5 위임 문서 | 완료 |
| `docs/tasks/02/06/T6-structure-check.md` | T6 위임 문서 | 완료 |
| `docs/tasks/02/06/T7-structure-fix.md` | T7 위임 문서 (r14: R1~R4 해소·PASS, r15: R5~R11 해소·PASS) | 완료 |

## 문서 영향

- `docs/research/02-client-retry-apply.md`: 신규(작성 완료). `docs/RESEARCH.md` 02 절에 3행 반영 완료.
- `docs/plans/02-client/04/spec.md`의 매핑 표: 정본은 그 표이고, 코드는 T2가 구현했다(두번 정의하지 않는다). 헤더 이후 본문 읽기 실패 행을 T3 보정에서 추가했다.
- `docs/plans/02-client/05/spec.md`의 오류 표: 헤더 이후 읽기 실패 행을 T3 보정에서 추가했다.
- `docs/DECISIONS.md`: 16번(`Retry-After` 헤더를 상태 오류에 담는 방법)은 T1에서, 17번(상용 난수원)은 T5에서 기록 완료.
- `docs/plans/02-client.md`·`docs/PLAN.md`·`docs/plans/02-client/06/*` 상태 줄 갱신 완료.
- 리뷰 기록: `docs/reviews/02/`에 T1~T7 개별 리뷰와 작업 단위 전체 리뷰(`02-6-unit-r1.md`) PASS 기록.

## DoD

[spec.md](spec.md)의 AC를 모두 충족하고, 게이트 3종(Windows·WSL)을 통과하고, 조사 적용 위치가 갱신되고, 작업 단위 전체 리뷰가 PASS한 상태.
