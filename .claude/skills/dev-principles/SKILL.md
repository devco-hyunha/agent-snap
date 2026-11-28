---
name: dev-principles
description: 코드를 작성, 수정, 리뷰, 설계할 때 적용하는 보편적 개발 원칙 색인(SOLID, DRY, KISS, YAGNI, 관심사 분리, 에러 처리, 보안 기본). 플레이북 호출 표가 고른 잎만 읽는다. 잎은 자동 스킬이 아니다. 프로젝트 규칙과 충돌하면 프로젝트 규칙을 우선한다.
---

# 개발 원칙 (색인)

결론: 본문은 `leaves/` 잎 파일이다. 호출 표가 고른 잎만 Read한다. 잎·플레이북은 자동 스킬이 아니다.
용어: 한국어 서술은 「시험」(`testing`/`test`·잎 파일명·`testing PASS`·명령은 원문). 짧은 포인터는 하네스 문서 1절.

## 1. 적용 우선순위

- 프로젝트의 CLAUDE.md, AGENTS.md, docs 등 프로젝트 규칙이 있으면 그것을 먼저 따른다.
- 이 스킬은 프로젝트 규칙이 다루지 않는 부분의 기본 기준으로만 쓴다. 프로젝트 규칙 우선은 하네스 문서 1절을, 승인·안전 규칙은 하네스 문서 5절을 따른다. (하네스 문서: harness/HARNESS.md)

## 잎 목록

경로: `.agents/skills/dev-principles/leaves/` 기준. Claude Code는 `.claude/skills/`로 치환. 정본은 `skills/dev-principles/leaves/`(3벌 동기화).

| 절 | 잎 |
|---|---|
| 2 | solid.md |
| 3 | dry.md |
| 4 | kiss-yagni.md |
| 5 | separation.md |
| 6 | naming.md |
| 7 | errors.md |
| 8 | security.md |
| 9 | testing.md |
| 10 | comments.md |
| 11 | workflow.md |
| 12 | commit.md |
| 13 | constants-locale.md |

## 호출 표

| 플레이북 | 지정 잎 |
|---|---|
| build · review | solid · dry · kiss-yagni · separation · naming · errors · security · testing · comments · workflow · constants-locale |
| 커밋(메인) | commit |
| fast-track | dry · kiss-yagni · comments · workflow |
| docs | comments · workflow |
| plan · research · install-sync | 없음 |

## 보고

지정 잎 목록 + 각 1줄. 「전 항목 확인」거짓 보고 금지.
