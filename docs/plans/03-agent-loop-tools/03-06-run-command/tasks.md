# 03-6 run_command·승인자 주입·deadline: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. `run_command` 도구를 만든다. 표준 출력·표준 오류·종료 코드를 받고, 4,000자 잘림(03-2 경로 공유)을 쓴다. (구현: `tools/run_command.rs`, 잘림은 루프의 `recorded`가 적용)
- [x] 2. 03-2 루프에 `run_command` 도구 등록을 추가한다. 도구 등록은 03-3와 같은 `Tool` 경로를 쓴다. (구현: `agent/record/registry.rs`의 `with_agent_tools`)
- [x] 3. 승인자 인터페이스를 만든다(CLI가 구현·도구가 인터페이스 호출). 기본 명령마다 확인. 확인: 승인자 인터페이스 호출 단위 테스트(spec AC 1). (구현: `tools/approval/approver.rs`, 시험: `tools/tests/approver.rs`·`tools/tests/run_command.rs`)
- [x] 4. 승인 응답 300초 deadline을 만든다. 미주입·비대화형·deadline 만료 시 거부로 처리한다. 확인: deadline 단위 테스트(spec AC 2). (구현: `request_approval`, 값: `config/read/agent.rs`)
- [x] 5. 실행 600초 deadline을 만든다. 만료 시 미완료로 결과를 전달한다. 확인: deadline 단위 테스트(spec AC 3). (구현: `run_command.rs`의 `select!`, 값: `config/read/agent.rs`)
- [x] 6. cancel 연결을 만든다. cancel 시 02의 cancel 토큰으로 연결한다. (구현: 승인 대기·실행 각각 `select!`, `runner.rs`의 `attach_cancel`)
- [x] 7. 03-3 헬퍼 경계 검사를 `run_command`에 사용한다. 확인: 경로 검사 단위 테스트. (구현: `is_secret_name` 재사용 보조 검사, 경계 밖 명령은 승인자(L1)가 막는 자리)
- [x] 8. `docs/RESEARCH.md` 작업 03 절 적용 위치를 갱신한다. (Prospero·opencode·Codex 절 적용 위치 갱신)

## 테스트 계획
- 승인자 인터페이스는 가짜 승인자(mock)로 승인·거부·만료·미주입·비대화형 상태를 만든다.
- `run_command`는 OS 셸을 사용할 수 있는 환경에서만 확인한다. 확인이 어려운 명령은 "실행 미확인"으로 표기한다.
- deadline 값은 설정으로 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 확인한다.

## DoD
- [x] [spec.md](spec.md) AC 6개 충족 (T1 라운드 보고에 AC별 확인 기록).
- [x] 검증 명령 3종 통과 (2026-11-11, exit 0 3건, 로그 `.local/scratch/gate03_t6_t1_cargo.log`).
- [x] `docs/reviews/03/` 리뷰 PASS (2026-11-12 t1-r4.md PASS 지적 0건, T2 분할은 t2-srp-r2.md PASS). T3·T4·T5는 `t3-srp-r4.md` PASS, `t5-integration-r2.md` PASS(2026-11-13, critical·major·minor 0건).

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절.
## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-06-run-command/T1-run-command.md` | T1 run_command·승인 | 완료 |
| `docs/tasks/03/03-06-run-command/T2-srp-split.md` | T2 SRP 분리(llm) | 완료 |
| `docs/tasks/03/03-06-run-command/T3-srp-tests-split.md` | T3 시험 SRP 분리 | 완료 |
| `docs/tasks/03/03-06-run-command/T4-srp-full-audit.md` | T4 전체 감사(결과 표 T4-audit-results.md) | 완료 |
| `docs/tasks/03/03-06-run-command/T5-srp-splits.md` | T5 감사 결과 R1~R4 분리 | 완료 |
| `docs/tasks/03/03-06-run-command/T6-folder-splits.md` | T6 agent·tools 하위 폴더 정리 | 완료 |
