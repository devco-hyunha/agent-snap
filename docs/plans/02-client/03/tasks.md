# 02-3 SSE 파싱과 델타 복원: 작업 계획

- 상태: 완료 (승인 2026-11-24. T1~T6 완료·리뷰 PASS 2026-11-25. T8 구조 보정 완료·4차 리뷰 PASS 2026-11-25. T7 구조 점검 완료·8차 Fast-track PASS 2026-11-25. 잔여 없음. PR #6으로 `feature/client` 병합)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.

## 단계

### T1 SSE 줄 처리
- 상태: 완료 (1차 리뷰 미달 major 1·minor 1 → 보정 → 재리뷰 2차 PASS 2026-11-24. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate-review2-{fmt,clippy,test}.log`. WSL 게이트는 커밋 직전 소관)
- 범위: `llm/`에 바이트 조각을 받아 SSE 이벤트로 만드는 줄 처리를 둔다 (T8 평면화 이후 자리). 줄 끝 CRLF·LF·CR, `:` 주석 줄 무시, 여러 `data` 줄 개행 결합, 맨 앞 BOM 하나 버림, 조각 경계 잘림·멀티바이트 문자 복원, `[DONE]`, 빈 줄 없이 끝난 마지막 이벤트(결정 9번).
- 금지: HTTP 전송 코드, 요청 시작·첫 토큰·마지막 토큰 시각 기록.
- 확인: 형식별 입력과 조각 경계 시험 단위 테스트(spec AC 1, 2).
- 검증: 게이트 3종.

### T2 조각 경계 불변성
- 상태: 완료 (1차 리뷰 미달 minor 1 → 보정 → 재리뷰 2차 PASS 2026-11-24. 게이트 3종 Windows·WSL 통과, 로그 `.local/gate-review2-t2-20261124-220553.log`·`.local/gate-review2-t2-wsl-20261124-220734.log`. 중간 시도 로그 2건은 PASS 확정 후 삭제)
- 범위: 형식이 섞인 바이트를 임의 경계와 한 바이트씩 나눠 넣어 한 번에 넣은 결과와 같은지 확인하는 시험을 둔다.
- 금지: 제품 코드 추가 중복.
- 확인: 경계 불변성 단위 테스트(spec AC 1).
- 검증: 게이트 3종.

### T3 델타 복원
- 상태: 완료 (1차 리뷰 미달 major 1·minor 1 → 보정 → 재리뷰 2차 PASS 2026-11-24. 게이트 3종 Windows·WSL 통과, 로그 `.local/gate-review2-t3-*-20261124-2255*.log`·`.local/gate-review2-t3-wsl-20261124-225611.log`)
- 범위: 청크 JSON에서 content, 생각(`reasoning_content`·`reasoning` 두 이름), tool_calls를 index별로 누적·합친다. 서로 다른 index 델타가 번갈아 오는 경우를 포함한다.
- 금지: 전송 계층 수정.
- 확인: 순서 보존과 교차 델타를 확인하는 단위 테스트(spec AC 2, 3).
- 검증: 게이트 3종.

### T4 모르는 필드 보존과 오류
- 상태: 완료 (4차 리뷰 PASS 2026-11-25. 게이트 3종 Windows·WSL 통과, 로그 `.local/gate-review4-t4-win-20261125-095159.log`·`.local/gate-review4-t4-wsl2-20261125-095642.log`, 리뷰 기록 `docs/reviews/02/02-3-t4-r4.md`)
- 범위: 모르는 델타 필드(예: `reasoning_details`) 원문 JSON 보존, 최상위 `error` 청크의 스트림 오류, 잘못된 JSON과 잘린 마지막 이벤트의 형식 오류.
- 금지: 오류에 키·base_url 호스트·청크 원문 담기.
- 확인: 오류 Display·Debug에 키·주소가 없음을 포함한 단위 테스트(spec AC 4).
- 검증: 게이트 3종.

### T5 usage 읽기
- 상태: 완료 (2차 리뷰 PASS 2026-11-25. 게이트 3종 Windows·WSL 통과, 로그 `.local/gate-review2-t5-win-20261125-103116.log`·`.local/gate-review2-t5-wsl-20261125-103221.log`, 리뷰 기록 `docs/reviews/02/02-3-t5-r1.md`·`02-3-t5-r2.md`)
- 범위: usage가 있으면 그 값, 없으면 "없음"으로 읽는다. usage 청크의 `finish_reason` 되풀이가 content·생각·도구 호출 중복을 만들지 않는다.
- 금지: 토큰별 도착 시각 기록.
- 확인: 단위 테스트(spec AC 5).
- 검증: 게이트 3종.

### T6 문서 연쇄
- 상태: 완료 (리뷰 PASS. 라운드 기록·리뷰 판정 상세는 `docs/tasks/02/03/T6-docs.md` 라운드 절에만 둔다)
- 범위: `docs/RESEARCH.md` 02 절의 02-3 항목 적용 위치를 코드 경로로 갱신하고, plan·spec·tasks·`docs/PLAN.md`·`docs/plans/02-client.md` 상태 줄을 완료로 바꾼다. 잔여로 어체 교정 2곳을 함께 다룬다.
- 잔여 (eli5-clarity 어체, T3 재리뷰 2차 참고, 해소 2026-11-25): `crates/snap-core/src/llm/event.rs`의 `맡는다`, `crates/snap-core/src/llm/state.rs`의 `읽지 않는다`를 명사형 한 줄로 교정. 근거: `docs/reviews/02/02-3-t3-r2.md` 참고 절. 두 대상은 현재 각각 `llm/event.rs`(담는 파일), `llm/lines/events.rs`(`이후 조각은 읽지 않음`)로 이동·교정되어 실재하지 않는다.
- 금지: 계획 문서 재설계.
- 확인: RESEARCH 적용 위치, DECISIONS 9번 상태, 상태 줄 반영.
- 검증: 게이트 3종.

### T7 구조 점검 (미커밋 작업 파일 전수)
- 상태: 완료 (표 산출물 작성·위반 5건 잔여 이관. 1~7차 리뷰 미달 보정 완료, 8차 Fast-track PASS 2026-11-25. 라운드 기록은 `docs/tasks/02/03/T7-structure-check.md` 라운드 절. 리뷰 기록 `docs/reviews/02/02-3-t7-r1-r7.md`)
- 범위: 미커밋 작업 파일 `.rs` 134종 + `Cargo.toml` 3종을 하나씩 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. (스코어 정본: `harness/PROJECT_RULES.md` "구조 점검 대상은 커밋되지 않은 모든 작업 파일 전체". T7 1~5차 기록의 "저장소 전체" 표현은 같은 점검의 옛 이름이고, 6차 보정에서 이 스코어로 정질했다) 100줄 초과 파일 전부에 SRP 검토 기록을 남긴다. 이전 T7(SRP 분리)은 T6 검토 표 3종만 대상으로 삼았으므로 점검 범위로서 무효이고, 코드 이동 자체는 유효하다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이라 주석 전용 경량 게이트(`cargo fmt --check`만).

### T8 구조 점검 위반 보정
- 상태: 완료 (구현 완료·게이트 3종 Windows·WSL 통과 2026-11-25. 1~4차 리뷰 미달 보정 후 4차 PASS 2026-11-25. 라운드 기록은 `docs/tasks/02/03/T8-structure-fix.md` 라운드 절, 리뷰 기록 `docs/reviews/02/02-3-t8-r1-r4.md`)
- 범위: T7이 남긴 위반 5건(파일 하나짜리 폴더 감쌈 2종, 서로 다른 요구 몰림 시험 3종)을 파일 이동·분리로 보정한다. 라운드 2에서 타입 분리 2건을 추가했다(`llm/lines.rs`의 `SplitError` → `llm/split_error.rs`, `delta/parse.rs`의 `ChunkDelta`·`FieldText`·`ToolCallDelta` → `delta/chunk_types.rs`). 동작·시그니처·런타임 문자열·공개 API 이름은 바꾸지 않는다.
- 금지: 동작 변경, 공개 API 이름 변경, 파일 삭제, 커밋·푸시, 임시 검증 스크립트.
- 확인: 이동 후에도 기존 시험 결과가 그대로 통과. 파일마다 책임 하나. `llm/sse/` 경로 참조를 `llm/`으로 바꾼 설명·주석·문서 참조를 함께 고친다.
- 검증: 게이트 3종 Windows·WSL 양쪽. 로그 `.local/gate-t8-*.log`.

## 테스트 계획

- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 없음(바이트 조각을 직접 넣는다).
- 테스트 입력은 확인된 사실의 형식(keep-alive 주석, reasoning 필드, CRLF, 청크 경계에서 잘린 줄)을 반영한다.
- 테스트 파일은 관심사별로 나눈다(줄 처리, 조각 경계, 델타 복원, 오류, 사용량). 측정은 dev-principles 색인의 지정 잎(DRY·이름과 함수)으로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD

- [spec.md](spec.md)의 AC 5개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-3 항목 적용 위치 갱신.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/RESEARCH.md` 02 절: WHATWG, llama.cpp, vLLM, LM Studio, OpenRouter Streaming, OpenRouter Reasoning Tokens 적용 위치 갱신.
- `docs/DECISIONS.md`: 9번(빈 줄 없는 마지막 이벤트) 상태 갱신. 구현 중 새 결정이 생기면 추가한다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/03/*` 상태 줄 갱신.

## 잔여

- 잔여 없음 (2026-11-25). 아래 항목은 모두 해소·채결됐고 새 잔여는 없다. 해소 근거는 각 절의 기록과 `docs/tasks/02/03/T7-structure-check.md`·`T8-structure-fix.md` 라운드 절.
- SRP 분리 (T7로 해소, 2026-11-25): `crates/snap-core/src/llm/delta/parse.rs`(221줄, 책임 2개)와 `crates/snap-core/src/llm/summary.rs`(186줄, 책임 3개)를 책임별 파일로 이동. 산출: `delta/chunk_error.rs`·`llm/redacted.rs`·`llm/redact.rs` 신규 (당시 `sse/` 경로는 T8 라운드 1 평면화로 `llm/` 한자리에 이동), `parse.rs`·`summary.rs`는 각 책임만 유지, `delta/merge.rs`(123줄)는 델타 누적 한 책임으로 유지. 근거: `harness/PROJECT_RULES.md` 개발 규칙 "개발 파일 100줄 초과 시 SRP 분리 검토" (기준은 파일 전체 줄 수), 검토 표는 `docs/tasks/02/03/T6-docs.md` 라운드 1.
- 구조 점검 위반 (T7 전수 점검에서 발견, 2026-11-25, T8에서 해소): 아래 5건. 표와 검토 기록은 `docs/tasks/02/03/T7-structure-check.md` 라운드 1. 해소 기록은 `docs/tasks/02/03/T8-structure-fix.md` 라운드 1.
  - `crates/snap-core/src/llm/mod.rs`: `llm/` 아래에 `sse/` 하나만 있어 파일 하나를 폴더로 감싼 자리. `llm/sse/`를 `llm/` 한자리로 평면화 (T8 해소: `llm/mod.rs`가 모듈 선언·재수출 담당, 하위 폴더는 `delta/`·`lines/`만 유지).
  - `crates/snap-core/tests/keys/main.rs`: `tests/keys/`에 `resolve.rs` 하나만 있어 파일 하나를 폴더로 감싼 자리. `tests/keys.rs` 한 파일로 평면화 (T8 해소).
  - `crates/snap-core/tests/sse/lines.rs`(186줄): 줄 처리 형식과 조각 경계가 한 파일에 몰림. `multibyte_char_across_chunk_boundary_is_restored`는 `boundaries.rs` 책임과 중복 → 제거하고 `boundaries.rs`로 이관 (T8 해소).
  - `crates/snap-core/tests/sse/error/redact.rs`(218줄): 가림 판정과 길이 상한이 한 파일에 몰림 → `error/redact.rs`(가림 판정)와 `error/limit.rs`(길이 상한)로 나눔 (T8 해소).
  - `crates/snap-core/tests/sse/delta/usage.rs`(182줄): usage 읽기와 `finish_reason` 되풀이 중복 방지가 한 파일에 몰림 → `delta/usage.rs`(읽기)와 `delta/finish.rs`(중복 방지)로 나눔 (T8 해소).
- (채결 완료, 잔여 아님) 상수 자리 (T5 2차 리뷰 참고): `crates/snap-core/src/llm/stream.rs`의 `ERROR_KEY`·`MESSAGE_KEY`는 JSON 키 이름이라 locale 대상이 아니다. `stream.rs` 유지로 채결 (판단 근거: `docs/tasks/02/03/T6-docs.md` 라운드 1). 현재 두 상수는 T8 라운드 7 보정으로 `crates/snap-core/src/llm/summary.rs`에 있다.
- 타입·로직 동거 위반 (T8 라운드 4에서 발견, 2026-11-25, T8 라운드 5에서 해소): 타입 정의 파일에 로직 함수를 두면 위반이라는 규칙에 걸린 미커밋 파일 3종을 타입 파일과 로직 파일로 분리. 근거: `harness/PROJECT_RULES.md` 타입 분리 규칙, 기록: `docs/tasks/02/03/T8-structure-fix.md` 라운드 4·5.
  - `llm/lines.rs` → `llm/lines/reassemble.rs`(타입) + `llm/lines/split.rs`(로직)
  - `llm/state.rs` → `llm/lines/events.rs`(타입) + 로직
  - `llm/stream.rs` → 타입 파일 + 로직
  - 최종 자리 (T8 라운드 7 보정): 로직 함수 2종(`data_payload`·`stream_error_text`)을 각각 `llm/lines/split.rs`와 `llm/summary.rs`에 병합하고 함수 1개 전용 파일(`lines/payload.rs`·`stream/error_text.rs`)을 삭제. `llm/stream.rs`는 타입 파일로 한자리 유지. 근거: PROJECT_RULES "함수 1개 전용 파일 금지", 기록: `docs/tasks/02/03/T8-structure-fix.md` 라운드 7.
