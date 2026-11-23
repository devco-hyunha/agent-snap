# 01 조사: 코딩 에이전트 동향 (2026-11)

## 핵심
2026년 11월 기준 코딩 에이전트 도구들의 설계 패턴을 조사했다. 참고 대상은 Codex CLI(Rust 로컬 에이전트), OpenCode(로컬 모델 지원·벤치 1위), Orca(병렬 에이전트 IDE), harness 벤치계측 자료다.

## 확인한 내용
- Codex CLI (Apache-2.0, Rust): 승인 대상을 명령 실행·stdin 쓰기·파일 수정·도구 호출·네트워크 접근 같은 동작 단위로 쪼개며, 기본 확인 방식은 요청 시 확인이고 만료 분기를 따로 둔다. "항상 허용" 승인 규칙을 규칙 파일에 한 줄로 추가하고 깃 기록으로 관리한다. 셸 도구 스펙에 `timeout_ms` 인자를 둔다.
- OpenCode (MIT): Ollama·LM Studio를 기본 포트에서 자동 발견하고, OpenAI 호환 URL을 커스텀 프로바이더로 등록한다. 하네스 벤치에서 정확도·비용 최상위다.
- Orca (MIT): 다른 에이전트 CLI를 git worktree별로 병렬로 돌리고 diff 리뷰·PR까지 이어주는 에이전트 IDE다. 에이전트 자체는 없다.
- harness-benchmark: 같은 모델을 써도 하네스 설계로 1회당 컨텍스트가 6,743~26,803까지 벌어진다. 과제 성공률은 같고 소비량만 차이가 난다.

## 링크
- https://openai.com/index/unrolling-the-codex-agent-loop/ (직접 열람 확인)
- https://dpsm.github.io/agent-harness-anatomy/codex/ (직접 열람 확인)
- https://github.com/openai/codex (직접 열람 확인)
- https://orca.dev/ · https://github.com/stablyai/orca (직접 열람 확인)
- https://faun.dev/toolbox/amp-coding-agent-vs-opencode/ · https://repoquarry.dev/guides/local-ai-coding-agents (직접 열람 확인)
- https://github.com/rajshah4/harness-benchmark · https://dev.to/supachai/same-model-11-harnesses-what-the-harness-leaderboard-v019-tells-engineers-3lpb (직접 열람 확인)

## 적용 위치
- 03 계획 승인·실행 절: 승인 요청을 동작 단위로 쪼개서 밝힌다 (격리 3단계는 자동화 단계 이후 검토로 미룸. 자동화 단계는 사람 확인을 거치지 않는 승인을 전제로 하되 격리는 나중에 정한다).
- 02 계획 모델 선택 절: 로컬 서버 자동 발견을 추가한다.
- 06 계획 보고서 지표: 모델 호출 수·세션당 소비량을 추가한다.
- 미적용: 다중 에이전트 병렬 조립(Orca 방식)은 snap이 혼자 쓰는 에이전트라 현 범위 밖. Tauri 앱은 PLAN.md의 "나중에 검토" 절에서 이 조사를 포인터로 쓴다.

## 상태
반영 (02·03·06 계획에 1줄씩 반영, 2026-11-23)
