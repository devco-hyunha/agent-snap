# 03-7 프로세스 트리 종료(Job Object 등): 상세 기획

- 상태: 완료 (T1 구현·보정 완료, 독립 리뷰 1차 미달(major 1·minor 4) → 보정 → 2차 재리뷰 PASS. 근거: `docs/reviews/03/03-07-process-tree/t1-r1.md`·`t1-r2.md`). 2차 판정 표기는 major·critical 0건 기준이며 남은 minor 1건(`t1-r2.md` 지적 1, 작업 문서의 파일:라인 표기)은 문서 표기만 고치는 보정으로 해소됐다. CI 매트릭스 4검사는 amend 보정 뒤 전부 초록이었고 PR #37로 `feature/agent`에 병합 완료(2026-11-14). 라운드 상세는 git 추적에서 제외된 로컬 리뷰 기록인 위 2종에 있다.
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md), 선행: [03-6](../03-06-run-command/plan.md)

## 목적
`run_command`가 만든 명령과 자식 프로세스를 취소하거나 프로그램을 종료할 때 정리합니다.

## 범위(In)
- Windows: Job Object에 자식을 넣어 한 번에 끝낸다(우선).
- Unix: process group에 자식을 넣어 한 번에 끝낸다.
- 명령 중단과 프로그램 종료 시에 연결된 프로세스를 함께 끝냅니다. 03-7 구현 당시에는 실행 시간이 지나도 같은 정리 경로를 사용했습니다.
- 플랫폼별로 끝내지 못한 자식이 없는지 테스트로 확인한다.
- OS 고유 코드는 #[cfg(windows)]·#[cfg(unix)]로 게이트를 채운다(PROJECT_RULES 개발 규칙).

## 비범위(Out)
- 실행 시간 상한이 지난 명령의 백그라운드 전환은 [03-9](../03-09-timeout-background/plan.md)에서 완료했습니다(2026-11-19).
- 실행 격리(D9).
- Windows가 아닌 경로 검증은 Linux CI 매트릭스로만 확인한다(PROJECT_RULES 머지 규칙).

## 선행 세부 작업
[03-6](../03-06-run-command/plan.md).

## 참고 자료
- Prospero(RESEARCH Prospero 절): 프로세스 트리 종료(Job Object).
- DECISIONS D22(프로세스 트리 정리), PROJECT_RULES 개발 규칙(두 OS 이식성).

## 리스크
Windows Job Object가 자식을 트리에 담는 상속 순서와 그룹 밖으로 나가는 경로(breakaway)는 2026-11-14 MSDN과 로컬 실측으로 확인을 마쳤다. MSDN 기준 상속은 Job Object를 셸보다 먼저 만들 때 성립하고, breakaway는 Job에 확장 플래그를 걸 때만 열린다(우리 Job Object는 두 플래그를 걸지 않는다). 조상은 우리 경로가 아니다. 근거: RESEARCH 03-7 실측 조사 절, `tools/tests/process_tree.rs`.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 트리 종료 | deadline·cancel 지점에서 프로세스 트리 종료(Windows Job Object 우선) | 격리(D9)와 목적 혼용 금지 | 종료 헬퍼·종료 테스트 | cargo test(자식 잔존 없음), cargo fmt | 03-6 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-07-process-tree/T1-process-tree.md` | 트리 종료 | 완료 |
