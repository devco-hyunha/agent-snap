# 에이전트 기능 목록 (타 에이전트 분석, 2026-11-07)

- 확인 방법: Claude Code·Cursor·Codex CLI·OpenCode의 공식 문서와 Prospero 문서를 리서치 서브가 직접 열어 확인했다. 확인 날짜는 2026-11-07.
- 역할: agent-snap에 추가할 수 있는 기능을 카테고리별로 적고, 어디 플랜에 넣을지 한 줄로 매긴다. 이 문서는 기획 참고이며 규범 정본이 아니다. 결정은 `docs/DECISIONS.md`에 남긴다.
- 상태 값: 채택(현 플랜에 포함) | 후보(새 작업 문서 필요) | 보류 | 미채택
- 배치 규칙(2026-11-07 사용자 확정, D43): **작업 03이 M1 기본 도구의 전부를 담는다. 작업 03이 끝난 시점에 에이전트가 "코딩 한 바퀴"를 도는 데 도구가 부족해서는 안 된다. 나머지 도구·기능은 늦어도 작업 04(03 이후, 04 이전)에 배치한다.** 05·06 이후로 미루는 것을 금지한다. 05·06은 감시·실사용 검증이고, 기능 추가 자리가 아니다. 07-09는 판정 모델 플랜이다.

## (a) 기본 내장 도구

| 기능 | 근거 | snap 대응 |
|---|---|---|
| 파일 읽기·쓰기·편집(검색-치환) | Claude Code Read/Write/Edit, Codex apply_patch, OpenCode read/edit/write | **채택** 03: `read_file`·`write_file`·`edit_file` |
| 셸 명령 실행 | Claude Bash, Codex shell, OpenCode bash | **채택** 03: `run_command` (승인자 주입) |
| grep·glob·폴더 목록 | OpenCode grep/glob(ripgrep), Claude Grep/Glob | **채택** 03: `grep`·`glob`·`list_dir` |
| 웹 검색·웹 열람 | Claude WebFetch/WebSearch, OpenCode webfetch | **04 배치(D43)**: `web_search`·`web_fetch` 도구로 04에 추가 (M1 코딩 한 바퀴에 웹 도구는 필수 아님) |
| 노트 편집(Jupyter) | Claude NotebookEdit | **미채택**: 스냅 코딩 범위와 무관 |
| LSP 연동(진단·이동) | OpenCode LSP 도구 | **04 배치(D43)**: 심볼 도구 1차(D33)와 함께 04에서 백엔드 선택. 03은 1차 기본 7개 도구로 먼저 돌리고, 심볼 도구는 03 세부 기획에서 03 포함 여부를 함께 정한다(D33 다시 볼 점) |
| 계획 모드(to-do 목록) | Claude TodoWrite, Codex plan mode | **04 배치(D43)**: 04에 계획 모드 도구로 추가 |
| (a) 배치 보정 메모 (2026-11-08) | — | (a)의 기본 7개(read/write/edit/list_dir/glob/grep/run_command)는 03 채택 그대로. 웹·LSP·계획 모드 도구를 04에 두는 것은 "도구라는 이름" 때문이 아니라 코딩 한 바퀴에 필수인 7개와 필수 아닌 도구를 가른 것이다(D43 1항). 03 DoD는 7개 +α 확정 시 (a) 전체와 대조해 구멍이 없음을 확인한다(D43 다시 볼 점). |

## (b) 승인·권한·샌드박스

| 기능 | 근거 | snap 대응 |
|---|---|---|
| allow/ask/deny 3단계, 명령 패턴별 규칙 | OpenCode Permissions, Claude permissions | **04 배치(D43)**: 세션 허용 규칙 설계와 함께 04. L2·L3 UI는 D22대로 06 이후 (단, 단계 정의는 D22가 정본) |
| 샌드박스(read-only / workspace-write) | Codex approvals-security | **미채택** (D9: 경량 목표와 충돌) |
| 세션 동안 허용 | OpenCode 승인 창 | **04 배치(D43)**: 03-2에서 승인자 interface로 허용 지점까지 만들고, 세션 허용은 04에서 결정 |
| 체크포인트·되돌리기 | Claude checkpointing, Gemini checkpointing | **04 배치(D43)**: git 연동 방식으로 04 |

## (c) 스킬·커맨드

| 기능 | 근거 | snap 대응 |
|---|---|---|
| SKILL.md 스킬 (frontmatter·필요 시 로드) | Claude skills, OpenCode skills | **채택(하네스)**: agent-snap 하네스가 같은 형식 사용 중. 제품 내장 시 04 확장 후보 |
| 슬래시 커맨드(custom command) | Claude slash commands, OpenCode commands | **후보**: 04 대화형 CLI 확장( `/new`·`/models` 설계와 함께) |
| 상태 라인·출력 형식 | Claude statusline/output styles | **04 후보(교정, 2026-11-08)**: 상태 라인은 표시 전용(토큰 0)이라 04 CLI 표시에 저비용 후보. 출력 형식은 세션 전체 지시(D36 3번 풀·토글)와 겹쳐 04 하네스 층 설계와 함께 판단. (근거: [statusline](https://code.claude.com/docs/en/statusline) "runs locally and does not consume API tokens", [output-styles](https://code.claude.com/docs/en/output-styles) "style's instructions add input tokens") |

## (d) 메모리·프로젝트 지식

| 기능 | 근거 | snap 대응 |
|---|---|---|
| AGENTS.md 읽기 (작업 폴더 지시) | Codex AGENTS.md, OpenCode agents, Cursor rules | **채택** 04: 작업 폴더 `AGENTS.md` 읽기 (이미 범위(In)에 있음) |
| CLAUDE.md·`.cursor/rules`·notepets(사용자 메모리) | Claude memory, Cursor memory/notepets | **04 배치(D43)**: 규칙 파일 계층. 04 설계 시 우선순위를 D43 목록과 대조해 정한다 |
| 세션 요약 자동 메모리 | Claude memory auto | **04 배치(D43)**: 메모리 계층 설계(MemGPT·Mem0·HiMem·TierMem 근거, [조사 상세](11-rag-okf-memory-skills.md)) |
| RAG 검색(조각 검색)·OKF 지식 형식 | D36, [openknowledgeformat.com](https://openknowledgeformat.com) | **04 배치(D43)**: 검색은 grep·BM25 기본, 임베딩은 보조. 세션 임시 문서가 OKF·BPE의 상태 쪽. [조사 상세](11-rag-okf-memory-skills.md) |

## (e) 컨텍스트 관리·요약

| 기능 | 근거 | snap 대응 |
|---|---|---|
| `/compact` 자동 압축 | Claude /compact, Prospero /compact, OpenCode compaction, Gemini | **채택** 04: 1차 도구 출력 가리기 → 2차 LLM 요약 (반영할 연구 2508.21433) |
| 모델 교체 시 대화 통째 재전송 경고 (Claude) | 2026-11-07 사용자 관찰 | **04 배치(D43)**: 압축 기록 + 세션 임시 문서 + 규칙 추출 보존 조합으로 교체 시 토큰 절약 (04 기획 반영 완료) |
| `/clear` 새 대화 | Claude, Codex | **채택** 04 확장: `/new` 설계 |
| 세션 지목 `@`·`#` | Claude `@` 세션, Prospero `@` 지목 | **채택** 04 AC 8·9 |
| 부모 대화를 서브에 계속 흘려보내기 | Claude Code 서브 세션 (2026-11-07 사용자 지적: 토큰 낭비) | **미채택(금지)**: 04에 "통째 전송 금지·짧은 지시 한 줄만" 규칙 반영 완료. 대체 방식 3가지: ① 위임 프롬프트 + 작업 문서 포인터만 전달(부모 대화 재구성 불필요), ② 색인 관리로 하네스·규칙 파일만 전달(세션 지시 2층), ③ 부모는 자식 최종 보고만 수신 — Claude는 서브 실행 중 부모 메시지를 폴라로 수신하는 구조라 반복 전송 비용이 발생. snap은 폴라 수신 없이 파일·문서로 전달 |

## (f) 하네스·시스템 프롬프트 적용

| 기능 | 근거 | snap 대응 |
|---|---|---|
| 짧은 시스템 프롬프트 + 규칙 파일 분리 | Pi minimal harness, Harness² 2610.14224 | **채택** 03: `agent/system_prompt.txt`·설정 파일 분리. Harness²는 하네스=프롬프트·기억·스킬·도구·오케스트레이션 묶음으로 정의하고, 하네스 개선 경험을 저장·재활용하는 구조(재귀 개선)를 제시 → [ Harness² 번역 전문](../../.local/research/01/arxiv-2610.14224.kr.md) (규칙 파일 분리 → 지시 파일 색인 계층) |
| 하네스를 서브 프롬프트에 주입(포인터 샌드위치) | agent-harness(자체 하네스), Cursor Task | **채택(하네스)**: 04 브리프 전달 확장 시 제품 규칙으로 반영 |
| 도구 호출 턴 낮은 온도 | Sub-2B 2609.07370 | **채택** 03 |

## (g) 서브 에이전트·세션 전달

| 기능 | 근거 | snap 대응 |
|---|---|---|
| Task 도구 서브 (빈 컨텍스트·요약만) | Cursor subagents, Claude sub-agents, Gemini subagents | **채택** 04 확장 후보 |
| 에이전트 팀·mailbox | Claude agent-teams, Prospero mailbox/`send_message` | **후보**: 04 확장(세션 간 대화) 착수 시 설계 |
| 부모→자식 중간 지시 | Codex `codex queue`, Prospero `send_message` | **채택** 04 확장 (짧은 지시 한 줄만, 통째 전송 금지) |
| 포크(self-fork) | Claude fork, Codex fork | **후보**: 04 확장 |
| 사이드 채팅 `/side` | Cursor side chat, Prospero `/side` | **채택** 04 확장 후보(2026-11-07 반영) |

## (h) 체크포인트·세션 저장

| 기능 | 근거 | snap 대응 |
|---|---|---|
| 세션 jsonl 저장·이어가기 | Claude `--continue`·`--resume`, Codex `codex resume --last`, OpenCode `--continue`·`--session` | **채택** 04 AC 2 |
| 세션 fork | Codex fork, Gemini checkpoint | **후보** |

## (i) 감시·교정

| 기능 | 근거 | snap 대응 |
|---|---|---|
| 턴·스텝 훅(`beforeTurn` 등) | Project Think 훅 | **채택** 05: `Monitor` 트레이트 |
| 훅(Claude hooks: 이벤트 스크립트) | Claude hooks | **후보**: 05 이후(규칙 자동화) |
| 판정 모델 얹기 | (snap M2 계획) | **채택** 08 (snap 독자 설계) |

## (j) 그 밖의 확장

| 기능 | 근거 | snap 대응 |
|---|---|---|
| MCP (외부 도구 연결) | Claude/Cursor/Codex/OpenCode 모두 MCP 지원 | **후보**: 제품 확장. 03 범위 밖. 필요하면 새 작업 문서 |
| 플러그인·확장 | Claude plugins | **미채택**: PLAN.md "다른 도구에 끼워 넣는 확장" 범위 밖 선언과 충돌 |
| GitHub Actions·CI 연동 | Claude GitHub Actions | **미채택** |

## 전체 결론

1. 03이 만드는 도구 7개와 승인·deadline 구조는 위 목록의 (a)(b) 중심을 채운다. 03이 "코딩 한 바퀴 기본 도구"의 전부이고, 나머지는 04 배치다(D43).
2. 웹 도구·스킬 내장·메모리 계층·`/compact`·서브 세션·계획 모드·LSP가 타 에이전트에는 있고 snap 계획에 없는 기능들이다.
3. 위 후보 기능 중 04에 늦게 넘기는 것을 05·06 이후로 미루지 않는다(D43). 03 완료 시점의 완성 여부는 이 목록의 (a)(b) 대조로 판단한다. 04 완료 시점의 완성 여부는 (b)(c)(d)(e)(g) 대조로 판단한다. 04 이후에 추가 배치할 도구가 05·06·07-09에 흩어지는 것을 막는다.

## 확인 링크 (직접 연 것만)

- Claude Code: [overview](https://code.claude.com/docs/en/overview), [sub-agents](https://code.claude.com/docs/en/sub-agents), [sessions](https://code.claude.com/docs/en/sessions), [agents](https://code.claude.com/docs/en/agents), [agent-teams](https://code.claude.com/docs/en/agent-teams), [permissions](https://code.claude.com/docs/en/permissions), [checkpointing](https://code.claude.com/docs/en/checkpointing), [sandboxing](https://code.claude.com/docs/en/sandboxing), [settings](https://code.claude.com/docs/en/settings), [security](https://code.claude.com/docs/en/security), [authentication](https://code.claude.com/docs/en/authentication)
- Cursor: [subagents](https://cursor.com/docs/subagents), [CLI overview](https://cursor.com/docs/cli/overview), [agent overview](https://cursor.com/docs/agent/overview)
- Codex CLI: [docs](https://learn.chatgpt.com/docs/cli/reference.md), [README](https://github.com/openai/codex), [approvals-security](https://learn.chatgpt.com/docs/agent-approvals-security)
- OpenCode: [tools](https://opencode.ai/docs/tools), [skills](https://opencode.ai/docs/skills), [agents](https://opencode.ai/docs/agents), [permissions](https://opencode.ai/docs/permissions/), [config](https://opencode.ai/docs/config/)
- Prospero: [usage docs](https://raw.githubusercontent.com/skeeto/prospero/master/docs/usage.md), [저자 블로그](https://nullprogram.com/blog/2026-10-31/)
- Gemini CLI: [subagents](https://github.com/google-gemini/gemini-cli/blob/main/docs/core/subagents.md), [session-management](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/session-management.md), [sandbox](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/sandbox.md)
