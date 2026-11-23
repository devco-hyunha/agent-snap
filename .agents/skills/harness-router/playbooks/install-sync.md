# 플레이북: 설치·동기화

결론: README의 적용·업데이트 순서를 그대로 따른다. Fast-track이다.

## 강제 Read (≤4)

1. `README.md` (AI 자동 적용·업데이트)
2. `harness/HARNESS.md` (2절 구성·경로 표)
3. `templates/AGENTS.harness.md`
4. (Cursor면) `adapters/cursor/agent-harness.mdc`

## 단계

1. BASE·PROJ를 정하고, 적용·갱신 계획을 사용자에게 보고한 뒤 승인을 받는다.
2. harness·skills·마커·VERSION·어댑터를 README 순서대로 복사·병합한다. 오버레이는 보존한다.
3. OpenCode는 `.agents/skills`만. Claude Code는 `.claude/skills`도.
4. 해시·존재 확인 후 적용 위치 목록을 보고한다. 동시 멀티 서브 없이 메인이 직접 한다.
5. C(실행 증명): 복사·병합·검증·보고 완료.
