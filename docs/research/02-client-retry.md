# 02-4 재시도 판단 리서치 상세

- 상태: 원문 확인 완료 (2026-11-25). 채택·참고 반영은 `docs/plans/02-client/04/plan.md` 근거 절과 `docs/RESEARCH.md` 02 절.
- 확인 방법: 리서치 서브가 각 링크를 직접 열어 실재와 원문 문구를 확인했다. 확인하지 못한 링크는 적지 않는다.
- 원문·번역 사본: `.local/research/02-client-retry/`(git 제외).

## 채택한 근거

### 1. AWS Architecture Blog — Exponential Backoff and Jitter

- 링크: `https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/` (Marc Brooker, 2015-03-04)
- 원문 공식:
  - Full Jitter: `sleep = random(0, min(cap, base × 2^attempt))`
  - Equal Jitter: `d/2 + random(0, d/2)`
  - Decorrelated: `min(cap, random(base, sleep_prev × 3))`
- 저자 측정 결론:
  - 지터 없는 백오프는 "clear loser"(완료 시간이 크게 증가).
  - 지터 세 종류 중 Equal Jitter가 패자.
  - Full과 Decorrelated의 우열은 "less clear". Full가 호출 수는 적고, 완료 시간은 약간 더 김.
  - 100 클라이언트에서 호출 수가 절반 이상 감소.
- 02-4 채택: Full Jitter. 저자가 Decorrelated의 우월을 결론내리지 못했고, 구현이 더 단순하며, 난수 주입만으로 확인 가능하다(결정 10번). Equal Jitter는 저자 측정에서 패자라 쓰지 않는다.

### 2. Google SRE Book — Addressing Cascading Failures

- 링크: `https://sre.google/sre-book/addressing-cascading-failures/`
- 원문 규칙:
  - "Always use randomized exponential backoff".
  - "Limit retries per request".
  - 여러 층에서 재시도하면 증폭된다. 예시: 층마다 3번씩 3층이면 요청 4^3 = 64번.
  - 서버 전체 재시도 예산 예시: "only allow 60 retries per minute in a process".
  - 재시도 가능·불가 오류를 분리한다.
- 02-4 채택: 클라이언트 한 층에서만 재시도(결정 10번), 시도 횟수 상한, 재시도 대상·비대상 분리. 프로세스 단위 예산은 단일 사용자 에이전트라 비범위.

### 3. Retry Amplification in Distributed Systems (arXiv)

- 링크: `https://arxiv.org/abs/2608.25403`
- 원문 실측: 오픈소스 200개 프로젝트 중 프로덕션 설정 113개를 분석. 지연 무작위화(지터)를 쓴 설정은 1건뿐. 지터 없는 naive 재시도는 성공률이 55.4%에서 41.5%로 떨어져 무재시도보다 나빴다. 재시도 증폭 계수(RAF) 1.18~1.34.
- 02-4 채택: 지터 필수·횟수 상한 근거를 실측으로 보강. naive 재시도가 무재시도보다 나쁘다는 결과가 이 세부 작업의 Full Jitter·상한 설계를 뒷받침한다.

### 4. Amazon Builders' Library — Timouts, retries, and backoff with jitter

- 링크: `https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/`
- 02-4 채택: 백오프 상한과 지터의 조합, 재시도 대상 분류의 실무 관례.

### 5. AWS SDK retry behavior / Google Cloud retry 전략 / Google C++ 재시도 정책

- 링크: `https://docs.aws.amazon.com/sdkref/latest/guide/feature-retry-behavior.html`, `https://docs.cloud.google.com/storage/docs/retry-strategy`, `https://docs.cloud.google.com/cpp/docs/retry-policies`
- 원문:
  - AWS SDK: transient base 50ms, throttling base 1,000ms, Full Jitter, 재시도 토큰 버킷(quota).
  - Google Cloud storage: 408·429·5xx·소켓 타임아웃만 재시도, 멱등성 확인.
  - Google C++ 기본: 초기 1초, 최대 5분, 배율 2.
- 02-4 채택: 백오프 기본값 1초·상한 30초·배율 2의 관례 근거.

### 6. RFC 9110 10.2.3 Retry-After

- 링크: `https://www.rfc-editor.org/rfc/rfc9110.html#name-retry-after`
- 원문 ABNF: `Retry-After = HTTP-date / delay-seconds`, `delay-seconds = 1*DIGIT`. "A delay-seconds value is a non-negative decimal integer, representing time in seconds."
- 503은 `Retry-After`를 낼 수 있다고 명시. 3xx는 최소 대기 시간. 429의 `Retry-After`는 RFC 6585 쪽 규정.
- 02-4 반영: 두 형식 해석, 음이 아닌 정수만 유효. "5xx에도 헤더를 따른다"는 503과 429를 묶은 일반화임을 밝혀 둔다(plan.md 근거 절에 각주).

### 7. RFC 6585 4절 429 Too Many Requests

- 링크: `https://www.rfc-editor.org/rfc/rfc6585.html`
- 원문: 429는 "too many requests in a given amount of time (rate limiting)". `Retry-After`를 MAY로 포함. 429 응답은 캐시 저장 금지. 7.2절: 공격 수준의 요청 폭주에는 429를 매번 쓰는 대신 연결을 끊는 편이 나을 수 있음.
- 02-4 반영: 429는 재시도 후보, `Retry-After` 우선.

### 8. OpenRouter API Credit & Rate Limits

- 링크: `https://openrouter.ai/docs/api-reference/limits`
- 원문:
  - `:free` 한도: 크레딧 10 미만은 분당 20·일당 50, 10 이상은 분당 20·일당 1000.
  - 429 권고: "Retry with exponential backoff ... Honor the `Retry-After` header when present."
  - `Retry-After`는 모든 시도 공급자가 재시도 힌트를 준 경우에만 붙는다. free-model·key-management 429에는 `X-RateLimit-Limit/Remaining/Reset`이 붙고, per-IP burst에는 이 헤더가 없다.
  - 402 `in_flight_budget_exhausted`(limit_source `openrouter_in_flight_budget`): transient이고 `Retry-After`를 주므로 대기 후 재시도.
  - 402 `weight_exceeds_budget`(limit_source `openrouter_credits`): "retrying will not help".
  - 403 예산 오류에는 `Retry-After`가 없다.
  - mid-stream 429는 HTTP 200 SSE `finish_reason: "error"`로 온다.
- 02-4 반영: 헤더 우선, 없으면 지수 백오프. `X-RateLimit-Reset`은 표준이 아니라 쓰지 않는다. mid-stream 429는 비범위(02-3·02-6).
- 402 처리 결정: 이 세부 작업은 402를 무조건 비재시도로 둔다(결정 12번). 원문은 in-flight 402의 재시도를 권고하지만, 크레딧 402과 구분하는 입력 타입을 이 층에서 정의하지 않고, 과금 중복 위험을 막는 쪽이 안전하다. 다시 볼 점: 02-6 적용 시 공급자 오류 코드를 구분할 수 있으면 in-flight 402 재시도를 다시 검토한다.

### 9. llama.cpp server README

- 링크: `https://raw.githubusercontent.com/ggml-org/llama.cpp/master/tools/server/README.md`
- 원문: `/v1/health`가 모델 로딩 중 503과 `{"error": {"code": 503, "message": "Loading model"}}`를 돌려준다. `/slots`는 `?fail_on_no_slot=1`일 때 503.
- 02-4 반영: 503 재시도 대상 근거. 단 이 README가 503을 명시한 자리는 health·slots 엔드포인트이고, chat completions의 503 자체는 이 README에 명시되지 않는다.

## 참고로만 둔 근거 (채택 아님)

- Arolla: Treating Retry Capacity as a Shared Resource (NSDI) `https://anakli.inf.ethz.ch/papers/arolla_nsdi27.pdf` — 재시도 용량을 공유 자원으로 프록시에서 토큰 버킷으로 수용·기각. 단일 사용자 에이전트라 비범위.
- RetryGuard (arXiv 2511.23278) `https://arxiv.org/html/2511.23278` — 서비스별 재시도 정책 관리가 지터·백오프·예산보다 낫다는 보고. 자동 스케일러·다중 계층 대상이라 참고만.
- Scaling Exponential Backoff (Bender, Fineman, Gilbert, Young, JACM 66(1) 2018) `https://dl.acm.org/doi/10.1145/3276769` — 지터 지수 백오프의 공유 자원 경합 이론. 참고만.
- The Throughput and Access Delay of Slotted-Aloha With Exponential Backoff (Barletta, IEEE/ACM ToN 26(1) 2018) `https://dl.acm.org/doi/10.1109/TNET.2017.2782696` — 이진 백오프 처리량 0.3706, 최적화 base 0.4303, 지수 상수 b=1.35가 b=2보다 매력적. 배율 2를 고정한 근거로만 기록.
- Method overloading the circuit (ICSE 2023) `https://doi.org/10.1145/3542929.3563466` — 클라이언트·서버 측 서킷 브레이커 분류와 오남용 경고. 02-4는 판단만 하는 층이라 상태 머신 비범위.
- Dispatch Control for Priority-Based, Multi-Tenant Workflows (arXiv 2611.02926) `https://arxiv.org/abs/2611.02926` — 멀티 테니트 수용 게이트. 비범위.

## 408 처리 근거

- Google Cloud 재시도 전략은 408을 재시도 대상으로 본다. 이 세부 작업은 408을 즉시 포기에 둔다(결정 12번). 근거: 408은 서버가 요청을 받기까지 기다리다 시간 초과했다는 뜻이고, snap에서 이 상황에 가까운 실패는 02-5의 헤더 대기 시간 초과로 나간다. 헤더 대기 초과는 재시도 비대상 입력으로 정의한다(spec 매핑 표). 02-6에서 전송 계층의 요청 타임아웃 재시도를 다룰 때 다시 검토한다.
