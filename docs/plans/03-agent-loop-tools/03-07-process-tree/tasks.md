# 03-7 프로세스 트리 종료(Job Object 등): 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. Windows Job Object 래퍼를 만든다(`#[cfg(windows)]`). CreateObject·AssignProcessToJobObject·TerminateJobObject 경로를 OS 문서로 확인해 쓴다. 확인: 자식 포함 트리가 끝나 남지 않는 것을 확인하는 통합 테스트(AC 1).
- [ ] 2. Unix process group 래퍼를 만든다(`#[cfg(unix)]`, setsid·killpg). 확인: 같은 확인 테스트(AC 1).
- [ ] 3. 03-6의 deadline·cancel 지점에서 트리 종료 헬퍼를 부르게 연결한다. 확인: 03-6 deadline 테스트가 트리 종료와 이어지는지 확인(AC 1).
- [ ] 4. 프로그램 종료 시 정리를 걸린다. 확인: 종료 후 자식 없음(AC 2).
- [ ] 5. Windows·Linux 경로를 같은 커밋에서 확인한다. Linux는 CI 매트릭스로 확인한다(PROJECT_RULES 머지 규칙).
- [ ] 6. `docs/RESEARCH.md` 작업 03 절 적용 위치를 갱신한다.

## 테스트 계획
- 트리를 확인하는 명령은 OS 셸에서 확인한다. 확인이 어려운 명령은 "실행 미확인"으로 표기한다.
- 값은 짧게 주고, 종료 확인은 하한·상한 표기로만 한다.

## DoD
- [ ] [spec.md](spec.md) AC 4개 충족.
- [ ] 검증 명령 3종 통과.
- [ ] `docs/reviews/03/` 리뷰 PASS.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절.
## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-07-process-tree/T1-process-tree.md` | T1 트리 종료 | planned |
