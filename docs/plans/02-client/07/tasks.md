# 02-7 모델 선택과 모델 목록 조회: 작업 계획

- 상태: 완료 (T1~T5 구현·개별 리뷰 PASS, 작업 단위 전체 리뷰 r1 미달 → 보정 → r2 PASS. `feature/client-endpoint`를 `feature/client`에 병합)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.
- 브랜치: `feature/client`에서 나눈 `feature/client-endpoint`에서 진행한다.

## 단계

### T1 선택 키 해석과 대상 조회
- 상태: 완료 (r1 PASS, `docs/reviews/02/02-7-T1-r1.md`)
- 범위: `엔드포인트이름/모델이름` 문자열을 해석해 `Config`에서 엔드포인트와 모델을 찾는 함수를 둔다. 구분자 없는 키·빈 칸은 형식 오류, 대상이 없으면 선택 오류로 낸다. 오류 Display·Debug에 키·base_url 호스트를 담지 않는다.
- 금지: `Config` 타입 변경, reqwest 타입 노출, 사람 문구를 함수 즉석 생성(`locale/` 사용).
- 확인: 선택 성공·형식 오류·대상 없음 시험(spec AC 1·2). 오류에 키·호스트가 없다.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T2 클라이언트 조립
- 상태: 완료 (r1 미달 → r2 미달 → r3 PASS, `docs/reviews/02/02-7-T2-r3.md`)
- 범위: T1이 찾은 엔드포인트·모델로 `HttpTransport`(02-5)를 만들고 자격 키와 `http` 설정을 넣은 뒤 02-6 재시도 래퍼를 씌워 `LlmClient`로 돌려준다. 모델 id를 함께 돌려준다.
- 금지: reqwest 타입 노출, 재시도 판단 로직 변경, 새 전송 구현.
- 확인: 조립 결과가 `LlmClient` 트레이트이고 재시도 래퍼가 감겨 있다. 모의 서버로 조립된 클라이언트가 같은 요청 본문으로 스트림을 여는 것을 확인(spec AC 1).
- 보정 확인: 허용 목록 밖 `kind`로 조립할 때 OpenAI 형식 본문으로 열림(직전 리뷰 minor 1건). 시험은 `crates/snap-core/tests/client/build/kind_fallback.rs`. 보정 r2는 `docs/tasks/02/07/T2-fix2-kind-fallback-discriminate.md`의 라운드 보고가 기준이며, 강도(`reasoning_effort`)·형식 구분 검증으로 게이트 3종 양쪽 통과.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T3 모델 목록 조회
- 상태: 완료 (r1 미달 → r2 직전 3건 해소 확인 + 새 minor 2건 → 주석 전용 경량 게이트 보정 → PASS, `docs/reviews/02/02-7-T3-r2.md`)
- 범위: `{base_url}/models` GET 요청으로 최상위 `data` 배열의 문자열 `id`만 모은다. `data` 없음·`id` 비문자열은 형식 오류. `/v1` 보정 규칙은 한곳의 문자열 헬퍼로 구현하고 `endpoint.rs` 경로 조립과 함께 쓴다. 서버 오류는 02-5 가리기·재시도 규칙을 따른다.
- 금지: LM Studio 고유 REST 사용, 모르는 필드 해석, 응답 원문 전체를 오류에 담기.
- 확인: 모의 서버로 정상·`data` 없음·`id` 타입 오류·HTTP 오류·`/v1` 이중 방지 경로를 확인(spec AC 3·4·5·6·8).
- 보정 확인: 직전 리뷰 minor 3건(`read_json`·`json_read_error` 책임용 시험 부재, 재료 창구 중복, 라운드 보고 삭제 기록 불일치)을 `docs/tasks/02/07/T3-fix-models-list.md`의 라운드 보고가 기준이며 시험·재료·문서만 고쳐 게이트 3종 양쪽 통과.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T4 자동 발견
- 상태: 완료 (r1 미달 → r2 직전 minor 해소 확인 + 문서 연쇄 minor → 메인 기록 작성으로 닫음 → PASS, `docs/reviews/02/02-7-T4-r2.md`)
- 범위: `kind`가 `ollama`·`lmstudio`이고 `base_url`이 비어 있으면 기본 주소 상수(`consts/`)로 한 번 프로브해 채운다. 실패는 발견 실패로 보고하고 재시도·폴링·캐시를 하지 않는다.
- 금지: 상시 폴링·mDNS·인벤토리 캐시, `llamacpp`·`vllm` 대상화, `/health` 선행 확인.
- 확인: reachable/unreachable 루프백 시험으로 채움·실패를 확인(spec AC 7). 요청이 늘지 않는다.
- 보정 확인: 직전 리뷰 minor 1건(`DiscoveryOutcome::Found`의 `base_url`가 `String`이라 Debug에 주소 원문 노출)을 `docs/tasks/02/07/T4-fix-discovery-secret.md`의 라운드 보고가 기준이며, `base_url`를 `SecretString`으로 바꾸고 가리기 시험(`crates/snap-core/tests/discovery/redact.rs`)을 추가해 게이트 3종 양쪽 통과.
- 작업 단위 리뷰 r1 보정 확인: 직전 리뷰 major 1건(`discover_base_url` 미사용 공개 창유지와 AC 7 목록 연결의 간접 확인)을 `docs/tasks/02/07/unit-fix-discovery-window.md`의 라운드 보고가 기준이며, `discover_base_url`와 재수출 3개를 삭제하고 채운 주소 목록 연결 시험(`crates/snap-core/tests/discovery/list.rs`)을 추가해 게이트 3종 양쪽 통과.
- 작업 단위 리뷰 r2 PASS: 보정 뒤 독립 재리뷰에서 지적 0건으로 PASS(`docs/reviews/02/02-7-unit-r2.md`). r1 minor 2건(문서 상태 줄 정합, `RESEARCH.md`의 `body_read` 누락)은 문서 정합으로 닫음.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T5 구조 점검 (미커밋 작업 파일 전수)
- 상태: 완료 (미추적·수정 `.rs` 59개 전수 표. 위반 후보 3건은 메인 규칙 대조에서 위반 아님으로 닫음 → 잔여 없음. `docs/tasks/02/07/T5-structure-check.md`)
- 범위: 커밋되지 않은 모든 작업 파일(`.rs`, `Cargo.toml`)을 하나씩 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일(파일 전체 줄 수, 빈 줄 포함) 전부에 SRP 검토 기록을 남긴다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.

## 잔여

없음. T5 구조 점검에서 위반 후보 3건(major 2·minor 1)을 메인 규칙 정본과 대조해 위반 아님으로 닫았고, 이관한 잔여가 없다. 상세는 `docs/tasks/02/07/T5-structure-check.md`의 "위반 후보 3건 → 메인 규칙 대조 판정: 위반 아님".

## 테스트 계획

- 종류: 로컬 모의 서버 통합 테스트(02-5 재료 재사용), 네트워크 없는 선택 키 단위 시험.
- 주입할 것: 자격 키 목록, 시험용 시간 값(짧게), 모의 서버의 `/v1/models` 응답 시나리오, 루프백 프로브 주소.
- 시간에 의존하는 시험은 경과 시간을 넉넉한 하한·상한으로만 확인한다.
- 시험 파일은 관심사별로 나눈다(선택 키 해석, 조립, 목록 형식, 목록 HTTP 오류, `/v1` 보정, 자동 발견).

## DoD

- [x] [spec.md](spec.md)의 AC 8개를 모두 충족한다. (r2 PASS, `docs/reviews/02/02-7-unit-r2.md`)
- [x] 게이트 3종 Windows·WSL 통과.
- [x] `docs/RESEARCH.md` 02 절의 02-7 항목 적용 위치 갱신.
- [x] 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다. (r1 미달 → 보정 → r2 PASS)

## 문서 영향

- `docs/research/02-client-models.md`: 신규(작성 완료).
- `docs/RESEARCH.md` 02 절: 02-7 항목 적용 위치 갱신.
- `docs/DECISIONS.md`: 선택 키 조립 위치·자동 발견 범위·`/v1` 보정 규칙 결정 추가.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/07/*` 상태 줄 갱신.
