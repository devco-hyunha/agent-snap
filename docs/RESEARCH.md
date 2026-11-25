# 조사 인덱스

- 이 파일은 조사 인덱스입니다. 항목당 핵심 한 줄, 적용 위치(파일·함수) 또는 미적용 이유, 상태, 상세 링크를 적습니다.
- 상세는 `research/<기획 순번>-<주제>.md`에 둡니다. 링크는 직접 열어 확인한 것만 적습니다.

| 기획 | 주제 | 핵심 | 적용 위치 / 미적용 이유 | 상태 |
|---|---|---|---|---|
| 01 | 코딩 에이전트 조사 (Codex CLI·OpenCode·Orca) | 승인 요청을 동작 단위로 쪼개고, 도구·승인 설계의 참고 자료로 쓴다 | `docs/research/01-coding-agent-survey.md`. 01은 조사 파일 자리만 반영. 승인 동작 단위·모델 목록 자동 발견은 02-7·03 범위(계획 문서)에 반영 | 반영 |
| 02 | llama.cpp 서버 README | 모델 `id`가 파일 경로일 수 있어 `name`과 `id`를 따로 둔다 | 02-2 적용: `crates/snap-core/src/config/types.rs`의 `ModelConfig`(name·id 분리), `crates/snap-core/src/config/validate/names.rs`(id trim 검증), `crates/snap-core/tests/validation/values/`(R4 분리 후: endpoint·sampling·http). 서버별 요청 필드 변환은 02-5 예정 | 반영 |
| 02 | Ollama OpenAI 호환 문서 | 로컬 서버는 키를 받지 않으므로 키 없는 엔드포인트를 허용한다 | 02-2 적용: `crates/snap-core/src/config/keys.rs`(자격 파일에 이름 없으면 키 없음), `crates/snap-core/tests/keys.rs`. `kind` 허용값 6종은 `crates/snap-core/src/consts/limits.rs`. 예시 안내는 `snap.example.jsonc` | 반영 |
| 02 | WHATWG Server-sent events | 줄 끝 CRLF·LF·CR, `:` 주석 줄 무시, `data` 여러 줄은 개행 결합, 빈 줄에서 내보냄, 맨 앞 BOM 하나 버림. 명세는 파일 끝 미완 이벤트를 버린다 | 02-3 적용: `crates/snap-core/src/llm/lines/split.rs`(줄 끝), `crates/snap-core/src/llm/lines/events.rs`(BOM·주석 무시·빈 줄 내보냄·`[DONE]`). 미완 이벤트는 `lines/events.rs`의 `finish()`가 내보냄(결정 9번). 시험: `crates/snap-core/tests/sse/lines.rs`·`boundaries.rs` | 반영 |
| 02 | llama.cpp server README | 스트림은 SSE, 생각은 `reasoning_content`(deepseek), 로딩 중 503 | 02-3 적용: `crates/snap-core/src/llm/delta/parse.rs`가 `reasoning_content`를 생각으로 읽음. 503 재시도는 02-4 | 반영 |
| 02 | vLLM Reasoning Outputs | 생각 필드 `reasoning`(스트림은 `delta.reasoning`), 옛 이름 `reasoning_content`. 한쪽만 읽으면 빈 값 경고 | 02-3 적용: `crates/snap-core/src/llm/delta/parse.rs`가 두 이름 모두 생각으로 읽음(`reasoning`·`reasoning_content`). 시험: `crates/snap-core/tests/sse/delta/reasoning.rs` | 반영 |
| 02 | LM Studio OpenAI Compatibility | `stream` 지원, reasoning 요청 필드 문서 없음 | 02-3 적용: 응답 생각 필드는 두 이름으로 읽고(`delta/parse.rs`), 모르는 필드는 `delta/parse.rs`의 `unknown_fields`가 원문 보존. 요청 변환은 02-5 | 반영 |
| 02 | OpenRouter Streaming | `: OPENROUTER PROCESSING` 주석 줄, 헤더 이후 오류는 HTTP 200 안 최상위 `error` 청크. usage 청크가 빈 `delta`로 `finish_reason`을 되풀이(마지막이 두 번) | 02-3 적용: 주석 무시는 `llm/lines/events.rs`, `error` 청크는 `llm/stream.rs`가 스트림 오류로 끝냄, usage는 `llm/delta/parse.rs`의 `usage_of`가 값으로 읽고 내용 중복 없음(시험 `tests/sse/delta/finish.rs`) | 반영 |
| 02 | OpenRouter Reasoning Tokens | 스트림 생각은 `delta.reasoning_details` 배열, 요청 쪽 `reasoning`·`reasoning_content`는 별칭 | 02-3 적용: `reasoning`은 `delta/parse.rs`가 생각으로, `reasoning_details`는 `delta/parse.rs`의 `unknown_fields`가 원문 보존(details 해석은 YAGNI). 시험: `crates/snap-core/tests/sse/delta/unknown.rs` | 반영 |
