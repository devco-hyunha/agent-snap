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
- 결정: 모델에 공통 `reasoning`(`off`, `low`, `medium`, `high`)을 두고 엔드포인트 `kind`에 맞는 요청 필드로 바꿔 보낸다. 값 매핑은 `off`→`none`, 나머지는 그대로. 변환 표(2026-11-03 문서 확인): `openai`/`ollama`/`vllm`/`llamacpp` → `reasoning_effort`; `openrouter` → `reasoning: { effort }`; `lmstudio` → 공통 필드 없음(필요 시 `extra`만). 상세·링크는 [research/02-client.md](research/02-client.md#kind별-reasoning-요청-필드-변환-02-5)와 [02-5 plan](plans/02-client/02-05-transport/plan.md). 공통 옵션에 없는 값은 모델의 `extra`(하위 객체 포함)에 적으면 요청 본문에 그대로 넣고, `model`, `messages`, `stream`, `tools`를 덮어쓰려 하면 설정 오류로 거부한다.
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
- 결정: HTTP-date 파싱은 `llm/codec/http_date.rs`로 두고 retry 정책과 모듈 경계를 나눈다. IMF-fixdate·민간 달력·`wait_until`은 Retry-After 정책이 아니므로 `retry/` 아래에 두지 않는다. `after.rs`는 정책·delay-seconds·상한만 남기고 HTTP-date 경로는 `crate::llm::codec::http_date` 파서를 호출한다. 필요 진입점은 같은 crate의 retry가 쓰도록 `pub(crate)`로 열고, `snap-core` lib 공개면은 기존 retry 공개 API를 유지한다. 하위 폴더로 감싸지 않는다(D25). (경로 표기는 2026-11-05 작업 08 이동 결과 실체로 정정, 결정 내용 그대로)
- 근거: 파일·모듈 단위 SRP(SOLID), D25(관심사 파일이 하나일 때 하위 폴더 금지), AGENTS 개발 규칙. 제품 공개 표면은 기존 retry 공개 API를 유지한다.
- 다시 볼 점: 다른 레이어가 같은 파서를 쓰게 되면 그때 위치를 재검토한다(추가 소비자가 생겼을 때의 이동이지, 지금 분리를 미루는 이유가 아님).
- 관련 작업: [docs/tasks/03-retry-after-http-date-split.md](tasks/03-retry-after-http-date-split.md)

## D28 02 클라이언트 브랜치 리뷰 1 보정 범위 (2026-11-03)
- 결정: `docs/reviews/02/branch-review-1.md` 미달에 대해 major 2건은 전부 고친다. (1) 제품 코드 `unreachable!` 제거(패닉 없는 경로). (2) `serde_json`에 `preserve_order`를 켜고 한 델타 안 생각·모르는 필드 순서를 테스트로 고정한다(02-3 AC·주석 약속과 맞춤). minor 중 이번에 고칠 것: `facade`→한국어, `llm.rs` 모듈 문서, 재시도 중간 API를 `pub(crate)`로 좁혀 lib 공개는 `decide_retry`와 02-6에 필요한 입력·결과 타입만, `index`는 `u32::try_from`. 미룸 중 **디코더 실패 후 Ok·알고 있는 키·호스트 가리기**는 02-5 plan/spec/tasks·AC에 반영함(2026-11-03). 남은 미룸: SSE 버퍼 비용, tasks 옛 경로·상위 상태 문구(문서 정리).
- 근거: 리뷰 major는 AC 미달. `preserve_order`는 승인된 “보낸 순서” 약속을 코드로 맞춘다. 공개면 축소는 02-4-4·ISP. 전송 관련 minor는 02-5 AC에 넣었다.
- 전이 의존 라이선스(`preserve_order`): `indexmap`·`equivalent`·`hashbrown`은 MIT 또는 Apache-2.0, `foldhash`는 Zlib(모두 허용형, 리뷰2 N2 확인).
- 다시 볼 점: D27의 “retry 공개 API 유지”는 http_date 분리 당시 범위다. 이번 축소 후 02-6이 `decide_retry`만으로 충분한지 확인한다.

## D29 모델별 맥락 창(`context_window`)
- 결정(요약): 모델에 선택 `context_window`(토큰, 1 이상 정수). 생략 시 **65536**. `max_tokens`와 별도. 요청 본문에 이 키로 실지 않음(02-5). 사용은 작업 04 `context`. 구현은 02-8 전 02-2 보강.
- 정본: [docs/decisions/D29-context-window.md](decisions/D29-context-window.md)
- 근거: 정본 참조(로컬 LLM 과대 기본 창 방지, 키 이름·64K 기본).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D30 02-5 전송·취소·모의 서버 크레이트
- 결정: 02-5 HTTP 전송은 `reqwest` 0.13(features: `json`, `stream`; 기본 TLS는 `rustls`)을 쓴다. 취소 토큰은 `tokio-util` 0.7(`sync` 모듈의 `CancellationToken`, `rt` feature 불필요)을 쓴다. 비동기 런타임은 `tokio` 1에 제품 코드는 `macros`·`time`만 넣고, 테스트는 `rt-multi-thread`·`sync`·`net`까지 `dev-dependencies`로 분리한다. 테스트 모의 서버는 `axum` 0.8과 본문 스트림 구현(조각·지연 제어)에 쓰는 `http-body` 1을 `dev-dependencies`에 둔다. crates.io·포장 LICENSE 표기: `reqwest`는 MIT OR Apache-2.0(LICENSE-MIT·LICENSE-APACHE), `tokio`·`tokio-util`·`axum`·`http-body`는 MIT(LICENSE). 후보 `wiremock` 0.6도 MIT/Apache-2.0 허용형이나, 헤더 지연·본문 멈춤·연결 종료 관찰을 핸들러로 직접 다루려고 `axum`을 고른다. 버전·features는 `crates/snap-core/Cargo.toml`에 맞춘다.
- 근거: RESEARCH·plan이 `reqwest`의 `connect_timeout`만 쓰고 헤더 대기·유휴는 따로 재며, 취소는 `CancellationToken`의 `cancelled()`로 안전하게 기다리라고 한다. 모의 서버는 헤더 지연·본문 멈춤·연결 종료를 핸들러로 직접 다루려고 `axum`을 쓰고, 본문 조각과 지연을 테스트가 제어할 때 쓰는 본문 제어 타입이 `http-body` 1이다. 허용형만 채택한다(D5·D18·D24와 같은 확인 방식).
- 다시 볼 점: `rustls`가 대상 PC에서 막히면 `native-tls`를 검토한다. 모의 서버가 과하면 더 얇은 스텁으로 바꿀지 본다.

## D31 02-6 백오프 난수 크레이트와 포기 이유 전달 (2026-11-04)
- 결정: 백오프(Full Jitter)용 `[0.0,1.0)` 균등 실수 값은 `fastrand` 2.5.0으로 공급한다(2026-11-04 crates.io 확인, 라이선스 Apache-2.0 OR MIT, 의존 0개). 공급은 `llm/retry/retry_loop.rs`의 판단 반복(`judge_once`) 한 곳에만 두고, 값 주입을 받으면 그 값을, 안 받으면 `fastrand::f64()`를 쓴다. 제품은 항상 생산 공급(주입 `None`)이고, 시험은 `with_unit_sample`(cfg(test))로 값을 고정해 대기 간격을 일정하게 유지한다. 포기 이유 전달은 새 오류 종류를 만들지 않고 `TransportError::ServerStatus`의 칸 `give_up: Option<RetryGiveUpReason>`으로만 얹는다(02-6 spec 33행 "새 오류 종류 금지" 준수). `Retry-After` 헤더 원문은 판단에만 쓰고 공개 오류에 담지 않는다.
- 근거: 02-4 `decide_retry`는 균등 난수(`unit_sample`)를 주입받도록 설계됐고(backoff.rs 독스트링), 02-6에서 그 공급처를 처음 둔다. 조사 후보는 `fastrand`(의존 0개, f64 한 값 생성), `getrandom`(난수 조각만, 조합 코드 필요), `rand`(전이 의존 과함)였고 가벼운 쪽을 골랐다. 허용형 라이선스만 채택하며(D5·D18·D24와 같은 확인 방식), 의존 0개라 전이 의존 라이선스 문제가 없다. 버전은 `Cargo.toml`의 "2.3" 요구에 `Cargo.lock`이 2.5.0으로 고정된다.
- 다시 볼 점: `fastrand` 버전 상승으로 `f64` 생성 방식이 바뀌면 다시 본다. 03에서 에이전트 루프가 붙을 때 재시도가 같은 한 층(`stream_chat`)을 거치는지 확인한다(D13). 포기를 공개 API로 더 널리 공개할지는 03·04에서 본다.

## D32 CLI 출력 언어 (2026-11-04)
- 결정: 프로그램이 화면에 찍는 말(오류·승인 안내·상태 문구 등)의 언어는 D2 `target_language`(모델이 쓸 말, 작업 05 감시)와 **따로** 둔다. 설정 키 이름·지원 언어·`snap-core` 오류와 `snap-cli` 표시 경계는 **작업 04 상세 기획**에서 정한다. 지금은 02·03 기획·구현 범위에 넣지 않는다.
- 근거: 사람용 문구는 대화형 CLI(작업 04)에 모인다. `target_language`에 섞으면 감시 설정과 UI 표시가 한 키로 묶여 바꾸기 어렵다. 02는 전송·설정 중이라 문구 표면이 아직 없다.
- 다시 볼 점: 04 착수 시 (1) UI 언어 설정 키를 `target_language`와 나눌지, (2) 기본값을 `ko`로 둘지, (3) 코어는 구조화 오류만 두고 CLI에서 문장으로 조립할지 확정한다. 앱(Tauri)은 CLI에서 정한 방식을 재사용하는지 그때 본다.

## D33 Serena 기능 참고·네이티브 심볼 도구 (2026-11-04)
- 결정: [Serena](https://github.com/oraios/serena)(MCP로 IDE식 심볼 검색·편집을 주는 서버)의 **동작만** 참고하고, Serena 프로세스·MCP 서버·Serena 앱 코드를 **의존·포함하지 않는다**. 같은 능력은 `snap-core`의 `Tool`로 **네이티브** 구현한다(D5: 설계 참고, 코드 가져오면 출처·라이선스). 참고 후보(문서 [Tools and APIs](https://oraios.github.io/serena/01-about/035_tools.html)): 심볼 찾기·파일 심볼 개요·참조·선언, 심볼 단위 교체·이름 바꾸기, 파일 진단. **비범위(당분간)**: JetBrains/유료 백엔드, Serena REPL(파이썬 한 도구로 조합), 대시보드·온보딩·Serena 메모리 도구, 외부 프로젝트 질의 MCP. 작업 03 본문의 기본 도구(`read_file`·`write_file`·`edit_file`·`list_dir`·`glob`·`grep`·`run_command`)에는 **지금 넣지 않는다**. 후보·우선순위·언어 서버(LSP) 백엔드 선택은 **작업 03 세부 기획** 또는 그 뒤 별도 작업에서 정한다.
- 근거: 심볼 단위 조회·편집은 큰 코드베이스에서 파일 전체 읽기보다 맥락이 짧아진다. Serena 앱은 GPL-3.0-or-later(SolidLSP만 MIT)라 앱 코드를 가져오면 라이선스가 무겁다. agent-snap은 독립 에이전트(D5)라 외부 MCP에 묶지 않고, 작업 폴더 경계·승인(D9·D17·D22)을 같은 `Tool` 경로로 지킨다. 03 1차는 최소 파일 도구로 루프를 먼저 굳힌다.
- 다시 볼 점: 03 세부 기획 때 (1) 1차로 넣을 도구(찾기·개요·참조 vs 이름 바꾸기·진단), (2) Windows에서 rust-analyzer 등 LSP 기동·수명, (3) 심볼 편집·이름 바꾸기의 승인 필요 여부, (4) SolidLSP(MIT) 아이디어만 참고할지 Rust LSP 클라이언트를 쓸지. 코드를 참고·이식하면 D5대로 출처와 라이선스를 적는다.

## D34 서브에이전트 모델과 제공자 장애 처리 (2026-11-05)
- 결정: OpenCode에서는 서브에이전트를 지금 세션과 같은 모델로만 열고, 위임 프롬프트에 모델 지정 인자를 넣지 않는다. 하네스 8절의 3단계(가벼운 모델·표준 모델·가장 강한 모델)는 모델을 지정할 수 있는 도구에 적용한다. 서브 호출이 제공자 쪽 오류로 두 번 나면 같은 내용으로 새 서브를 다시 연다.
- 근거: 사용자가 OpenCode의 서브는 현재와 같은 모델로만 열라고 정했다. 하네스 8절은 "서브에이전트에 모델을 지정할 수 없는 도구는 기본 모델을 쓴다"로 이 규칙과 맞는다. build-harness 실패 처리는 기존 서브를 이어 쓰지 않고 새 서브에 원인과 현재 상태를 적어 진행하라고 한다. `cursor-acp` 호출이 출력 없이 exit code 58로 두 번 끝나 그 제공자를 쓸 수 없었다.
- 다시 볼 점: 모델을 지정할 수 있는 도구(Claude Code·Cursor)로 위임할 때만 8절 단계를 쓰고, OpenCode에서는 같은 모델로 실패 원인만 바꿔 재시도한다. 결과가 미달이면 모델 문제인지 작업량 문제인지 나눠 본다.

## D35 프로젝트 규칙 정본 분리 (2026-11-05)
- 결정: 프로젝트 고유 규칙 정본을 `harness/PROJECT_RULES.md`로 옮긴다. `AGENTS.md`의 `## 프로젝트 오버레이`에는 `[안전 예외 승인: 3번]` 선언과 정본을 가리키는 색인만 둔다. OpenCode는 서브에이전트를 지금 세션과 같은 모델로만 열고, 모델 지정 인자를 쓰지 않는다.
- 근거: 사용자가 프로젝트 규칙을 하네스 파일 한 곳에 별도 작성을 정했다. HARNESS.md 1절은 프로젝트 규칙을 오버레이 한 곳에 두고 분량이 크면 링크 한 줄만 두라고 한다. 하네스 마커 블록은 읽기 전용이라 프로젝트 고유 규칙이 블록에 섞이지 않는다. OpenCode 서브 모델 지정은 같은 세션 모델만 허용한다는 사용자 지시를 반영한다.
- 다시 볼 점: 하네스 템플릿이 오버레이 파일 경로를 `docs/PROJECT_RULES.md`로 정하면 이름을 맞춘다. 서브 모델 지정이 가능한 도구로 옮기면 하네스 8절 3단계를 다시 쓰고, OpenCode는 같은 모델 규칙을 유지한다.

## D36 세션 맥락·하네스 주입과 RAG/OKF 범위 (2026-11-05)
- 결정:
  1. EvoHarness-RL의 BPE(Belief·Progress·Experience)와 "필요할 때만 외부 상태를 쓴다"는 선택적 외부 상태 사용은 작업 04(세션·맥락) 설계 참고로만 채택한다. 모델 가중치를 학습하는 SFT·GRPO 파이프라인 전체는 미적용이다(M1·D6과 맞춤).
  2. RAG와 OKF는 Experience·장기 지식의 후보로 연관 조사·기획 대상이며, 02·03 구현 범위에 넣지 않는다. OKF는 지식을 포장하는 형식, RAG는 런타임에 조각을 골라 넣는 검색 패턴으로 구분해 쓰고 서로 대체하지 않는다.
  3. 스킬·룰·개발 하네스를 전 세션에 무조건 전부 주입하지 않는다. 불변 규범(비밀 금지·언어 순수성·작업 폴더 경계)만 짧게 항상 넣고, 나머지는 세션·작업 프로필·토글 또는 요청 시 pull로 켠다. 앱은 목록 토글 UI, CLI는 같은 상태를 쓰는 설정 키·명령을 쓰며 상세는 04 이후 기획에서 정한다. "끄기"는 파일 삭제가 아니라 그 세션 기본 맥락에 실지 않는 것이다.
  4. 맥락이 길어질 때 우선 수단은 세션 임시 문서(진행·결정·열린 질문 등 현재 상태를 의도적으로 갱신하는 문서)다. 대화 요약은 보조이며, 하네스·켠 스킬·룰·불변 규범 원문은 요약 입력에서 제외하고 필요 시 원문 그대로 다시 넣는다. 세션 임시 문서는 OKF·BPE의 상태 쪽, 대화 요약은 과거 대화의 손실 압축, RAG는 기존 조각 검색으로 문서에 명시해 혼동하지 않는다.
  5. 프로젝트 `harness/`(개발·승인·위임 규칙)와 논문의 런타임 하네스·세션 지식은 이름이 겹치므로 층을 구분해 쓴다.
  6. 외부 OKF·RAG·메모리 제품·MCP에 런타임을 묶지 않는 것을 기본으로 한다(D5·D33과 같음). 형식·동작만 참고하고, 코드를 이식할 때는 출처·라이선스를 적는다.
  7. 위의 상세 설정 키·프로필 목록·저장 경로·검색 방식·앱 UI는 작업 04 상세 기획(필요 시 그 뒤 별도 작업)에서 정하고, 지금은 방향만 고정한다. 02·03에는 코드·설정 키·UI·RAG/OKF/토글/요약기 구현을 넣지 않는다.
- 근거: 긴 세션에서 전부 주입하고 전부 요약하면 토큰 낭비와 규범 오염이 난다(사용자의 다중 세션 실사용 관찰, OpenCode가 하네스·스킬·룰까지 요약해 규범이 흐려진 관찰). EvoHarness-RL·OKF·작업 04 기획(압축·컨텍스트 블록)이 같은 문제(상태를 밖에 두고 선택적으로 넣기)를 다룬다. M1은 에이전트 루프가 먼저다(D6). 규범(보안·형식·승인)은 늘 넣는 push로, 도메인·교훈은 필요할 때 꺼내는 pull로 나눈다.
- 참고 자료(직접 열어 확인함): [arXiv:2608.05446 EvoHarness-RL](https://arxiv.org/abs/2608.05446), [AI타임스 기사](https://www.aitimes.com/news/articleView.html?idxno=214593), [OKF vs RAG 비교](https://openknowledgeformat.com/compare/okf-vs-rag).
- 다시 볼 점:
  - 04 착수 시: BPE의 세 역할(Belief·Progress·Experience)을 세션 임시 문서·체크리스트·영속 구역에 어떻게 매핑할지.
  - 항상 켜지는 목록, 토글 프로필 목록, 설정 키, CLI 명령, 앱 토글 UI 순서와 세부.
  - 세션 임시 문서 위치(프로젝트 git 안 vs `.agent-snap/`만).
  - 요약 파이프라인 입력에서 제외할 정확한 목록(시스템·스킬·룰·AGENTS 등).
  - BM25만으로 충분한지, 임베딩 RAG가 필요한지는 06 실사용 뒤 판단. 지식 규모가 커지면 OKF식 저장·가벼운 검색을 별도 기획으로. 벡터 RAG·외부 서버 의존은 더 뒤.
  - `docs/RESEARCH.md` 작업 04 절 조사 항목 갱신은 사용자가 시킬 때 별도로.

## D37 02-7 작업 1: 선택 키 조립은 02-5 `new` 재사용, 공개면은 `Config::client_for` (2026-11-05)
- 결정: 선택 키 클라이언트 조립은 02-5의 `OpenAiCompatClient::new`을 그대로 재사용하고, WIP로 넣었던 `transport/client_parts.rs`의 `TransportParts` 트레이트와 `build_http_client` 헬퍼는 삭제한다. 공개면은 `Config::client_for(selection_key, credentials)` 하나로 두고, `SelectedModel`은 준비 값 타입으로만 내보낸다. `SelectionError`는 커스텀 `Debug` 대신 `thiserror::Error` derive를 쓴다.
- 근거: dev-principles 3절 재사용 사다리(같은 목적의 기존 함수 재사용)와 "조립 재정의 금지" 해설이 같은 뜻이다. `client_parts.rs`는 `client.rs:34-52`의 `new`과 reqwest 조립을 통째로 دوب게 복사한 것이라 DRY 위반이었고, `OpenAiCompatClient::new`을 부르는 쪽은 `tests/common/support.rs:74`(시험 지원) 한 곳이다. 커스텀 `Debug`은 표시할 비밀 값이 없는 타입이라 derive와 같은 결과를 내는데, derive와 같이 두면 충돌(E0119)이 나서 derive 하나로 합쳤다. 오류에 입력 키만 담는 규격은 `#[error]` 문자열과 derive `Display`로 채워진다.
- 다시 볼 점: `SelectedModel` 필트의 공개 여부는 02-7 작업 2(`GET /models`)와 작업 04·05 CLI에서 쓸 때 필요한 만큼만 좁힌다. `From`/조립 트레이트로 넓힐 요구가 생기면 02-5 공개면 좁히기(D28)와 함께 판단한다.

## D38 02-7 작업 2: 모델 목록 조회는 전송·해석·조립을 나누고, 공개 API는 전송 키·선택 키 두 자리 (2026-11-05)
- 결정: 모델 목록 조회의 공개 API는 `OpenAiCompatClient::list_models(cancel) -> Result<Vec<String>, TransportError>` 한 가지를 그대로 쓰고, 선택 키로 만든 `SelectedClient`에도 같은 이름의 조회 길을 얹는다. 전송 파일(`transport/fetch_models.rs`)은 한 번 보내기만 하고, 목록 JSON 해석(`codec/models_list.rs`)과 Bearer 헤더 조립(`transport/client.rs`)은 파일을 나눠 한 책임씩 둔다. 채팅 전송의 헤더 대기·오류 본문 읽기는 transport 안에서 같은 만들기를 재사용한다.
- 근거: 목록 조회는 재시도하지 않는 한 번짜리 조회이고(D14), 02-5의 시간·취소 규격을 그대로 쓴다. 전송·해석·조립을 한 파일에 몰면 책임 분리(SRP)에 걸리고, 헤더 조립을 채팅과 목록에서 다시 적으면 중복(DRY)이 된다.
- 다시 볼 점: 본문 해석 실패는 02-7 작업 2에서는 형식 오류로 나누지 않고 빈 목록으로 보는 자리를 비운다. 형식 오류 규칙은 02-7 작업 3에서 정하고, 이 자리에 맞는 오류 종류를 DECISIONS에 남긴다. 시간·취소 동작 확인은 02-7 작업 4에서 한다.

## D39 02-7 작업 3: 목록 형식 오류는 종류만 담는 전용 타입, 본문·키·주소는 담지 않음 (2026-11-05)
- 결정: 목록 조회의 형식 오류는 `llm::error::models_format`의 전용 타입 `ModelsListFormatError`로 낸다. 종류는 `ModelsListFormatKind`(잘못된 JSON·`data` 없음·`data`가 배열 아님·`id` 없음·`id`가 문자열 아님)로 두고, Display·Debug에는 종류 표기와 안내 말만 남기고 본문 원문은 남기지 않는다. 전송 오류 `TransportError`에는 종류만 통째로 실어 통과시키는 `#[from]` 변형 하나를 더해, 채팅 오류 체인 규칙(키·주소 비노출)을 목록에도 그대로 적용한다. 본문이 빈 200 응답은 JSON을 읽지 못한 것으로 같은 종류를 쓴다(빈 목록 성공 금지). 항목의 `id`가 정한 형식이 아니면 목록 전체 조회를 실패로 본다.
- 근거: spec 38~39행·51행은 `data`·`id` 형식 실패와 본문 비노출을 요구한다. 본문·키·주소를 섞기 쉬운 채팅 오류 타입에 변형을 더하면 요약 경로(`scrub`)를 거치지 않는 자리가 생겨 가리기가 망가진다. 종류만 담는 전용 타입이면 요약할 본문이 애초에 생기지 않는다.
- 다시 볼 점: 서버마다 형식이 다른 목록(llama.cpp의 `meta: null`, Ollama식 추가 필드)이 더 엄격한 요구로 번지면 종류를 더 넓힐 때 spec 38행과 함께 판단한다.

## D40 02-8 저장 편집 도구와 원자적 교체 (2026-11-05)
- 상태: 정본 **recorded** (2026-11-05 사용자 승인).
- 결정(요약): 주석 보존 편집은 `jsonc-parser` 0.34.0의 `cst` 기능을 켠 것으로 쓴다(라이선스 MIT, crates.io·저장소 LICENSE 확인, `cst`는 의존 추가 없음). 저장은 같은 폴더 임시 파일에 쓰고 `sync_all` 뒤 `std::fs::rename`으로 교체하며, 교체 실패 시 기존 파일은 그대로 두고 임시 파일만 지운다. 두 파일을 함께 바꿀 때는 자격 파일을 먼저 쓰고 설정 파일을 그 뒤에 쓴다. 기각 후보 `monochromatic-jsonc-edit` 0.1.2는 LGPL-3.0-or-later라 D5 기준에 맞지 않는다.
- 정본: [docs/decisions/D40-save-editor.md](decisions/D40-save-editor.md)
- 근거: D18의 같은 크레이트로 새 의존 없이 해결(dev-principles 3절 재사용 사다리 3번), 원자적 교체는 `std::fs::rename`·`rename(2)`·`MoveFileExW` 문서 근거.
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다. Windows의 열려 있는 파일 교체 실패 코드 세부(값)는 문서에 없어 02-8 작업 4에서 직접 확인했고, 원시 번호 32(공유 위반)임을 확인했다(정본 D40의 「구현 후 확인 메모」).

## D41 두 파일 저장 순서는 검증 → 자격 교체 → 설정 교체 (2026-11-06)
- 상태: 정본 **recorded** (2026-11-06 사용자 승인).
- 결정(요약): 두 파일을 함께 고치는 저장은 검증을 자격 파일 쓰기 전에 마친다. 순서는 검증 → 자격 교체 → 설정 교체이며, 검증에 걸리는 입력은 어느 파일도 바꾸지 않고 거부한다(AC 2 정본). 교체 순서는 D40 규칙 3을 그대로 따른다.
- 정본: [docs/decisions/D41-save-order-validation.md](decisions/D41-save-order-validation.md)
- 근거: spec의 "쓰기 전에 검증 통과" 규칙과 AC 2가 정본. 거부가 자격 교체 뒤에 있으면 자격 파일만 바뀐 채로 거부되어 AC 2가 깨짐(2026-11-06 사용자 승인).
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D42 설정 오류는 읽기·검증·저장 부분 열거형으로 나눈다 (2026-11-06)
- 상태: 정본 **recorded** (2026-11-06 사용자 승인·02-8 잔여 반영).
- 결정(요약): `ConfigError` 공개 경로는 유지하고, 안을 `ReadingError`·`ValidationError`·`SaveError` 부분 열거형으로 나눈 뒤 `#[from]`으로 감싼다. Display·Debug 가리기는 부분 열거형마다 두어 합친 문구는 이전과 같다. 파일은 `config/error/`·`config/save/`·`config/read/`·`config/tests/save/`에 둔다.
- 정본: [docs/decisions/D42-config-error-partial.md](decisions/D42-config-error-partial.md)
- 근거: 하네스 10절·dev-principles 2절 SRP(변경 이유별 파일 분리). 동작·문구·담는 값·D41 순서는 바꾸지 않음.
- 다시 볼 점: 정본의 「다시 볼 점」을 따른다.

## D43 에이전트 도구 배치 시점 (2026-11-07)
- 상태: **approved** (2026-11-07 사용자 확정).
- 결정: M1 "돌아가는 에이전트" 다음에 추가 기능을 붙인다는 것은 05·06처럼 뒤로 미루라는 말이 아니다. (1) 에이전트가 코딩 한 바퀴를 도는 데 필요한 **기본 도구는 작업 03에서 전부 만든다**. 03 완료 시점에 기본 도구 구멍이 있어서는 안 된다. (2) 03 이후에 필요한 추가 도구·기능(웹 도구, 슬래시 커맨드, 계획 모드, 메모리 계층, 심볼 도구(네이티브), hooks, 체크포인트, MCP 등)은 **늦어도 작업 04에 배치**한다. 05·06은 각각 감시·실사용 검증 작업이고 기능 추가 자리가 아니다. 07-09는 판정 모델 작업이다. (3) 배치 위치를 정할 때 기능 목록([research/agent-feature-inventory-2026-11-07.md](research/agent-feature-inventory-2026-11-07.md))을 대조하고, 03 DoD 보고는 기본 도구 완성 여부와 기능 목록 대조로 한다.
- 근거: 프로젝트 시작 전제 "돌아가는 에이전트를 만들고 나서 원했던 추가 기능을 넣자"(PLAN.md 목표 1, D6). 사용자가 2026-11-07에 "도구 추가는 늦어도 04 이후, 기본 도구는 03 이후 04 이전에"로 확정. 06의 결함 목록·새 작업 문서 규칙(06 AC 5)은 조사를 남기는 규칙이지 도구 배치 미룸 규칙이 아니다. D33의 심볼 도구도 "그 뒤 별도 작업"을 04로 확정한다.
- 다시 볼 점: 03 세부 기획 착수 시 기본 도구 목록(7개 +α)을 확정한다. 04 착수 시 확장 목록의 우선순위를 위 기능 목록과 대조해 정한다.
- 2026-11-08 보정: (a) 배치 보정 메모를 [기능 목록](research/agent-feature-inventory-2026-11-07.md)에 반영. 코딩 한 바퀴 필수 7개(read/write/edit/list_dir/glob/grep/run_command)는 03 유지이고, 웹·LSP·계획 모드는 04로 둔다. AGENTS.md 읽기는 03 범위(2026-11-08 사용자 승인). (c) 상태 라인·출력 형식은 표시 전용(statusline "does not consume API tokens" 근거, 2026-11-08 확인)이므로 04 후보로 이동. 심볼 도구 1차는 03이 아니라 04 배치로 확정(D33·D43).

## D44 04 세부 작업 3갈래 분활 (2026-11-08)
- 상태: **recorded** (2026-11-08 사용자 승인).
- 결정: 04(대화형 CLI·세션·맥락)는 한 개 문서로 담기엔 항목이 많아 세부 작업을 3갈래로 나눈다. 04-1 대화형 CLI·세션 저장, 04-2 컨텍스트·메모리·색인, 04-3 확장 도구·클래식 확장. 각 갈래마다 `docs/plans/04-cli-session-context/<세부>/`에 plan·spec·tasks를 둔다.
- 근거: 04 범위(In)가 11개 항목+ 확장 후보 5개로, D11(세부 작업 = 위임 1회+리뷰 1회 기능) 기준 여러 위임 단위다. D25(문서는 plans 상세 폴더에)·D7(세부 분할)을 따른다.
- 다시 볼 점: 04 착수 시 각 갈래의 AC·선행 순서를 확정한다.

## D45 04 지시 파일 색인 배치 (2026-11-08)
- 상태: **recorded** (2026-11-08).
- 결정: 지시 파일 색인(스킬·하네스 규칙·AGENTS.md·CLAUDE.md·rules·프롬프트의 목록·요약·경로·mtime·크기·해시·전달 우선순위)은 04 세부 04-2에 배치한다. 형식 jsonc, 변경 감지는 git 비교가 아니라 snap의 mtime·크기·해시 기록 비교. 전달 우선순위는 `always`/`on-demand`/`off`를 색인 기본값으로 두고, 사용자가 세션마다 강제 지정하는 `user-pinned`는 색인이 아니라 세션 기록에 담는다(색인 기본값 > 세션 지정 순이 아니라 세션 지정 > 색인 기본값).
- 근거: [기능 목록](research/agent-feature-inventory-2026-11-07.md)·[조사 11 5절·5-1](research/11-rag-okf-memory-skills.md) 효과 검토. Claude의 "파일마다 100줄 열기"를 없애는 목적. 색인 선정 단계 토큰 절감(20스킬 3000→1000, 항목당 50토큰). 05·06 배치 금지(D43).
- 다시 볼 점: 04-2 착수 시 색인 스캔 주기(세션 시작+읽기 직전 vs 주기 스케줄), 항목 요약 길이 실측, 2층 토글 반영 시점 확정.

## D46 도구 호출 형식 3갈래 조합 (2026-11-08)
- 상태: **approved** (2026-11-08 사용자 결정).
- 결정: 도구 호출 형식을 하나로 정하지 않고 3갈래 조합으로 쓴다. (1) 기본은 OpenAI `tools`/`tool_calls`(1안) — 03-2 루프. (2) 로컬 미지원 모델은 텍스트 프로토콜 폴백(2안) — 03-4. 스냅의 메인 사용처가 로컬 AI이므로 폴백은 스모크(03-10)보다 앞서 만드는 순서 필수. (3) 코드 모드는 `execute` 브리지(3안)로 두되 03-8에서 1차 읽기 전용으로 제한 — 도구 정의·실행·승인 게이트는 03의 `Tool` 경로 그대로, 스크립트 안 쓰기·명령 호출은 승인 게이트에서 승인 시까지 대기. (4) 모델 프로필에 호출 형식 필드를 두고(auto/`openai`/`text`) 경로 선택을 설정으로 정한다.
- 근거: 2026-11-08 사용자 판단 — Cursor 단독은 로컬 도구 호출 실패가 잦음, OpenAI 단독은 로컬이 약점(스냅 메인과 충돌), OpenCode는 로컬 호출이 가장 원만하나 보안 부담. 비교 근거: [research/12-tool-calling-forms.md](research/12-tool-calling-forms.md)(4절 다섯 축, 5절 준수율 수치). 코드 모드 승인 축 근거: 12번 2절 3갈래·6절 한계.
- 배치(2026-11-08 번호 재정렬로 확정): 03-1 트레이트 설계 → 03-2 루프 → 03-3 파일 도구 → 03-4 폴백 → 03-5 AGENTS.md → 03-6 run_command → 03-7 트리 종료 → 03-8 execute → 03-9(후보) → 03-10 스모크. 스모크는 3형식을 다 갖춘 뒤 최후에 한다.
- 다시 볼 점: 03-8 엔진 선택(boa/deno_core/rquickjs)과 라이선스는 착수 전 확정. 03-4의 다중 호출 파싱(2개 이상)은 06 실측 후. 모델 프로필 형식(02 설정 스키마와 결합)은 03-4 착수 시 확정.
