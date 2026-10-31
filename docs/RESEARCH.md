# 조사 기록

작업마다 논문·자료를 조사해 아래 형식으로 남깁니다. 링크는 직접 열어 실제로 있는지 확인한 것만 적습니다.

## 기록 형식

- 제목
- 링크
- 확인 날짜
- 핵심
- 적용 위치(파일, 함수) 또는 미적용 이유
- 관련 작업

## 기획 단계 조사

[docs/research/2026-10-28-planning.md](research/2026-10-28-planning.md)를 봅니다.

## 작업 01

### Harness²

- 링크: [arXiv 2610.14224](https://arxiv.org/abs/2610.14224)
- 확인 날짜: 2026-10-28
- 핵심: 시스템 프롬프트, 도구 설명, 감시 규칙을 코드와 분리한 설정 파일로 두고 git 이력으로 관리한다.
- 적용 위치: `prompts/`, `rules/`
- 관련 작업: 01 (내용은 03, 05에서 채움)

## 작업 02

확인 날짜는 모두 2026-10-28입니다. OpenAI API 문서(platform.openai.com)는 403으로 열리지 않아 적지 않았습니다.

### RFC 9110 HTTP Semantics, 10.2.3 Retry-After

- 링크: [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html)
- 핵심: `Retry-After = HTTP-date / delay-seconds`, delay-seconds는 음이 아닌 10진 정수입니다. 503과 3xx에 쓰는 예가 정의되어 있습니다.
- 적용: 02-4(초와 HTTP 날짜 두 형식 모두 해석, 5xx에도 헤더를 따름).
- 관련 작업: 02

### RFC 6585, 4절 429 Too Many Requests

- 링크: [RFC 6585](https://www.rfc-editor.org/rfc/rfc6585.html)
- 핵심: 429는 일정 시간 안에 요청이 너무 많다는 뜻이며, 기다릴 시간을 `Retry-After`로 줄 수 있습니다(예: 3600초).
- 적용: 02-4, 02-6. 대기 시간이 상한을 넘으면 기다리지 않고 대기 초를 밝힌 오류로 알립니다.
- 관련 작업: 02

### WHATWG HTML Standard, Server-sent events

- 링크: [Server-sent events](https://html.spec.whatwg.org/multipage/server-sent-events.html)
- 핵심: 줄 끝은 CRLF, LF, CR 모두 허용합니다. `:`로 시작하는 줄은 주석이라 무시합니다. 여러 `data` 줄은 개행으로 잇고, 빈 줄에서 이벤트를 내보냅니다. 항상 UTF-8이며 맨 앞 BOM 하나를 버립니다. 명세는 파일 끝의 미완 이벤트를 버립니다.
- 적용: 02-3(줄 끝 세 종류, 주석, data 결합, BOM, 멀티바이트 보호). 파일 끝의 미완 이벤트는 명세와 달리 내보냅니다(D12).
- 관련 작업: 02

### llama.cpp server README

- 링크: [llama.cpp tools/server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- 핵심: 스트리밍은 SSE입니다. `--reasoning-format`이 `deepseek`면 생각을 `reasoning_content`에 넣고, `none`이면 `content`에 그대로 둡니다(기본 `auto`). 모델 로딩 중에는 503과 `{"error":{...}}`를 돌려줍니다.
- 적용: 02-3(`reasoning_content` 처리), 02-4(503). `<think>` 태그를 content에서 떼어 내는 일은 하지 않습니다(서버 설정으로 해결, 감시는 작업 05).
- 관련 작업: 02

### Ollama OpenAI compatibility

- 링크: [Ollama OpenAI compatibility](https://docs.ollama.com/api/openai-compatibility)
- 핵심: `/v1/chat/completions`에서 스트리밍, `stream_options.include_usage`, `tools`를 지원합니다. `tool_choice`, `logit_bias`, `n` 등은 지원하지 않습니다. reasoning은 `reasoning_effort`로 조절합니다.
- 적용: 02-5. 요청에 `tool_choice`, `n`을 보내지 않습니다. `stream_options`는 서버마다 지원이 달라 보내지 않습니다(비범위). `reasoning_effort`는 02-5에서 모델의 `reasoning`을 `kind`가 `ollama`일 때 이 필드로 변환해 보냅니다(D15).
- 관련 작업: 02

### vLLM Reasoning Outputs

- 링크: [vLLM Reasoning Outputs](https://docs.vllm.ai/en/latest/features/reasoning_outputs.html)
- 핵심: 생각 출력 필드는 `reasoning`(스트리밍은 `delta.reasoning`)이며, 예전 이름이 `reasoning_content`였습니다. 한쪽만 읽으면 빈 값을 읽을 수 있다고 경고합니다.
- 적용: 02-3. 두 이름을 모두 생각(reasoning) 이벤트로 바꿉니다.
- 관련 작업: 02

### LM Studio OpenAI Compatibility Endpoints

- 링크: [LM Studio OpenAI compat](https://lmstudio.ai/docs/developer/openai-compat), [Chat Completions](https://lmstudio.ai/docs/developer/openai-compat/chat-completions)
- 핵심: `/v1/chat/completions`를 지원하고 `stream` 매개변수를 받습니다. 문서에 reasoning 필드 설명은 없습니다.
- 적용: 02-3(별도 처리 없음). reasoning 필드가 오면 두 이름 처리 또는 모르는 필드 보존으로 받습니다.
- 관련 작업: 02

### OpenRouter Streaming

- 링크: [OpenRouter Streaming](https://openrouter.ai/docs/api/reference/streaming)
- 핵심: 연결 유지를 위해 `: OPENROUTER PROCESSING` 같은 주석 줄을 보냅니다. 헤더 이후 오류는 HTTP 200 안에서 최상위 `error` 객체와 `finish_reason: "error"`를 가진 data 청크로 오고, 그 뒤 스트림이 끝납니다. 연결을 끊으면 지원하는 공급자에서는 생성과 과금이 멈춥니다.
- 적용: 02-3(주석 줄 무시, 스트림 중간 `error` 청크는 스트림 오류로 종료), 02-5(취소 시 연결 정리).
- 관련 작업: 02

### OpenRouter Reasoning Tokens

- 링크: [OpenRouter Reasoning Tokens](https://openrouter.ai/docs/use-cases/reasoning-tokens)
- 핵심: 스트리밍에서 생각은 `choices[].delta.reasoning_details` 배열로 오고, 메시지의 `reasoning` 필드에도 나타납니다.
- 적용: 02-3. `reasoning`은 생각 이벤트로, `reasoning_details`는 모르는 필드로 원문 보존합니다. 구조화된 details 해석은 필요해질 때 합니다(YAGNI).
- 관련 작업: 02

### OpenRouter API Rate Limits

- 링크: [OpenRouter Limits](https://openrouter.ai/docs/api/reference/limits)
- 핵심: `:free` 모델은 분당 20회, 일일 50회(크레딧 10 미만) 또는 1000회 제한입니다. 초과하면 429와 `X-RateLimit-*` 헤더를 주며, 지수 백오프와 `Retry-After` 준수를 권합니다.
- 적용: 02-4, 02-6(헤더 우선, 없으면 지수 백오프, 최대 횟수). `X-RateLimit-Reset`은 표준이 아니라 쓰지 않습니다. 일일 한도처럼 긴 대기는 상한 초과로 즉시 포기합니다.
- 관련 작업: 02

### tokio-util CancellationToken

- 링크: [CancellationToken (docs.rs)](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
- 핵심: 복제본끼리 취소 상태를 공유하고, `cancelled()`는 cancel-safe라 `select!`에 쓸 수 있습니다. `child_token()`은 부모 취소를 따릅니다.
- 적용: 02-5(스트림 도중 취소, 트레이트 인자), 02-6(재시도 대기 중 취소).
- 관련 작업: 02

### reqwest ClientBuilder

- 링크: [reqwest ClientBuilder (docs.rs)](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
- 핵심: `connect_timeout`은 연결 단계만, `timeout`은 요청 전체, `read_timeout`은 읽기마다 적용되고 성공한 읽기 후 다시 셉니다.
- 적용: 02-5(연결 시간 초과만 라이브러리에 맡기고 헤더 대기와 유휴는 따로 잽니다). reqwest의 `timeout`과 `read_timeout`은 두 구간을 구분하지 못해 쓰지 않습니다.
- 관련 작업: 02

확인 날짜는 모두 2026-10-28입니다. `platform.openai.com` 문서는 403으로 열리지 않아 적지 않았고, 같은 내용을 담은 `developers.openai.com`을 대신 적었습니다. 서버별 문서에 응답 필드가 적혀 있지 않은 곳은 "문서에 없음"으로 두었으며, 실제 서버 응답으로 확인하기 전에는 필수로 믿지 않습니다.

### OpenAI API 참조, 모델 목록

- 링크: [OpenAI List models](https://developers.openai.com/api/reference/resources/models/methods/list)
- 핵심: 응답은 `object: "list"`와 `data` 배열입니다. 각 모델은 `id`, `created`(Unix 초), `object: "model"`, `owned_by`를 가지며 `shutdown_date`는 선택입니다. 인증은 Bearer 키를 씁니다.
- 적용 위치: 02-7. 모델 목록 조회 응답 해석. 필수로 읽는 것은 `data[].id`뿐이고 나머지 필드는 없어도 받아들이며 버립니다.
- 관련 작업: 02

### llama.cpp server README, 모델 목록

- 링크: [llama.cpp tools/server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- 핵심: `GET /v1/models`를 지원합니다. 항목에 `id`(기본은 `-m`으로 준 모델 파일 경로, `--alias`로 바꿈), `object`, `created`, `owned_by`, `meta`(모델 로딩 중에는 `null`일 수 있음), `architecture`(입출력 형태 목록)가 있습니다. 단일 모델 모드에서는 `data`에 항목이 하나뿐입니다. 라우터 모드에서는 로드된 모델 정보가 `/v1/models`에 합쳐집니다. `/models` 경로도 있다고 합니다. `--api-key`로 키를 요구할 수 있고, 예시는 Bearer 토큰 또는 `x-api-key` 헤더를 씁니다. 키 검사에서 빠지는 경로 목록은 열어 본 범위에서 확인하지 못했습니다(`/health`는 공개).
- 적용 위치: 02-7(`data[].id`만 필수로 읽고 모르는 필드와 `null`인 `meta`는 무시, 인증은 Bearer 한 가지만 보냄), 02-2(`id`가 파일 경로일 수 있으므로 id를 의미 있는 이름으로 가정하지 않고 모델 이름과 id를 따로 둠).
- 관련 작업: 02

### Ollama OpenAI 호환 문서, 모델 목록

- 링크: [Ollama OpenAI compatibility](https://docs.ollama.com/api/openai-compatibility)
- 핵심: `/v1/models`와 `/v1/models/{model}`을 지원합니다. 문서가 밝히는 필드는 `created`(모델 마지막 수정 시각)와 `owned_by`(기본 "library")뿐입니다. 로컬 서버는 키 값을 받아도 무시하지만 클라이언트는 아무 값이나 넣어야 합니다. 로컬에서는 내려받은 모델만 목록에 나옵니다.
- 적용 위치: 02-1·02-2(키가 없는 엔드포인트 허용), 02-5(이때 인증 헤더를 보내지 않음), 02-7(응답 해석은 `data[].id`만 필수로 읽음).
- 관련 작업: 02

### vLLM OpenAI 호환 서버

- 링크: [vLLM Online Serving](https://docs.vllm.ai/en/latest/serving/online_serving/), [vllm serve CLI](https://docs.vllm.ai/en/latest/cli/serve/)
- 핵심: `/v1/models`를 "사용 가능한 모델 목록"으로 지원한다고만 적혀 있고 응답 필드는 문서에 없습니다(OpenAI 형식을 따른다고 보고 실서버로 확인해야 함). `--api-key`를 주면 `/v1`, `/v2`, `/inference` 접두 경로만 키를 검사하며 그 밖의 경로는 인증 없이 열려 있다고 경고합니다.
- 적용 위치: 02-5, 02-7(키가 있으면 항상 Bearer 헤더를 보내 키 요구 여부와 관계없이 동작하게 함). 응답은 OpenAI 형식으로 해석하되 `data[].id` 외에는 요구하지 않습니다.
- 관련 작업: 02

### LM Studio 모델 목록과 인증

- 링크: [LM Studio OpenAI compat, models](https://lmstudio.ai/docs/developer/openai-compat/models), [LM Studio REST list](https://lmstudio.ai/docs/developer/rest/list), [LM Studio Authentication](https://lmstudio.ai/docs/developer/core/authentication)
- 핵심: OpenAI 호환 `GET /v1/models`는 서버가 볼 수 있는 모델을 돌려주며 JIT 로딩이 켜져 있으면 내려받은 모델 전체가 나올 수 있다고 합니다. 응답 필드는 그 문서에 없습니다. 별도의 고유 REST `GET /api/v1/models`는 `models` 배열(`key`, `display_name`, `loaded_instances` 등)로 형식이 완전히 다릅니다. 인증은 기본 꺼짐이며 켜면 Bearer 토큰이 필요합니다(0.4.0 이상).
- 적용 위치: 02-7(OpenAI 호환 `GET {base_url}/models`만 쓰고 고유 REST는 쓰지 않음). 응답이 `data` 배열이 아니면 형식 오류로 알립니다.
- 관련 작업: 02

### OpenRouter 모델 목록

- 링크: [OpenRouter List models](https://openrouter.ai/docs/api/api-reference/models/get-models)
- 핵심: `GET /api/v1/models`가 Bearer 키를 요구하는 것으로 문서화되어 있습니다. 응답은 `data` 배열과 `total_count`, `links`를 가지며 모델 항목은 `id`, `canonical_slug`, `name`, `created`, `pricing`, `context_length` 등 필드가 매우 많습니다. 기본 필터가 출력 형태 `text`라서 목록이 한정되며, `offset`과 `limit`(기본 500, 최대 1000)로 쪽을 나눌 수 있고 둘 다 생략하면 전체를 돌려준다고 합니다. 오류는 `error` 객체(`code`, `message`)입니다.
- 적용 위치: 02-7(`data[].id`만 읽고 나머지는 버림). 쪽 나누기는 적용하지 않았습니다(미적용 이유: 매개변수 없이 호출하면 전체를 돌려준다고 문서화됨). HTTP 오류 본문은 요약하고 키와 주소를 가려 싣습니다.
- 관련 작업: 02

### 모델 목록 서버별 정리

- `data[].id`는 여섯 곳 모두에서 확인되거나 OpenAI 형식에서 전제되는 유일한 공통 필드입니다.
- `object`, `created`, `owned_by`는 OpenAI, llama.cpp, Ollama에서만 문서로 확인했고 OpenRouter에는 `object`, `owned_by`가 없습니다. 필수로 믿지 않습니다.
- 인증 헤더는 Bearer로 통일해 보내면 키가 필요한 서버(OpenRouter, vLLM, 설정한 LM Studio)와 무시하는 서버(Ollama 로컬)가 모두 동작합니다.
- 적용 위치: 02-7. 최상위 `data` 배열의 각 항목에서 문자열 `id`만 필수로 읽고, `id`가 없거나 문자열이 아니면 형식 오류로 알립니다.

### dotenv(Node) README, .env 형식

- 링크: [motdotla/dotenv](https://github.com/motdotla/dotenv)
- 핵심: `#`로 시작하는 줄은 주석이고 값 뒤 `#`도 인용 밖이면 주석입니다. 빈 줄은 건너뜁니다. 작은따옴표, 큰따옴표, 역따옴표를 쓰며 따옴표 안 앞뒤 공백은 보존하고 따옴표 없는 값은 앞뒤 공백을 제거합니다. 큰따옴표 값에서 `\n`이 개행으로 바뀝니다. 여러 줄 값은 v15 이상에서 지원합니다. 파서 자체는 변수 치환을 하지 않습니다. `export` 접두어와 키 이름 규칙은 그 문서에 없습니다.
- 미적용: 키는 자격 파일 `credentials.jsonc`에 둔다(D10).
- 관련 작업: 02

### dotenvy(Rust), .env 형식

- 링크: [allan2/dotenvy](https://github.com/allan2/dotenvy), [dotenvy (docs.rs)](https://docs.rs/dotenvy/latest/dotenvy/)
- 핵심: 원래 `dotenv` 크레이트의 후속이며 여러 줄 값을 지원하고 변수 치환은 기본 꺼짐입니다(켜야 동작). 오류에 파일 경로와 변수 이름을 담습니다. 두 문서 모두 주석, 따옴표, `export`, 키 이름 규칙은 설명하지 않아 확인하지 못했습니다.
- 미적용: 키는 자격 파일 `credentials.jsonc`에 둔다(D10).
- 관련 작업: 02

### Docker Compose, .env 형식

- 링크: [Compose 변수 보간](https://docs.docker.com/compose/how-tos/environment-variables/variable-interpolation/), [Compose 환경 변수 설정](https://docs.docker.com/compose/how-tos/environment-variables/set-environment-variables/)
- 핵심: `#`로 시작하는 줄과 빈 줄은 무시합니다. 따옴표 없는 값의 인라인 주석은 `#` 앞에 공백이 있어야 하고, 따옴표 값은 닫는 따옴표 뒤의 `#`만 주석입니다. 작은따옴표 값은 리터럴이라 치환과 이스케이프를 하지 않고, 큰따옴표 값은 `\n`, `\r`, `\t`, `\`를 해석하며 `$VAR`, `${VAR}`를 치환합니다. 작은따옴표 값은 여러 줄을 가질 수 있습니다. 키와 값 구분은 `=` 또는 `:`입니다. 보간은 Compose CLI 기능이며 `docker run --env-file`에는 없습니다. `export` 접두어와 키 이름 규칙은 그 문서에 없습니다.
- 미적용: 키는 자격 파일 `credentials.jsonc`에 둔다(D10).
- 관련 작업: 02

### `.env` 지원 범위 추천안

미적용: 키는 자격 파일 `credentials.jsonc`에 둔다(D10).

추천안 본문(참고로 남김): 공통 부분만 지원하고, 벗어나는 줄은 줄 번호만 알리는 오류로 거부합니다(값과 키 이름은 오류에 넣지 않음).

- 한 줄은 `이름=값` 형식입니다. 구분자는 `=`만 인정합니다(`:` 거부).
- 이름은 영문자나 밑줄로 시작하고 영문자, 숫자, 밑줄, `-`만 씁니다(키 이름 규칙은 문서에서 확인하지 못했으므로 좁은 관례에 `-`만 더함, D10). 대소문자와 `-`·`_`는 서로 다르게 봅니다. 이름 앞뒤의 공백은 허용하지 않습니다.
- 빈 줄과 `#`로 시작하는 줄(앞 공백 허용)은 무시합니다.
- 값은 따옴표 없는 한 줄, 작은따옴표 한 줄, 큰따옴표 한 줄 중 하나입니다. 따옴표 없는 값은 앞뒤 공백을 제거하고, 따옴표 값은 안의 내용을 그대로 씁니다.
- 따옴표 없는 값의 인라인 주석은 지원하지 않고 `#`이 값에 들어가면 거부합니다(도구마다 달라 모호함). `#`가 필요한 값은 따옴표로 감쌉니다. 따옴표 닫은 뒤에 남은 글자는 공백이 아니면 거부합니다.
- 이스케이프(`\n` 등), 여러 줄 값, 변수 치환, `export ` 접두어, 역따옴표는 지원하지 않고 거부합니다. 큰따옴표 값 안의 백슬래시는 해석하지 않고 그대로 둡니다(작은따옴표와 같은 동작이라 안전).
- 시작 부분 BOM은 한 번 버리고, 줄 끝은 LF와 CRLF를 모두 받습니다(Windows 편집기 호환).
- 같은 이름이 두 번 나오면 마지막 값을 씁니다(dotenv, Compose의 관례). (채택하지 않음)

확인 날짜는 모두 2026-10-30입니다.

### DistServe (arXiv 2401.09670)

- 링크: [DistServe](https://arxiv.org/abs/2401.09670)
- 핵심: TTFT는 요청 시작부터 첫 토큰까지의 지연, TPOT는 출력 토큰 하나당 시간으로 정의합니다. 논문 자체는 서버의 프리필·디코딩 분리 구조를 다룹니다.
- 적용: 02-5. 요청 시작·첫 토큰·마지막 토큰 시각을 돌려줘 이 정의로 지표를 계산할 수 있게 합니다. 서버 구조 내용은 클라이언트와 관계없어 적용하지 않습니다.
- 관련 작업: 02

### Etalon (arXiv 2407.07000)

- 링크: [Etalon](https://arxiv.org/abs/2407.07000)
- 핵심: TTFT, TBT, TPOT 같은 기존 지표가 실시간 사용 경험을 충분히 담지 못한다고 보고 fluidity-index를 제안합니다. 초록만 확인했습니다.
- 적용: 02-5. 평균으로 줄이지 않고 요청별 원시 시각을 돌려줍니다. 토큰마다 도착 시각을 남기는 일과 fluidity-index 계산은 지표 공식을 확인하지 못했고 지금 필요하지 않아 적용하지 않습니다(필요하면 작업 06).
- 관련 작업: 02

### When Is the Same Model Not the Same Service? (arXiv 2605.02821)

- 링크: [arXiv 2605.02821](https://arxiv.org/abs/2605.02821)
- 핵심: 호스팅 LLM API는 모델 이름이 아니라 제공자별 서비스로 측정해야 하며, 지연과 처리량은 제공자와 시점에 따라 크게 다릅니다. 초록만 확인했습니다.
- 적용: 02-5. 요청별 원시 시각을 돌려줘 작업 06에서 엔드포인트별 분포를 볼 수 있게 합니다.
- 관련 작업: 02

### AWS Architecture Blog, Exponential Backoff And Jitter

- 링크: [Exponential Backoff And Jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)
- 핵심: Full, Equal, Decorrelated 지터를 비교해 Equal이 가장 불리하다고 보고, 지터를 둔 백오프를 원격 클라이언트의 표준 방식으로 권합니다.
- 적용: 02-4, D13. `Retry-After`가 없을 때 Full Jitter를 씁니다. Decorrelated는 Full보다 낫다는 결론이 없어 더 단순한 Full을 고릅니다.
- 관련 작업: 02

### Google SRE Book, Addressing Cascading Failures

- 링크: [Addressing Cascading Failures](https://sre.google/sre-book/addressing-cascading-failures/)
- 핵심: 재시도에는 무작위 지수 백오프를 쓰고 요청당 횟수를 제한하며, 재시도 가능한 오류만 재시도하라고 권합니다. 층마다 재시도하면 시도가 곱으로 늘어나므로 한 층에서만 재시도하라고 합니다.
- 적용: 02-4, 02-6, D13. 재시도 대상 상태 코드 한정, 최대 횟수, 클라이언트 한 층 재시도. 프로세스 단위 재시도 예산은 단일 사용자 에이전트라 지금 필요하지 않아 적용하지 않습니다.
- 관련 작업: 02

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
  - 적용 위치: 인자 JSON 복구와 검증은 `tools/args.rs`, 오류를 모델에 돌려 이어 가는 흐름은 `agent/runner.rs`, 기본 온도 0은 `llm/openai/mod.rs`의 `with_default_temperature`와 `agent/runner.rs`의 `DEFAULT_TEMPERATURE`.
- 코드 편집 출력 형식
  - 링크: [arXiv 2604.27296](https://arxiv.org/abs/2604.27296)
  - 핵심: 제목은 "To Diff or Not to Diff? Structure-Aware and Adaptive Output Formats for Efficient LLM-based Code Editing"입니다. 줄 번호 기반 일반 diff는 모델이 쓰기 어렵다고 하고, 블록·함수 단위 diff와 diff/전체 코드 선택 학습(AdaEdit)을 제안합니다. 검색-치환 형식은 초록에 나오지 않아, 이 논문은 "줄 번호 diff를 피한다"는 근거로만 씁니다. 형식별 실패율 비교는 작업 06에서 합니다.
  - 적용 위치: `tools/edit_file.rs`(줄 번호 없이 정확한 문자열을 찾아 바꿈), `agent/system_prompt.txt`가 이 방식을 안내합니다.
- 관련 작업: 03

### 에이전트 도구 설계 참고 (opencode, Codex CLI, Pi, Flue)

- opencode
  - 링크: [Tools](https://opencode.ai/docs/tools/), [Permissions](https://opencode.ai/docs/permissions/)
  - 핵심: 편집 도구는 정확한 텍스트 일치를 바꾸는 방식이라고만 적혀 있고, 인자와 다중 일치 처리는 문서에 없습니다. grep과 glob은 ripgrep을 쓰고 `.gitignore`를 존중하며 glob은 수정 시각순으로 돌려줍니다. 권한은 allow, ask, deny 세 가지이고 bash는 명령 패턴(와일드카드, 마지막으로 맞는 규칙 우선)별로 정할 수 있으며 편집·쓰기·패치는 하나의 edit 권한으로 묶입니다. 승인 창에서는 한 번 허용, 세션 동안 허용, 거부를 고릅니다.
  - 적용 위치: 명령 실행 전 사용자 확인(허용·거부 두 갈래)은 `tools/run_command.rs`와 `tools/approval.rs`, 콘솔 구현은 `snap-cli/src/console.rs`. `.gitignore` 존중은 적용하지 않았습니다(미적용 이유: 목록·검색은 `.git`, `target` 등 고정 제외만 하고 규칙 해석은 범위 밖).
  - 미적용: 패턴별 자동 허용 규칙과 세션 동안 허용(범위 밖, 필요해지면 별도 작업).
- Codex CLI
  - 링크: [저장소 README](https://github.com/openai/codex), [승인과 보안](https://learn.chatgpt.com/docs/agent-approvals-security)
  - 핵심: 샌드박스 모드(read-only, workspace-write, danger-full-access)와 승인 정책(on-request, never, granular)을 따로 둡니다. 샌드박스 안 명령은 묻지 않고 실행하며, 밖으로 나가거나 네트워크가 필요하면 승인을 요청합니다. 승인 검토자가 기본은 사용자입니다. 검색-치환 편집 도구와 다중 일치 처리는 열어 본 문서에 없어 확인하지 못했습니다.
  - 적용 위치: `tools/approval.rs`(`CommandApprover`, `NoApproval`), `snap-cli/src/select.rs`의 `ask_approval`이 비대화형이면 거부합니다.
  - 미적용: 샌드박스 격리(운영체제별 구현이 크고 범위 밖).
- Pi 코딩 에이전트
  - 링크: [coding-agent README](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent), [사용법](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/usage.md), [동작 방식](https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/how-pi-works.md)
  - 핵심: 최소 하네스를 표방하며 하위 에이전트와 계획 모드를 기본에서 뺍니다. 도구 호출마다 묻지 않고 샌드박스를 권합니다. 시스템 프롬프트는 기본 지시와 발견한 맥락 파일로 만들고, 요청에는 도구 정의와 스킬 설명이 실리며 스킬 본문은 필요할 때 불러옵니다. 기본 도구 목록과 프롬프트 길이는 열어 본 문서에서 확인하지 못했습니다.
  - 적용 위치: `agent/system_prompt.txt`(8줄)와 `agent/prompt.rs`.
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
  - 적용 위치: `agent/runner.rs`(매 요청에 `tools`, 응답의 호출을 순서대로 실행, `tool_call_id`를 붙인 결과를 기록에 덧붙임), 인자 문자열 해석은 `tools/args.rs`.
- 도구 미지원 서버·모델
  - 링크: [llama.cpp 함수 호출](https://github.com/ggml-org/llama.cpp/blob/master/docs/function-calling.md), [OpenRouter 도구 호출](https://openrouter.ai/docs/guides/features/tool-calling)
  - 핵심: llama.cpp 서버는 `--jinja`와 도구를 아는 채팅 템플릿이 있어야 하고, 템플릿을 모르면 Generic 처리로 떨어져 토큰이 더 들고 효율이 낮습니다. `/props`의 `chat_template`, `chat_template_tool_use`로 확인할 수 있습니다. 극단적인 KV 캐시 양자화는 도구 호출을 해칠 수 있습니다. OpenRouter는 `supported_parameters=tools`로 모델을 거를 수 있다고 합니다. 도구를 못 쓰는 모델이 오류를 내는지 무시하는지는 열어 본 문서 어디에도 적혀 있지 않아 확인하지 못했습니다.
  - 적용 위치: 4xx 응답은 `agent/runner.rs`의 `map_llm_error`가 `AgentError::ToolsUnsupported`로 알립니다. 도구 호출 없이 일반 텍스트만 오는 경우의 판별은 미적용(미적용 이유: 최종 답과 구분할 수 없어 스모크의 실제 응답으로 확정). 판별 방식은 스모크에서 확정합니다.
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
