# 03-6 run_command·승인자 주입·deadline: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.
- `tools/`·`agent/`·`config/` 경로는 `crates/snap-core/src/` 기준입니다.

## 구현 단계
- [x] 1. `tools/command/tool.rs`의 `run_command`는 표준 출력·표준 오류·종료 코드를 받습니다. 출력은 `agent/record/record_fill.rs`에서 `truncate_tool_result`를 호출해 기본 4,000자로 자릅니다.
- [x] 2. 03-2 루프에 `run_command` 도구 등록을 추가한다. 도구 등록은 03-3와 같은 `Tool` 경로를 쓴다. (구현: `agent/record/registry.rs`의 `with_agent_tools`)
- [x] 3. `tools/approval/approver.rs`에 승인자 인터페이스를 둡니다. 기본값은 명령마다 확인하는 것입니다. 확인: `tools/tests/approver/request/reply.rs`와 `tools/tests/run_command/approval_gate.rs`(명세의 인수 조건 1).
- [x] 4. `tools/approval/request.rs`의 `request_approval`에서 승인 응답을 기다립니다. 승인자가 없거나 비대화형이면 즉시 거부하고, 300초 안에 응답이 없으면 거부합니다. 설정은 `config/read/agent.rs`에서 읽습니다. 확인: `tools/tests/approver/request/expiry_cancel.rs`(명세의 인수 조건 2).
- [x] 5. `tools/command/approve.rs`에서 실행 시간 상한 600초를 적용합니다. 03-9부터 시간이 지나면 `tools/background/registry.rs`의 목록에 등록합니다. 설정은 `config/read/agent.rs`에서 읽습니다. 확인: `tools/tests/run_command/deadline.rs`(명세의 인수 조건 3).
- [x] 6. `agent/loop/runner.rs`에서 `attach_cancel`로 중단 신호를 연결합니다. 승인 대기와 명령 실행에서 각각 `select!`로 신호를 받습니다. 확인: `tools/tests/approver/request/expiry_cancel.rs`와 `tools/tests/run_command/cancel.rs`.
- [x] 7. `tools/command/secret_name.rs`에서 기존 `is_secret_name`을 재사용합니다. 경계 밖 명령은 명령마다 확인하는 승인자 정책(L1)으로 다룹니다. 확인: `tools/tests/run_command/secret_names.rs`.
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
|---|---|---|
| `docs/tasks/03/03-06-run-command/T1-run-command.md` | T1 run_command·승인 | 완료 |
| `docs/tasks/03/03-06-run-command/T2-srp-split.md` | T2 SRP 분리(llm) | 완료 |
| `docs/tasks/03/03-06-run-command/T3-srp-tests-split.md` | T3 시험 SRP 분리 | 완료 |
| `docs/tasks/03/03-06-run-command/T4-srp-full-audit.md` | T4 전체 감사(결과 표 T4-audit-results.md) | 완료 |
| `docs/tasks/03/03-06-run-command/T5-srp-splits.md` | T5 감사 결과 R1~R4 분리 | 완료 |
| `docs/tasks/03/03-06-run-command/T6-folder-splits.md` | T6 agent·tools 하위 폴더 정리 | 완료 |
