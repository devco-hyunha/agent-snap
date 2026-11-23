# 플레이북: 구현

결론: 승인된 작업만 구현한다. 표준은 구현 서브 → 독립 리뷰(순차).  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `.agents/skills/build-harness/SKILL.md`
2. `.agents/skills/delegation-brief/SKILL.md`
3. 승인된 작업 문서(해당 작업 번호 절)
4. `.agents/skills/dev-principles/SKILL.md` (색인) — 호출 표 build 지정 잎만 추가 Read

## 지정 잎

색인 호출 표의 build 행. 보고: 지정 잎 목록 + 각 1줄. 허위 「전체 확인」금지.

## 단계

1. 승인·Fast-track 여부를 가린다. Fast-track이면 fast-track 플레이북으로 간다. 메인 판단은 작업 문서·build 플레이북/SKILL 직접 읽기 후. 서브 요약만으로 승인·방향 금지.
2. 쓰기 전 `경로 → 책임 1줄` 표를 만든다. 예외: Fast-track·순수 문서/주석만은 표 생략 가능(보고에 예외 사유 1줄). 「단일 사용처라 한 파일」은 위반(한곳 정본 `leaves/solid.md`).
3. 위임 전 오버레이 경로를 채운다. 포인터 샌드위치(≤500자)로 구현 서브 하나만 연다.
4. 동시 둘 이상 서브 금지. 구현이 끝난 뒤 리뷰 서브를 순차로 연다.
5. 지정 파일만 수정. 검증은 `python -I -c`만. 임시 `_verify*.py` 금지.
6. C(실행 증명): 구현·인라인 검증·`파일→책임` 맵(또는 예외 1줄)·지정 잎 보고·형식 완료(리뷰는 review 플레이북).
