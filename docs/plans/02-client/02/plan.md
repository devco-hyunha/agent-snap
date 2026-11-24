# 02-2 설정 읽기와 검증

- 상태: 완료 (T1~T5 리뷰 PASS. R SRP 분리 R1~R4 리뷰 PASS. T6 구조 점검은 3차 PASS 무효 → 5차 전수 점검 → 6차 시험 분리 PASS → 7차 잔여 2건 보정 PASS → 8차 잔여 1건(`tests/credentials/lookup.rs`의 jsonc 해석 시험 2개) 해소 PASS 2026-11-24. T6 잔여 없음. 커밋 `0090f3a`, PR #4 병합으로 `feature/client` 반영 완료)

## 근거
- 기획 정본: `docs/plans/02-client.md` 범위(In) 02-2와 설계 요약의 설정·`language`·`http` 항목, AC 2·3.
- 02-1 완료 산출물 계승: `SecretString`(비밀 값 래퍼), `config/credentials.rs`의 이름별 키 조회, `config/config_dir`(OS별 파일), `consts/defaults.rs`, `locale/ko|en/config.rs`.
- 선행 계획 잔여 없음(02-1 완료, 6차 5차 리뷰 PASS).
- 사용자 확정(2026-11-23 채팅): kind 6종 라벨 + 목록 밖 서버는 `extra`로 우회, `sampling` 3종 포함, 모델 `name`·`id` 분리, `base_url`을 `SecretString`으로 보관, JSONC 해석기를 `jsonc-parser`로 통일, 옛 형식 4종 명시 거부, 설정 폴더 환경 변수 우선은 02-2 범위 밖, 검증은 게이트 3종 Windows + WSL.

## 목적
`snap.jsonc`를 읽어 엔드포인트와 그 아래 모델, 샘플링, 시간·재시도 값으로 검증한다. 잘못된 설정은 읽는 시점에 이름만 밝힌 오류로 거부한다. 이후 세부 작업(02-3~02-8)은 이 설정 값만 믿고 쓴다.

## 범위(In)
- 설정 항목: 최상위 `language`(기본 `ko`), `endpoints` 배열(`name`, `base_url`, `kind`, 그 아래 `models`), 모델(`name`, 서버에 보낼 `id`, `sampling`, `reasoning`, `extra`), 최상위 `sampling` 기본값, `http` 시간·재시도 값.
- `endpoints`·`models` 생략·빈 배열은 허용하고 `[]`로 정규화한다.
- `base_url`은 `SecretString`으로 담는다.
- 키 정하기: 자격 파일 `api_keys`에 trim한 엔드포인트 이름이 있고 빈 문자열이 아니면 그 키, 없거나 빈 문자열이거나 자격 파일이 없으면 키 없음. 02-1의 조회를 쓴다.
- JSONC 해석기를 `jsonc-parser`로 통일한다. 자격 파일 읽기도 같은 해석기를 쓰게 교체한다.
- `snap.example.jsonc`를 저장소 루트에 자리표시자만으로 만든다.
- `.gitignore`에 `credentials.jsonc`를 추가한다.
- 설정 오류 문구를 `locale/ko/config.rs`와 `locale/en/config.rs`에 둔다.

## 비범위(Out)
- 모델 `context_window` 읽기·검증: 02-2b(02-7 뒤).
- `reasoning`을 서버별 요청 필드로 바꾸고 `extra`를 요청 본문에 합치는 일: 02-5.
- 선택 키(`엔드포인트이름/모델이름`)로 클라이언트 만들기: 02-7.
- 설정 저장: 02-8.
- 설정 폴더 환경 변수 우선(`SNAP_CONFIG_DIR`): 이번 범위에 넣지 않는다. 02-1의 `config_dir`를 쓴다.
- `language` 값으로 locale을 고르는 전환: 작업 04. 02-2는 값 읽고 검증만 한다.
- 옛 단일 엔드포인트 설정 형식 호환, 엔드포인트 연결 여부 확인(네트워크 접근 없음).

## 설계 요약
- 설정 진실은 중첩(`endpoints[].models`)으로 보관한다. 최상위 `models`와 모델의 `endpoint` 참조는 없다.
- 배열만 쓰고 객체 맵은 쓰지 않는다. 이름은 `name` 필드다.
- `kind`는 서버 전체 목록이 아니다. OpenAI 호환 공통 형식에서 요청 필드 이름이 다른 서버를 가르는 값이다. 허용값은 `openai`, `openrouter`, `ollama`, `vllm`, `llamacpp`, `lmstudio` 6종이고, 모르는 값은 엔드포인트 이름만 밝히고 거부한다. 목록에 없는 서버는 `extra`로 우회하고, 목록을 늘리는 일은 새 결정으로 연다.
- 검증: 엔드포인트 이름 중복(trim 후 정확히 같을 때만), trim 후 빈 엔드포인트·모델 이름·모델 `id`, trim 후 `/`가 든 이름, 같은 엔드포인트 안 모델 이름 중복, `kind` 값, `reasoning` 값(`off`, `low`, `medium`, `high`), `base_url` 형식(scheme `http`/`https`만, 빈 문자열·앞뒤 공백·끝 슬래시 거부, `/v1` 경로는 검사하지 않음), `sampling` 항목과 범위(`temperature` 0 이상 2 이하, `top_p` 0 초과 1 이하, `max_tokens` 1 이상 정수), `http` 키 7종과 값(시간은 0 초과 유한, `retries`는 0 이상 정수, `backoff_min`이 `backoff_max`보다 크면 거부), `extra`의 예약 키(`model`, `messages`, `stream`, `tools`).
- 거부 칸: 엔드포인트의 `api_key`·`api_key_env`(값은 읽지도 출력하지 않음), 최상위 옛 `models`, 모델의 `endpoint` 칸. 그 밖 알 수 없는 칸은 무시한다.
- 오류에는 이름만 담고 값·주소·키는 담지 않는다. JSONC 문법 오류는 줄 번호만 알린다.
- 상수는 `consts/defaults.rs`(기본값)와 `consts/limits.rs`(범위 한도)에 둔다. 사람 문구는 `locale/ko|en/config.rs`에 둔다.
- 설정 모듈은 `config/` 아래에 둔다. 읽기·검증·키 정하기를 책임별로 나눈다. 공용 JSONC 해석기는 자격 파일과 설정 파일이 함께 쓰므로 `src/jsonc.rs` 최상위에 둔다. OS별 파일 규칙은 02-1을 계승한다.

## 근거 자료
| 자료 | 반영하는 점 |
|---|---|
| llama.cpp 서버 README | 모델 `id`가 파일 경로일 수 있어 `name`과 `id`를 따로 둔다 |
| Ollama OpenAI 호환 문서 | 로컬 서버는 키를 받지 않으므로 키 없는 엔드포인트를 허용한다 |
| 서버별 생각 옵션 필드 | `kind`별 변환은 02-5가 맡고 02-2는 값 검증만 한다 |

`docs/RESEARCH.md` 작업 02 절에 적용 위치를 갱신한다.

## 검증 명령
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽에서 실행한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 해석기 통일 | `jsonc-parser` 도입, `snap-core/src/jsonc.rs` 공용 모듈로 파서·줄 번호 오류를 묶고 자격 파일 읽기가 이를 쓰게 교체 | 자격 파일 읽기 규칙(형식·trim·None) 변경 금지 | 공용 모듈, `credentials.rs` 교체, `docs/DECISIONS.md` 결정 기록 | 게이트 3종 |
| T2 설정 타입·기본값 | `Config`·`EndpointConfig`·`ModelConfig`·`Sampling`·`HttpSettings` 타입과 기본값, `language` 기본 `ko`, `base_url`을 `SecretString`으로 보관, 생략은 `[]` 정규화 | `context_window`·`agent`·`tool_call_form` 타입 추가 금지 | 타입과 단위 테스트 | 게이트 3종 |
| T3 검증 규칙 | 이름 trim·`/`·중복·빈 값, `kind` 6종, `reasoning` 4종, `base_url` 형식, `sampling` 범위·모르는 항목, `http` 7키·값, `extra` 예약 키, 거부 칸 4종 | 값·주소·키를 오류에 담기 금지 | 검증 코드와 단위 테스트 | 게이트 3종 |
| T4 키 정하기 | 검증된 엔드포인트와 자격 파일 값으로 키 매핑(이름 있음/빈 문자열/파일 없음) | 자격 파일 해석 규칙 변경 금지 | 키 매핑 코드와 단위 테스트 | 게이트 3종 |
| T5 예시·문서 정리 | `snap.example.jsonc`(자리표시자만), `.gitignore`에 `credentials.jsonc`, 게이트 3종 Windows·WSL, `docs/RESEARCH.md`·`docs/DECISIONS.md`·상태 갱신 | 계획 문서 재설계 금지 | 예시 파일, 게이트 기록, 문서 갱신 | 게이트 3종 (02-2 전체 1회) |
| T6 구조 점검 | 저장소 전체를 파일·함수·폴더 단위로 `harness/PROJECT_RULES.md` 개발 규칙에 견줘 확인 (책임 중복 파일, 100줄 초과 함수, 300줄 초과 시험 파일, 요구 중복 시험 파일, 순수 재수출·함수 1개 전용 파일, 접두 되풀, 파일 하나 폴더 감싸기, OS 분기 동거) | 기능 단위 통과로 갈음 금지 | 파일·함수·폴더 경로와 위반 사유 목록, 분리 필요 시 잔여 이관 | 게이트 3종 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/02/02/T1-jsonc-parser.md` | T1 | 완료 |
| `docs/tasks/02/02/T2-config-types.md` | T2 | 완료 |
| `docs/tasks/02/02/T3-validation.md` | T3 | 완료 (5차 리뷰 PASS 2026-11-24) |
| `docs/tasks/02/02/T4-key-resolve.md` | T4 | 완료 (2차 리뷰 PASS 2026-11-24) |
| `docs/tasks/02/02/T5-example-docs.md` | T5 | 완료 (5차 리뷰 PASS 2026-11-24) |
| `docs/plans/02-client/02/tasks.md` T6 절 | T6 | 완료 (3차 리뷰 PASS 2026-11-24) |

## AC
1. 주석이 든 정상 설정(`endpoints[].models` 중첩)을 읽으면 엔드포인트·모델·샘플링·시간 값이 읽고, 빠진 값과 생략한 `endpoints`·`models`는 기본값(`[]` 포함)이 됨을 테스트로 확인한다.
2. 이름 중복(trim 후 정확히 같을 때만), trim 후 빈 이름·`id`, trim 후 `/`가 든 이름, 모르는 `kind`·`reasoning`, 범위를 벗어나거나 모르는 `sampling` 항목, `base_url` 형식 오류, `extra` 예약 키, `http`의 모르는 키와 값 오류, 거부 칸 4종이 각각 있을 때 거부되고 오류에 이름만 있으며 값·주소·키가 없음을 테스트로 확인한다. 다른 엔드포인트의 같은 모델 별칭은 허용한다.
3. 검증된 엔드포인트와 자격 파일 값으로 키를 정할 때, trim한 이름이 있고 빈 문자열이 아니면 그 키, 없거나 빈 문자열이거나 자격 파일이 없으면 키 없음으로 매핑됨을 테스트로 확인한다.
4. JSONC 문법 오류는 줄 번호만 알리고 값·줄 내용이 없음을 테스트로 확인한다.
5. 설정의 `Debug`와 모든 설정 오류의 `Display`·`Debug`에 `base_url` 호스트와 키가 없음을 테스트로 확인한다. `snap.example.jsonc`는 자리표시자만 담고 읽으면 검증을 통과한다.
6. 게이트 3종이 Windows와 WSL(Ubuntu)에서 통과하고, 테스트 밖 `unwrap`/`expect`/`panic` 0건, 공개 항목에 한국어 문서 주석.
7. `jsonc-parser` 라이선스 확인이 `docs/DECISIONS.md`에 있고, 리뷰 기록이 `docs/reviews/02/`에 있다.

## 테스트 계획
- 네트워크 없는 단위 테스트만 쓴다. 설정 읽기 시험은 임시 폴더를 쓰고, 자격 파일 값과 운영체제 관례 값은 주입한다.
- 시험 파일은 책임별로 나눈다(읽기와 기본값, 검증, 키 정하기, 오류 가리기, 예시 파일). 같은 접두를 공유하는 시험은 접두 폴더로 묶는다.
- 실제 키 값과 주소는 시험 로그에 남기지 않는다.

## 문서 영향
- `docs/DECISIONS.md`: `jsonc-parser` 라이선스 확인과 해석기 통일 결정을 새 번호로 추가한다.
- `docs/RESEARCH.md`: 작업 02 절에 llama.cpp·Ollama 항목의 02-2 적용 위치를 갱신한다.
- `docs/plans/02-client.md`: 02-2 상태 줄을 세부 작업 종료 시 반영한다.
- `snap.example.jsonc`: 새로 만든다(자리표시자만).
- `.gitignore`: `credentials.jsonc`를 추가한다.
- `crates/snap-core/src/locale/ko/config.rs`·`locale/en/config.rs`: 설정 오류 문구 키를 추가한다.

## 리스크
- 자격 파일 읽기를 같은 해석기로 교체하면 02-1의 시험 13개가 함께 탄다. 규칙(형식·trim·None)은 그대로 두고 파서만 바꾼다.
- `jsonc-parser`는 마지막 쉼표를 허용한다. 02-1의 기록은 마지막 쉼표를 허용하지 않는 것이므로 자격 파일 규칙이 느려진다. 결정 기록에 그 차이를 밝힌다.
- 설정에 `api_key`를 적거나 `base_url` 값이 오류에 찍히게 할 수 있다. 칸 이름만으로 거부하고 값을 담지 않는 것을 테스트로 확인한다.
- 서버가 실제로 받는 `sampling` 범위가 문서와 다르면 작업 06에서 다시 본다.

## DoD
AC 1~7 충족, 게이트 3종 Windows·WSL 통과, `docs/reviews/02/`에 리뷰 PASS 기록, 문서 연쇄 반영 목록 완료.
