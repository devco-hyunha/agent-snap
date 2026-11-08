# 03-6 run_command·승인자 주입·deadline: 상세 기획

- 상태: planned
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md), 선행: [03-2](../03-02-agent-loop/plan.md)

## 목적
명령 실행 도구 `run_command`와 승인자 인터페이스를 만든다. 실행·승인 deadline과 L1 기본(D22)을 지킨다.

## 범위(In)
- `run_command`: 셸 명령 실행, 표준 출력·표준 오류 수집, 종료 코드 전달, 출력 4,000자 잘림(03-2 공유).
- 승인자 인터페이스: CLI가 주입한다. M1 기본은 명령마다 확인(L1). 승인자 미주입·비대화형은 즉시 거부.
- 승인 응답 deadline 기본 300초(D22). 만료는 거부.
- 실행 deadline 기본 600초(D22). 만료는 미완료로 결과를 알린다.
- 명령 중단(cancel) 시 실행 중지·자식 정리로 이어진다.
- 작업 폴더 안에서만 실행한다(작업 폴더 밖 명령 거부).

## 비범위(Out)
- 프로세스 트리 종료(Job Object): [03-7](../03-07-process-tree/plan.md).
- L2·L3 자동화 단계·세션 허용 규칙: 04 배치.
- timeout→백그라운드: [03-9](../03-09-timeout-background/plan.md) 후보.
- 실행 격리(D9).

## 선행 세부 작업
[03-2](../03-02-agent-loop/plan.md).

## 참고 자료
- Prospero(RESEARCH Prospero 절): 셸·승인 deadline, 프로세스 트리 종료.
- DECISIONS D22(실행 600초·승인 300초·L1 기본), D9(격리 보류), D17(비밀 파일 우회 한계).

## 리스크
- 승인 timeout과 「무인 계속」을 혼동하지 않는다.
- `run_command`의 비밀 파일 읽기 우회는 이름 규칙으로 다 막지 못한다(스모크 보고에 한계 기록).

- `run_command`의 비밀 파일 읽기 우회는 이름 규칙으로 다 막지 못한다(스모크 보고에 한계 기록).
## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 run_command·승인자 | 명령 실행·출력 수집·종료 코드, 승인자 주입(CommandApprover L1), 승인·실행 deadline(300초·600초) | 샌드박스 격리(D9), L2·L3(D22) | 도구·승인자·단위/모의 테스트 | cargo test(승인 거부·deadline 만료), cargo clippy, cargo fmt | 03-2·03-3 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-06-run-command/T1-run-command.md` | run_command·승인자 | 계획 |