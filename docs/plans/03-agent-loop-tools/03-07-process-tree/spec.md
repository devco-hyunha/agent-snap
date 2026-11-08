# 03-7 프로세스 트리 종료(Job Object 등): 명세

- 상위 문서: [plan.md](plan.md), 상위 기획: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md)

## 입력과 출력
- 입력: 03-6가 만든 명령 프로세스 트리와 종료·만료·cancel 신호.
- 출력: 트리 종료 확인 기록(남은 자식 없음), 상태 표시.

## 동작 규칙
1. Windows: Job Object에 자식을 넣는다. Job Object를 닫으면 트리가 끝나는지 확인한다.
2. Unix: process group을 만든다. process group에 시그널을 보내고, 끝까지 확인한다.
3. 프로그램 종료 시(AC 5)에 Job Object가 닫힌다.
4. 03-6가 만든 명령은 03-6의 03-7 헬퍼를 호출한다.
5. 자식이 그룹 밖으로 나가는 경로는 테스트로 표시한다.

## AC
1. deadline·cancel 시 명령 프로세스와 그 자식이 남지 않는다(테스트, 확인된 OS).
2. 프로그램 종료 시 Job Object가 닫혀 자식이 끝나 있다(테스트).
3. #[cfg(windows)]·#[cfg(unix)] 경로를 같은 커밋에서 확인한다(PROJECT_RULES).
4. 검증 명령 3종 통과(공통 DoD).
