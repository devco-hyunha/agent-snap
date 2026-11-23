# 플레이북: 문서

결론: 문서·색인·포인터를 고치면 연결을 같은 턴(또는 즉시 후속)에 맞춘다.  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `harness/HARNESS.md` (1절 문서 연쇄)
2. `AGENTS.md`의 `## 프로젝트 오버레이`
3. 고칠 문서·연결 후보(검색으로 확정)
4. `.agents/skills/dev-principles/SKILL.md` (색인) — 호출 표 docs 지정 잎만 추가 Read. 문장 교정 요청이면 `.agents/skills/proofread-harness/SKILL.md`도.

## 지정 잎

색인 호출 표의 docs 행(comments · workflow). 보고: 지정 잎 목록 + 각 1줄.

## 단계

1. 연결 표·포인터·사본을 검색한다. 한쪽만 고치고 끝내지 않는다.
2. 추가·수정·삭제를 같은 턴에 한다. 못 끝내면 즉시 후속(같은 번호·다음 턴 전).
3. 보고에 반영 경로 목록을 남긴다. 후속 미완이면 DoD(완료 기준)·PASS 금지.
4. `.md`만 변경은 문서 연쇄 + 문서·언어 리뷰(주석만 분기 아님).
5. C(실행 증명): 반영 목록·후속 유무·지정 잎 보고.
