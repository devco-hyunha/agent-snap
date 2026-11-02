# D20 설정 스키마(중첩 모델·이름·http·선택 키) (정본)

- 상태: recorded
- 요약 인덱스: [docs/DECISIONS.md](../DECISIONS.md#d20-설정-스키마중첩-모델이름http선택-키)
- 이 문서가 해당 결정의 상세 정본. DECISIONS.md에는 요약과 링크만.
- 변경: 사용자 승인 후 이 파일과 DECISIONS 요약을 함께 고친다. 서브에이전트 임의 재해석 금지.

## 결정

- 결정: `endpoints`는 `name` 필드를 가진 배열이고, 모델은 각 엔드포인트 아래 `models` 배열(`endpoints[].models`)에 둔다. 최상위 `models`와 모델의 `endpoint` 참조는 폐기한다. 객체 맵이 아니라 배열을 쓴다. `endpoints`/`models` 빈 배열·생략은 허용하고 생략은 `[]`로 정규화한다(“고를 모델 없음”은 02-7·CLI). 엔드포인트·모델 `name`과 모델 `id`는 trim 후 비면 거부하고, trim한 이름을 저장·자격 매칭에 쓴다. 엔드포인트·모델 `name`에는 `/`를 넣지 않는다(추천 가정). trim 후 `/`가 있으면 이름만 밝힌 설정 오류로 거부한다. 모델 `name` 유일성은 같은 엔드포인트 안에서만 적용하고, 다른 엔드포인트끼리는 같은 별칭을 허용한다. 필수: 엔드포인트 `name`·`base_url`·`kind`(6종만: `openai`, `openrouter`, `ollama`, `vllm`, `llamacpp`, `lmstudio` — strata 전용 kind 없음), 모델 `name`·`id`. 선택: 모델 `sampling`·`reasoning`·`extra`. 루트 `target_language` 기본 `ko`, `sampling`·`http` 생략 시 기존 기본. `http` 키는 `retries`, `backoff_min`, `backoff_max`, `retry_after_max`, `connect_seconds`, `header_wait_seconds`, `stream_idle_seconds`(기본 3 / 1 / 30 / 60 / 10 / 300 / 120). 옛 toml 이름 호환 없음. 거부: 엔드포인트 `api_key`·`api_key_env`(값은 읽지도 출력하지도 않음, D10·02-2와 동일), 최상위 옛 `models`(“모델은 endpoints[].models에”), 모델의 `endpoint` 칸. 그 밖 알 수 없는 칸은 무시(02-1과 동일 취지). `base_url`의 `/v1` 경로는 검증하지 않는다(D19). 내부 설정 진실은 중첩으로 보관하고, 모델 조회는 헬퍼(펼친 뷰)로 한다(AC에 타입 이름 불필요). 02-7 선택 키는 `엔드포인트이름/모델이름`(예: `pc1/fast`)이며 모델 이름만으로 클라이언트를 만들지 않는다. `GET /models`는 참고용이며 설정 `models`와 자동 동기화하지 않고, 조회 실패해도 손쓴 id로 요청 가능하다(02-7 비범위·동작).

## 근거

- 근거: 엔드포인트 아래에 모델을 두면 설정이 직관적이다. 여러 로컬 PC에 같은 별칭(예: `fast`)을 써도 엔드포인트가 다르면 구분된다. OpenCode의 `provider/model` 선택 키와 비슷한 형태다. 선택 키를 첫 `/`로만 나누면 이름에 `/`가 있을 때 모호하고, 여러 PC 별칭보다 구분자가 깨지는 쪽이 더 위험하다.

## 다시 볼 점

- 다시 볼 점: 실제 서버 확인(작업 06)과 02-5의 `kind`별 요청 변환이 이 스키마와 맞는지 본다. 이름 `/` 금지를 완화할 필요가 생기면 선택 키 구분 규칙을 함께 다시 본다.
