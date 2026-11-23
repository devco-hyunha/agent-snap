# 플레이북: 기획

결론: 승인된 작업 문서가 생기기 전에는 코드를 쓰지 않는다.  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `.agents/skills/plan-harness/SKILL.md`
2. `harness/HARNESS.md` (1·3·5절 포인터)
3. `AGENTS.md`의 `## 프로젝트 오버레이` (또는 docs/PROJECT_RULES.md)
4. 제품·기능 기획이면 `.agents/skills/project-plan-harness/SKILL.md`

## 단계

1. 메인에서 오버레이·PROJECT_RULES·INDEX·CLAUDE를 확인한다. Research Gate면 research 플레이북을 먼저 탄다.
2. 질문은 배치로 이어서(개수로 끝 아님·선택지 무제한). 목적·In/Out·AC·설계 요약이 가정 없이 채워질 때까지 묻고, 채우면 초안 → 승인 요청. 빈칸 가정·답만으로 구현 금지.
3. 작업 문서를 쓰고 승인을 받는다(5절 1번). 메인이 작업 문서·plan 플레이북/SKILL을 직접 읽은 뒤에만 기획·방향·승인·내용을 정한다. 서브 요약만으로 금지.
4. 문서·색인을 쓰면 연결 연쇄·반영 목록을 남긴다.
5. C(실행 증명): 작업 문서 경로·승인 대기/완료.
