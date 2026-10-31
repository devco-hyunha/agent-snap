# 02-4 재시도 판단: 상세 기획

- 상태: planned
- 상위 문서: [02-client.md](../../02-client.md)

## 목적
네트워크 없이 응답 상태, 헤더, 시도 횟수로 다시 시도할지와 대기 시간을 정한다. 판단을 전송과 나눠 두면 시간 의존 없이 단위 테스트로 확인할 수 있고, 적용([02-6](../02-6-retry-apply/plan.md))은 이 판단을 부르기만 한다.

## 범위(In)
- 재시도 대상: 429와 500/502/503/504, 연결 오류만.
- `Retry-After`: 초와 HTTP 날짜 두 형식을 읽는다. 해석할 수 없거나 음수이면 헤더가 없는 것으로 본다. 0과 과거 날짜는 대기 0초다(D14).
- 상한을 넘는 `Retry-After`는 기다리지 않고 포기한다(대기 초를 밝힌다).
- 헤더가 없으면 지수 백오프에 Full Jitter를 쓴다. 난수는 주입받는다.
- 최대 횟수를 넘으면 포기한다.
- 그 밖의 4xx는 바로 포기한다.

## 비범위(Out)
- 실제 대기와 재요청: [02-6](../02-6-retry-apply/plan.md).
- 시간·재시도 값의 설정 읽기: [02-2](../02-2-config/plan.md). 이 세부 작업은 값을 받아 판단만 한다.
- `X-RateLimit-Reset` 헤더(표준이 아니라 쓰지 않는다).
- 프로세스 단위 재시도 예산(단일 사용자 에이전트라 지금 필요하지 않다).
- Decorrelated Jitter(Full보다 낫다는 결론이 없어 더 단순한 Full을 고른다).
- 에이전트 루프 쪽 재시도(D13).

## 선행 세부 작업
[02-3 SSE 파싱과 델타 복원](../02-3-sse/plan.md). 작업 순서상 앞선다.

## 참고 자료
- RESEARCH.md 작업 02 절
  - RFC 9110 HTTP Semantics, 10.2.3 Retry-After: 초와 HTTP 날짜 두 형식, 5xx에도 헤더를 따른다.
  - RFC 6585, 4절 429: 429 의미. 대기 시간이 상한을 넘으면 기다리지 않고 대기 초를 밝힌 오류로 알린다.
  - llama.cpp server README: 모델 로딩 중 503.
  - OpenRouter API Rate Limits: 헤더 우선, 없으면 지수 백오프, 최대 횟수. 일일 한도처럼 긴 대기는 상한 초과로 즉시 포기한다.
  - AWS Architecture Blog, Exponential Backoff And Jitter: Full Jitter.
  - Google SRE Book, Addressing Cascading Failures: 무작위 지수 백오프, 횟수 제한, 재시도 가능한 오류만, 한 층에서만.
- DECISIONS D13: 재시도는 클라이언트 한 층에서, 백오프는 Full Jitter.

## 리스크
- OpenRouter 무료 모델의 일일 한도에 걸리면 `Retry-After`가 매우 길다. 상한을 넘으면 바로 포기하고 대기 초를 알린다.
- HTTP 날짜 형식 해석이 시계에 의존한다. 현재 시각도 주입받아야 시간 의존 없이 시험할 수 있다(spec.md 입력 참조).
- 경계는 D14에서 정했다. `Retry-After` 0은 바로 재요청, 과거 날짜는 0초, 음수와 해석 불가는 헤더 없음으로 본다. 재시도 대상 연결 오류는 헤더 전 실패로 한정하고 헤더 대기 시간 초과는 제외한다. 시도 횟수는 첫 요청이 1이다.
