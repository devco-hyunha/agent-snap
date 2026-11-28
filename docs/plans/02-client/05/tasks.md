# 02-5 클라이언트 트레이트와 스트리밍 전송: 작업 계획

- 상태: 완료 (T1~T10 완료·리뷰 PASS·작업 단위 전체 리뷰 r2 PASS 2026-11-27. 커밋·PR #9 병합. 라운드 이력은 `docs/reviews/02/02-5-*.md` 참조.)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽. T1은 `cargo deny check`를 더한다.
- 브랜치: `feature/client`에서 나눈 `feature/client-transport`에서 진행한다.

## 단계

### T1 새 크레이트 라이선스 확인
- 범위: reqwest, tokio, tokio-util, 시험용 axum·http-body의 라이선스를 crates.io와 포장 LICENSE로 확인하고 결정 14번으로 기록한다. `crates/snap-core/Cargo.toml`에 버전·features를 정해 넣는다.
- 금지: 실서버 호출, 커밋·푸시, 임시 스크립트.
- 확인: 결정 기록에 크레이트 이름·버전·라이선스·근거가 있다.
- 검증: 게이트 3종, `cargo deny check -t x86_64-pc-windows-msvc -t x86_64-unknown-linux-gnu`(대상 밖 wasm32 경로의 CDLA 크레이트를 걸러낸다. 결정 14번의 다시 볼 점).

### T2 시험용 모의 서버
- 범위: 헤더 지연, 본문 멈춤, 연결 종료를 테스트가 제어·관찰한다. 받은 요청 헤더·본문을 기록한다. 모의 서버 자체를 시험하는 테스트를 둔다.
- 금지: 제품 공개 API에 모의 서버 노출, 재시도 로직.
- 확인: 모의 서버 자체 시험이 지연·멈춤·연결 종료·요청 기록을 확인한다.
- 검증: 게이트 3종.

### T3 클라이언트 트레이트와 타입 골격
- 범위: 요청과 취소 토큰을 받아 이벤트 스트림을 여는 트레이트, 요청·스트림 결과·전송 오류 타입 골격. `EndpointKind`를 설정 검증의 허용값과 한자리에서 연결한다.
- 금지: reqwest 타입이 트레이트 밖으로 새기, 재시도 호출, 설정·SSE·재시도 모듈 책임 침범(호출만).
- 확인: HTTP 라이브러리 타입이 전송 구현 안에서만 쓰인다.
- 검증: 게이트 3종.

### T4 요청 본문과 인증 헤더
- 범위: `{base_url}/chat/completions` 본문에 모델 id·메시지·도구·샘플링·`stream`을 담고, `kind`별 `reasoning` 변환과 `extra` 병합을 적용한다. 키가 있을 때만 `Authorization: Bearer`를 보낸다.
- 금지: `tool_choice`·`n`·`stream_options` 전송, `context_window`를 본문에 넣기, 재시도.
- 확인: `kind`별 본문과 무키 요청을 모의 서버로 확인(spec AC 2 요청 부분).
- 검증: 게이트 3종.

### T5 응답 스트림 복원 연결
- 범위: 응답 바이트 조각을 02-3 복원(`StreamState`)에 연결한다. 실패 후 재호출 계약을 코드·테스트로 고정한다.
- 금지: SSE·델타 규칙 재구현, 재시도.
- 확인: 02-3 AC 1과 같은 형식의 복원 결과, 첫 실패 이후 성공 오해 없음(spec AC 1).
- 검증: 게이트 3종.

### T6 시간 초과와 취소
- 범위: 연결은 라이브러리 연결 시간 설정에 맡기고, 헤더 대기·스트림 유휴를 따로 잰다. 스트림 도중 취소하면 취소 오류를 내고 연결을 정리한다.
- 금지: reqwest `timeout`·`read_timeout`로 헤더·유휴를 한 방에 묶기, 재시도 대기 중 취소(02-6).
- 확인: 유휴보다 늦은 헤더 성공, 헤더 미도착, 본문 멈춤, 취소 후 500ms 내 오류와 스트림 종료, 서버 쪽 연결 3초 내 닫힘(spec AC 3·4).
- 검증: 게이트 3종.

### T7 오류 종류와 가리기
- 범위: 헤더 전 연결 실패를 `PreHeaderConnectKind` 여섯 종류로 구분한다. 서버 오류 본문과 연결 실패를 어림 가리한 뒤 알고 있는 키·base_url 호스트 문자열로 한 번 더 가린다.
- 금지: 02-4 `decide_retry` 호출과 재시도 적용, 오류에 키·base_url 원문 담기.
- 확인: 세부 종류와 Display·Debug에 키·호스트가 없다(spec AC 2 가리기 부분·AC 6).
- 검증: 게이트 3종.

### T8 사용량·시각과 조사 갱신
- 범위: 스트림 종료 시 사용량(있으면 값, 없으면 없음)과 시각(요청 시작·첫 토큰·마지막 토큰)을 돌려준다. `docs/RESEARCH.md` 02 절과 `docs/research/02-client-transport.md`에 02-5 코드 적용 위치를 갱신한다.
- 금지: 평균 지표 계산, 토큰별 도착 시각 기록.
- 확인: 사용량 있음·없음 경우와 시각 순서(spec AC 5), 조사 적용 위치에 코드 경로가 있다.
- 검증: 게이트 3종.

### T9 구조 점검 (미커밋 작업 파일 전수)
- 범위: 커밋되지 않은 모든 작업 파일(`.rs`, `Cargo.toml`)을 하나씩 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일 전부에 SRP 검토 기록을 남긴다. 대상은 `git status`와 미추적 파일까지 포함해 정한다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.

### T10 구조 보정 (T9 잔여 4건)
- 범위: T9가 넘긴 잔여 4건을 파일 이동·분리·정본 재사용으로 해소한다. 동작·시그니처·런타임 문자열·공개 API 이름은 바꾸지 않는다.
- 금지: 동작 변경, 파일 삭제, 커밋·푸시.
- 확인: 잔여 4건이 표에서 해소로 갱신된다. 이동·분리로 만든 파일이 한 책임씩 담는다.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽. 로그는 `.local/gate/T10-*.log`.

## 잔여

T9 구조 점검에서 넘긴 4건. 같은 계획 02-5 안에서 잇는다. 상세 표는 `docs/tasks/02/05/T9-structure-check.md`의 "위반 잔여 이관". T10에서 해소했다.

- 잔여 1 (T10 해소): 강도 허용값 정본을 `crates/snap-core/src/llm/client/effort.rs`의 `ReasoningEffort::PAIRS` 한곳으로 모으고 `consts/limits.rs`의 `ALLOWED_REASONING`을 제거했다. 설정 검증 `check_reasoning`은 같은 타입의 `from_setting`만 쓴다.
- 잔여 2 (T10 해소): `BODY_MAX_BYTES`를 `crates/snap-core/src/consts/status.rs`로 옮겼다.
- 잔여 3 (T10 해소): 타입·로직 분리는 `llm/client/request.rs`(타입만)·`llm/client/effort.rs`(강도 로직), `tests/mock_server/types.rs`(타입만)·`tests/mock_server/store.rs`(저장 로직). 시각 기록은 `llm/client/token_times.rs`로 분리. 시험은 `tests/client/timeout.rs`(시간 초과)·`tests/client/timeout_connect.rs`(연결 실패), `tests/client/connect_kinds.rs`(세부 구분)·`tests/client/connect_redact.rs`(가리기), `tests/client/restore.rs`(복원)·`tests/client/restore_timings.rs`(시각)로 나눴다.
- 잔여 4 (T10 해소): `base_url`·`transport_timings` 정본을 `tests/client/stream_open.rs`에, `FIRST_CHUNK`·`SECOND_CHUNK` 정본을 `tests/client/stream_payload.rs`에, `ECHO_KEY` 정본을 `tests/client/stream_echo.rs`에 두고, `cancel.rs`·`timeout.rs`·`redact.rs`·`mock_request.rs`·`connect_kinds.rs`의 중복 정의를 지웠다.

## 테스트 계획

- 종류: 로컬 모의 서버 통합 테스트.
- 주입할 것: 취소 토큰, 시험용 시간 값(짧게), 모의 서버의 응답 시점과 연결 종료 제어, 알고 있는 키·호스트 문자열.
- 시간에 의존하는 테스트는 경과 시간을 넉넉한 하한·상한으로만 확인한다.
- 테스트 파일은 관심사별로 나눈다(복원·스트림 상태 계약, 요청 본문과 `kind`별 reasoning, 인증과 비밀 값·가리기, 연결 오류 세부, 시간 초과, 취소, 사용량과 시각).

## DoD

- [spec.md](spec.md)의 AC 6개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-5 항목 적용 위치 갱신.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/research/02-client-transport.md`: 신규.
- `docs/RESEARCH.md` 02 절: 02-5 항목 적용 위치 갱신.
- `docs/DECISIONS.md`: 14번 기록 완료(크레이트). 15번은 T4에서 `reasoning` 변환과 `extra` 병합으로 추가한다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/05/*` 상태 줄 갱신.
