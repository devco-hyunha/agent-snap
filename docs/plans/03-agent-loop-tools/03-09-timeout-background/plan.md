# 03-9 timeout→백그라운드·주기 wake: 상세 기획

- 상태: 완료 (2026-11-18 착수 승인, AC 확정. 2026-11-19 T1 8차 재리뷰 PASS(t1-r8, 지적 0건). 검증 3종 exit 0, 시험 483개(lib 326·통합 157) 통과. 리뷰: docs/reviews/03/03-09-timeout-background/t1-r8.md. `docs/tasks/03/03-09-timeout-background/T1-timeout-background.md` 상태 완료로 함께 갱신.)
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md), 선행: [03-6](../03-06-run-command/plan.md), [03-7](../03-07-process-tree/plan.md)

## 목적
실행 deadline이 만료된 명령을 끝내지 않고 백그라운드로 옮기고, 주기적으로 wake해 결과를 확인하는 경로를 정한다. 백그라운드 이동은 03-7이 정한 프로세스 트리 묶음을 그대로 유지한 채로 이어진다.

## 범위(In)
- deadline 만료 후에도 명령을 살려 두고, wake마다 표준 출력·표준 오류와 종료 코드를 확인하는 경로를 설계한다.
- wake 주기를 설정 키 `agent.wake_interval_seconds`(기본 30초)로 정한다. 0·음수·무한대는 거부한다.
- 03-6의 600초 deadline이 wake와 이어지는 경로를 확인한다. 값과 판단을 한자리에 두고 wake 경로를 추가한다.
- 완료 결과를 03-2의 도구 결과 규격으로 모델에 전달한다(한 번 깨우고 다음 wake는 그 다음 턴에서 확인한다).
- 백그라운드 명령이 프로그램 종료 시 03-7 경로로 정리됨을 확인한다.

## 비범위(Out)
- 백그라운드 명령이 대화형 세션에서 이어지는 상태 관리: 04 배치.
- L2·L3 자동화 단계·세션 동안 허용·Prospero식 기본 무인: 04 배치(D22).
- 백그라운드 명령 승인·종료 코드를 모델이 받는 방식의 자동 재시도: L3로 읽으므로 두지 않는다.

## 선행 세부 작업
[03-6](../03-06-run-command/plan.md), [03-7](../03-07-process-tree/plan.md).

## 리서치 및 설계 근거
- Prospero(RESEARCH Prospero 절): 셸 timeout 후 비동기 백그라운드와 주기 wake. Prospero `job_timeout` 1800초를 초과하면 종료한다(03-6의 600초보다 절댓값이 크다). wake 주기와 상태 목록은 03-9 설계 참고로만 쓰고, 03-6 값을 바꾸지 않는다.
- Prospero는 wake마다 명령 상태를 목록으로 보여주고, 출력은 파일에 쌓으며 읽기 요청으로 최대 2,000자만 넘긴다. 03-9는 03-2의 도구 결과 잘림(4,000자)을 재사용한다.
- Prospero `cancel_job`, `job_status`, `jobs` 도구는 사용자(사람)가 쓰는 명령 창구이고, snap의 백그라운드 확인을 사람이 제어하는 방식은 04 배치.
- Prospero는 프로그램 종료에 앞서 백그라운드 명령을 종료한다. 03-7 Job Object·process group이 같은 정책을 이미 만족한다.
- Claude Code(2026-11-18 `Interactive mode` 문서 직접 열람): timeout에 닿은 명령을 종료하지 않고 백그라운드로 옮긴다. 출력은 파일에 쌓고 Read로 읽게 하며, 종료 코드는 다음 요청에서 알린다. 프로그램 종료에 백그라운드 작업을 정리한다. 03-9는 출력을 파일에 두지 않고 wake 결과로만 넘기고, 종료가 03-7 경로임을 AC로 확인한다.
- 상세: [research/03-agent-loop-tools.md Prospero 절과 03-9 wake 설계 비교 절](../../../research/03-agent-loop-tools.md#prospero-코딩-하네스-벤치마크), 정책 정본: [D22 4.3절](../../../decisions/D22-execution-reliability-autonomy.md).

## 리스크
wake 확인이 에이전트 루프 밖에 태스크를 새로 만들면 승인·비밀 파일 차단(D9·D17)과 cancel 경로가 한쪽으로 흔들린다. 승인·cancel·트리 종료를 03-7의 같은 창구로만 묶는 것을 AC로 확인한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 백그라운드 전환 | deadline 만료 후 백그라운드 이동·주기 wake·완료 결과 도구 결과 전달·wake 설정 3곳(읽기·기본값·예시 파일) | L3 무인 해석(D22), Prospero의 사용자 명령 창구(`jobs`·`cancel_job`) 제품화, 03-6 deadline 값 변경, 출력 파일 기록(Claude Code식) | 전환 경로·wake 상태·왕복 테스트 | cargo test(왕복·종료 정리), cargo fmt --check, cargo clippy --all-targets -- -D warnings | 03-6·03-7 |

## DoD
AC 충족, 검증 명령 3종 통과, `docs/reviews/03/`에 리뷰 PASS.

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-09-timeout-background/T1-timeout-background.md` | 백그라운드 전환 | 완료 (t1-r8 8차 재리뷰 PASS, 지적 0건·검증 3종 exit 0, 2026-11-19) |
