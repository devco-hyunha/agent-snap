# 02-6 재시도 적용: 작업 계획

- 상태: 완료 (T1~T7 구현·개별 리뷰 PASS. 작업 단위 전체 리뷰 PASS(`docs/reviews/02/02-6-unit-r1.md`))
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.
- 브랜치: `feature/client`에서 나눈 `feature/client-retry-transport`에서 진행한다.

## 단계

### T1 상태 오류에 `Retry-After` 헤더 값 담기
- 범위: 02-5의 상태 오류(`TransportError::Status`)에 서버가 준 `Retry-After` 헤더 값을 담는 칸을 더한다. 값이 없을 때는 없는 것으로 둔다. 결정 16번을 `docs/DECISIONS.md`에 기록한다.
- 금지: 02-4 판단 로직 변경, 오류에 키·base_url 호스트 담기, 헤더 원문 외 값 담기.
- 확인: 상태 오류가 헤더 값을 담고, Display·Debug에 키·호스트가 없다. 기존 상태 오류 시험이 그대로 통과한다.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T2 전송 오류 → 02-4 입력 매핑
- 범위: 02-5 전송 오류를 02-4 spec의 매핑 표대로 02-4 입력(`FailureInput`)으로 옮기는 함수를 한 곳에 둔다. 매핑 표는 02-4 spec가 정본이고 코드에는 한 곳만 둔다.
- 금지: 02-4 판단 규칙 재정의, 매핑 표를 문서 두 곳에 두기, 스트림·취소 오류를 판단에 넣기.
- 확인: 매핑 표의 각 행이 함수 시험으로 확인된다(연결 6종→후보, 헤더 대기 초과→비재시도, 상태 코드 후보·비대상, 스트림·유휴·취소→판단 입력 아님).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T3 재시도 래퍼 골격
- 범위: `LlmClient`를 구현하는 래퍼 타입을 `llm/client/`에 둔다. 안쪽 클라이언트를 감싸고, `open` 단계 실패에서만 02-4 판단을 따라 다시 연다. 시도 횟수는 첫 요청을 1로 센다. 포기가 정해지면 안쪽 오류를 그대로 돌려준다.
- 금지: 헤더 수신 후 실패 재생, reqwest 타입 노출, 판단 규칙 재정의, 스트림 조각 소비 후 재생.
- 확인: 모의 서버로 429 재시도 성공, 헤더 전 연결 재시도, 비대상 즉시 포기, 횟수 초과 포기를 확인한다(spec AC 1·2·6).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.
- 보정(r1 critical): 헤더를 받은 뒤 조각 읽기 실패를 재시도 대상이 아닌 전용 오류로 낸다. 상세는 `docs/tasks/02/06/T3-fix-stream-reset.md`.

### T4 백오프 대기와 취소 경합
- 범위: 02-4가 준 대기 시간을 tokio 시간 대기와 취소 토큰의 `select!`(biased)로 잰다. 취소가 먼저 오면 대기하지 않고 취소 오류로 끝난다.
- 금지: sleep만 쓰는 취소 불가 대기, 취소 후 추가 요청 발생.
- 확인: 대기 중 취소 시 요청이 늘지 않고 취소 오류로 끝난다(spec AC 5).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.
- 보정(r1 minor 2건): 시험 독스트링에 백오프 취소 반영, 시험 쪽 `RETRY_AFTER` 상수 한곳 정본 정리. 상세는 `docs/tasks/02/06/T4-fix-docs-const.md`.

### T5 상용 난수원과 설정 변환
- 범위: 02-4의 `RandomSource` 상용 구현을 제품 코드에 둔다(균등 분포, 새 난수 크레이트 의존 없음). 02-2의 `http` 값(f64)을 `RetrySettings`(Duration·정수)로 바꾸는 변환을 둔다.
- 금지: 새 난수 크레이트 의존, 설정 타입(`HttpSettings`)을 판단이 직접 받게 하기.
- 확인: 균등 분포 경계 시험, 변환 값이 02-4 기본값과 맞는다. 결정 17번 기록.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T6 구조 점검 (미커밋 작업 파일 전수)
- 범위: 커밋되지 않은 모든 작업 파일(`.rs`, `Cargo.toml`)을 하나씩 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일(파일 전체 줄 수, 빈 줄 포함) 전부에 SRP 검토 기록을 남긴다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.

### T7 구조 보정 (T6 잔여)
- 범위: T6가 넘긴 잔여를 파일 이동·분리·정본 재사용으로 해소한다. 동작·시그니처·런타임 문자열·공개 API 이름은 바꾸지 않는다.
- 금지: 동작 변경, 파일 삭제, 커밋·푸시.
- 확인: 잔여가 표에서 해소로 갱신된다. 이동·분리로 만든 파일이 한 책임씩 담는다.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽. 로그는 `.local/gate/T7-*.log`.
- 상태: r14에서 R1~R4 해소 완료(게이트 3종 양쪽 통과·리뷰 PASS). r15에서 R5~R11 해소 완료(게이트 3종 양쪽 통과·리뷰 PASS). 상세는 `docs/tasks/02/06/T7-structure-fix.md` 라운드 보고.

## 잔여

T6 구조 점검에서 넘긴 잔여. 같은 계획 02-6 안에서 잇는다. 상세 표는 `docs/tasks/02/06/T6-structure-check.md`의 "위반 잔여 이관". T7에서 해소한다.

## 테스트 계획

- 종류: 로컬 모의 서버 통합 테스트(02-5 재료 재사용).
- 주입할 것: 취소 토큰, 시험용 시간 값(짧게), 값이 고정된 난수원, 모의 서버의 응답 시점과 연결 종료 제어.
- 시간에 의존하는 테스트는 경과 시간을 넉넉한 하한·상한으로만 확인한다.
- 테스트 파일은 관심사별로 나눈다(재시도 성공 경로, 매핑 표, 헤더 수신 후 비재시도 경계, 백오프 취소, 난수원·설정 변환).

## DoD

- [spec.md](spec.md)의 AC 8개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-6 항목 적용 위치 갱신.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/research/02-client-retry-apply.md`: 신규(작성 완료).
- `docs/RESEARCH.md` 02 절: 02-6 항목 적용 위치 갱신.
- `docs/DECISIONS.md`: 16번(`Retry-After` 헤더 담기)은 T1에서, 17번(상용 난수원)은 T5에서 추가한다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/06/*` 상태 줄 갱신.
