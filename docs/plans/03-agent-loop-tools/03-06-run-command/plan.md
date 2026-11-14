# 03-6 run_command·승인자 주입·deadline: 상세 기획

- 상태: done (2026-11-12 T1 r4 리뷰 PASS·지적 0건, 검증 3종 통과. 2026-11-13 후속 SRP 분리 T2·T3·T5 완료 — 작업 단위 통합 리뷰 2차 PASS(critical·major·minor 0건, docs/reviews/03/03-06-run-command/t5-integration-r2.md). 2026-11-13~14 사용자 추가 지시로 T6 agent·tools 하위 폴더 정리 완료 — F1·F2 독립 리뷰와 F2 재리뷰 PASS(docs/reviews/03/03-06-run-command/t6-f1-review.md·t6-f2-review-2.md)와 minor 2건 보정, 언어 순수성 전수 정리(한자·깨진 글자 0건)까지 마쳤다. 이 작업 단위를 담은 브랜치 feature/agent-03-06-run-command를 origin에 푸시하고 PR #35로 feature/agent에 병합했다(2026-11-14, CI 매트릭스 ubuntu·windows 4검사 전부 초록 뒤 병합). 커밋 해시는 문서가 바뀌면 갱신 대상이라 박아두지 않는다.)
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
- L2·L3 자동화 단계·세션 허용 UI: 04 배치. 3단계 승인 규칙 표(allow/ask/deny)는 03-11에 배치(D43 2차 개정, 2026-11-09).
- timeout→백그라운드: [03-9](../03-09-timeout-background/plan.md) 후보.
- 실행 격리(D9).

## 선행 세부 작업
[03-2](../03-02-agent-loop/plan.md).

## 참고 자료
- Prospero(RESEARCH Prospero 절): 셸·승인 deadline, 프로세스 트리 종료.
- DECISIONS D22(실행 600초·승인 300초·L1 기본), D9(격리 보류), D17(비밀 파일 우회 한계).

## 리서치 및 설계 근거 (2026-11-11 조사)
- Prospero: 셸 timeout 후 백그라운드 이동(`job_timeout` 1800초), 미응답 승인은 timeout 후 거부(`ask_timeout` 60초). 03-6은 D22 값(실행 600초·승인 300초)을 그대로 두고, `0`(무한 대기)은 허용하지 않는다. timeout→백그라운드는 03-9 후보로 유지.
- tokio::process: `Child::kill`은 SIGKILL+대기, `kill_on_drop` 기본 false라 드롭만으로는 프로세스가 산다. `wait()`은 cancel-safe, stdout·stderr는 별도 태스크로 읽어야 파이프 데드락을 피한다. → 채택: `tokio::select!`+`timeout`+`kill`, 출력은 파이프 태스크 분리.
- Job Object: `TerminateJobObject`와 `KILL_ON_JOB_CLOSE`로 트리 종료, `BREAKAWAY_OK` 옵션은 트리를 깬다. → 03-7 배치로 유지, 03-6은 kill만.
- Codex: 승인(`approval_policy`)과 샌드박스는 별개 레이어, 승인 대기 timeout 수치 없음, 비대화형은 승인 없이 실행하는 경로를 별도 플래트로만 허용. → 03-6의 승인자 인터페이스 교체 지점(Codex reviewer식)과 비대화형 즉시 거부 유지.
- OpenCode: 규칙 미매칭 기본 `ask`, `once`/`always` 패턴 저장. → 3단계 패턴 허용은 03-11 배치로 유지, L1은 미매칭=거부(ask)로만.
- 출처: https://raw.githubusercontent.com/skeeto/prospero/master/docs/usage.md, https://raw.githubusercontent.com/skeeto/prospero/master/docs/windows.md, https://docs.rs/tokio/latest/tokio/process/struct.Child.html, https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects, https://learn.chatgpt.com/docs/agent-approvals-security, https://opencode.ai/docs/permissions/

## 리스크
- 승인 timeout과 「무인 계속」을 혼동하지 않는다.
- `run_command`의 비밀 파일 읽기 우회는 이름 규칙으로 다 막지 못한다(스모크 보고에 한계 기록). (위 한계는 한 번만 적는다 — 2026-11-11 문장 정리)

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 run_command·승인자 | 명령 실행·출력 수집·종료 코드, 승인자 주입(CommandApprover L1), 승인·실행 deadline(300초·600초) | 샌드박스 격리(D9), L2·L3(D22) | 도구·승인자·단위/모의 테스트 | cargo test(승인 거부·deadline 만료), cargo clippy, cargo fmt | 03-2·03-3 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-06-run-command/T1-run-command.md` | run_command·승인자 | 완료 |
| `docs/tasks/03/03-06-run-command/T2-srp-split.md` | SRP 파일 분할(리뷰 후속) | 완료 |
| `docs/tasks/03/03-06-run-command/T3-srp-tests-split.md` | SRP 시험 지원·본문 분할(리뷰 후속) | 완료 |
| `docs/tasks/03/03-06-run-command/T4-srp-full-audit.md` | SRP 전수 감사(읽기 전용) | 완료 |
| `docs/tasks/03/03-06-run-command/T4-audit-results.md` | 전수 감사 결과 표 | 완료 |
| `docs/tasks/03/03-06-run-command/T5-srp-splits.md` | SRP 분리 라운드(감사 major 반영) | 완료 |
| `docs/tasks/03/03-06-run-command/T6-folder-splits.md` | agent·tools 하위 폴더 정리(F1·F2) | 완료 |