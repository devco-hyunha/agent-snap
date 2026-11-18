# 코드 주석 짧게 정리: 작업 계획

- 상위: [plan.md](plan.md), [spec.md](spec.md)
- 검증: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (Windows + WSL Ubuntu)
- 기획 순번표에는 행을 넣지 않는다.

## 구현 단계

- [x] 1. T1 config 주석 정리. 확인: AC 1·3·4, 게이트 3종.
- [x] 2. T2 llm 주석 정리. 확인: 동일.
- [x] 3. T3 agent 주석 정리. 확인: 동일.
- [x] 4. T4 tools(execute_bridge 제외) 주석 정리. 확인: 동일.
- [x] 5. T5 execute_bridge 주석 정리. 확인: 동일.
- [x] 6. T6 tests 주석 정리. 확인: 동일.
- [x] 7. T7 cli_other 주석 정리. 확인: 동일 + 전수 잔여(라운드·보정 표기) 0건.

## DoD

- [x] spec AC 5개 충족 (AC 2 Win·WSL 동등은 게이트 3종 exit 0 + PR CI 매트릭스로 확인).
- [x] 모듈별 `docs/reviews/comment-shorten/` PASS (T1 1차, T2 2차, T3 3차, T4 2차, T5 1차, T6 2차, T7 2차).
- [x] 시험 수 불변 (전역 `#[test]` 265·`#[tokio::test]` 156 = src 217·97 + tests 48·59).
- [x] `docs/PLAN.md`·`docs/plans/03-agent-loop-tools.md` 세부 표 무변경.

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/comment-shorten/T1-config.md` | T1 config | 완료 |
| `docs/tasks/comment-shorten/T2-llm.md` | T2 llm | 완료 |
| `docs/tasks/comment-shorten/T3-agent.md` | T3 agent | 완료 |
| `docs/tasks/comment-shorten/T4-tools.md` | T4 tools | 완료 |
| `docs/tasks/comment-shorten/T5-execute-bridge.md` | T5 execute_bridge | 완료 |
| `docs/tasks/comment-shorten/T6-tests.md` | T6 tests | 완료 |
| `docs/tasks/comment-shorten/T7-cli-other.md` | T7 cli_other | 완료 |
