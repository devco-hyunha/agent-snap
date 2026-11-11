# 03-5 AGENTS.md 읽기 조립: 상세 기획

- 상태: done (2026-11-11 구현 완료, 독립 리뷰 2차 PASS 지적 0건)
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md), 선행: [03-2](../03-02-agent-loop/plan.md)

## 목적
작업 폴더의 `AGENTS.md`를 읽어 시스템 프롬프트에 조립하는 최소 경로를 만든다. 03은 04 규칙 계층 설계에 앞서 최소 형태만 쓴다.

## 범위(In)
- `AGENTS.md` 읽기: 작업 폴더 최상위에서 `AGENTS.md`를 읽고, 있으면 시스템 프롬프트에 추가한다.
- 파일이 없거나 읽지 못하면 읽지 않은 상태로 둔다.
- 파일 내용 원문은 잘라 넣지 않는다.
- 03은 `AGENTS.md` 전문만 조립에 사용한다.
- 길이 표시(2026-11-11 사용자 승인): 지시 파일이 상한(기본 32 KiB, 설정으로 변경 가능)을 넘으면 원문을 잘라 넣지 않고 그대로 넣되, 조립된 프롬프트 끝에 "지시가 길다" 표시 한 줄을 붙인다. 잘라 넣는 동작은 어디에도 없다.

## 비범위(Out)
- 규칙 파일 여러 종류를 계층으로 조립하는 설계: 04 배치(D43·D45).
- 규칙 파일 목록·색인·메모리: 04 배치(D43·D45).

## 선행 세부 작업
[03-2](../03-02-agent-loop/plan.md)의 시스템 프롬프트 조립.

## 참고 자료
- 2307.03172(지시·규칙 앞머리 위치).
- Codex AGENTS.md, OpenCode agents, Cursor rules.
- DECISIONS D4(마커 블록 경로 실체화)는 하네스 마커 블록 내용과 이어 본다.
- 번역 전문: `.local/research/04/arxiv-2307.03172.kr.md`.

## 리서치 및 설계 근거 (2026-11-11 조사)
결론: 03-5의 최소 설계(읽기 → 규범 뒤에 원문 그대로 붙임, 잘라넣기 없음)는 네 에이전트 공식 문서의 실제 동작과 일치한다.
- Codex AGENTS.md: 매 실행마다 읽고 아래에서 위로 합쳐 앞머리에 둔다. `project_doc_max_bytes`(기본 32 KiB) 상한이 있다. https://developers.openai.com/codex/guides/agents-md
- Claude Code memory: CLAUDE.md·AGENTS.md를 세션 시작에 읽고, 파일마다 200줄 이하를 권한다. 긴 파일은 순응도가 떨어진다. https://code.claude.com/docs/en/memory
- Cursor rules: 규칙 내용을 모델 컨텍스트 앞머리에 넣는다. https://cursor.com/docs/context/rules
- OpenCode rules: AGENTS.md를 읽고, AGENTS.md가 있으면 CLAUDE.md는 읽지 않는다(단일 파일). https://opencode.ai/docs/rules/
- Lost in the Middle 확인: arXiv 2409.10372는 이 주제와 무관한 논문이다. 03-5의 앞머리 근거는 2307.03172가 맞고 어떤 문서에도 2409.10372를 인용하지 않는다.
- 채택: 규범 → AGENTS.md 순서(Codex Concat·Cursor 앞머리), 상한 초과 표시(Codex 32 KiB·Claude 200줄 근거, 사용자 승인 2026-11-11).
- 기각·04 배치로 넘김: 가장 가까운 AGENTS.md가 앞 지침을 덮어쓰는 계층 우선순위, 규칙 파일 색인·메모리(D43·D45). 03-5는 루트 단일 파일만 유지.

## 리스크
AGENTS.md가 길면 작은 모델의 컨텍스트를 소모한다. 2026-11-11 보정: 상한 초과 표시(범위 In)로 잘라 넣기 없이 표시만 붙이고, 컨텍스트 소모는 스모크(03-10)에서 측정한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 AGENTS.md 조립 | 작업 폴더 AGENTS.md를 읽어 시스템 프롬프트에 합침(규범 → AGENTS.md 순), 상한 초과 시 길이 표시 한 줄 | 규칙 파일·색인·메모리(04 배치), 잘라넣기 | 조립 경로·단위 테스트 | cargo test(조립 순서·3종 상태·길이 표시), cargo fmt | 03-2 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-05-agents-md/T1-agents-md.md` | AGENTS.md 조립 | 완료 |