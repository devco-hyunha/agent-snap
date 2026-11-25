# 플레이북: 리뷰

결론: 구현 맥락 없는 새 세션이 검증을 다시 돌리고 판정한다. 수정하지 않는다.  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `.agents/skills/review-harness/SKILL.md`
2. `.agents/skills/delegation-brief/SKILL.md` (리뷰 프롬프트)
3. 작업 문서(검증 명령·AC·해당 번호)
4. `.agents/skills/dev-principles/SKILL.md` (색인) — 호출 표 review 지정 잎만 추가 Read. 판정은 `harness/HARNESS.md` 7·9절.

## 지정 잎

색인 호출 표의 review 행. 보고: 지정 잎 목록 + 각 1줄. 허위 「전체 확인」금지.

## 단계

1. 전달받은 가드레일·규칙 경로를 먼저 읽는다. 산출물·diff를 직접 확인한다.
2. 검증 명령을 다시 실행한다. 서브 보고를 믿지 않는다.
3. 체크리스트·문서 연쇄·반영 목록·지정 잎·`파일↔책임`(또는 Fast-track·문서 예외)을 본다. 「단일 사용처라 한 파일」핑계의 신규 동거=major(한곳 정본 `leaves/solid.md`). 시험 파일 SRP·비대 혼재=major(「시험 파일이라 대상 아님」 PASS 금지). 플랫폼 분기 한 파일 동거=major. 지적 있으면 미달(minor만 있어도 PASS 아님).
4. 동시 둘 이상 서브 금지. 보정은 별도 구현 턴(순차).
5. C(실행 증명): 판정 한 줄·지적(경로·식별자·원문) 또는 지적 0·동일 지적(경로+내용)·지정 잎 보고.
