# 에이전트별 도구 호출 형식 비교·검토 (2026-11-08)

- 확인 방법: 2026-11-08에 공식 문서를 직접 열었다. 열지 못한 것은 적지 않았다. Prospero 호출 형식은 이 문서에서 직접 확인하지 못했고, 기존 조사(03-agent-loop-tools.md Prospero 절)가 OpenAI Chat Completions·Anthropic Messages 제공자만 쓴다고 기록한다.
- 역할: 조사 상세. 규범 정본이 아니다.
- 상태(2026-11-08 사용자 지적 반영): 호출 형식을 확정한 적은 없다. 이 문서의 3안은 승인 대기 제안이다. 확정은 `docs/DECISIONS.md`에 결정으로 남긴 뒤에만 정본이 된다.
- 2026-11-08 추가 지적 반영: 아래 3절의 "03-1은 그대로 유지" 표현은 확정 표현이 아니다. 03-1이 OpenAI 형식 문서를 쓰고는 있으나, 호출 형식 결정 기록은 아직 없다. 3절은 사용자의 승인을 받을 때까지 제안으로만 둔다.

## 1. 에이전트별 호출 형식 (확인한 것만)

| 에이전트 | 호출 형식 | 근거 |
|---|---|---|
| snap 03 (설계) | OpenAI `tools` 배열 + `tool_calls` + `role:"tool"` 정본. 미지원 모델은 오류 | [OpenAI 함수 호출](https://developers.openai.com/api/docs/guides/function-calling), `research/03-agent-loop-tools.md` OpenAI 절 |
| Claude Code | Anthropic Messages API의 네이티브 함수 호출(`tool_use` 블록) | skills 문서의 invoked/load 동작, sub-agents 문서 |
| OpenCode | 네이티브 함수 호출 기본. GPT 계열에는 `patch`도구 노출(나머지 모델은 `edit`·`write`). Code Mode `execute`(JavaScript로 도구 조합)는 별도 경로 | [OpenCode tools](https://opencode.ai/docs/tools/) "OpenCode exposes `patch` for supported GPT models; other models receive `edit` and `write`" |
| Codex CLI | Responses API 쪽 자기 하네스. 파일 편집은 `apply_patch` 텍스트 형식, 셸은 승인 정책과 결합 | [approvals-security](https://learn.chatgpt.com/docs/agent-approvals-security) (샌드박스·승인 정책 중심 서술) |
| Cursor | 모델별로 하네스를 맞춰 포장하는 자기 전용 계층. "tuning instructions and tools specifically for every frontier model" | [Cursor agent tools](https://cursor.com/docs/agent/tools) |
| Gemini CLI | `functionCall`·`functionResponse` (Gemini 네이티브 API) | gemini-cli subagents 문서 |
| Prospero | 제공자는 OpenAI Chat Completions·Anthropic Messages. 호출 형식 세부(텍스트 프로토콜 여부)는 2026-11-08에 README 원문·`docs/usage.md` 원문을 열어 확인했으나 호출 형식 설명이 없어 미확정으로 남음 | [03-agent-loop-tools.md Prospero 절](03-agent-loop-tools.md), 캐시: `.local/research/03/03-00-prospero-readme.md`·`03-40-prospero-usage.md` |

## 2. 형식의 갈래

1. 네이티브 함수 호출 (API가 자리를 정해줌)
   - OpenAI `tools`/`tool_calls`, Anthropic `tool_use`, Gemini `functionCall`.
   - 장점: 파서가 이미 있어 모델이 JSON을 섞어 쓰지 않는다. 결과 전달 자리가 정해져 있다.
   - 약점: 로컬 소형 모델은 채팅 템플릿이 필요하고(llama.cpp `--jinja`), 미지원 서버는 4xx로 떨어진다.

2. 텍스트 프로토콜 (프롬프트에 스키마를 적고 JSON·태그를 쓰게 함)
   - llama.cpp가 템플릿을 모르면 Generic으로 빠져 토큰이 늘고 효율이 떨어지는 것이 이 층의 위험이다.
   - Codex `apply_patch`·OpenCode `patch`처럼 편집을 JSON이 아니라 자체 텍스트 문법으로 부르는 것도 이 층에 가깝다.
   - 장점: API가 자리를 몰라도 된다. 로컬 모델 범위가 넓다.
   - 약점: 파서 책임이 에이전트로 온다(2609.07370의 JSON 복구·저온이 필요한 이유). 결과 전달 규격이 에이전트마다 제각각이다.

3. 코드 모드 (도구를 부르는 대신 코드를 실행해 조합)
   - OpenCode `execute`: JavaScript로 도구들을 모아 호출하고 중간 값을 컨텍스트에 안 올린다.
   - 장점: 병렬 호출·결과 가공에서 컨텍스트가 줄어든다.
   - 약점: 런타임·권한 설계가 필요하다. M1 경량 목표에 무겁다.

## 3. snap 적용 검토 (안건, 2026-11-08 — 2026-11-08 확정, DECISIONS D46)

- 2026-11-08 사용자 확정으로 아래 안건은 채택되었다. 정본은 `docs/DECISIONS.md` D46.
- 안건 1 채택(보류→확정): 기본 경로는 1안(OpenAI `tools`/`tool_calls`)로 둔다. 단, 단독 사용이 아니다.
- 안건 2 변형 채택: 텍스트 프로토콜을 03 구현에 얹지 않는 것이 아니라, 로컬 폴백(2안)으로 03-4에 분리하고, 코드 모드 3안은 `execute` 브리지로 03-8에 둔다(읽기 전용 1차).
- 안건 3 채택: 2609.07370 규칙(인자 JSON 복구·저온)은 03-2(루프)와 03-4(폴백)에서 같이 쓴다.
- 배치 순서(2026-11-08 확정): 03-1 트레이트 → 03-2 루프(1안) → 03-3 파일 도구 → 03-4 폴백(2안) → 03-8 execute(3안) → 03-10 스모크(3형식 최종 검증).
- 조합 설계 근거(사용자): Cursor 단독은 로컬 호출 실패 잦음, OpenAI 단독은 로컬 약점, OpenCode는 로컬 원만·보안 부담 → 형식 3갈래를 모델 프로필로 고르는 조합.

## 4. 다섯 축 비교 (2026-11-08, 리서치 서브 반영)

쉬운 말: 도구를 부르겠다고 말하는 법을 세 갈래로 놓고, 다섯 가지를 견줍니다. 토큰 비용, 왕복·병렬, 파서 부담, 준수율, 지원 범위입니다.

정리: 자리 정해진 방식은 4개 축(토큰·왕복·파서·지원 범위)에서 앞섭니다. 준수율만 형식 문제가 아니라 모델 문제입니다. 파서는 형식이 아니라 스키마 검증·복구로 해결합니다.

| 축 | ① 자리 정해진 방식 | ② 글자로 쓰는 방식 | ③ 코드 실행 방식 |
|---|---|---|---|
| 토큰 비용 | 도구 정의·스키마가 매 요청에 들고 가고, 호출은 자리 한 칸이라 글자가 줄어든다 | 스키마를 글자로 적고 답도 글자로 쓰니 두 배로 든다 | 도구 정의를 여러 개 몰아 넣고 한 번에 부른다 |
| 왕복·병렬 | 한 답에 여러 호출, 호출마다 id로 답하기 | 여러 호출은 파서가 거른 뒤 세 번으로 늘어난다 | 여러 호출을 한 번에 합칠 수 있다 |
| 파서 부담 | 서버가 파서를 준다. 에이전트는 받은 JSON만 확인한다 | 파서·복구를 에이전트가 직접 만든다 | 코드 한 줄을 확인하는 검사만 남는다 |
| 준수율 | 자리가 있어도 작으면 못 쓴다 | 형식만 바꾸면 0이 79가 아니라 모델이 정한다 | 도구를 고르고 결과를 다듬는 일은 코드가 한다 |
| 지원 범위 | OpenAI·OpenRouter·Ollama /v1·Claude·Gemini가 잘 쓴다 | 모든 서버에 된다. 로컬 소형 모델에 넓다 | 런타임·권한 설계가 있다 |

준수율 열 보충(2026-11-08): 2609.07370의 원시 응답 감사에서 1,000개 중 순수 JSON은 5개(모두 Qwen2.5-1.5B 표본)였다. 1안을 쓰는 큰 모델도 응답은 자리에 맞춰 쓰지만, 소형 모델은 5절 표 2처럼 복원 파서에 기대야 한다. 준수율은 형식 선택이 아니라 모델 크기·디코딩 설정의 문제이고, 5절의 파서 권고(스키마 검증·최소 권한·확인)가 형식과 별개로 필요하다.

## 5. 리서치 근거 (2026-11-08)

쉬운 말: 큰 모델은 자리가 정해져 있으면 잘 따른다. 작은 모델은 글자 규칙을 지켜야 따라간다. llama.cpp처럼 서버가 형식을 못 맞추면 글자 규칙이 길어진다.

- 준수율 수치
  - Qwen2.5-1.5B 탐욕 75%, 표본 79%, Qwen2.5-0.5B 탐욕 72%→표본 32%, Phi-1.5 0%, TinyLlama·Pythia ≤7%.
  - 근거: `.local/research/03/arxiv-2609.07370.kr.md`, `docs/research/03-agent-loop-tools.md`.
- 원시 응답 감사
  - 원시 1,000개 중 순수 JSON 5개. 파서 없이 파싱하면 0.
  - 근거: `.local/research/03/arxiv-2609.07370.kr.md` (복원 파서 기준 표 2)
- 파서 권고
  - 스키마 검증, 제약 생성, 최소 권한 실행, 중요한 동작의 사람 확인. 재시도·복구는 원문 표현 아님.
  - 근거: 03-agent-loop-tools.md, 2609.07370 초록.
- llama.cpp 서버
  - `--jinja`와 도구 지원 채팅 템플릿 필수, 모르면 Generic으로 떨어져 토큰 증가·비효율. `/props`로 확인.
  - 근거: `.local/research/03/03-17-llamacpp-function-calling.md`, 03-agent-loop-tools.md.
- OpenAI 형식
  - `tools` 매 요청 넣기, 응답에 호출 여러 개, `parallel_tool_calls`로 묶기, `tool_call_id`로 답하기.
  - 근거: `docs/research/03-agent-loop-tools.md` OpenAI 절 (OpenAI 공식 문서·OpenRouter·Ollama 확인).
- diff·편집 형식
  - 줄 번호 diff는 쓰고 어렵다, 블록·함수 단위 diff와 어뎃트형 제안.
  - 근거: `.local/research/03/arxiv-2604.27296.kr.md` (요약 번역).
- 피어 검증
  - TinyAgent·Octopus: 소형 모델 함수 호출 특화 학습.
  - 근거: `.local/research/03/arxiv-2609.07370.kr.md` 2.1절.

## 6. 확인 한계

- 리서치 서브가 웹을 직접 열지 못했고, `.local/research/03` 기록과 `docs/research` 정리를 근거로 삼았다. 1절의 직접 확인 표와 중복을 막기 위해 새 수치는 넣지 않았다.
- Prospero의 호출 형식 세부(텍스트 프로토콜 사용 여부)는 이번 조사에서 확정하지 못했다.
- 2609.07370은 도구 5개·프롬프트 100개·단발 호출 조건이라, 긴 실사용에서 준수율이 더 나빠질 수 있다.
- 3절의 3개 안건은 승인 대기이며, 이 절은 근거만 담고 채택·기각은 적지 않았다.

## 7. 근거 링크 (직접 연 것)
- https://developers.openai.com/api/docs/guides/function-calling
- https://opencode.ai/docs/tools/
- https://cursor.com/docs/agent/tools
- https://learn.chatgpt.com/docs/agent-approvals-security
- https://github.com/ggml-org/llama.cpp/blob/master/docs/function-calling.md (03-agent-loop-tools.md에서 확인 기록)
