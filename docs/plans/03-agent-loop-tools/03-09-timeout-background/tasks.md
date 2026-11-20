# 03-9 timeout→백그라운드·주기 wake: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.
- 상태: 완료 (2026-11-18 착수 승인, AC 확정. 2026-11-19 T1 8차 재리뷰 PASS(t1-r8, 지적 0건). 검증 3종 exit 0, 시험 483개(lib 326·통합 157) 통과. 리뷰: docs/reviews/03/03-09-timeout-background/t1-r8.md)
- `tools/` 경로는 `crates/snap-core/src/` 기준입니다.

## 구현 단계
- [x] 1. 백그라운드 목록의 등록·해제·재등록을 `tools/background/registry.rs`와 `tools/background/state.rs`에서 관리합니다. 확인: `tools/tests/background/register.rs`와 `tools/tests/background/wake.rs`.
- [x] 2. `tools/check_background.rs`에서 설정한 간격(기본 30초)을 사용해 명령 상태를 확인합니다. wake 한 번은 에이전트 턴 하나에 해당합니다. 확인: `tools/tests/background/wake.rs`와 `tools/tests/background/check.rs`.
- [x] 3. 실행 시간 상한(`tools/command/settings.rs`의 `run_limit`)이 지나면 `tools/command/approve.rs`에서 백그라운드 목록에 등록하고 전환을 알립니다. 확인: `tools/tests/background/roundtrip.rs`(명세의 인수 조건 1).
- [x] 4. wake 결과를 03-2 도구 결과로 전달한다. 확인: 모의 서버 테스트. → `tests/agent_loop/background.rs`(check_background 호출→도구 기록 전달).
- [x] 5. 03-7의 프로세스 정리 경로를 사용합니다. 확인: `tools/tests/background/drop.rs`에서 목록이 해제될 때 손자 프로세스가 표시 파일을 만들지 않는지, `tools/tests/run_command/cancel.rs`에서 취소 후 목록에 등록되지 않는지 확인합니다(명세의 인수 조건 3).
- [x] 6. wake 주기 설정 키 `agent.wake_interval_seconds`를 03-6 읽기 경로에 추가한다. 0·음수·무한대 거부 확인. 설정 읽기·기본값·예시 파일 3곳을 함께 고친다. → `config/read/agent.rs`(읽기)·`config/read/types/agent.rs`·`config/read/types/mod.rs`(기본값 30)·`agent-snap.example.jsonc`(예시), `src/config/tests/agent.rs`(0·음수·무한대 거부).
- [x] 7. `docs/RESEARCH.md` 작업 03 절 적용 위치를 갱신한다. → 2026-11-18 코드 적용 위치로 갱신.

## 테스트 계획
- OS 셸에서 확인이 어려운 명령은 "실행 미확인"으로 표기한다.
- wake 값은 짧게 주고, 경과 확인은 하한·상한으로만 한다.
- 파이프가 살아 있는 상태에서 wake 주기보다 짧은 출력을 낸 뒤 확인한다.

## DoD
- [spec.md](spec.md) AC 5개 충족.
- 검증 명령 3종 통과.
- `docs/reviews/03/` 리뷰 PASS.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절.
- `crates/snap-core/src/config`의 `agent` 섹션 읽기·기본값·예시 파일(`wake_interval_seconds`).
- `docs/decisions/D22-execution-reliability-autonomy.md` 7절 작업 매핑 표 정합.
## 작업 파일
| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-09-timeout-background/T1-timeout-background.md` | T1 백그라운드 전환 | 완료 (t1-r8 8차 재리뷰 PASS, 지적 0건·검증 3종 exit 0, 2026-11-19) |
