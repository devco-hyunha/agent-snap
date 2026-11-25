# 작업 02-3 SSE 파싱·델타 복원 조사 상세

- 인덱스: [docs/RESEARCH.md](../RESEARCH.md) 02 절
- 역할: 조사 상세. 규범 정본이 아님. 정책은 `docs/DECISIONS.md`와 `docs/decisions/`에 둔다.
- 확인 날짜: 2026-11-24. OpenAI API 문서(platform.openai.com)는 열리지 않아 적지 않았다. vLLM은 공식 문서 사이트가 막혀 저장소 원문(raw)으로 확인했다.

## WHATWG HTML Standard, Server-sent events

- 링크: [Server-sent events](https://html.spec.whatwg.org/multipage/server-sent-events.html)
- 핵심: 줄 끝은 CRLF, LF, CR 모두 허용한다. `:`로 시작하는 줄은 주석이라 무시한다. 여러 `data` 줄은 개행으로 잇고, 빈 줄에서 이벤트를 내보낸다. 항상 UTF-8이며 맨 앞 BOM 하나를 버린다. 명세는 파일 끝의 미완 이벤트를 버린다.
- 적용: 02-3(줄 끝 세 종류, 주석, data 결합, BOM, 조각 경계 멀티바이트 보호). 파일 끝의 미완 이벤트는 명세와 달리 내보낸다(결정 9번).
- 관련 작업: 02-3

## llama.cpp server README

- 링크: [llama.cpp tools/server README (raw)](https://raw.githubusercontent.com/ggml-org/llama.cpp/master/tools/server/README.md)
- 핵심: 스트리밍은 SSE다. `--reasoning-format`이 `deepseek`이면 생각을 `reasoning_content`에 넣고, `none`이면 `content`에 그대로 둔다(기본 `auto`). 모델 로딩 중에는 503과 `{"error":{...}}`를 돌려준다.
- 적용: 02-3(`reasoning_content`를 생각으로 읽음), 02-4(503 재시도 대상). `<think>` 태그를 content에서 떼어 내는 일은 하지 않는다(서버 설정으로 해결, 감시는 작업 05).
- 관련 작업: 02-3

## vLLM Reasoning Outputs

- 링크: [vLLM reasoning_outputs.md (raw)](https://raw.githubusercontent.com/vllm-project/vllm/main/docs/features/reasoning_outputs.md)
- 핵심: 생각 출력 필드는 `reasoning`(스트림은 `delta.reasoning`)이고, 예전 이름이 `reasoning_content`다. 한쪽만 읽으면 조용히 빈 값을 읽는다고 경고한다.
- 적용: 02-3. 두 이름을 모두 생각으로 읽는다.
- 관련 작업: 02-3

## LM Studio OpenAI Compatibility Endpoints

- 링크: [LM Studio OpenAI compat](https://lmstudio.ai/docs/developer/openai-compat), [Chat Completions](https://lmstudio.ai/docs/developer/openai-compat/chat-completions)
- 핵심: `/v1/chat/completions`를 지원하고 `stream` 매개변수를 받는다. 문서에 reasoning(thinking) 요청 필드 설명이 없다.
- 적용: 02-3(응답에 생각 필드가 오면 두 이름으로 읽고, 모르는 필드는 원문 보존). 요청 쪽 변환은 02-5.
- 관련 작업: 02-3

## OpenRouter Streaming

- 링크: [OpenRouter Streaming](https://openrouter.ai/docs/api/reference/streaming)
- 핵심: 연결 유지를 위해 `: OPENROUTER PROCESSING` 같은 주석 줄을 보낸다. 헤더 이후 오류는 HTTP 200 안에서 최상위 `error` 객체와 `finish_reason: "error"`를 가진 data 청크로 오고, 그 뒤 스트림이 끝난다.
- 신규(백업 조사에 없던 것): usage 청크가 빈 `delta`로 `finish_reason`을 되풀이해 보내고, 마지막 `finish_reason`이 두 번 나타난다(회계 프레임). usage는 마지막 청크에만 들어온다.
- 적용: 02-3(주석 줄 무시, 최상위 `error` 청크는 스트림 오류로 종료, usage는 값으로 읽고 `finish_reason` 되풀이는 내용 중복 없이 사용량만 반영).
- 관련 작업: 02-3

## OpenRouter Reasoning Tokens

- 링크: [OpenRouter Reasoning Tokens](https://openrouter.ai/docs/use-cases/reasoning-tokens)
- 핵심: 스트리밍에서 생각은 `choices[].delta.reasoning_details` 배열로 오고, 메시지의 `reasoning` 필드에도 나타난다. 요청 쪽은 `reasoning`·`reasoning_content`가 같은 값의 별칭이다.
- 적용: 02-3. `reasoning`은 생각으로, `reasoning_details`는 모르는 필드로 원문 보존한다. 구조화된 details 해석은 필요해질 때 한다(YAGNI).
- 관련 작업: 02-3
