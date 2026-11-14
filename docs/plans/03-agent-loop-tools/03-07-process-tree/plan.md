# 03-7 프로세스 트리 종료(Job Object 등): 상세 기획

- 상태: 완료 (T1 구현·보정 완료, 독립 리뷰 1차 미달(major 1·minor 4) → 보정 → 2차 재리뷰 PASS(critical·major 0·minor 1, `docs/reviews/03/03-07-process-tree/t1-r2.md`). CI 매트릭스 `ubuntu-latest`·`windows-latest` 4검사는 amend 보정 뒤 전부 초록으로 확인했다)
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md), 선행: [03-6](../03-06-run-command/plan.md)

## 목적
`run_command`가 만든 명령과 자식 프로세스를 deadline 만료·cancel·종료 시에 남기지 않고 끝낸다.

## 범위(In)
- Windows: Job Object에 자식을 넣어 한 번에 끝낸다(우선).
- Unix: process group에 자식을 넣어 한 번에 끝낸다.
- 명령 중단·deadline 만료·프로그램 종료 시에 트리를 끝낸다.
- 플랫폼별로 끝내지 못한 자식이 없는지 테스트로 확인한다.
- OS 고유 코드는 #[cfg(windows)]·#[cfg(unix)]로 게이트를 채운다(PROJECT_RULES 개발 규칙).

## 비범위(Out)
- timeout→백그라운드: [03-9](../03-09-timeout-background/plan.md) 후보.
- 실행 격리(D9).
- Windows가 아닌 경로 검증은 Linux CI 매트릭스로만 확인한다(PROJECT_RULES 머지 규칙).

## 선행 세부 작업
[03-6](../03-06-run-command/plan.md).

## 참고 자료
- Prospero(RESEARCH Prospero 절): 프로세스 트리 종료(Job Object).
- DECISIONS D22(프로세스 트리 정리), PROJECT_RULES 개발 규칙(두 OS 이식성).

## 리스크
Windows Job Object가 자식·조상 모두를 담는지, 자식이 그룹 밖으로 나가는 경로가 있는지 실측 확인이 필요하다. 세부 작업에서 OS 문서·실측으로 확인한다.

Windows Job Object가 자식·조상 모두를 담는지, 자식이 그룹 밖으로 나가는 경로가 있는지 실측 확인이 필요하다. 세부 작업에서 OS 문서·실측으로 확인한다.
## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 트리 종료 | deadline·cancel 지점에서 프로세스 트리 종료(Windows Job Object 우선) | 격리(D9)와 목적 혼용 금지 | 종료 헬퍼·종료 테스트 | cargo test(자식 잔존 없음), cargo fmt | 03-6 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-07-process-tree/T1-process-tree.md` | 트리 종료 | 완료 |