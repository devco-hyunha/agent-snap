# 02-6 재시도 적용 리서치 상세

- 상태: 원문 확인 완료 (2026-11-27). 채택·미적용은 `docs/plans/02-client/06/plan.md` 근거 절과 `docs/RESEARCH.md` 02 절.
- 확인 방법: 리서치 서브가 각 링크를 직접 열어 실재와 원문 문구를 확인했다. 확인하지 못한 링크는 적지 않는다.
- 선행: 02-4 판단 리서치는 [02-client-retry.md](02-client-retry.md). 이 문서는 02-6 신규 쟁점만 담는다.

## 채택한 근거

### 1. OpenAI Rate limits guide — 스트림 시작 후 재생 금지

- 링크: `https://developers.openai.com/api/docs/guides/rate-limits`
- 원문:
  - `Retry-After` 취급: "Treat this value as a minimum: wait at least that long and add a small random delay".
  - 헤더가 없을 때: "If `Retry-After` is missing, increase the delay between retries and add a small random delay".
  - 비재시도 구분: "Don't retry quota, billing, or other errors that require you to take action".
  - 스트림 경계: "For streaming requests, these HTTP error responses apply before the stream starts. An error after streaming begins can arrive as a stream event; don't automatically replay a request after consuming output".
- 02-6 채택: 재시도 지점을 헤더 수신 전으로 한정한다. 스트림 본문을 소비한 뒤에는 재생하지 않는다. 헤더 우선 + 지터는 02-4가 이미 채택한 대로이고, 헤더를 최소로 취급하는 관례를 확인했다.
- 02-6 미적용: ramp-rate 안내(1M TPM 이상에서 50%씩 15분 단위 증속)는 단일 사용자 에이전트라 비범위.

### 2. reqwest-retry / reqwest-middleware — 재시도 지점과 스트리밍 본문 거부

- 링크: `https://docs.rs/reqwest-retry`, `https://docs.rs/reqwest-middleware`
- 원문:
  - 판정 지점: `default_on_request_success`는 "5XX, 408, 429"를 보며, "success here means that the request finished without interruption, not that it was logically OK".
  - 스트리밍 본문 요청 거부: "Request object is not cloneable. Are you passing a streaming body?".
- 02-6 채택: reqwest-retry를 그대로 도입하지 않는다. 근거로만 쓴다 — 재시도 판정은 헤더까지 온 응답만 본다. 02-5 결정 14번은 reqwest를 직접 구성하는 쪽이므로 미들웨어 체인을 새로 넣지 않는다.
- 02-6 미적용: reqwest-middleware·reqwest-retry 크레이트 도입. 02-5 결정 14번과 충돌하고, 스트리밍 요청을 미들웨어가 거부한다.

### 3. openai-python — 재시도 지점과 스트림 내 재시도 부재

- 링크: `https://github.com/openai/openai-python/blob/main/src/openai/_base_client.py`, `https://github.com/openai/openai-python/blob/main/src/openai/_streaming.py`
- 원문:
  - 재시도는 `raise_for_status()` 직후, 요청 본문을 되감기(`request_body_replay.rewind()`)할 수 있을 때만 수행한다.
  - `_streaming.py`는 스트림 안에서 재시도 없이 `APIError`를 던지고 `response.close()`만 한다.
- 02-6 채택: 재시도 판정 지점은 "헤더·본문 머리까지 완료된 응답". 스트림 본문을 소비한 뒤에는 재생하지 않는다.

### 4. async-openai — 재생 가능한 요청 팩토리 경계와 429 본문 구분

- 링크: `https://docs.rs/async-openai/latest/async_openai/` (크레이트 문서), `https://github.com/640006/async-openai`의 tower 레이어 `should_retry`
- 원문: `HttpRequestFactory`(재생 가능한 팩토리) 경계에서 429·5xx·connect 에러만 재시도한다. 429는 본문을 읽어 rate limit(재시도)와 insufficient quota(영구)를 구분한다.
- 02-6 채택: 429의 본문 구분 관례를 근거로 확인한다. 이 세부 작업에서 채택할지는 02-6 기획 질문에서 정한다.
- 02-6 미적용: tower 레이어 도입. 02-5는 reqwest를 직접 구성한다.

### 5. tokio — 백오프 대기 취소와 cancel-safe

- 링크: `https://docs.rs/tokio/latest/tokio/time/fn.sleep.html`, `https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html`, `https://docs.rs/tokio/latest/tokio/macro.select.html`
- 원문:
  - `tokio::time::sleep`: "Canceling a sleep instance is done by dropping the returned future".
  - `CancellationToken::cancelled()`: "This method is cancel safe".
  - `select!`은 기본이 랜덤 공정이고, `biased;`로 순서를 고정한다. "place the shutdown future earlier in the select! list".
  - `run_until_cancelled`는 "only cancel safe if fut is cancel safe".
- 02-6 채택: 백오프 대기는 `select! { biased; ... }`로 취소와 경합한다. 02-5가 이미 쓰는 `CancellationToken`과 같은 재료다.
- 02-6 미적용: openai-python의 동기 `time.sleep` 방식(취소 불가).

## 참고로만 둔 근거 (채택 아님)

- openai-rs (crates.io 0.1.1, 2022 정적용): 재시도 기능이 없어 조사 대상에서 기각.

## 02-4의 "다시 볼 점" 계승 항목

- 402 `in_flight_budget_exhausted` 재시도: 02-4는 402를 무조건 비재시도로 뒀고, 02-6에서 공급자 오류 코드를 구분할 수 있으면 재검토하기로 했다. 근거: [02-client-retry.md](02-client-retry.md) 8항.
- 408 처리: 02-4는 408을 즉시 포기에 뒀고, 02-6에서 전송 계층의 요청 타임아웃 재시도를 다룰 때 재검토하기로 했다.
