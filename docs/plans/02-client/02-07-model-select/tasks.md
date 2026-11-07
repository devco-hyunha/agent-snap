# 02-7 모델 선택과 모델 목록 조회: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 0. 선행(모의 서버): 요청 기록에 상태 코드를 남긴다. 응답을 만든 기록에 `status`를 채워 시험이 요청별로 상태까지 대조하게 한다. `handler.rs`에 섞인 본문 스트림 구현(`ScriptBody`·`DisconnectBody`·`GuardedBody`의 조각·지연·소거 루프)을 `mock_server/body_stream.rs`로 분리한다. 모델 목록 조회 시험을 위해 `/models` 경로(`GET`)를 추가하고, 목록 응답 스크립트를 둔다(현재 경로는 `/chat/completions` 하나뿐, server.rs 52행). 확인: fmt·clippy·test 통과, 시험 이름·수 이동 전과 동일, 기록에 상태가 찍힌 것을 자기 시험으로 확인.
  - 2026-11-05 완료: `RecordedRequest.status`(script.rs 144행), `body_stream.rs`로 분리(ScriptParts·ScriptBody·DisconnectBody·GuardedBody), `/models` `GET` 라우트(server.rs 54행)와 `models_url()`(83~86행), `MockResponseScript::models_list()`(script.rs 118행), 자기 시험 `records_response_status`(self_tests.rs 193행)·`models_list_is_served`(238행). fmt·clippy·test 통과, 시험 이름 5종과 수 유지.
  - 2026-11-05 메모: 구현했다가 02-6 커밋에 넣지 않으려고 트리에서 되돌렸다(`body_stream.rs` 삭제, `status` 칸·자기 시험 제거). 02-7 착수 때 0단계부터 다시 구현 → 독립 리뷰 → 이 문서 단계 체크 후 커밋한다.
- [x] 1. 선택 키(`엔드포인트이름/모델이름`)로 클라이언트를 만든다. 설정에서 엔드포인트와 그 아래 모델을 찾고, 키를 정하고(02-2), 서버용 id와 샘플링(모델별 값이 기본값보다 앞섬), `reasoning`, `extra`를 요청에 쓴다. 모델 이름만으로는 만들지 않는다. 없는·형식 틀린 선택 키는 키만 밝힌 오류로 끝낸다. 확인: 엔드포인트 2개(같은 별칭 허용), 선택 키 여러 개, 모의 서버 2개로 요청이 해당 서버로만 가고 본문의 id와 샘플링이 맞는지 확인하는 통합 테스트(AC 1), 없는·형식 틀린·모델 이름만 입력 오류 단위 테스트(AC 2).
  - 2026-11-05 구현·검증 완료: `config/selection.rs`(선택 키 나누기와 찾기, `SelectionError`는 `thiserror::Error` derive), `llm/selection.rs`(준비 값 `SelectedModel`과 `Config::client_for`, 샘플링 모델별 우선 잇기), `llm/selection/client_build.rs`(02-5 `OpenAiCompatClient::new` 재사용 조립), `llm/selection/apply.rs`(본문 값 채우기와 전송 재사용). 테스트: AC 1 통합(`tests/selection/keys.rs`, 모의 서버 2개·같은 별칭 구분·본문 id와 샘플링·키 헤더), AC 2 단위(`src/config/tests/selection.rs`, 없는 키·형식 오류·모델 이름만·빈 목록), 준비 단위(`src/llm/selection/tests.rs`). fmt·clippy·test 통과. 독립 리뷰 1차 미달(critical 1·major 7) → 보정 → 2차 PASS(`docs/reviews/02-client/02-07-model-select-task1-review-{1,2}.md`). minor 3건(N2-계속, N3, N4)은 2차가 후속 메모로 걸러 냄.
- [x] 2. 모델 목록 조회를 만든다. `GET {base_url}/models`, 인증은 [02-5](../02-05-transport/tasks.md)와 같고, `data[].id`만 필수로 읽는다. 설정 `models`와 자동 동기화하지 않는다. 재시도하지 않고 시간 초과는 02-5 설정을 쓴다(D14). 확인: 정상 응답의 id 목록과 모르는 필드·`null` 필드 무시를 확인하는 통합 테스트(AC 3).
  - 2026-11-05 완료: `llm/transport/fetch_models.rs`(1회짜리 `GET /models` 조회, 재시도 없음), `llm/transport/send_headers.rs`(Bearer·취소·헤더 대기 재사용), `llm/transport/status_error.rs`(비정상 상태·초과를 전송 오류로, 키·주소 가림), `llm/codec/models_list.rs`(`data[].id` 해석, 모르는 필드·`null` 무시). SRP 보정으로 `send_once.rs`에서 헤더·오류 만들기를 분리. 통합 시험 `tests/selection/models_list/ids_read.rs`(id 순서 읽기·모르는 필드·`null` 무시·설정 `models`와 동기화 없음·`/v1` 이중 결합 없음) 4종 통과. fmt·clippy·test 통과.
- [x] 3. 형식 오류를 만든다. `data`가 없거나 배열이 아님, `id`가 없거나 문자열이 아님. 확인: 경우마다 형식 오류이고 오류에 키와 주소가 없음을 확인하는 통합 테스트(AC 3).
  - 2026-11-05 완료: `llm/error/models_format.rs`(`ModelsListFormatError`, 표시에 종류만, 키·주소·본문 없음), 단위 시험 `llm/codec/tests.rs` 6종(JSON 구문 실패·`data` 없음·비배열·`id` 없음·비문자열·표시 가리기), 통합 시험 `tests/selection/models_list/format_error.rs` 2종(빈 본문 200도 형식 오류, 형식 오류 표시에 키·호스트·본문 조각 없음) 통과.
- [x] 4. 취소와 시간 초과가 02-5와 같이 동작함을 확인한다. 확인: 응답을 지연시키는 모의 서버와 취소 토큰으로 확인하는 통합 테스트(AC 3).
  - 2026-11-05 완료: `tests/selection/models_list/cancel.rs`(헤더 받기 전 취소·본문 읽는 중 취소, 요청 1회로 멈춤), `tests/selection/models_list/timeouts.rs`(헤더 지연 성공, 헤더 대기 구간 `HeaderWait`, 본문 멈춤 구간 `StreamIdle`, 설정 초 값 그대로, 취소 상한 500밀리초) 6종 통과. 모의 서버 보강: `models_handler.rs`가 스크립트의 헤더 지연·행·본문 지연을 따르고, `MockServer::spawn_models_script()` 추가.
  - 2026-11-05 메모 (해당 보정 완료): `list_models_timeout_seconds_follow_config_http`가 헤더 대기 300초 기본값 설정을 그대로 써서 시험이 300초 걸리는 문제가 있었다. 브랜치 리뷰 1차 M2(major) 지적 → 헤더 대기 2초 설정(`ac3_http_config_text`)으로 고쳐 `tests\selection.rs` 타깃 2.01초로 단축 완료 (리뷰 2차 확인, `docs/reviews/02/07-model-select/branch-review-2.md`).
- [x] 5. `docs/RESEARCH.md` 작업 02 절의 모델 목록 항목들(OpenAI, llama.cpp, Ollama, vLLM, LM Studio, OpenRouter, 서버별 정리)에 02-7 적용 위치를 갱신한다.
  - 2026-11-05 완료: `docs/RESEARCH.md` 02 절 표의 모델 목록 5행 + 서버별 정리 행을 "적용(코드 경로)"로 갱신(`fetch_models.rs`·`models_list.rs`·`send_headers.rs`·`models_format.rs`와 확인 시험 경로 반영, 확인 날짜 2026-11-05). `docs/research/02-client.md`의 LM Studio 절에 토큰 안내 단서 재확인, OpenRouter 절에 기본 필터 미적용 사유 추가. Ollama·vLLM은 02-5에서 이미 적용(코드 경로)라 갱신 대상 아님.
- [x] 6. 작업 전체 마무리로, 설정이 있으면 실제 엔드포인트로 1회 확인한다. 출력에 URL과 키를 찍지 않는다.
  - 2026-11-05 완료: 사용자가 준 엔드포인트(여기에 주소는 남기지 않음)로 `GET {base_url}/models`를 3회 조회해 확인. 응답 1건(모델 1종), 최상위 필드 `object`·`data`, 항목 필드 `id`·`object`·`created`·`owned_by`로 우리 해석 규칙(`data[].id`만 필수, 모르는 필드·`null` 무시)에 그대로 부합. 키 없는 서버라 Bearer 헤더 없이도 동작(02-5 정책과 일치). 출력에 URL·키를 남기지 않고 id 1개와 필드 이름만 보고했다.
  - 2026-11-05 메모: `Invoke-RestMethod` 1회당 약 20초 걸림(서버 응답 지연). 우리 기본 헤더 대기 300초 설정 안에서 정상.
  - 2026-11-05 추가 확인: 서버에 `POST /api/pull`(스트림)로 모델 1종 추가 후 `/v1/models` 재조회 → 항목 2개로 증가 확인(목록에 새 id 반영). pull 진행률 스트림(`pulling ...` → `success`)과 목록 반영까지 02-7이 요구한 "조회는 참고용, 자동 동기화 없음" 동작과 같은 모습으로 관측. 주소·키 출력 없음.

## 테스트 계획
- 종류: 로컬 모의 서버 통합 테스트(모의 서버 2개가 각자 받은 요청을 기록한다), 오류 종류별 단위 테스트.
- 주입할 것: 설정(임시 폴더의 설정 파일 또는 직접 만든 설정), 자격 파일에서 읽은 이름별 값, 취소 토큰, 시험용 시간 값.
- 테스트 파일은 관심사별로 나눈다(모델 선택, 모델 목록). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [x] [spec.md](spec.md)의 AC 3개를 모두 충족한다. (2026-11-05 체크: AC 1·2 근거는 1단계 완료 메모(11행), AC 3 근거는 2·3·4단계 완료 메모(13·15·17행), 6단계 실엔드포인트 확인은 22~24행 메모. 브랜치 전체 리뷰 4차 PASS(`docs/reviews/02/07-model-select/branch-review-4.md`) 뒤에 체크했다. 2차·3차 리뷰가 완료가 안 된 상태에서의 DoD 체크를 요구한 것은 사용자 기준으로 기각했고, 4차 리뷰가 그 기각 판단을 인정했다. 3차 R3-2가 DoD 줄을 28행으로 센 것은 헤더 자리이고 실제 DoD 줄은 32행이다 (4차 R4-1).)

## 문서 영향
- `docs/RESEARCH.md` 작업 02 절: 모델 목록 항목들.
- `docs/DECISIONS.md`: 구현 중 결정이 있으면 추가한다.
