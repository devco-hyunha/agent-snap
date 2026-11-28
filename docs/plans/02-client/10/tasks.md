# 02-10 작업 02 전체 리뷰: 작업 계획

- 상태: 진행 (기획 승인 2026-11-28. `feature/client`에서 나눈 `feature/client-review`에서 착수)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo deny check`(게이트 4종). Windows 로컬과 WSL(Ubuntu) 양쪽.
- 브랜치: `feature/client`에서 나눈 `feature/client-review`에서 진행한다.

## 단계

### T1 계획-코드 정합 감사
- 상태: 완료 (r1 감사 반영. 독립 리뷰 r1 미달 minor 1건 → 표 근거 열 보정 후 닫음)
- 범위: 02-1~02-9 계획 문서의 범위·AC 항목을 코드·시험과 대조해 항목별 확인 근거와 차이를 표로 남긴다. 계획 간 경계(02-2 검증 규칙과 02-9 저장 검증 재사용, 02-5 전송과 02-6 재시도, 02-7 선택 키와 02-8 `context_window`)를 먼저 본다.
- 금지: 코드 수정, 계획 재설계, 집계 명령만으로 감사 끝내기.
- 확인: 표에 대상 항목 전부를 담는다. 표에 없는 항목은 미점검으로 본다.
- 검증: 게이트 4종 Windows·WSL(Ubuntu) 양쪽.
- 산출물: `docs/tasks/02/10/T1-plan-code-fit.md`의 정합 감사 표

### T2 하네스·프로젝트 규칙 감사
- 상태: 완료 (r1 감사 표 작성. 독립 리뷰 r1 미달(major 1·minor 2) → 보정 후 닫음)
- 범위: 하네스 1·5·7절과 PROJECT_RULES 개발 규칙·문서 규격을 02 계획 범위 코드·문서에 견줘 위반을 표로 남긴다.
- 금지: 코드 수정.
- 확인: 규정마다 대상 경로와 위반 여부를 적는다.
- 검증: 게이트 4종 Windows·WSL(Ubuntu) 양쪽.
- 산출물: `docs/tasks/02/10/T2-harness-rules.md`의 규칙 감사 표

### T3 코드 리뷰
- 상태: 완료 (r1 코드 리뷰 표 3축 채움·지적 minor 4건 → 잔여 R5~R7 이관. r2 미달 minor 2 → 표 보정 후 닫음. 리뷰 기록 `docs/reviews/02/02-10-T3-r1.md`·`docs/reviews/02/02-10-T3-r2.md`)
- 범위: review-harness 체크리스트 7항목과 dev-principles 2~11절을 02 계획 범위 코드 전체에 적용해 지적을 심각도와 함께 남긴다. 비밀 가리기는 키 값·`base_url` 호스트 미노출 시험 경로를 근거로 확인한다.
- 금지: 코드 수정, 리뷰 없는 PASS.
- 확인: 축별 확인 항목과 지적을 표로 남긴다.
- 검증: 게이트 4종 Windows·WSL(Ubuntu) 양쪽.
- 산출물: `docs/tasks/02/10/T3-code-review.md`의 코드 리뷰 표

### T4 02 기획 AC 확정
- 상태: 완료 (1라운드 AC 확인 표 작성. AC 1·2·3 충족, AC 4 미충족(minor 4건 잔여 이관). 02 기획 상태는 `진행` 유지)
- 범위: 02 기획 AC 1~4를 항목별로 확인하고 근거(시험 경로·확인 명령)와 함께 기록한다. 02 기획 상태를 확정한다.
- 금지: 새 AC 추가, 코드 수정.
- 확인: AC마다 충족 여부와 근거 경로를 적는다.
- 검증: 게이트 4종 Windows·WSL(Ubuntu) 양쪽.
- 산출물: `docs/tasks/02/10/T4-plan-ac.md`의 AC 확인 표, `docs/PLAN.md`·`docs/plans/02-client.md` 상태 갱신

### T5 잔여 보정
- 상태: 진행 (R1·R2·R4 완료, R3 기록 유지, R5·R6·R7 완료(2라운드 게이트 3종 Windows·WSL 통과). 재리뷰 잔여)
- 범위: T1~T4가 밝힌 결함을 02-10 잔여로 이관해 보정하고 재리뷰 PASS를 받는다.
- 금지: 새 계획 번호 분리, 푸시·병합, 파일 삭제 시 대상·이유·근거 없이 진행.
- 확인: 잔여마다 대상·사유·할 일과 완료 기록을 남긴다.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.
- 산출물: `docs/tasks/02/10/T5-residual-fixes.md`의 잔여 이관 목록과 보정 기록

## 잔여 (T1~T4 감사 결과 이관분)

- 상태: 진행 (T1 r1 minor 1건은 표 근거 보정으로 닫음. T2 r1 보정 후 잔여 4건 이관 → R1·R2·R4 보정 완료(Fast-track). R3은 기록만 유지. T3 r1 minor 4건 → R5·R6·R7로 이관 → R5·R6·R7 보정 완료(2라운드). T4 완료: AC 1·2·3 충족, AC 4는 R5·R6·R7 해소로 근거가 생겼고 충족 여부는 재리뷰에서 판단. 리뷰 기록 `docs/reviews/02/02-10-T1-r1.md`·`02-10-T2-r1.md`·`02-10-T3-r1.md`·`02-10-T3-r2.md`)

| 번호 | 대상 | 사유 | 할 일 |
|---|---|---|---|
| R1 | `crates/snap-core/src/consts/defaults.rs`의 `DEFAULT_OLLAMA_BASE_URL`·`DEFAULT_LMSTUDIO_BASE_URL` | 오버레이 "엔드포인트 주소를 코드와 문서에 넣지 않는다"와 결정 19번 "자동 발견 기본 주소 상수"가 충돌 | 사용자 확인으로 정본을 정한다 (완료: 결정 19번 유지로 오버레이에 예외 명시, `harness/PROJECT_RULES.md`·`docs/DECISIONS.md` 19번 갱신) |
| R2 | `crates/snap-core/tests/client/retry_wrapper/boundary.rs`(186줄)·`crates/snap-core/tests/client/error/values.rs`(106줄) | 100줄 초과 SRP 판단 근거 기록이 없음 | 02-6 T7 SRP 근거 절에 판단 근거를 남긴다 (완료: 책임 각각 하나로 판단, 코드 변경 없이 `docs/tasks/02/06/T7-structure-fix.md`에 근거 기록) |
| R3 | `agent-snap.example.jsonc` 0자 빈 파일 | 사람이 읽는 예시 파일이 빈 파일. 02-8 계획이 "손대지 말 것"으로 정한 자리 | 기록만 유지하고 손대지 않는다 (유지: 02-8 계획 금지 유지로 코딩·삭제 안 함) |
| R4 | `docs/plans/02-client/08/plan.md` | `## 작업계획`·`## 작업 파일` 절 누락 | 문서 규격 절로 정정한다 (완료: `## 작업계획`·`## 작업 파일` 절로 교체. T-파일 상태 줄 1건(T3)도 함께 정정) |
| R5 | `crates/snap-core/src/llm/retry/system_random.rs`의 `SystemRandomSource` + `next_state`·`accepted_count`·`epoch_nanos` | 타입 정의 파일에 로직 함수 동거 (PROJECT_RULES 타입·로직 분리) | 타입 정의와 로직 함수를 나눠 정리하고 옮긴 자리 설명을 함께 고친다 (완료: 로직 함수를 `crates/snap-core/src/llm/retry/mixing.rs`로 이동, `system_random.rs`는 타입과 `RandomSource` 구현만 유지. 상세 `docs/tasks/02/10/T5-residual-fixes.md` 1라운드) |
| R6 | `crates/snap-core/src/llm/client/model/list.rs`의 `ModelsAttempt` + `list_models`·`models_decision` / `crates/snap-core/src/llm/delta/merge.rs`의 `DeltaState` + `apply_tool_call`·`new_tool_call`·`keep_first_id`·`append_text`·`apply_usage` | 필드만 담는 타입과 로직 함수의 한 파일 동거 (PROJECT_RULES 타입 분리 규칙) | 타입 정의와 로직 함수를 나누고 옮긴 자리 설명을 함께 고친다 (완료: `ModelsAttempt`를 `crates/snap-core/src/llm/client/model/attempt.rs`로, `DeltaState`를 `crates/snap-core/src/llm/delta/state.rs`로 이동하고 계산은 `merge.rs`가 맡음. 상세 `docs/tasks/02/10/T5-residual-fixes.md` 1라운드) |
| R7 | `tests/client/mock_request.rs`의 `const KEY` / `tests/client/error/redaction.rs`의 `const KEY` / `tests/client/build/materials.rs`·`tests/model/materials.rs`·`tests/selection/material.rs`의 `const KEY` | 같은 성격의 시험 키 재료 상수를 다섯 곳이 각각 정의 (DRY). 02-5 잔여 4는 값 재료 정본만 통합 | 시험 키 재료 정본 자리를 정하고 재사용으로 통일한다 (완료: 본문이 직접 정의한 2종만 각 묶음 재료 파일로 옮겨 `pub(crate)` 재사용. 나머지 3종은 각 시험 묶음 재료 파일이 정본이고 값이 서로 다른 요구의 재료라 통합 대상 아님. 상세 `docs/tasks/02/10/T5-residual-fixes.md` 2라운드) |

## 테스트 계획

- 새 시험을 만들지 않는다. 기존 시험 트리 22개 묶음(455건)을 게이트로 재실행해 근거로 쓴다.
- 가리기는 기존 `tests/*/hidden.rs`·`tests/save/redact.rs` 경로를 근거로 삼는다.
- 네트워크는 쓰지 않는다. 시간은 쓰지 않는다.

## DoD

- [spec.md](spec.md)의 AC 6개를 모두 확인한다.
- 게이트 4종 Windows·WSL 통과, PR CI 매트릭스 초록.
- T1~T4 지적이 0건이거나 02-10 잔여 보정 PASS로 닫힌다.
- 02 기획 AC 1~4가 확정되고 `docs/PLAN.md`·`docs/plans/02-client.md` 상태가 참이다.
- 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/reviews/02/`: 02-10 감사·리뷰 기록.
- `docs/RESEARCH.md` 02 절: 02-10 조사 미적용 이유.
- `docs/PLAN.md`·`docs/plans/02-client.md`: 02-10 상태 갱신.
- `docs/plans/02-client/10/*`: 상세 기획·명세·작업 계획.
