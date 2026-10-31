# 02-1 비밀 값 래퍼와 자격 파일 읽기 — 리뷰 2 (거리 review)

- 리뷰 세션: 구현 맥락 없는 읽기 전용 재리뷰 (직전 리뷰: [02-1-secrets-review-1.md](02-1-secrets-review-1.md))
- 재리뷰 범위 (하네스 문서 9절): 직전 리뷰 이후 변경분(`docs/DECISIONS.md`, `docs/plans/02-client/02-1-secrets/tasks.md`, `crates/snap-core/src/credentials.rs`)과 직전 리뷰의 미달 항목 4건만 확인. 직전 리뷰의 통과 항목(AC 1~5, 금지 규격, 검증 명령, 에러 처리·보안 기본, 범위 밖 변경 없음)은 재검토하지 않았다.
- 판정: **PASS** (critical 0건, major 0건, minor 2건)

## 1. 직전 미달 항목 4건 충족 근거

| # | 직전 지적 | 심각도 | 충족 여부 | 근거 |
|---|---|---|---|---|
| 1 | `docs/DECISIONS.md`에 `jsonc-parser` 라이선스 확인 기록 없음 | major | **충족** | `docs/DECISIONS.md:92-95`에 `## D18 JSONC 해석기로 jsonc-parser 0.34`로 결정/근거/다시 볼 점 3줄 형식이 기록됨. D10~D17과 같은 `## D<번호> 제목` + 결정/근거/다시 볼 점 형식이고, 라이선스 표기(crates.io MIT/Apache-2.0-1.0, 포장 LICENSE MIT)가 비밀 값 없이 적혀 있으며(D18:93) 비밀 값·엔드포인트 주소 노출은 없다. `Cargo.toml:7`의 `jsonc-parser = "0.34.0"`, `Cargo.lock:6-8`의 버전 0.34.0과 버전 표기도 일치. |
| 2 | `tasks.md` 단계 1~7 체크리스트 미반영 | minor | **충족** | `docs/plans/02-client/02-1-secrets/tasks.md:6-12`의 단계 1~7이 모두 `- [x]`로 갱신되고 각 단계에 통과한 테스트 이름(예: `secret_value_debug_and_display_hide_the_value`, `non_string_key_value_is_rejected_with_endpoint_name_only`)이 반영됨. DoD(`tasks.md:22`)도 `- [x]`로 11개 테스트 통과 근거가 적힘. |
| 3 | `credentials.rs:53-56` 최상위 객체 검증 경로 문서 주석 보강 | minor | **충족** | `crates/snap-core/src/credentials.rs:53-55`에 최상위가 객체가 아닌 경우(문자열·숫자·배열 등) `ApiKeysNotObject`로 거부하고 `take`로 엔트리를 소비한다는 주석 3줄 추가. 주석 내용과 실제 코드(`credentials.rs:56-62`)가 일치한다. |
| 4 | `credentials.rs:63`의 `api_keys_object.clone().take_inner()` 복제 경로 | minor | **충족** | `crates/snap-core/src/credentials.rs:59`에서 `root_object.take("api_keys")`로 최상위에서 직접 꺼내고, `credentials.rs:66`에서 `api_keys_object.take_inner()`로 복제 없이 소비. `credentials.rs` 전체 grep 결과 `clone` 0건. 동작 유지: 테스트 11개 통과(`cargo test -p snap-core`, 아래 3절 로그). |

## 2. 6항목별 결과 (재리뷰 대상분 기준)

1. **작업 문서 AC 충족 여부**: 충족. AC 1~5를 테스트 11개로 확인(직전 리뷰에서 통과 확인된 항목이고, 이번 변경이 동작을 유지함을 `cargo test -p snap-core`의 11 passed로 확인). `credentials.rs:97-215`의 테스트가 AC 1(주석·마지막 쉼표·이름 찾기), AC 2(`absent`), AC 3(`KeyNotString`), AC 4(`JsoncSyntax` 줄 번호만), AC 5(Display·Debug 가림)을 각기 커버.
2. **금지 규격 위반 여부**: 위반 없음. 지정 파일(`credentials.rs`, `secrets.rs`, `lib.rs`, `Cargo.toml`)과 문서 갱신(`docs/DECISIONS.md`, `tasks.md`, `plan.md` 상태 줄) 외 추가 파일 없음(git status 확인). 커밋·푸시 없음. `git diff --stat`에 범위 밖 항목(`config.rs`, `llm.rs`, `snap-cli`) 변경 없음.
3. **검증 명령 결과와 로그 근거**: 전부 통과 (3절 참조).
4. **에러 처리와 보안 기본 (dev-principles 2~10절)**: 충족. 변경분 한정의 확인: `clone` 제거로 메모리 복제 경로 제거(10절 성능 개선), `take`/`take_inner` 소비로 라이브러리 오류 타입 규칙(`thiserror`)·테스트 밖 `unwrap`/`expect` 없음·공개 항목 한국어 문서 주석 유지(12절 프로젝트 규칙). 오류에 값·줄 내용 미포함 경로는 코드 변경에도 그대로(`credentials.rs:56-73`).
5. **문서 동기화**: 충족. `docs/plans/02-client/02-1-secrets/plan.md:3`의 `- 상태: in-progress` 확인. `docs/DECISIONS.md` D18 기록으로 tasks.md 단계 1·7의 "결정 기록" 요구 충족. `docs/reviews/README.md` 형식(줄 머리에 `판정: PASS`, 파일 마지막 `판정:` 줄)을 이 기록이 준수.
6. **범위 밖 변경 여부**: 없음. `git diff --stat` 변경 6개 파일(`Cargo.lock`, `crates/snap-core/Cargo.toml`, `crates/snap-core/src/lib.rs`, `docs/DECISIONS.md`, `plan.md`, `tasks.md`) + 신규 `credentials.rs`·`secrets.rs`로, 1차 리뷰에서 확인된 범위와 같고 추가 변경은 주석·clone 제거·문서 기록뿐.

## 3. 검증 명령 로그 근거 (워크스페이스 루트 `C:\workspace\agent-snap`, 직접 실행)

- `cargo fmt --check`: 종료 부호 0 (출력 없음)
- `cargo clippy --all-targets -- -D warnings`: `Finished \`dev\` profile [unoptimized + debuginfo] target(s)` 후 경고 0, 종료 부호 0
- `cargo test -p snap-core`: `test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` (doc-test 0 passed; 종료 부호 0)

## 4. 문제 목록

| # | 파일:라인 | 심각도 | 내용 |
|---|---|---|---|
| 1 | `docs/DECISIONS.md:93` | minor | 라이선스 표기 문자열 `MIT/Apache-2.0-1.0`의 원문자 표기(`Apache-2.0-1.0` 부분이 표준 표기와 다른 여지)를 이 재리뷰에서는 crates.io 인덱스를 열어 독립 검증하지 못함. 표기 문자열 재확인 권고(판정에는 영향 없음). |
| 2 | `docs/reviews/02-client/02-1-secrets-review-2.md` (이 파일) | minor | 리뷰 기록 파일 이름이 `docs/reviews/README.md`의 `docs/reviews/<작업번호>-<순번>.md` 예시(`03-1.md`)와 달리 1차 리뷰와 같은 `<작업번호>-secrets-review-<순번>.md` 형식을 따름. 1차 기록과 일관되며 필수 `판정:` 줄은 형식 준수. |

## 5. 판정

- 심각도 critical: 0건 (보안 관련 critical 없음 → 사용자 보고 대상 없음).
- 심각도 major: 0건 (직전 major 1건 D18 기록 충족으로 해소).
- 심각도 minor: 2건 (위 1~2, 신규 경미한 항목).
- 하네스 문서 7절: minor만 있으면 PASS.

판정: PASS
