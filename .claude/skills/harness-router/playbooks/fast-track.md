# 플레이북: Fast-track

결론: 조건에 맞으면 서브 없이 메인이 직접 고치고 셀프 체크로 PASS한다.  
경로: `.agents/skills/` 기준. Claude Code는 `.claude/skills/`로 치환.

## 강제 Read (≤4)

1. `harness/HARNESS.md` (3·9절 Fast-track)
2. `.agents/skills/build-harness/SKILL.md` (소규모 Fast-track)
3. 대상 파일 1개
4. `.agents/skills/dev-principles/SKILL.md` (색인) — 호출 표 fast-track 지정 잎만 추가 Read. 설치·동기화면 `README.md` 해당 절도.

## 지정 잎

색인 호출 표의 fast-track 행(dry · kiss-yagni · comments · workflow). 보고: 지정 잎 목록 + 각 1줄.

## 단계

1. 조건: ① 단일 파일·바뀐 줄 20줄 이하(사본 동기화 제외) 단순 수정, 또는 ② 하네스 설치·업데이트 동기화. 보안·비가역이면 Out.
2. 서브·별도 리뷰 세션을 열지 않는다. 메인이 직접 수행한다.
3. 인라인 검증(`python -I -c`)과 셀프 체크(AC·범위·해시·연쇄·지정 잎) 후 PASS를 판정한다.
4. 바뀐 줄 원문을 보고에 남긴다. 조건 이탈 시 plan/build 플레이북으로 돌아간다.
5. C(실행 증명): 조건 충족·검증·셀프 PASS·지정 잎 보고.
