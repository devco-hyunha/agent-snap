# 02-4 재시도 판단: 상세 기획

- 상태: 완료 (초안 2026-11-25. T1~T5 완료, 02-4 작업 단위 전체 리뷰 PASS 2026-11-25. 결정 10·12·13번 기록 완료. 리뷰 기록 `docs/reviews/02/02-4-unit-r1.md`. PR #7로 `feature/client` 병합)
- 상위 문서: [02-client.md](../../02-client.md)
- 백업 정본 이식: `C:\Workspace\agent-snap-backup\docs\plans\02-client\02-04-retry\`(done)의 판정 규칙·AC를 그대로 따르고, 경로·결정 번호·연결 문서를 현재 저장소 규칙에 맞췄다.

## 목적

네트워크 없이 응답 상태, 헤더, 시도 횟수로 다시 시도할지와 대기 시간을 정한다. 판단을 전송과 나눠 두면 시간 의존 없이 단위 테스트로 확인할 수 있고, 적용([02-6](../06/plan.md), 예정)은 이 판단을 부르기만 한다.

## 리서치 및 설계 근거

- 상세(신규): [docs/research/02-client-retry.md](../../../research/02-client-retry.md) — 2026-11-25 리서치 서브가 9종 링크를 직접 열어 원문 문구를 확인했다. 원문·번역 사본은 `.local/research/02-client-retry/`.
- RFC 9110 HTTP Semantics, 10.2.3 Retry-After: `Retry-After = HTTP-date / delay-seconds`, delay-seconds는 음이 아닌 10진 정수. 헤더 명시 대상은 503과 3xx이고 429의 헤더는 RFC 6585가 규정한다. 이 계획의 "5xx에도 헤더를 따른다"는 503과 429를 묶은 일반화다.
- RFC 6585, 4절 429: 429는 요청이 너무 많다는 뜻이고 `Retry-After`로 대기 시간을 줄 수 있다. 429 응답은 캐시 저장 금지. 7.2절은 요청 폭주에 429를 매번 쓰기보다 연결을 끊는 편이 나을 수 있다고 본다.
- OpenRouter API Credit & Rate Limits: `:free` 모델은 크레딧 10 미만 분당 20·일당 50, 10 이상 분당 20·일당 1000 한도가 있고 초과 시 429와 `Retry-After`를 준다. 헤더 우선, 없으면 지수 백오프, 최대 횟수. `Retry-After`는 모든 시도 공급자가 재시도 힌트를 준 경우에만 붙고, free-model·key-management 429에는 `X-RateLimit-*`가 붙는다. 원문의 402 `in_flight_budget_exhausted`는 transient로 `Retry-After`를 주며 재시도를 권고하고, 402 `weight_exceeds_budget`은 "retrying will not help"다. 이 세부 작업은 402를 입력 구분 없이 무조건 비재시도로 둔다(결정 12번).
- AWS Architecture Blog, Exponential Backoff And Jitter (Marc Brooker 2015-03-04): Full Jitter는 `sleep = random(0, min(cap, base*2^attempt))`. 저자 측정에서 지터 없는 백오프는 패자, Equal Jitter는 지터 중 패자, Full과 Decorrelated의 우열은 "less clear"(Full는 호출 수가 적고 완료 시간은 약간 더 김). 더 단순한 Full을 고른다.
- Google SRE Book, Addressing Cascading Failures: 무작위 지수 백오프, 요청당 재시도 제한, 재시도 가능한 오류만, 한 층에서만(층마다 3번씩 3층이면 4^3=64번 증폭). 프로세스 단위 재시도 예산 예시 60번/분.
- Retry Amplification in Distributed Systems (arXiv 2608.25403): 프로덕션 설정 113개 중 지연 무작위화는 1건뿐, 지터 없는 naive 재시도는 성공률 55.4%에서 41.5%로 떨어져 무재시도보다 나쁨. 재시도 증폭 계수 1.18~1.34. 지터·횟수 상한 근거.
- Amazon Builders' Library Timouts, retries, and backoff with jitter, AWS SDK retry behavior, Google Cloud retry 전략, Google C++ 재시도 정책: 실무 관례. AWS SDK는 transient base 50ms·throttling base 1,000ms에 Full Jitter, Google C++는 초기 1초·최대 5분·배율 2. Google Cloud는 408·429·5xx·소켓 타임아웃을 재시도 대상으로 본다. 이 계획의 백오프 기본값 1초·상한 30초·배율 2 근거.
- llama.cpp server README: `/v1/health`가 모델 로딩 중 503과 `Loading model`을 돌려준다. 503 재시도 대상 근거. README가 503을 명시한 자리는 health·slots 엔드포인트이고 chat completions의 503은 그 README에 명시되지 않는다.
- 참고로만 두고 채택하지 않은 근거: Arolla(NSDI), RetryGuard(arXiv 2511.23278), Scaling Exponential Backoff(JACM 66(1) 2018), Slotted-Aloha Exponential Backoff(ToN 26(1) 2018, 지수 상수 b=1.35가 b=2보다 낫다는 수치), Method overloading the circuit(ICSE 2023), Dispatch Control(arXiv 2611.02926). 프록시·멀티테넌트·서킷 브레이커·공유 자원 이론은 이 세부 작업의 비범위다. 상세는 [docs/research/02-client-retry.md](../../../research/02-client-retry.md)의 참고 근거 절.
- 백업 결정 계승(현재 번호로 재부여): 한 층 재시도·Full Jitter(10번), 재시도 비대상·입력 매핑·판정 우선순위(12번), HTTP-date 파서 분리(13번). 백업 D14의 세부 경계값은 spec에 그대로 담고 결정으로 따로 남기지 않는다.

## 범위(In)

- 재시도 대상: 429와 500/502/503/504, 헤더 전 연결 오류만(결정 12번).
- `Retry-After`: 초와 HTTP 날짜 두 형식을 읽는다. 해석할 수 없거나 음수이면 헤더가 없는 것으로 본다. 0과 과거 날짜는 대기 0초다.
- 상한을 넘는 `Retry-After`는 기다리지 않고 포기한다(대기 초를 밝힌다).
- 헤더가 없으면 지수 백오프에 Full Jitter를 쓴다. 난수는 주입받는다.
- 최대 횟수를 넘으면 포기한다. 횟수 초과와 `Retry-After` 상한 초과가 동시에 해당하면 횟수 초과를 포기 이유로 쓴다(결정 12번).
- 그 밖의 4xx(429 제외)·목록 밖 5xx·402·408·425·헤더 대기 초과는 바로 포기한다.
- 재시도 입력 타입을 이 세부 작업에서 정의한다(헤더 전 연결 오류 종류, 상태 코드, 헤더 대기 초과). 02-5 착수 시 전송 오류와 매핑만 확인한다.

## 비범위(Out)

- 실제 대기와 재요청: [02-6](../06/plan.md).
- 시간·재시도 값의 설정 읽기: [02-2](../02/plan.md)에서 끝났다. 이 세부 작업은 값을 받아 판단만 한다.
- `X-RateLimit-Reset` 헤더(표준이 아니라 쓰지 않는다).
- OpenRouter 402(크레딧·in-flight 등): 재시도하지 않는다(결정 12번).
- 408·425, 화이트리스트 밖 5xx(500/502/503/504 외).
- 프로세스 단위 재시도 예산(단일 사용자 에이전트라 지금 필요하지 않다).
- Decorrelated Jitter.
- 에이전트 루프 쪽 재시도(결정 10번).
- mid-stream 429(HTTP 200 SSE `error` 청크): 02-3이 스트림 오류로 끝내고, 02-6 경계다.
- 공급자 멱등 키·POST 과금 중복 방지 장치(02-6 적용 시 일시 오류만 재시도).

## 선행 세부 작업

[02-3 SSE 파싱과 델타 복원](../03/plan.md) 완료(PR #6 병합 2026-11-25). 이 세부 작업은 설정 값을 직접 쓰지 않고, 02-2가 검증한 `http` 기본값을 입력으로 받는다.

## 리스크

- OpenRouter 무료 모델의 일일 한도에 걸리면 `Retry-After`가 매우 길다. 상한을 넘으면 바로 포기하고 대기 초를 알린다.
- HTTP 날짜 형식 해석이 시계에 의존한다. 현재 시각을 주입받아 시간 의존 없이 시험한다.
- 02-5가 아직 없어 입력 타입을 이 세부 작업에서 정의한다. 02-5 착수 시 매핑 표를 spec에 고정해 두번 정하는 일을 막는다.
- POST chat completions는 공급자 멱등 키 없이 일시 오류만 재시도하므로 과금 중복 잔여 리스크가 있다(결정 12번의 다시 볼 점).

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| 1 | 재시도 대상 판별: 상태 코드 화이트·블랙리스트, 헤더 전 연결 오류 후보, 헤더 대기 초과 비재시도. 입력 타입 정의 | 실제 대기·재요청 코드 | `llm/retry/` 입력 타입·판별 모듈과 단위 테스트 | 게이트 3종 |
| 2 | `Retry-After` 해석·상한: 초·HTTP 날짜, 0·과거→0초, 음수·해석불가→헤더 없음, 상한 이하 대기·초과 포기. 현재 시각 주입 | 설정 읽기 재구현 | `llm/retry/` 헤더 해석 모듈과 단위 테스트 | 게이트 3종 |
| 3 | Full Jitter 백오프·횟수: 난수 주입, 시도 횟수(첫 요청=1), 횟수 초과를 상한 초과보다 먼저 | 실제 대기 | 백오프 모듈과 단위 테스트 | 게이트 3종 |
| 4 | 공개 API·모듈 배치, 포기 이유 필드 제한(키·주소·본문 안 담기), 문서 연쇄(`docs/RESEARCH.md` 02 절 5종 추가, `docs/DECISIONS.md` 10·12·13번, 상태 줄) | 계획 문서 재설계 | `decide_retry` 공개 API, `docs/research/02-client-retry.md`, 문서 연쇄 반영 목록 | 게이트 3종 |
| 5 | 미커밋 작업 파일 전체 구조 점검 표 작성, 위반 잔여 이관 | 코드 변경·커밋 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/plans/02-client/04/plan.md` | 상세 기획 | 완료 |
| `docs/plans/02-client/04/spec.md` | 명세·AC | 완료 |
| `docs/plans/02-client/04/tasks.md` | 작업 계획·DoD | 완료 |
| `docs/tasks/02/04/T1-classify.md` | T1 위임 문서 | 완료 |
| `docs/tasks/02/04/T2-retry-after.md` | T2 위임 문서 | 완료 |
| `docs/tasks/02/04/T3-backoff.md` | T3 위임 문서 | 완료 |
| `docs/tasks/02/04/T4-api-docs.md` | T4 위임 문서 | 완료 |
| `docs/tasks/02/04/T5-structure-check.md` | T5 위임 문서 (미커밋 작업 파일 전수 구조 점검) | 완료 |

## 문서 영향

- `docs/RESEARCH.md` 02 절: RFC 9110, RFC 6585, OpenRouter API Rate Limits(402 미적용), AWS Jitter, Google SRE 항목 추가(적용 위치는 T4 이후 코드 경로로 갱신).
- `docs/research/02-client-retry.md`: 신규 상세.
- `docs/DECISIONS.md`: 10번(재시도는 클라이언트 한 층·Full Jitter), 12번(재시도 비대상·입력 매핑·판정 우선순위), 13번(HTTP-date 파서 분리) 추가. 11번은 T4 구현 중에도 새 결정이 생기지 않아 비운다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/04/*` 상태 줄 갱신.
