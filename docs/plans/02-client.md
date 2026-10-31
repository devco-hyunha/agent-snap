# 02 설정과 스트리밍 클라이언트

- 상태: planned (승인 대기)

## 목적
설정을 읽고 OpenAI 호환 엔드포인트와 스트리밍으로 대화하는 클라이언트를 만든다. 감시기(작업 05)가 스트림을 중간에 끊을 수 있어야 하고, 에이전트 루프(작업 03)는 이 클라이언트를 트레이트로만 받는다.

설정은 JSONC(주석 허용 JSON) 파일의 엔드포인트 목록과 모델 목록 구조이고, 키는 사용자 설정 폴더의 자격 파일 `credentials.jsonc`에서 엔드포인트 이름으로 찾는다(DECISIONS D10). 모델별 생각(`reasoning`)과 `extra` 옵션을 서버 종류에 맞게 요청에 싣는다(D15). 엔드포인트의 모델 목록 조회와 설정 저장도 라이브러리에 둔다(D16).

## 범위(In)
- 02-1: 비밀 값 래퍼, 자격 파일 `credentials.jsonc` 읽기, 이름으로 키 찾기.
- 02-2: `agent-snap.jsonc` 읽기와 검증, 엔드포인트 `api_key` 칸 거부, 키 찾기(자격 파일에 이름이 있으면 그 키, 없으면 키 없음), `agent-snap.example.jsonc`(자리표시자만).
- 02-3: SSE 파싱과 델타 복원.
- 02-4: 재시도 판단.
- 02-5: LLM 클라이언트 트레이트와 OpenAI 호환 스트리밍 전송(시간 초과, 취소, 오류 가리기, 사용량과 시각, 모델별 `reasoning` 변환과 `extra` 병합).
- 02-6: 재시도를 전송에 적용.
- 02-7: 모델 이름으로 클라이언트 만들기, 엔드포인트의 모델 목록 조회.
- 02-8: 설정 저장(엔드포인트 추가·수정·삭제, 키는 자격 파일에만 저장, 두 파일 모두 주석 보존).

## 비범위(Out)
- 에이전트 루프와 도구(작업 03), 대화형 CLI와 세션 로그, `ls` 같은 사용자 명령(작업 04).
- 화면(UI). 설정 저장은 라이브러리 기능만 둔다.
- `stream_options`, 비스트리밍 요청.
- 응답 형식이 다른 API(Responses API, Ollama 고유 API, LM Studio 고유 REST).
- 옛 단일 엔드포인트 설정 형식 호환(쓰는 사용자가 없다).
- 토큰마다 도착 시각을 모두 남기는 기록. 이 작업은 요청 단위 시각만 돌려주고, 분포 분석이 필요하면 작업 06에서 다룬다.
- 에이전트 루프 쪽의 LLM 요청 재시도. 재시도는 이 클라이언트 한 층에서만 한다.

## 확인된 사실
첫 시도에서 실측한 내용이다.
- keep-alive 주석 줄과 reasoning 필드가 오고, CRLF 줄 끝과 청크 경계에서 잘리는 줄이 있다.
- OpenRouter 무료 모델은 429가 잦다. `Retry-After` 기준으로 재시도한다.
- 오류 메시지에 base_url과 키가 들어가면 안 된다.
- 헤더 대기와 스트림 유휴 시간 초과를 나눠야 한다.

## 참고 논문·자료
자세한 내용과 적용 위치는 `docs/RESEARCH.md` 작업 02 절에 있다.

| 자료 | 이 작업에 반영하는 점 | 세부 작업 |
|---|---|---|
| DistServe (arXiv 2401.09670) | TTFT(요청 시작부터 첫 토큰까지)와 토큰당 시간 정의를 따라 요청 시작·첫 토큰·마지막 토큰 시각을 돌려준다. | 02-5 |
| Etalon (arXiv 2407.07000) | 평균 지표가 생성 지연의 들쭉날쭉함을 가린다는 지적. 평균으로 줄이지 않고 요청별 원시 시각을 돌려준다. | 02-5 |
| When Is the Same Model Not the Same Service? (arXiv 2605.02821) | 지연이 제공자와 시점에 따라 크게 다르다. 요청별 원시 값을 남겨 분포를 볼 수 있게 한다. | 02-5 |
| AWS Architecture Blog, Exponential Backoff And Jitter | `Retry-After`가 없을 때 지수 백오프에 Full Jitter를 쓴다. | 02-4 |
| Google SRE Book, Addressing Cascading Failures | 재시도 횟수 제한, 무작위 백오프, 재시도 가능한 오류만 재시도, 재시도는 한 층에서만. | 02-4, 02-6 |
| RFC 9110, RFC 6585 | `Retry-After` 두 형식과 429 의미. | 02-4 |
| WHATWG Server-sent events | 줄 끝 세 종류, 주석 줄, data 결합, BOM. | 02-3 |
| 서버별 문서(llama.cpp, Ollama, vLLM, LM Studio, OpenRouter) | reasoning 두 이름, 스트림 중간 `error` 청크, 보내지 않을 요청 매개변수, `/models` 응답 차이. | 02-3, 02-5, 02-7 |
| 서버별 문서(생각 옵션 요청 필드) | 서버마다 같은 기능의 필드 이름이 달라 `kind`별 변환 규칙을 02-5 첫 단계에서 문서로 확인해 확정한다. 지금 확인된 것은 Ollama의 `reasoning_effort`뿐. | 02-5 |

## 설계 요약
- 네트워크 없이 확인할 수 있는 부분(비밀 값과 자격 파일, 설정, SSE와 델타, 재시도 판단)과 HTTP 전송을 나눈다. 앞의 것은 단위 테스트로, 전송은 모의 서버 통합 테스트로 확인한다.
- 상위 로직(작업 03)은 LLM 클라이언트를 트레이트로만 받는다. HTTP 라이브러리는 전송 구현 안에서만 쓴다.
- 설정 파일 `agent-snap.jsonc`와 자격 파일 `credentials.jsonc`는 사용자 설정 폴더(Windows `%APPDATA%\agent-snap\`)에 있다(D10). 엔드포인트 항목은 `name`, `base_url`, `kind`를 가지며 키 칸은 없고, `api_key`가 있으면 설정 오류로 거부한다. 키는 자격 파일의 `api_keys`에서 엔드포인트 이름으로 찾고, 없으면 키 없이 요청한다.
- 모델 항목의 공통 `reasoning`(`off`, `low`, `medium`, `high`)은 엔드포인트 `kind`에 맞는 요청 필드로 바꿔 보내고, `extra`는 요청 본문에 그대로 합친다. `extra`가 클라이언트가 채우는 키를 덮어쓰려 하면 설정 오류로 거부한다(D15).
- 설정 저장은 엔드포인트·모델 옵션을 설정 파일에, 키를 자격 파일에만 쓰고, 두 파일 모두 사용자가 단 주석과 손대지 않은 항목을 보존한다(D16).
- base_url은 `/v1`까지 포함한 주소로 받는다. 대화는 `{base_url}/chat/completions`, 모델 목록은 `{base_url}/models`로 요청한다.
- 재시도는 이 클라이언트 한 층에서만 하고, 백오프에는 Full Jitter를 쓴다(DECISIONS D13).
- 시간·재시도 기본값(설정의 `http` 항목으로 바꾼다): 재시도 3회, 백오프 1~30초, `Retry-After` 상한 60초, 연결 10초, 헤더 대기 300초, 스트림 유휴 120초.
- 키와 base_url은 Debug, 오류, 로그에 넣지 않는다. 서버 오류 본문은 가린 뒤 요약하고 길이를 제한해 담는다.
- 새 외부 크레이트는 처음 쓰는 세부 작업에서 라이선스를 확인하고 `docs/DECISIONS.md`에 적는다.
- 모듈 하나는 책임 하나이며, 오류 처리 규칙은 AGENTS.md 개발 규칙을 따른다. 파일·함수·테스트에 줄 수 기준은 두지 않고 중복을 기준으로 삼으며, 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`). 파일 구성과 타입 이름은 구현하면서 정하고 코드 품질 리뷰로 확인한다.

## 세부 작업
세부 작업 하나는 한 번 위임하고 한 번 코드 리뷰로 끝나는 기능 하나다(DECISIONS D11). 아래 순서대로 하나씩 진행한다. 세부 작업마다 plan.md(상세 기획), spec.md(명세와 AC), tasks.md(작업 계획과 DoD)를 둔다.

| 번호 | 이름 | 선행 | 문서 |
|---|---|---|---|
| 02-1 | 비밀 값 래퍼와 자격 파일 읽기 | 없음 | [plan](02-client/02-1-secrets/plan.md) · [spec](02-client/02-1-secrets/spec.md) · [tasks](02-client/02-1-secrets/tasks.md) |
| 02-2 | 설정 읽기와 검증 | 02-1 | [plan](02-client/02-2-config/plan.md) · [spec](02-client/02-2-config/spec.md) · [tasks](02-client/02-2-config/tasks.md) |
| 02-3 | SSE 파싱과 델타 복원 | 02-2 | [plan](02-client/02-3-sse/plan.md) · [spec](02-client/02-3-sse/spec.md) · [tasks](02-client/02-3-sse/tasks.md) |
| 02-4 | 재시도 판단 | 02-3 | [plan](02-client/02-4-retry/plan.md) · [spec](02-client/02-4-retry/spec.md) · [tasks](02-client/02-4-retry/tasks.md) |
| 02-5 | 클라이언트 트레이트와 스트리밍 전송 | 02-4 | [plan](02-client/02-5-transport/plan.md) · [spec](02-client/02-5-transport/spec.md) · [tasks](02-client/02-5-transport/tasks.md) |
| 02-6 | 재시도 적용 | 02-5 | [plan](02-client/02-6-retry-apply/plan.md) · [spec](02-client/02-6-retry-apply/spec.md) · [tasks](02-client/02-6-retry-apply/tasks.md) |
| 02-7 | 모델 선택과 모델 목록 조회 | 02-6 | [plan](02-client/02-7-model-select/plan.md) · [spec](02-client/02-7-model-select/spec.md) · [tasks](02-client/02-7-model-select/tasks.md) |
| 02-8 | 설정 저장 | 02-7 | [plan](02-client/02-8-config-save/plan.md) · [spec](02-client/02-8-config-save/spec.md) · [tasks](02-client/02-8-config-save/tasks.md) |

### 세부 작업 공통 기준
각 세부 작업의 tasks.md는 이 절을 따른다.
- 작업 방식: 한 번 위임하고 한 번 코드 리뷰로 끝낸다(D11).
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- 공통 DoD: 그 세부 작업의 AC를 충족한다. 검증 명령이 통과한다. `docs/RESEARCH.md` 적용 위치를 갱신했다. 코드 품질 리뷰(review-harness 체크리스트와 dev-principles 2~10절(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`), 테스트 코드 포함)가 PASS했고 기록이 `docs/reviews/`에 있다.

## AC
1. 세부 작업 02-1~02-8의 AC를 모두 충족한다.
2. 설계 요약의 경계를 따른다. LLM 클라이언트는 트레이트로만 노출되고, HTTP 라이브러리는 전송 구현 밖에서 쓰이지 않으며, 02-1~02-4는 네트워크 없이 테스트된다.
3. 키 값과 base_url 호스트가 이 작업의 어떤 Debug·Display·오류에도 나오지 않는다(세부 작업별 테스트로 확인).
4. 테스트 밖 unwrap/expect/panic 0, 공개 항목 한국어 문서 주석. 검증 명령(세부 작업 공통 기준)이 통과하고 dev-principles 2~10절(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`) 코드 품질 리뷰가 PASS한다.
5. `docs/RESEARCH.md` 작업 02 절의 각 항목에 세부 작업별 적용 위치 또는 미적용 이유가 있다.

## 테스트 계획
작업 전체 수준만 적는다. 세부 계획은 각 세부 작업의 tasks.md에 있다.
- 02-1~02-4: 네트워크 없는 단위 테스트. 난수와 파일 위치는 주입하고 임시 폴더를 쓴다.
- 02-5~02-7: 로컬 모의 서버 통합 테스트.
- 02-8: 임시 폴더에서 파일 쓰기 단위 테스트.
- 시간에 의존하는 테스트는 시험용 시간 값을 짧게 주고, 경과 시간은 넉넉한 하한·상한으로만 확인한다.
- 설정이 있을 때 실제 엔드포인트로 1회 확인한다. 출력에 URL과 키를 찍지 않는다.

## 문서 영향
- `docs/RESEARCH.md` 작업 02 절: 세부 작업마다 적용 위치를 갱신한다.
- `docs/DECISIONS.md`: 크레이트 라이선스 확인 결과와 구현 중 결정을 추가한다.
- `agent-snap.example.jsonc`: 새로 만든다(02-2, 자리표시자만).
- `.gitignore`: `agent-snap.toml` 줄을 `agent-snap.jsonc`로 바꾼다(02-2).

## 리스크
작업 전체 수준만 적는다. 세부 작업별 리스크는 각 plan.md에 있다.
- 서버마다 스트림 형식과 모델 목록 응답이 다르다. 모르는 필드를 보존하고 필수로 읽는 값을 최소로 둔다.
- 시간에 의존하는 모의 서버 테스트가 흔들릴 수 있다. 테스트 계획의 시간 원칙을 따른다.
- 스트림이 시작된 뒤 끊기면 부분 응답이 남는다. 처리 방식은 작업 03에서 정한다.
- 서버별 `reasoning` 변환 규칙을 아직 문서로 확인하지 못했다. 02-5 첫 단계에서 확인하고, 변환할 수 없는 `kind`가 나오면 처리를 정한다(D15).
- 설정 파일을 다시 쓸 때 주석이 사라지거나 파일이 깨질 수 있다. 02-8에서 주석 보존과 임시 파일 교체로 막는다(D16).

## DoD
세부 작업 02-1~02-8이 각자의 DoD(각 tasks.md)를 만족하고, 작업 전체 AC를 충족하며, 작업 전체 코드 품질 리뷰가 PASS한 상태.
