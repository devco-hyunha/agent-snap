---
name: harness-router
description: 하네스 작업을 시작할 때 쓴다. 요청 종류를 보고 playbooks에서 하나만 고른 뒤 그 파일만 읽고 따른다. 조사·기획·구현·리뷰·문서·Fast-track·설치동기화 분기. 큰 규칙 뭉치를 먼저 읽지 않는다.
---

# harness-router

결론: 표에서 플레이북 하나만 고르고, 그 경로만 Read한다. 플레이북·잎은 자동 스킬이 아니다.

## 분기 표

| 신호 | 플레이북 |
|---|---|
| 조사·논문·벤치마크·학술 근거 | .agents/skills/harness-router/playbooks/research.md |
| 기획·작업 문서·승인 전 | .agents/skills/harness-router/playbooks/plan.md |
| 승인된 구현·보정 | .agents/skills/harness-router/playbooks/build.md |
| 리뷰·재리뷰·판정 | .agents/skills/harness-router/playbooks/review.md |
| 문서·색인·연쇄·교정 | .agents/skills/harness-router/playbooks/docs.md |
| 단일 파일·20줄 이하 또는 정본 동기화 | .agents/skills/harness-router/playbooks/fast-track.md |
| 설치·업데이트·복사 적용 | .agents/skills/harness-router/playbooks/install-sync.md |

경로: 표는 `.agents/skills/`를 Read한다. Claude Code만 `.claude/skills/`로 바꾼다. 저장소 정본은 `skills/harness-router/playbooks/`(3벌 동기화).

## 규칙

1. 표에서 신호에 맞는 행 하나만 고른다. 애매하면 사용자에게 한 줄로 묻는다.
2. 고른 플레이북 파일만 Read한다. 표 전체를 추측으로 대체하지 않는다.
3. 플레이북이 적은 강제 Read(상한 ≤4)만 추가로 연다.
4. 동시 둘 이상 서브 실행 금지. 순차 구현→리뷰는 허용한다.
5. 승인·안전 6원칙·언어·eli5는 always-on 핵을 따른다. 절차 장문은 플레이북·대스킬 경로로만 간다.
