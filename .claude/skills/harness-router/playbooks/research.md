# 플레이북: 조사

결론: 조사만 한다. 코드·승인 전 구현은 하지 않는다.  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `.agents/skills/plan-harness/SKILL.md` (조건부 리서치 절)
2. `harness/HARNESS.md` (3절 Research Gate)
3. `AGENTS.md`의 `## 프로젝트 오버레이` (또는 docs/PROJECT_RULES.md)
4. (선택) 작업 문서가 있으면 그 경로의 `## 리서치 및 설계 근거`

## 단계

1. Research Gate 조건(사용자 요청·학술 근거·오버레이 필수)을 확인한다. 해당 없으면 이 플레이북을 쓰지 않는다.
2. 메인이 직접 검색하지 않는다. 읽기 전용 리서치 서브(표준 모델)에 맡긴다. 동시 둘 이상 서브 금지.
3. 보고는 출처 링크·핵심 수치·교훈·채택/기각만. 작업 문서 리서치 절에 남긴다.
4. C(실행 증명): Research Gate 확인·위임·기록 완료.
