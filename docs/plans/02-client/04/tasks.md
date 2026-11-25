# 02-4 재시도 판단: 작업 계획

- 상태: 완료 (초안 2026-11-25. T1~T5 완료, 02-4 작업 단위 전체 리뷰 PASS 2026-11-25. 결정 10·12·13번 기록 완료. 리뷰 기록 `docs/reviews/02/02-4-unit-r1.md`. WSL(Ubuntu) 게이트는 커밋 직전 소관)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.

## 단계

### T1 재시도 대상 판별
- 상태: 완료 (1차 리뷰 미달 major 1·minor 2 → 보정 → 재리뷰 2차 PASS 2026-11-25. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T1-r2.md`·`.local/gate/T1-r2.log`. WSL 게이트는 커밋 직전 소관. 리뷰 기록 `docs/reviews/02/02-4-t1-r1.md`·`02-4-t1-r2.md`)
- 범위: 상태 코드 화이트·블랙리스트(429·500/502/503/504 vs 402·408·425·목록 밖 5xx·기타 4xx), 헤더 전 연결 오류 후보, 헤더 대기 초과 비재시도. 재시도 입력 타입(연결 오류 종류·상태 코드·헤더 대기 초과)을 이 단계에서 정의한다.
- 금지: 실제 대기·재요청 코드, 설정 읽기 재구현.
- 확인: 판별 단위 테스트(spec AC 5·6).
- 검증: 게이트 3종.

### T2 `Retry-After` 해석·상한
- 상태: 완료 (구현·게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T2.md`. 1차 리뷰 미달 major 1·minor 2 → 보정 → 2차 PASS 2026-11-25. 3~4차 미달 minor는 라운드 보고 서술·서식 지적이었고 PROJECT RULES "리뷰 루프 차단"·HARNESS 9절 234번에 따라 5차 재리뷰 없이 닫음. WSL 게이트는 커밋 직전 소관)
- 범위: 초·HTTP 날짜 두 형식, 0·과거 날짜→0초, 음수·해석불가→헤더 없음, 상한 이하 대기·초과 포기. 현재 시각 주입. HTTP 날짜 파서는 정책 파일과 분리(결정 13번).
- 금지: 실제 대기.
- 확인: 단위 테스트(spec AC 1·2).
- 검증: 게이트 3종.

### T3 Full Jitter 백오프·횟수
- 상태: 완료 (r2 보정까지 반영·게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T3-r2-fmt.log`·`T3-r2-clippy.log`·`T3-r2-test.log`. r3 재리뷰 minor 2건(독스트링 어체)은 메인 교정으로 HARNESS 9절 234번에 따라 재리뷰 세션 없이 닫음. 최종 게이트 로그 `.local/gate/T3-r4-fmt.log`·`T3-r4-clippy.log`·`T3-r4-test.log`. WSL(Ubuntu) 게이트는 커밋 직전 소관)
- 범위: 난수 주입, 시도 횟수(첫 요청=1), 횟수 초과를 `Retry-After` 상한 초과보다 먼저 판정(결정 12번).
- 금지: 실제 대기, Decorrelated Jitter.
- 확인: 난수 고정 단위 테스트(spec AC 3·4).
- 검증: 게이트 3종.

### T4 공개 API·문서 연쇄
- 상태: 완료 (구현 완료·r2 보정 완료·게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T4-r2-fmt.log`·`T4-r2-clippy.log`·`T4-r2-test.log`. r2 재리뷰 minor 2건은 T5 잔여로 이관 후 해소. WSL(Ubuntu) 게이트는 커밋 직전 소관)
- 범위: `decide_retry` 공개 API와 재시도 모듈 배치, 포기 이유 필드 제한(키·주소·본문 안 담기). `docs/RESEARCH.md` 02 절에 RFC 9110, RFC 6585, OpenRouter API Rate Limits(402 미적용), AWS Jitter, Google SRE 5종을 추가하고 적용 위치를 코드 경로로 갱신. `docs/research/02-client-retry.md` 신규. `docs/DECISIONS.md`에 10·12·13번 기록. 상태 줄 갱신.
- 금지: 계획 문서 재설계.
- 확인: API·포기 필드 단위 테스트(spec AC 7), RESEARCH 적용 위치, DECISIONS 10·12·13 상태, 상태 줄 반영.
- 검증: 게이트 3종.

### T5 구조 점검 (미커밋 작업 파일 전수)
- 상태: 완료 (r1 표 31건 전수·100줄 초과 SRP 검토 4건 기록. r2에서 잔여 4건 코드 보정 완료 2026-11-25, 게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T5-r2-fmt.log`·`.local/gate/T5-r2-clippy.log`·`.local/gate/T5-r2-test.log`. r1 경량 게이트 로그 `.local/gate/T5-fmt.log`. 02-4 작업 단위 전체 리뷰에서 AC 1~7 7/7·DoD 확인, r2 잔여 2건(중복 상수·잔여 절 기록) 해소 완료. 최종 로그 `.local/gate/T4-r4-fmt.log`·`T4-r4-clippy.log`·`T4-r4-test.log`)
- 범위: 커밋되지 않은 모든 작업 파일(`.rs`, `Cargo.toml`)을 하나씩 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일 전부에 SRP 검토 기록을 남긴다. 대상은 `git status`와 미추적 파일까지 포함해 정한다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.

## 테스트 계획

- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 난수 공급, 현재 시각. 실제 대기는 하지 않는다.
- 테스트 입력은 확인된 사실(OpenRouter 429와 `Retry-After`, llama.cpp 503)의 형식을 반영한다.
- 테스트 파일은 관심사별로 나눈다(대상 판별, `Retry-After` 해석, 백오프, API). 측정은 dev-principles 색인의 지정 잎(DRY·이름과 함수)으로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD

- [spec.md](spec.md)의 AC 7개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-4 항목 적용 위치 갱신.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/RESEARCH.md` 02 절: RFC 9110, RFC 6585, OpenRouter API Rate Limits, AWS Jitter, Google SRE 추가 후 적용 위치 갱신.
- `docs/research/02-client-retry.md`: 신규 상세.
- `docs/DECISIONS.md`: 10·12·13번 추가. 구현 중 새 결정이 생기면 11번으로 채운다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/04/*` 상태 줄 갱신.

## 잔여

- T5 r1 잔여 4건 해소 완료(2026-11-25, T5 r2 코드 보정). ① `crates/snap-core/tests/retry/backoff/cap.rs`의 `const SETTINGS`를 없애고 `crates/snap-core/tests/retry/fixtures.rs`의 `SETTINGS`를 import로 재사용. ② `crates/snap-core/tests/retry/after/date.rs`의 상한 경계 시험 3건(`cap_seconds_is_wait`·`cap_date_is_wait`·`over_cap_date_is_exceeds_cap`)과 상한 `CAP`을 `crates/snap-core/tests/retry/after/cap.rs`로 이관. ③ `crates/snap-core/tests/retry/decide/header.rs`의 `unparseable_header_falls_back_to_backoff`를 `crates/snap-core/tests/retry/decide/backoff.rs`로 이관. ④ `crates/snap-core/tests/retry/decide/give_up.rs`의 `reason_text_has_no_secret_fields`를 `crates/snap-core/tests/retry/decide/redact.rs`로 이관하고 `crates/snap-core/tests/retry/decide/mod.rs`에 선언을 추가. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate/T5-r2-fmt.log`·`.local/gate/T5-r2-clippy.log`·`.local/gate/T5-r2-test.log`.
- T5 r2 잔여(2026-11-25, 코드 보정 작업): `crates/snap-core/tests/retry/decide/header.rs` 134줄·`crates/snap-core/tests/retry/decide/backoff.rs` 129줄은 이동 후에도 100줄을 넘는다. 두 파일은 책임이 하나씩으로 확인됐으므로 SRP 트리거만 연 상태이고, 분리 지시 대상은 아니다.
- T5 r2 잔여 해소 완료(2026-11-25, 메인 Fast-track 보정): `crates/snap-core/tests/retry/after/seconds.rs`의 `const CAP`을 없애고 `crates/snap-core/tests/retry/after/cap.rs`의 `CAP` 정본을 `use super::cap::CAP;` import로 재사용.
