# 작업 03 agent-loop-tools 조사 상세

- 인덱스: [docs/RESEARCH.md](../RESEARCH.md) 해당 작업 절
- 역할: 조사 상세. 규범 정본이 아님. 정책은 docs/decisions/… 및 DECISIONS.
- D22 정본: [docs/decisions/D22-execution-reliability-autonomy.md](../decisions/D22-execution-reliability-autonomy.md) (요약은 [DECISIONS.md](../DECISIONS.md) D22)

## 작업 03

확인 날짜는 모두 2026-10-28입니다.

### 설정·키 위치와 비밀 파일 차단

- opencode
  - 링크: [Config](https://opencode.ai/docs/config/), [Providers](https://opencode.ai/docs/providers/), [Permissions](https://opencode.ai/docs/permissions/)
  - 핵심: 전역 설정은 `~/.config/opencode/opencode.json`이고 프로젝트의 `opencode.json`과 병합합니다. 키는 `~/.local/share/opencode/auth.json`에 두거나 설정에서 `{env:}`, `{file:}`로 참조합니다. 권한 기본값은 `*.env`, `*.env.*` 읽기에 승인을 요청하고 `*.env.example`은 허용합니다.
- Codex CLI
  - 링크: [config 기본](https://learn.chatgpt.com/docs/config-file/config-basic), [인증](https://learn.chatgpt.com/docs/auth)
  - 핵심: `~/.codex/config.toml`을 읽고, 신뢰한 프로젝트의 `.codex/config.toml`만 불러옵니다. 키는 `~/.codex/auth.json` 또는 운영체제 자격 증명 저장소에 둡니다.
- Claude Code
  - 링크: [Settings](https://code.claude.com/docs/en/settings), [Authentication](https://code.claude.com/docs/en/authentication), [Security](https://code.claude.com/docs/en/security), [Permissions](https://code.claude.com/docs/en/permissions)
  - 핵심: 설정은 `~/.claude/settings.json`과 프로젝트의 `.claude/settings.json`, `settings.local.json`입니다. 키는 환경 변수 또는 자격 파일로 받습니다. `Read(./.env)` 같은 읽기 거부 규칙이 있지만 `grep -r` 같은 명령은 막지 못한다고 문서에 밝혀 두었습니다.
- aider
  - 링크: [.env](https://aider.chat/docs/config/dotenv.html), [aider.conf.yml](https://aider.chat/docs/config/aider_conf.html)
  - 핵심: `.aider.conf.yml`과 `.env`를 홈, git 루트, 현재 폴더 순으로 읽고 나중에 읽은 것이 우선합니다.
- 사용자 설정 폴더 관례
  - 링크: [dirs::config_dir](https://docs.rs/dirs/latest/dirs/fn.config_dir.html), [XDG Base Directory](https://specifications.freedesktop.org/basedir/latest/)
  - 핵심: Windows는 Roaming AppData, 리눅스는 `$XDG_CONFIG_HOME` 또는 `$HOME/.config`입니다. XDG 값이 상대 경로면 무시합니다.
- 적용 위치: 사용자 설정 폴더, `AGENT_SNAP_CONFIG_DIR`, 파일 도구의 비밀 파일 차단. 작업 02 D10: 설정 파일과 자격 파일 분리(02-1, 02-2, 02-8).
- 미적용: 프로젝트별 덮어쓰기(지금 필요 없음), `dirs` 크레이트(표준 라이브러리로 충분), macOS `Library/Application Support`(Windows 우선, 다른 에이전트도 홈 아래 폴더를 씀).
- 관련 작업: 03

### 도구 인터페이스와 작은 모델 논문

확인 날짜는 2026-10-28이며 arxiv.org 초록 페이지를 직접 열어 제목과 내용을 확인했습니다.

- SWE-agent
  - 링크: [arXiv 2405.15793](https://arxiv.org/abs/2405.15793)
  - 핵심: 제목은 "SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering"입니다. 언어 모델 에이전트도 사람 개발자의 IDE처럼 자기에게 맞춘 인터페이스가 필요하다고 보고, 파일 만들기·편집, 저장소 탐색, 테스트 실행을 돕는 인터페이스로 SWE-bench 12.5% pass@1을 얻었다고 합니다. 초록에는 "도구 최소", "짧은 오류"라는 표현이 없으며 인터페이스 설계가 성능을 좌우한다는 데까지만 확인했습니다.
  - 적용 위치: `crates/snap-core/src/tools/`(도구 7개, `tool.rs`의 `Tool` 트레이트, `set.rs`가 실패를 `오류: 이유` 한 줄로 변환).
- 작은 모델의 도구 호출 신뢰성
  - 링크: [arXiv 2609.07370](https://arxiv.org/abs/2609.07370)
  - 핵심: 제목은 "Beyond Fluent Generation: A CPU Reliability Benchmark for MCP-Style Tool Calling in Sub-2B Small Language Models for Edge Deployment"입니다. 2B 미만 모델 5종을 100개 과제에서 탐욕 디코딩과 샘플링으로 비교했습니다. 복구 파서를 거치기 전 원시 응답 1,000개 중 유효한 JSON은 5개뿐이어서 결과가 파서에 크게 좌우됐고, 샘플링에서 0.5B 모델은 72%에서 32%로 떨어졌습니다. 권고는 스키마 검증, 제약 생성, 최소 권한 실행, 중요한 동작의 사람 확인입니다. 초록에 "재시도"는 없습니다.
  - 적용 위치: 인자 JSON 복구와 검증은 `tools/args.rs`, 오류를 모델에 돌려 이어 가는 흐름은 `agent/loop/runner.rs`, 기본 온도 0은 `llm/openai/mod.rs`의 `with_default_temperature`와 `agent/loop/runner.rs`의 `DEFAULT_TEMPERATURE`.
- 코드 편집 출력 형식
  - 링크: [arXiv 2604.27296](https://arxiv.org/abs/2604.27296)
  - 핵심: 제목은 "To Diff or Not to Diff? Structure-Aware and Adaptive Output Formats for Efficient LLM-based Code Editing"입니다. 줄 번호 기반 일반 diff는 모델이 쓰기 어렵다고 하고, 블록·함수 단위 diff와 diff/전체 코드 선택 학습(AdaEdit)을 제안합니다. 검색-치환 형식은 초록에 나오지 않아, 이 논문은 "줄 번호 diff를 피한다"는 근거로만 씁니다. 형식별 실패율 비교는 작업 06에서 합니다.
  - 적용 위치: `tools/edit_file.rs`(줄 번호 없이 정확한 문자열을 찾아 바꿈), `agent/system_prompt.txt`가 이 방식을 안내합니다.
- 관련 작업: 03

### 에이전트 도구 설계 참고 (opencode, Codex CLI, Pi, Flue)

- opencode
  - 링크: [Tools](https://opencode.ai/docs/tools/), [Permissions](https://opencode.ai/docs/permissions/)
  - 핵심: 편집 도구는 정확한 텍스트 일치를 바꾸는 방식이라고만 적혀 있고, 인자와 다중 일치 처리는 문서에 없습니다. grep과 glob은 ripgrep을 쓰고 `.gitignore`를 존중하며 glob은 수정 시각순으로 돌려줍니다. 권한은 allow, ask, deny 세 가지이고 bash는 명령 패턴(와일드카드, 마지막으로 맞는 규칙 우선)별로 정할 수 있으며 편집·쓰기·패치는 하나의 edit 권한으로 묶입니다. 승인 창에서는 한 번 허용, 세션 동안 허용, 거부를 고릅니다.
  - 적용 위치: 명령 실행 전 사용자 확인(허용·거부 두 갈래)은 03-6에서 `tools/run_command.rs`와 `tools/approval/approver.rs`로 구현했고, 콘솔 구현은 `snap-cli/src/console.rs`(04 배치). `.gitignore` 존중은 적용하지 않았습니다(미적용 이유: 목록·검색은 `.git`, `target` 등 고정 제외만 하고 규칙 해석은 범위 밖).
  - 미적용: 패턴별 자동 허용 규칙과 세션 동안 허용(범위 밖, 필요해지면 별도 작업).
- Codex CLI
  - 링크: [저장소 README](https://github.com/openai/codex), [승인과 보안](https://learn.chatgpt.com/docs/agent-approvals-security)
  - 핵심: 샌드박스 모드(read-only, workspace-write, danger-full-access)와 승인 정책(on-request, never, granular)을 따로 둡니다. 샌드박스 안 명령은 묻지 않고 실행하며, 밖으로 나가거나 네트워크가 필요하면 승인을 요청합니다. 승인 검토자가 기본은 사용자입니다. 검색-치환 편집 도구와 다중 일치 처리는 열어 본 문서에 없어 확인하지 못했습니다.
  - 적용 위치: 03-6에서 `tools/approval/approver.rs`가 `CommandApprover`을 두고, 승인자 미주입은 `request_approval`이 기다리지 않고 거부합니다. 콘솔(사람 확인 창)은 `snap-cli/src/console.rs`로 04 배치.
  - 미적용: 샌드박스 격리(운영체제별 구현이 크고 범위 밖).
- Pi 코딩 에이전트
  - 링크: [coding-agent README](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent), [사용법](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/usage.md), [동작 방식](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/how-pi-works.md)
  - 핵심: 최소 하네스를 표방하며 하위 에이전트와 계획 모드를 기본에서 뺍니다. 도구 호출마다 묻지 않고 샌드박스를 권합니다. 시스템 프롬프트는 기본 지시와 발견한 맥락 파일로 만들고, 요청에는 도구 정의와 스킬 설명이 실리며 스킬 본문은 필요할 때 불러옵니다. 기본 도구 목록과 프롬프트 길이는 열어 본 문서에서 확인하지 못했습니다.
  - 적용 위치: `agent/system_prompt.txt`(8줄)와 `agent/prompt/assemble.rs`.
  - 미적용: 승인 없이 실행하는 방식(이 프로젝트는 명령 실행에 사용자 확인을 요구).
- Flue
  - 링크: [withastro/flue](https://github.com/withastro/flue)
  - 핵심: Apache-2.0, TypeScript 에이전트 하네스입니다. 에이전트를 함수로 선언하고 그 안에서 모델, 샌드박스, 스킬, 도구를 조합합니다. 도구는 일반 모듈에서 가져오는 타입 있는 동작이고, 샌드박스는 가상·로컬·원격 컨테이너로 바꿔 끼웁니다. Pi와의 관계는 열어 본 페이지에 나오지 않았습니다.
  - 적용 위치: `tools/tool.rs`(이름·설명·스키마·실행을 한 트레이트에 선언)와 `tools/context.rs`(`ToolContext`가 작업 폴더·승인자를 주입).
  - 미적용: 함수 선언 방식의 에이전트 정의(언어와 맞지 않음), 원격 샌드박스.

### OpenAI 호환 도구 호출 형식과 미지원 서버 동작

- 형식
  - 링크: [OpenAI 함수 호출](https://developers.openai.com/api/docs/guides/function-calling), [OpenRouter 도구 호출](https://openrouter.ai/docs/guides/features/tool-calling), [Ollama 도구 호출](https://docs.ollama.com/capabilities/tool-calling)
  - 핵심: 요청의 `tools` 배열은 `type: "function"`과 `function`(`name`, `description`, JSON Schema `parameters`)입니다. 응답 메시지의 `tool_calls` 각 항목은 `id`와 `function.name`, `function.arguments`(JSON 문자열)를 가지며 `finish_reason`은 `tool_calls`입니다. 결과는 assistant 메시지를 기록에 덧붙인 뒤 `role: "tool"`, `tool_call_id`, `content` 메시지로 돌려줍니다. 한 응답에 호출이 여러 개일 수 있고 `parallel_tool_calls`를 `false`로 제한할 수 있습니다. OpenRouter는 `tools`를 매 요청에 넣으라고 합니다. OpenAI 함수 호출 문서의 Chat Completions 기본값은 strict가 꺼져 있습니다. Ollama의 네이티브 API는 `arguments`가 객체이고 결과에 `tool_name`을 쓰는 등 달라서, 호환 경로(`/v1`)만 씁니다. OpenAI `platform.openai.com` API 참조(chat create)는 403으로 열리지 않았습니다.
  - 적용 위치: `agent/loop/runner.rs`(매 요청에 `tools`, 응답의 호출을 순서대로 실행, `tool_call_id`를 붙인 결과를 기록에 덧붙임), 인자 문자열 해석은 `tools/args.rs`.
- 도구 미지원 서버·모델
  - 링크: [llama.cpp 함수 호출](https://github.com/ggml-org/llama.cpp/blob/master/docs/function-calling.md), [OpenRouter 도구 호출](https://openrouter.ai/docs/guides/features/tool-calling)
  - 핵심: llama.cpp 서버는 `--jinja`와 도구를 아는 채팅 템플릿이 있어야 하고, 템플릿을 모르면 Generic 처리로 떨어져 토큰이 더 들고 효율이 낮습니다. `/props`의 `chat_template`, `chat_template_tool_use`로 확인할 수 있습니다. 극단적인 KV 캐시 양자화는 도구 호출을 해칠 수 있습니다. OpenRouter는 `supported_parameters=tools`로 모델을 거를 수 있다고 합니다. 도구를 못 쓰는 모델이 오류를 내는지 무시하는지는 열어 본 문서 어디에도 적혀 있지 않아 확인하지 못했습니다.
  - 적용 위치: 4xx 응답은 `agent/loop/runner.rs`의 `map_llm_error`가 `AgentError::ToolsUnsupported`로 알립니다. 도구 호출 없이 일반 텍스트만 오는 경우의 판별은 미적용(미적용 이유: 최종 답과 구분할 수 없어 스모크의 실제 응답으로 확정). 판별 방식은 스모크에서 확정합니다.
  - 미적용: 모델 목록 메타데이터(`supported_parameters`)로 사전 판별(서버마다 없어서 공통 근거가 안 됨).
- 관련 작업: 03

### 에이전트 도구의 일방향/양방향 문 분류

확인 날짜는 2026-10-30이며 네 링크 모두 직접 열어 확인했습니다.

- 링크: [Mental Models in the Agentic Era](https://productmindset.substack.com/p/mental-models-in-the-agentic-era), [Amazon 2015 주주서한](https://www.sec.gov/Archives/edgar/data/1018724/000119312516530910/d168744dex991.htm), [Replit 운영 DB 삭제 보도(The Register)](https://www.theregister.com/software/2025/07/21/vibe-coding-service-replit-deleted-production-database/719783), [Building effective agents(Anthropic)](https://www.anthropic.com/research/building-effective-agents)
- 핵심: 행동을 되돌릴 수 있는지로 나눕니다. 읽기·요약·초안은 승인 없이 실행하고, 지출·발송·삭제·게시처럼 되돌릴 수 없는 행동은 모두 승인 게이트를 겁니다. 근거는 베이조스 2015년 주주서한의 Type 1/Type 2 결정과 Replit 운영 DB 삭제 사례이며, 사례는 당사자 주장을 전한 보도입니다. 자율성은 제안, 초안, 승인 후 실행, 실행 후 보고의 단계로 두고 신뢰가 쌓이면 올립니다. Anthropic 글은 체크포인트에서 사람 피드백과 중단 조건을 권하지만 단계 정의는 없습니다.
- 적용 위치: 작업 03의 도구 승인 정책(도구별 문 유형 분류). 구체적 분류는 격리 방식이 정해진 뒤 03 기획에 반영합니다(D9).
- 미적용: "도구를 문 유형으로 나누면 권한 구조의 80%가 정해진다"는 수치(원문에 근거 없음), 자율성 단계 확대(04 실사용 뒤 판단).
- 관련 작업: 03

### 코딩 에이전트별 승인·되돌리기·샌드박스 비교

확인 날짜는 2026-10-30이며 아래 링크는 모두 직접 열어 확인했습니다.

- 링크: [Claude Code 권한](https://code.claude.com/docs/en/permissions), [Claude Code 체크포인트](https://code.claude.com/docs/en/checkpointing), [Claude Code 샌드박스](https://code.claude.com/docs/en/sandboxing), [Codex 승인과 보안](https://learn.chatgpt.com/docs/agent-approvals-security), [Gemini CLI 샌드박스](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/sandbox.md), [Gemini CLI 체크포인트](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/checkpointing.md), [Aider git 연동](https://aider.chat/docs/git.html), [OpenHands 보안](https://docs.openhands.dev/sdk/guides/security), [Goose 권한](https://goose-docs.ai/docs/guides/managing-tools/goose-permissions/)
- 핵심
  - Claude Code: 파일 쓰기와 셸 모두 기본 승인입니다. 작업 폴더 안 편집을 자동 허용하는 모드가 있습니다. 프롬프트마다 체크포인트를 남기지만 셸로 바뀐 파일은 복원하지 않습니다. 샌드박스는 기본 꺼짐이고 네이티브 Windows는 지원하지 않습니다.
  - Codex CLI: 작업 폴더 안 쓰기는 승인 없이, 폴더 밖과 네트워크는 승인합니다. 샌드박스가 쓰기를 작업 폴더로 제한하고 `.git` 등은 읽기 전용입니다. 내장 되돌리기는 문서에서 확인하지 못했습니다.
  - Gemini CLI: 체크포인트(기본 꺼짐)와 여러 샌드박스 방식이 있고, Windows 네이티브 방식은 낮은 무결성 수준을 씁니다.
  - Aider: 편집마다 git 자동 커밋, 편집 전 미커밋 변경을 먼저 커밋, 직전 커밋 되돌리기를 합니다.
  - OpenHands: 위험한 행동만 승인하는 정책을 고를 수 있고 Docker 격리가 기본입니다.
  - Goose: 기본은 전부 자동이고, 쓰기 도구만 승인받는 모드가 있습니다.
  - 공통: 파일 쓰기 승인 여부는 갈리고, 승인을 받지 않는 쪽은 되돌리기나 샌드박스로 보완합니다. git이 추적하지 않는 파일을 따로 다루는 곳은 확인하지 못했습니다.
- 적용 위치: D9 판단 근거.
- 미적용: 각 에이전트의 샌드박스 구현(Docker, WSL, 별도 프로그램, 플랫폼 의존은 snap의 경량 목표와 맞지 않음, D9).
- 관련 작업: 03

### Prospero 코딩 하네스 벤치마크

확인 날짜는 2026-11-02이며 아래 링크를 직접 열어 확인했습니다. [prospero.org](http://www.prospero.org/)는 동명 디렉터리 서비스로 본 하네스와 무관합니다(이름 충돌).

- Prospero (nullprogram / skeeto)
  - 링크: [nullprogram (2026-10-31)](https://nullprogram.com/blog/2026-10-31/), [skeeto/prospero](https://github.com/skeeto/prospero)(README·`docs/windows.md` 요약), [GeekNews](https://news.hada.io/topic?id=36346)
  - 핵심: 모든 작업에 deadline을 두고(Plan·`ask_user`만 무기한), 권한 미응답은 timeout으로 거부하며, 네트워크·watchdog·mailbox bound·셸 timeout 후 비동기 백그라운드와 주기 wake를 씁니다. 자식 프로세스는 트리와 함께 수명을 맞추고 Windows에서는 Job Object를 씁니다. 모델 제공자는 비내장(OpenAI Chat Completions·Anthropic Messages, INI·`env:KEY`)이며, 권한은 완전한 보안 경계가 아니라 실수·범위 상기용입니다(실격리는 하네스 격리). 서브에이전트·원격 헤드리스·데스크톱 제어·패시브 렌더링·스킬/자기문서가 제품 차별점입니다.
  - snap 대조: 엔드포인트·키 비내장(D10/D20, PLAN), HTTP connect/header/stream_idle(D20, config), `run_command` 승인(03)은 이미 가깝거나 기획됨. Job Object는 D9 후보. 03에 아직 없는 것은 셸·도구 deadline, timeout→백그라운드 폴링, 프로세스 트리 종료, 권한 응답 timeout입니다. GUI·ImGui, 원격 엔진, 데스크톱 제어, 런타임 서브에이전트, CE/Pro 모델은 범위 밖·보류입니다.
  - 적용 위치: 03-6 T1에서 D22 값(실행 600초·승인 300초)을 `tools/run_command.rs`의 `select!`과 `tools/approval/approver.rs`의 `request_approval`에 적용했고, 값은 `config/read/agent.rs`의 `agent` 섹션(`run_command_deadline`·`approval_deadline`)으로 받는다. 미응답 승인은 timeout 뒤 거부로만 본다(Prospero `ask_timeout`과 같은 결, 값은 D22의 300초). Job Object/프로세스 트리 정리는 03-7 배치, timeout→백그라운드 전환은 03-9 후보로 두었다.
  - 본 절은 **조사 기록**이며 **규범 정본이 아니다**. 적용·금지·단계 정의의 정본은 `docs/decisions/D22-execution-reliability-autonomy.md` (요약은 DECISIONS D22).
  - 미적용: 네이티브 GUI·패시브 렌더(PLAN은 Tauri+Solid, 범위 밖). 원격 헤드리스·데스크톱 제어(경량·D9/Flue 미적용과 충돌). 권한 timeout 거부는 snap의 'run_command는 확인 후 실행'과 긴장 관계 — 비대화형·미응답 시 Codex식 거부와 맞춰 문서화 필요.
  - 이미 정렬: 제공자 비내장, 권한≠완전 보안 경계(D9 경량).
  - 관련 작업: 03-agent-loop-tools, D9(HTTP는 02-client)

### 멀티 세션·서브세션 기능 비교 (타 에이전트)

확인 날짜는 2026-11-07이며 공식 문서와 저장소 원문을 직접 열었습니다. Prospero는 README 페이지 본문이 비어 있어 같은 저장소의 `docs/usage.md` 원문과 저자 블로그로 확인했습니다.

- Claude Code
  - 링크: [Sub-agents](https://code.claude.com/docs/en/sub-agents), [Sessions](https://code.claude.com/docs/en/sessions), [Agents](https://code.claude.com/docs/en/agents), [Agent teams](https://code.claude.com/docs/en/agent-teams)
  - 핵심: 서브에이전트는 빈 컨텍스트에서 부모가 만든 위임 문맥만 받고 요약만 돌려줍니다. 포크(fork)만 부모 대화 전체를 상속합니다. 에이전트 팀의 mailbox와 cross-session messaging이 세션 간 전달을 맡습니다. `claude --continue`·`--resume`·`/resume`·`/clear`·`/compact`가 있습니다.
  - snap 관련: 서브 세션 열기와 부모-자식 전달은 04 범위 확장 항목입니다. 03의 단발 run에는 들어가지 않습니다.
- Cursor
  - 링크: [Subagents](https://cursor.com/docs/subagents), [CLI overview](https://cursor.com/docs/cli/overview), [Agent overview](https://cursor.com/docs/agent/overview)
  - 핵심: Task 도구로 서브에이전트를 열고, 부모가 프롬프트에 필요한 정보를 담아 보낸 뒤 결과만 받습니다. CLI는 `agent resume`·`--continue`, 사이드 채팅(`/.side`)로 부모를 숨은 참고용으로 씁니다.
- Codex CLI
  - 링크: [CLI reference](https://learn.chatgpt.com/docs/cli/reference.md)
  - 핵심: `codex resume --last`·`codex fork`·세션 저장(기본 `~/.codex`)을 지원합니다. `codex queue --thread <id> --message`로 이미 열린 세션에 메시지를 보냅니다.
- Prospero
  - 링크: [docs/usage.md 원문](https://raw.githubusercontent.com/skeeto/prospero/master/docs/usage.md), [저자 블로그](https://nullprogram.com/blog/2026-10-31/)
  - 핵심: 서브에이전트가 부모를 깨우는 통지로 보고하고, `send_message`와 `@세션 지목`으로 세션 간 대화를 합니다. `/compact`·`/side`·`/fork`가 있고, 폴더 없는 대화("No folder")로 프로젝트와 무관한 대화도 합니다.
- Gemini CLI·OpenCode·Antigravity
  - 링크: [Gemini CLI subagents](https://github.com/google-gemini/gemini-cli/blob/main/docs/core/subagents.md), [Gemini CLI session management](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/session-management.md), [OpenCode agents](https://opencode.ai/docs/agents/)
  - 핵심: 서브에이전트(격리 컨텍스트)와 세션 이어가기를 지원합니다. OpenCode는 `mode: subagent`로 자식 세션을 새로 열고 `--continue`·`--session`·`--fork`로 이어갑니다. Gemini는 `/resume` 체크포인트가 있습니다. Antigravity는 공식 문서가 없어 Google 블로그 기준으로 서브에이전트 위임과 새 주제별 새 채팅 안내만 확인했습니다.
- 공통 소견: 새 세션 열기와 브리프 전달을 한 개의 명령으로 묶은 도구는 없습니다. 새 세션은 `prospero DIR`, `/clear`로 열고, 브리프는 `send_message`(Prospero), `codex queue`, Claude Code의 `@` 세션 지목으로 따로 보냅니다.
- 적용 제안(승인 대기): 서브 세션 열기, 부모→자식 중간 지시, 자식→부모 보고, 세션 간 대화, 사이드 대화, 새 세션 + 브리프 전달은 03 범위 밖이고 04(세션 저장·지목 호출) 위에 얹는 확장입니다. 하네스의 delegation-brief 포인터 샌드위치가 브리프 전달 규격 정본이므로 제품 규칙으로 옮깁니다.
- 미적용: Prospero식 GUI·데스크톱 제어, Antigravity 블로그 기반 설계(공식 문서 아님). 03에는 반영하지 않습니다.
- 관련 작업: 04(주), 03(단발 run 경계 확인)

### AGENTS.md 규칙 파일 주입 조사 (03-5, 2026-11-11)

확인 날짜는 2026-11-11이며 아래 링크를 모두 직접 열어 확인했습니다.

- Codex AGENTS.md
  - 링크: [agents-md 가이드](https://developers.openai.com/codex/guides/agents-md)
  - 핵심: 매 실행마다 AGENTS.md를 읽고, 루트에서 현재 폴더로 내려가며 아래쪽 파일이 앞 지침을 덮어쓰도록 아래에서 위로 이어 붙인다. 빈 파일은 무시하고 `project_doc_max_bytes`(기본 32 KiB)에 닿으면 멈춘다.
  - 적용 위치: 03-5 `agent/agents_md/`(읽기·표시) — 읽기 → 규범 뒤 원문 붙임, 상한 초과 시 길이 표시만(Codex와 잘 달리 잘라 넣지는 안 함). 계층 우선순위는 04 배치.
- Claude Code memory
  - 링크: [memory 문서](https://code.claude.com/docs/en/memory)
  - 핵심: CLAUDE.md·AGENTS.md를 세션 시작에 컨텍스트로 읽고, 파일마다 200줄 이하를 권한다. 긴 파일은 컨텍스트를 더 소모해 순응도가 떨어진다. AGENTS.md가 있으면 CLAUDE.md는 읽지 않는다.
  - 적용 위치: 03-5 길이 표시 근거(상한 권고). 단일 파일만 읽기는 03-5가 채택.
- Cursor rules
  - 링크: [rules 문서](https://cursor.com/docs/context/rules)
  - 핵심: 규칙 내용(AGENTS.md 포함)이 모델 컨텍스트 앞머리에 들어간다. 참조 파일은 프롬프트에 인라인하지 않고 필요할 때 읽는다.
  - 적용 위치: 03-5 조립 순서(규범 → AGENTS.md, 앞머리 유지)와 2307.03172 근거를 각주로 명시.
- OpenCode rules
  - 링크: [rules 문서](https://opencode.ai/docs/rules/)
  - 핵심: AGENTS.md를 프로젝트·전역에서 읽고, AGENTS.md가 있으면 CLAUDE.md는 읽지 않는다(첫 번째로 맞는 파일만 사용).
  - 적용 위치: 03-5는 루트 `AGENTS.md` 단일 파일만 읽는다.
- 관련 확인: Lost in the Middle는 arXiv 2307.03172가 맞고, 2409.10372는 이 주제와 무관한 논문이라 어떤 문서에도 인용하지 않는다.
- 미적용·04 배치로 넘김: 가장 가까운 AGENTS.md가 앞 지침을 덮어쓰는 계층 우선순위, 규칙 파일 색인·메모리(D43·D45).
- 관련 작업: 03-5

### 자동화 단계·체크포인트·무인 옵션 검토
- 확인 날짜: 2026-11-02
- 링크: Prospero RESEARCH 절, opencode allow/ask/deny(RESEARCH 작업 03), 문 유형·자율성 단계(RESEARCH), D22
- 비교: (1) 무인 on/off 토글 — 단순하나 체크포인트·허용 범위가 뭉개짐. (2) 체크포인트만 — 사람 간섭 지점은 명확하나 Prospero식 무인 옵션이 없음. (3) 자동화 단계(L1~L3)+단계 안 체크포인트 — 기본은 L1, 이후 L3를 옵션으로 켤 수 있음.
- 적용 위치: `docs/decisions/D22-execution-reliability-autonomy.md`, `docs/plans/03-agent-loop-tools.md` (신뢰성·승인자 주입). L2·L3 설정 UI/키는 06 이후 기획.
- 본 절은 **조사 기록**이며 **규범 정본이 아니다**. 적용·금지·단계 정의의 정본은 `docs/decisions/D22-execution-reliability-autonomy.md` (요약은 DECISIONS D22).
- 미적용(지금): L2·L3 구현, 기본 무인.
