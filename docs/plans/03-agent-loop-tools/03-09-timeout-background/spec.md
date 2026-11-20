# 03-9 timeout→백그라운드·주기 wake: 명세

- 상위 문서: [plan.md](plan.md), 상위 기획: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md)
- 상태: 완료 (2026-11-19). 검증과 최종 검토 기록은 [상세 기획](plan.md)을 따릅니다.

## 입력과 출력
- 입력: 03-6의 실행 시간 상한 만료(`tools/command/settings.rs`의 `run_limit`, 기본 600초), wake 주기 설정 `agent.wake_interval_seconds`(초 단위, 기본 30초).
- 출력: 백그라운드로 옮겼다는 표시, wake마다 확인한 명령 상태(실행 중·완료·종료 코드).

## 동작 규칙 (확정)
1. 실행 시간 상한(`run_limit`, 기본 600초)이 지나면 명령을 끝내지 않고 백그라운드로 옮긴다. 03-7 `ProcessTree` 묶음을 그대로 유지하고, 묶음에 시간 상한을 새로 걸지 않는다.
2. wake 주기(기본 30초)마다 03-7 트리의 종료 코드를 확인하고, 새 출력이 있으면 03-6에서 모아 둔 표준 출력·표준 오류를 이어서 읽는다.
3. wake마다 확인한 결과를 03-2의 도구 결과(`ToolOutcome::Done`)로 모델에 전달한다. wake 한 번은 에이전트 턴 하나와 대응한다.
4. 완료되면 종료 코드와 마지막 표준 출력·표준 오류를 도구 결과 하나로 전달하고 백그라운드 목록에서 뺀다.
5. 백그라운드 목록 등록·해제와 마지막 상태는 03-2 기록 자리(`ToolContext`)에만 남긴다. 프로그램 로그 파일 저장은 04 배치다.
6. `run_command` 승인 정책(L1)은 그대로 둔다. 백그라운드로 옮긴다고 승인을 자동으로 넘기지 않는다(D22 L3 금지).
7. 백그라운드 명령은 03-7 `ProcessTree`의 `Drop`·cancel·프로그램 종료 경로를 그대로 지난다.
8. wake 주기 설정 키는 `agent` 섹션의 `wake_interval_seconds`로 두고, 0·음수·무한대는 거부한다.

## AC (확정)
1. 실행 시간 상한 만료 → 백그라운드 등록 → wake 완료 결과를 받는 왕복을 모의 명령으로 확인한다(테스트).
2. wake 전에 모델에 결과를 알리지 않는다(옮겼다는 표시 외의 도중 결과는 만들지 않는다).
3. 프로그램 종료와 cancel에 백그라운드 트리가 03-7 종료 경로로 정리됨을 확인한다(테스트).
4. `agent.wake_interval_seconds`가 설정 읽기·기본값·예시 파일 3곳에 자리한다. 값 30초 확인과 `0`·음수 거부를 테스트로 확인한다. 저장은 02-8 범위가 엔드포인트·모델이라 이 항목은 저장 대상에서 뺀다.
5. 검증 명령 3종 통과(공통 DoD).

## 테스트 계획
- OS 셸에서 확인이 어려운 명령은 "실행 미확인"으로 표기한다.
- wake 값은 짧게 주고, 경과 확인은 하한·상한으로만 한다.
- wake 간격보다 짧은 출력을 낸 뒤 상태를 확인한다.

## DoD
- [spec.md](spec.md) AC 5개 충족.
- 검증 명령 3종 통과.
- `docs/reviews/03/` 리뷰 PASS.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절(03-9 적용 위치 갱신).
- `docs/decisions/D22-execution-reliability-autonomy.md` 7절 매핑 표의 세부 작업 번호(03-4 → 03-9).
- `agent.wake_interval_seconds`는 `crates/snap-core/src/config/read/agent.rs`에서 읽습니다. 기본값은 `crates/snap-core/src/config/read/types/mod.rs`에 정의하고 `crates/snap-core/src/config/read/types/agent.rs`에서 적용합니다. 예시는 `agent-snap.example.jsonc`에 있습니다. [02-8 설정 저장 명세](../../02-client/02-08-config-save/spec.md)는 접속 주소와 모델을 다룹니다.
