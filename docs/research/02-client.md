# 작업 02 client 조사 상세

- 인덱스: [docs/RESEARCH.md](../RESEARCH.md) 해당 작업 절
- 역할: 조사 상세. 규범 정본이 아님. 정책은 docs/decisions/… 및 DECISIONS.

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

### llama.cpp 서버 README, HTTP serve와 기본 base URL

- 링크: [llama.cpp tools/server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- 확인 날짜: 2026-11-01
- 핵심: 서버는 HTTP로 serve하고 기본 base URL은 `http://localhost:8080/v1`이다.
- 적용 위치: 02-2 base_url 검증(scheme을 `http`/`https`로 한정하는 근거). 구현은 `crates/snap-core/src/config/validate.rs`의 `is_valid_base_url`이고, 호출은 `config/parse.rs`의 `read_endpoint`입니다.
- 관련 작업: 02

### Ollama OpenAI compatibility, 인증

- 링크: [Ollama OpenAI compatibility](https://docs.ollama.com/api/openai-compatibility), [Ollama Authentication](https://docs.ollama.com/api/authentication)
- 확인 날짜: 2026-11-01
- 핵심: 로컬 서버는 `http://localhost:11434/v1`이고 인증 헤더를 요구하지 않으며 키를 보내도 무시한다. 클라우드(ollama.com)는 `https`와 API 키가 필요하다.
- 적용 위치: 02-2 base_url 검증과 키 정하기(키 없는 엔드포인트 허용 근거). base_url 검증은 `crates/snap-core/src/config/validate.rs`의 `is_valid_base_url`, 키 정하기는 `config/keys.rs`의 `EndpointConfig::api_key`(이름이 없거나 빈 값이면 `None`)입니다.
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
