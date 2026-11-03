# 결정 기록 (인덱스)

짧은 결정은 이 파일에 결정·근거·다시 볼 점 전문을 둡니다. 길고 규범 밀도가 높은 결정은 [`docs/decisions/`](decisions/) 정본에 두고, 여기에는 **요약·정본 링크·다시 볼 점(또는 정본 위임)** 만 둡니다. 조사는 RESEARCH, 정책 MUST는 decisions/ 정본을 따릅니다.

## D1 백엔드는 Rust로 한다
- 결정: 백엔드를 Rust로 만든다.
- 근거: rustc 1.99.1과 MSVC Build Tools가 설치되어 있다. Tauri 백엔드가 Rust라서 앱이 코어 크레이트를 그대로 쓴다. 실행 파일 하나로 배포된다.
- 다시 볼 점: 판정 모델 런타임의 Windows 빌드가 막히면 다시 검토한다.

## D2 대상 언어
- 결정: 대상 언어는 기본 한국어이고 설정으로 바꾼다.
- 근거: 사용자가 쓰는 언어를 기본으로 삼는다.
- 다시 볼 점: 언어 감시 규칙(작업 05)을 만들 때 설정 방식이 맞는지 확인한다.

## D3 설정 파일과 비밀 값
- 결정: 설정 파일 `agent-snap.toml`(base_url, model, api_key_env)은 git에서 제외한다. 키는 api_key_env가 가리키는 환경 변수에서만 읽는다. 예시 파일에는 자리표시자만 둔다. 세션 로그에는 base_url과 키를 남기지 않는다.
- 근거: 엔드포인트 주소와 키가 저장소나 로그로 새는 일을 막는다.
- 다시 볼 점: 여러 엔드포인트가 필요해지면 설정 구조를 다시 검토한다.
- 2026-10-28 수정: 키는 사용자 설정 폴더의 키 파일(git 밖)을 먼저 읽고, 없으면 api_key_env가 가리키는 환경 변수에서 읽는다. 여러 엔드포인트를 지원한다. 위 결정 문장의 "환경 변수에서만"과 단일 엔드포인트 전제는 이 수정으로 바뀌었다.

## D4 AGENTS.md 마커 블록
- 결정: 마커 블록은 템플릿 원문을 따르되, 이 PC에 없는 경로는 실제 경로로 고친다.
- 근거: 템플릿 갱신 시 블록을 통째로 교체할 수 있다. 경로가 틀리면 에이전트가 하네스를 찾지 못한다.
- 다시 볼 점: 템플릿 형식이 바뀌면 마커 규칙을 다시 확인한다.
- 2026-10-28 수정: 원문 경로가 이 PC에 없는 경로라 에이전트가 하네스를 찾지 못했다. 블록 안 경로를 실제 경로로 바꾼다.

## D5 기존 에이전트를 포크하지 않고 새로 짠다
- 결정: opencode(MIT), Codex CLI(Apache-2.0), Hermes Agent(MIT)의 설계는 참고한다. 코드를 가져오면 출처와 라이선스를 표기한다. Claude Code와 Cursor는 동작만 참고한다.
- 근거: 스트림 경로에 감시·교정 지점을 직접 두려면 구조를 직접 통제해야 한다.
- 다시 볼 점: 직접 구현 비용이 과하면 일부 모듈의 코드 재사용을 라이선스 표기와 함께 검토한다.

## D6 에이전트(M1) 먼저, 판정 모델(M2) 나중
- 결정: 에이전트를 먼저 만들고 판정 모델은 나중에 얹는다. 판정 평가는 M1이 실제 과제를 수행한 뒤 그 세션 출력으로만 한다.
- 근거: 이전 시도에서 에이전트 바탕 없이 판정 모델만 더미 데이터로 검토하다 한도를 소진했다.
- 다시 볼 점: M1 실사용 점검(작업 06) 결과에 따라 M2 일정을 조정한다.

## D7 작업 시작 시 논문 참고와 기능별 세부 작업 분할
- 결정: 작업을 시작할 때 관련 논문을 찾아 참고하고 작업을 기능별 세부 작업으로 나눈다. 기획 문서에 참고 논문 절과 세부 작업 절이 채워져 있어야 승인을 요청하고, 구현은 세부 작업 하나씩 위임한다.
- 근거: AGENTS.md에 규칙만 적었을 때는 지켜지지 않았다. 사용자가 승인 단계에서 기획 문서로 직접 확인할 수 있게 한다.
- 다시 볼 점: 세부 작업 크기 기준이 필요하면 첫 적용 작업(02)에서 정한다.

## D8 실행 지표 수집
- 결정: 대화당 초당 평균 생성 토큰·대화 시간과 세션당 초당 평균 토큰·작업 시간·이어진 턴 수를 측정한다. 02에서 사용량과 시각을 받고, 04에서 세션 기록에 남기고, 06 보고서에서 감시 끔과 켬으로 나눠 집계한다. "대화"는 사용자 입력 하나부터 응답이 끝날 때까지(도구 호출 반복 포함), "세션"은 jsonl 세션 하나 전체다.
- 근거: 사용자 제안(2026-10-30). 과제 성공 판정(과제별 자동 검증 명령의 종료 코드 0)에 기대지 않고 측정할 수 있다.
- 다시 볼 점: 서버가 사용량을 주지 않으면 토큰 추정값을 쓰므로 오차가 있다. 검증 명령 통과가 실제 성공을 보장하지 않는 문제(지표 공략)는 따로 다룬다.

## D9 도구 격리와 승인 정책 보류
- 결정(요약): 작업 03은 작업 폴더 경계·비밀 파일 차단·`run_command` 실행 전 확인 등 **현재 정책**을 유지하고, Docker·AppContainer 등 **실행 격리 방식은 보류**한다. 도구별 일방향/양방향 문 유형 분류는 격리가 정해진 뒤 03에 반영한다. **프로세스 트리 정리·셸 deadline·승인 응답 deadline**은 격리가 아니라 **D22 신뢰성**으로 다룬다.
- 정본: [docs/decisions/D9-tool-isolation-approval.md](decisions/D9-tool-isolation-approval.md)
- 근거: 정본 참조(경량 목표, 라이선스 확인, RESEARCH 작업 03 비교).
- 다시 볼 점: 정본의 「다시 볼 점」 및 D9/D22 경계 표를 따른다.

## D10 설정 파일과 자격 파일
- 결정(요약): 설정(`agent-snap.jsonc`)과 자격(`credentials.jsonc`)을 사용자 설정 폴더의 **JSONC 두 파일**로 나눈다. 설정에는 `endpoints`·중첩 `models`·`http`만 두고 키 칸은 금지하며, `api_key`/`api_key_env`는 오류로 거부한다. 자격 파일은 엔드포인트 **이름**(trim·대소문자·`-`/`_` 구분)별 키만 Bearer로 쓴다. 스키마·거부 규칙 세부는 **D20**, 검증 세부는 **D21** 정본.
- 정본: [docs/decisions/D10-config-credentials.md](decisions/D10-config-credentials.md)
- 근거: 정본 참조(키·설정 분리, Windows env 대소문자).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D11 세부 작업 크기 기준
- 결정: 세부 작업 하나는 한 번 위임하고 한 번 코드 리뷰로 끝나는 기능 하나이며, 자기 AC를 3~5개 가진다. 네트워크 없이 확인하는 해석·판단과 전송은 다른 세부 작업으로 나눈다.
- 근거: D7의 다시 볼 점(크기 기준은 02에서 정함). 위임과 리뷰 단위를 맞추면 리뷰 범위가 분명해진다.
- 다시 볼 점: 02 구현 뒤 위임·보정 횟수를 보고 조정한다.

## D12 빈 줄 없이 끝난 마지막 SSE 이벤트
- 결정: 스트림이 빈 줄 없이 끝나면 남은 data를 마지막 이벤트로 처리한다. WHATWG 명세는 이 이벤트를 버린다.
- 근거: 마지막 청크 뒤 빈 줄을 빠뜨리는 서버에서 마지막 내용과 사용량을 잃지 않게 한다. 잘린 JSON이면 형식 오류로 끝나므로 잘못된 내용을 받아들이지 않는다.
- 다시 볼 점: 실제 서버에서 문제가 되면 다시 검토한다.

## D13 재시도는 클라이언트 한 층에서, 백오프는 Full Jitter
- 결정: LLM 요청 재시도는 작업 02 클라이언트에서만 하고, 상위 층(작업 03 에이전트 루프)은 같은 요청을 다시 재시도하지 않는다. `Retry-After`가 없을 때는 지수 백오프 상한 안에서 0부터 균등 무작위로 기다린다(Full Jitter).
- 근거: Google SRE Book은 층마다 재시도하면 시도가 곱으로 늘어난다며 한 층에서만 재시도하라고 권한다. AWS Architecture Blog의 비교에서 지터를 둔 백오프가 경합을 줄인다.
- 다시 볼 점: 작업 03 기획에서 재시도 중복이 없는지 확인한다.

## D14 작업 02 세부 동작 확정
- 결정: 샘플링 항목과 범위, `Retry-After` 경계값과 재시도 대상 연결 오류, 시도 횟수 세는 법, 모델 목록 조회의 재시도·시간 초과, 오류에 담는 내용, 자격 파일의 빈 키 처리를 정했다. 내용은 docs/plans/02-client/의 각 spec.md에 있다.
- 근거: 세부 작업 명세를 쓰며 기존 문서에 근거가 없어 남은 항목이다. 비밀 값이 오류로 새지 않게 하고, 재시도가 대기 시간을 불필요하게 늘리지 않는 쪽으로 정했다.
- 다시 볼 점: 실제 서버 확인(작업 06)에서 다른 샘플링 항목이 필요하거나 헤더 대기 시간 초과 재시도가 필요해지면 다시 검토한다.

## D15 생각 옵션과 추가 요청 값
- 결정: 모델에 공통 `reasoning`(`off`, `low`, `medium`, `high`)을 두고 엔드포인트 `kind`에 맞는 요청 필드로 바꿔 보낸다. 값 매핑은 `off`→`none`, 나머지는 그대로. 변환 표(2026-11-03 문서 확인): `openai`/`ollama`/`vllm`/`llamacpp` → `reasoning_effort`; `openrouter` → `reasoning: { effort }`; `lmstudio` → 공통 필드 없음(필요 시 `extra`만). 상세·링크는 [research/02-client.md](research/02-client.md#kind별-reasoning-요청-필드-변환-02-5)와 [02-5 plan](plans/02-client/02-5-transport/plan.md). 공통 옵션에 없는 값은 모델의 `extra`(하위 객체 포함)에 적으면 요청 본문에 그대로 넣고, `model`, `messages`, `stream`, `tools`를 덮어쓰려 하면 설정 오류로 거부한다.
- 근거: 같은 기능도 서버마다 필드 이름과 모양이 다르다. 문서로 확인한 뒤 표를 고정해 02-5가 임의 변환하지 않게 한다.
- 다시 볼 점: 작업 06 실제 서버에서 `lmstudio`·모델별 effort 거절을 다시 본다.

## D16 설정 저장
- 결정: 라이브러리에 설정 저장 기능을 둔다(화면은 앱 작업). 엔드포인트와 모델 옵션은 설정 파일에, 키는 자격 파일에만 쓴다. 두 파일 모두 사용자가 단 주석과 손대지 않은 항목을 보존한다. 엔드포인트 이름을 바꾸면 자격 파일의 키 이름도 바꾸고, 삭제하면 그 키도 지운다. 임시 파일에 쓴 뒤 바꿔 기존 파일이 깨지지 않게 한다.
- 근거: 사용자가 손으로 고친 설정을 저장 기능이 덮어쓰면 안 된다.
- 다시 볼 점: 주석 보존 방식은 구현 때 크레이트 라이선스 확인과 함께 정한다.

## D17 설정·자격 파일 도구 차단
- 결정: 에이전트 파일 도구의 비밀 파일 차단 목록에 `agent-snap.jsonc`, `credentials.jsonc`를 넣는다. 반영은 작업 03에서 한다.
- 근거: 도구가 읽은 내용은 대화 맥락으로 LLM 서버에 전송된다. 설정 파일에는 엔드포인트 주소가 있다.
- 다시 볼 점: 명령 실행 도구로 읽는 경로는 실행 전 확인으로만 걸러진다. 설정 수정을 에이전트에 맡길 때는 키와 주소를 가리는 전용 도구를 작업 03에서 검토한다.

## D18 JSONC 해석기로 jsonc-parser 0.34
- 결정: JSONC 해석기로 `jsonc-parser` 0.34 를 쓴다. crates.io 라이선스 표기는 MIT 이고, 포장 LICENSE 와 Cargo.toml 의 license 표기도 MIT 이며 순수 Rust이고 네트워크 코드가 없다.
- 근거: 주석·마지막 쉼표 JSONC 해석을 직접 구현하지 않는다. 02-2·02-8 이 같은 해석기를 쓴다.
- 다시 볼 점: 02-8 에서 주석 보존 CST 가 필요하면 버전 상한 검토.

## D19 base_url 검증과 설정 폴더 우선순위
- 결정: `base_url` 검증을 strict로 한다. scheme을 `http`/`https`로 한정하고, 빈 문자열·앞뒤 공백·끝 슬래시를 금지한다. `/v1` 경로 여부는 검증하지 않는다(문서·예시 안내만). 설정 폴더는 환경 변수 `AGENT_SNAP_CONFIG_DIR`에 값이 있으면 그 폴더를 쓰고, 값이 없으면 운영체제 관례 폴더(Windows는 Roaming AppData 아래 `agent-snap`, 그 밖은 XDG 관례)를 쓴다. D10과 D14의 검증·오류 규칙을 보강한다.
- 근거: llama.cpp server README([https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md))는 서버가 HTTP로 serve하고 기본 base URL이 `http://localhost:8080/v1`임을 밝힌다. Ollama OpenAI compatibility([https://docs.ollama.com/api/openai-compatibility](https://docs.ollama.com/api/openai-compatibility))와 Ollama Authentication([https://docs.ollama.com/api/authentication](https://docs.ollama.com/api/authentication))은 로컬 서버가 `http://localhost:11434/v1`이고 인증 헤더를 요구하지 않으며 키를 보내도 무시하고, 클라우드(ollama.com)는 `https`와 API 키가 필요함을 밝힌다. 이 예시들이 scheme을 `http`/`https`로 한정하는 근거가 된다.
- 다시 볼 점: 02-5가 URL을 조립할 때 scheme을 붙이지 않고 base_url을 그대로 붙이는 방식과 충돌하면 다시 본다.

## D20 설정 스키마(중첩 모델·이름·http·선택 키)
- 결정(요약): `endpoints[]` 아래 중첩 `models[]`, 6종 `kind`, trim·이름 `/` 금지, 선택 키 `엔드포인트/모델`, `http` 7키 기본값(3/1/30/60/10/300/120), 모델 선택에 `context_window`(D29), 옛 toml·최상위 `models`·모델 `endpoint`·엔드포인트 키 칸 **거부**, `GET /models`는 참고만. 전문 규범은 정본.
- 정본: [docs/decisions/D20-config-schema.md](decisions/D20-config-schema.md)
- 근거: 정본 참조(중첩 직관성, OpenCode식 선택 키, `/` 구분 위험).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D21 02-2 설정 검증 세부와 JSONC 공용 모듈
- 결정(요약): `http` 시간·`retries`·`backoff_min`/`max`·trim·scheme 대소문자·`http` 알 수 없는 키 거부·빈/비객체 JSONC 거부; JSONC 공용 모듈 `jsonc` 한 곳(DRY); 단위 테스트는 `config/tests/`, 통합은 `crates/snap-core/tests/`.
- 정본: [docs/decisions/D21-config-validation-jsonc.md](decisions/D21-config-validation-jsonc.md)
- 근거: 정본 참조(0초 무의미, Full Jitter, RFC 3986, DRY).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D22 실행 신뢰성과 자율성 단계
- 상태: 정본 **approved** (2026-11-02 사용자 승인).
- 결정(요약): 실행 **신뢰성**(셸·승인·네트워크 deadline, 프로세스 트리 정리, timeout→백그라운드는 후보)은 M1에서 항상 켠다. `run_command` **실행 deadline** 기본 **600초**, 대화형 **승인 응답 deadline** 기본 **300초**(둘 다 설정으로 변경 가능). **자율성**은 L1(명령마다 확인)을 기본으로 하고, L2·L3(체크포인트·무인 옵션)는 06 이후 별도 기획으로 켜고 끌 수 있게 한다. 작업 03은 승인자 주입과 신뢰성만 넣는다. 실행 격리(D9)와 자식 정리(Job Object)는 분리한다.
- 정본: [docs/decisions/D22-execution-reliability-autonomy.md](decisions/D22-execution-reliability-autonomy.md)
- 근거: 요약형 결정만 두면 이후 에이전트가 오독·임의 확장하기 쉽다. Prospero 신뢰성 요소와 자율성 옵션을 한 정본에 규범적으로 둔다.
- 다시 볼 점: 정본 문서의 「다시 볼 점」을 따른다. 정본을 바꾸면 이 요약과 함께 갱신한다.

## D23 snap-cli와 snap-core 의존
- 결정: snap-cli는 버전 출력만 할 때까지 snap-core 의존을 두지 않는다(YAGNI). CLI가 core를 쓸 때 다시 추가한다.
- 근거: dev-principles minors 보정 리뷰에서 미사용 의존으로 지적되었고, 현재 바이너리가 core API를 부르지 않는다.
- 다시 볼 점: CLI가 `load_config` 등 snap-core와 연동할 때 `Cargo.toml`에 의존을 복원한다.

## D24 02-3 SSE 청크 JSON에 serde_json
- 결정: 02-3 델타 복원의 청크 JSON 해석에 `serde_json` 1.x를 쓴다. crates.io 라이선스 표기는 MIT OR Apache-2.0 이고, 저장소 LICENSE-MIT·LICENSE-APACHE와 Cargo.toml license 표기도 같다. 순수 해석 크레이트이며 네트워크 코드가 없다. JSONC 설정 파일은 기존 `jsonc-parser`(D18)를 그대로 쓴다.
- 근거: OpenAI 호환 스트림 청크는 엄격 JSON이고, 모르는 델타 필드를 원문 JSON 문자열로 보존하려면 소유 `Value`와 `to_string`이 필요하다. `jsonc-parser`의 빌린 트리만으로는 필드 원문 보존이 번거롭다.
- 다시 볼 점: 02-5 전송 계층에서 같은 크레이트로 요청 본문을 만들면 버전을 한곳에 고정한다.

## D25 모듈 하위 폴더·위임 brief 위치 (2026-11-02)
- 결정: (1) 같은 관심사 파일이 둘 이상일 때만 하위 폴더를 만들고, 파일 하나짜리는 평탄 파일로 둔다. (2) 위임용 작업 문서(brief)는 `docs/tasks/`에 두고 `docs/plans/`에 두지 않는다(`docs/tasks/`는 gitignore). (3) 파일 단위 SRP는 AGENTS 개발 규칙·하네스 10절·dev-principles 2절을 따르며, 세부 작업 plan에만 보정 일지를 남기지 않고 공통 규칙으로 둔다. 테스트 트리에도 동일하게, 같은 관심사 파일이 둘 이상이면 `llm/tests/retry/`, `llm/tests/sse/`처럼 하위 폴더를 둔다.
- 근거: 02-3에서 plan에만 SRP·폴더링 메모를 적어 다음 세부 작업에 전달되지 않았다. 단파일 폴더 시위와 brief를 plans에 둔 실수를 막기 위함이다.
- 다시 볼 점: 없음.

## D26 02-4 재시도 비대상(402 등)과 입력 매핑
- 결정(요약): OpenRouter **402**는 재시도하지 않는다. `Retry-After` 0/과거→0초, 음수·해석불가→헤더 없음, 헤더 대기 초과·408·425·목록 밖 5xx·기타 4xx(429 제외)는 즉시 포기. 02-5→02-4 입력 매핑 표를 spec에 고정(`PreHeaderConnectKind` 전부·TLS·프록시 포함). POST는 멱등 키 없이 일시 오류만 재시도. 횟수 초과를 `Retry-After` 상한 초과보다 먼저 판정.
- 정본: [docs/decisions/D26-retry-exclusions-mapping.md](decisions/D26-retry-exclusions-mapping.md)
- 근거: 정본 참조(잔액은 사용자 조치, in-flight YAGNI, 경계 AC·매핑 표 고정).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D27 Retry-After 정책과 IMF-fixdate 파싱 분리 (2026-11-02)
- 결정: HTTP-date 파싱은 `llm/http_date.rs`로 두고 retry 정책과 모듈 경계를 나눈다. IMF-fixdate·민간 달력·`wait_until`은 Retry-After 정책이 아니므로 `retry/` 아래에 두지 않는다. `after.rs`는 정책·delay-seconds·상한만 남기고 HTTP-date 경로는 `crate::llm::http_date` 파서를 호출한다. 필요 진입점은 같은 crate의 retry가 쓰도록 `pub(crate)`로 열고, `snap-core` lib 공개면은 기존 retry 공개 API를 유지한다. 하위 폴더로 감싸지 않는다(D25).
- 근거: 파일·모듈 단위 SRP(SOLID), D25(관심사 파일이 하나일 때 하위 폴더 금지), AGENTS 개발 규칙. 제품 공개 표면은 기존 retry 공개 API를 유지한다.
- 다시 볼 점: 다른 레이어가 같은 파서를 쓰게 되면 그때 위치를 재검토한다(추가 소비자가 생겼을 때의 이동이지, 지금 분리를 미루는 이유가 아님).
- 관련 작업: [docs/tasks/03-retry-after-http-date-split.md](tasks/03-retry-after-http-date-split.md)

## D28 02 클라이언트 브랜치 리뷰 1 보정 범위 (2026-11-03)
- 결정: `docs/reviews/02-client-branch-review-1.md` 미달에 대해 major 2건은 전부 고친다. (1) 제품 코드 `unreachable!` 제거(패닉 없는 경로). (2) `serde_json`에 `preserve_order`를 켜고 한 델타 안 생각·모르는 필드 순서를 테스트로 고정한다(02-3 AC·주석 약속과 맞춤). minor 중 이번에 고칠 것: `facade`→한국어, `llm.rs` 모듈 문서, 재시도 중간 API를 `pub(crate)`로 좁혀 lib 공개는 `decide_retry`와 02-6에 필요한 입력·결과 타입만, `index`는 `u32::try_from`. 미룸 중 **디코더 실패 후 Ok·알고 있는 키·호스트 가리기**는 02-5 plan/spec/tasks·AC에 반영함(2026-11-03). 남은 미룸: SSE 버퍼 비용, tasks 옛 경로·상위 상태 문구(문서 정리).
- 근거: 리뷰 major는 AC 미달. `preserve_order`는 승인된 “보낸 순서” 약속을 코드로 맞춘다. 공개면 축소는 02-4-4·ISP. 전송 관련 minor는 02-5 AC에 넣었다.
- 전이 의존 라이선스(`preserve_order`): `indexmap`·`equivalent`·`hashbrown`은 MIT 또는 Apache-2.0, `foldhash`는 Zlib(모두 허용형, 리뷰2 N2 확인).
- 다시 볼 점: D27의 “retry 공개 API 유지”는 http_date 분리 당시 범위다. 이번 축소 후 02-6이 `decide_retry`만으로 충분한지 확인한다.

## D29 모델별 맥락 창(`context_window`)
- 결정(요약): 모델에 선택 `context_window`(토큰, 1 이상 정수). 생략 시 **65536**. `max_tokens`와 별도. 요청 본문에 이 키로 실지 않음(02-5). 사용은 작업 04 `context`. 구현은 02-8 전 02-2 보강.
- 정본: [docs/decisions/D29-context-window.md](decisions/D29-context-window.md)
- 근거: 정본 참조(로컬 LLM 과대 기본 창 방지, 키 이름·64K 기본).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.
