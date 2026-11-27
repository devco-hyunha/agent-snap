# 02-5 클라이언트 트레이트와 스트리밍 전송: 상세 기획

- 상태: 완료 (T1~T10 리뷰 PASS·작업 단위 전체 리뷰 r2 PASS 2026-11-27. AC 6개 충족, 게이트 4종 Windows·WSL(Ubuntu) 양쪽 통과. 커밋. 라운드 이력과 리뷰 기록은 `docs/reviews/02/02-5-*.md` 참조.)
- 상위 문서: [02-client.md](../../02-client.md)

## 목적

OpenAI 호환 엔드포인트에 스트리밍 요청을 보내고 02-3으로 복원한 이벤트를 돌려준다. 이 세부 작업에서는 재시도하지 않는다. 에이전트 루프(작업 03)는 이 클라이언트를 트레이트로만 받고, 감시(작업 05)는 스트림을 중간에 끊을 수 있게 취소를 지원한다.

## 리서치 및 설계 근거

- 상세(신규): [docs/research/02-client-transport.md](../../../research/02-client-transport.md) — 2026-11-26 조사 서브가 13종 링크를 직접 열어 원문 문구를 확인했다. 새 크레이트 5종의 라이선스도 함께 확인했다.
- `kind`별 `reasoning` 요청 필드 변환 표(2026-11-03 문서 확인): `openai`·`ollama`·`vllm`·`llamacpp`는 `reasoning_effort`, `openrouter`는 `reasoning: { "effort": … }`, `lmstudio`는 공통 필드 없음(`extra`만). 값 매핑은 `off`→`none`, `low`·`medium`·`high`는 그대로.
- Ollama OpenAI compatibility: `tool_choice`·`n`·`logit_bias` 미지원. 로컬 서버는 인증 헤더를 요구하지 않고 키를 보내도 무시한다.
- OpenRouter Streaming: 헤더 이후 오류는 HTTP 200 안 최상위 `error` 청크로 온다. 연결을 끊으면 지원하는 공급자에서 생성과 과금이 멈춘다 → 취소 시 연결 정리 근거.
- reqwest `ClientBuilder`: `connect_timeout`은 연결 단계만, `timeout`은 요청 전체, `read_timeout`은 읽기마다 잰다. 헤더 대기와 스트림 유휴를 구분하지 못해 `timeout`·`read_timeout`은 쓰지 않는다.
- tokio-util `CancellationToken`: 복제본이 취소 상태를 공유하고 `cancelled()`는 cancel-safe라 `select!`에 쓸 수 있다.
- DistServe (arXiv 2401.09670)·Etalon (arXiv 2407.07000)·When Is the Same Model Not the Same Service? (arXiv 2605.02821): TTFT·토큰당 시간 정의를 따라 요청 시작·첫 토큰·마지막 토큰 원시 시각만 돌려준다. 평균 지표·토큰별 시각 기록은 만들지 않는다.
- 결정 기록: 14번(전송·취소·모의 서버 크레이트)을 T1에서 기록했고, 15번(`reasoning` 변환과 `extra` 병합)은 T4에서 기록한다. 헤더 전 연결 오류 매핑은 02-4의 12번 매핑 표를 그대로 쓴다.
- 확인된 사실: 오류에 base_url과 키를 넣지 않는다. 헤더 대기와 스트림 유휴를 나눠 잰다.

## 범위(In)

- LLM 클라이언트 트레이트: 요청과 취소 토큰을 받아 이벤트 스트림을 연다.
- OpenAI 호환 스트리밍 전송 구현.
  - 요청 본문: 모델 id, 메시지, 도구, 샘플링, `stream`. `tool_choice`·`n`·`stream_options`는 보내지 않는다.
  - 모델의 `reasoning`을 엔드포인트 `kind`에 맞는 요청 필드로 바꿔 싣고, 모델의 `extra`를 요청 본문에 그대로 합친다(결정 15번).
  - 인증: 자격 파일에서 정해진 키가 있으면 `Authorization: Bearer`, 없으면 헤더를 보내지 않는다.
  - 시간 초과: 연결, 헤더 대기, 스트림 유휴를 나눠 잰다.
  - 취소 시 연결 정리.
  - 오류 가리기: 어림 가리기 뒤 알고 있는 키·base_url 호스트 문자열로 한 번 더 가린다.
  - 헤더 전 연결 오류를 02-4의 `PreHeaderConnectKind` 여섯 종류로 구분한다. 재시도 호출은 02-6.
  - 사용량과 시각(요청 시작·첫 토큰·마지막 토큰)을 돌려준다.
- `StreamState` 실패 후 재호출 계약: 한 번 실패한 스트림 상태에 `push`·`finish`를 다시 부르면 성공으로 오해하지 않음을 테스트로 고정한다.
- 엔드포인트 종류를 요청 변환이 쓰는 타입(`EndpointKind`)으로 만들고, 설정 검증의 `kind` 허용값과 한자리에서 연결한다.
- 모의 서버: 응답 시점(헤더 지연, 본문 멈춤)과 연결 종료를 테스트가 제어하고 관찰한다. 받은 요청 헤더·본문을 기록한다.

## 비범위(Out)

- 재시도 적용: [02-6](../06/plan.md).
- 선택 키(`엔드포인트이름/모델이름`)로 클라이언트 만들기, 모델 목록 조회: [02-7](../07/plan.md).
- SSE 해석 규칙 재구현: 02-3 API만 쓴다.
- `stream_options`, 비스트리밍 요청.
- `reasoning`·`extra` 값 검증(범위·예약 키 거부): 02-2에서 끝났다. 이 세부 작업은 검증된 값을 싣기만 한다.
- `context_window`: 요청 본문에 싣지 않는다(생략 시 65536, 사용은 작업 04).
- 토큰별 도착 시각 기록과 지표 계산, 평균 지표 계산.
- SSE 버퍼 비용 개선.

## 선행 세부 작업

[02-4 재시도 판단](../04/plan.md) 완료(PR #7 병합 2026-11-25). 02-1의 비밀 값 래퍼와 키 정하기, 02-2의 설정 타입·`http` 값, 02-3의 스트림 복원을 쓴다.

## 리스크

- `lmstudio`는 문서에 공통 thinking 요청 필드가 없어 `extra`에 의존한다. 서버·버전에 따라 무시될 수 있다(작업 06에서 실서버로 다시 본다).
- 시간에 의존하는 모의 서버 테스트가 흔들릴 수 있다. 시험용 시간 값은 짧게 주고 경과 시간은 넉넉한 하한·상한으로만 확인한다.
- 서버가 오류 본문에 키를 되돌려 줄 수 있다. 어림 가리기 뒤 알고 있는 키·호스트 문자열로 한 번 더 가린다.
- 연결 실패 오류는 HTTP 라이브러리가 주소를 오류 문자열에 넣을 수 있다. 주소를 담지 않는 오류로 바꿔 보고한다.
- 새 외부 크레이트(HTTP 클라이언트, 비동기 런타임, 취소 토큰, 모의 서버)를 처음 쓰므로 T1에서 라이선스를 확인해 `docs/DECISIONS.md`에 남긴다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 | 새 외부 크레이트(reqwest, tokio, tokio-util, 시험용 axum·http-body) 라이선스를 crates.io·포장 LICENSE로 확인하고 결정 14번 기록, `Cargo.toml`에 의존 추가 | 실서버 호출, 커밋·푸시, 임시 스크립트 | 결정 14번 기록, `crates/snap-core/Cargo.toml` 의존 | 게이트 3종, `cargo deny -t x86_64-pc-windows-msvc -t x86_64-unknown-linux-gnu check` |
| T2 | 시험용 모의 서버: 헤더 지연·본문 멈춤·연결 종료 제어, 요청 헤더·본문 기록, 모의 서버 자체 시험 | 제품 공개 API에 모의 서버 노출, 재시도 로직 | 시험 전용 모의 서버 모듈과 자체 시험 | 게이트 3종 |
| T3 | 클라이언트 트레이트와 요청·스트림·오류 타입 골격, `EndpointKind` 타입을 설정 검증과 연결 | reqwest 타입이 트레이트 밖으로 새기, 재시도 호출 | `llm/client/` 트레이트·타입, `llm/mod.rs` 재수출 | 게이트 3종 |
| T4 | 요청 본문 조립과 인증 헤더: `kind`별 `reasoning` 변환, `extra` 병합, Bearer 있으면만 전송 | `tool_choice`·`n`·`stream_options` 전송, `context_window` 본문 실기 | 요청 조립 모듈과 `kind`별 본문 시험 | 게이트 3종 |
| T5 | 응답 스트림을 02-3 복원에 연결, 실패 후 재호출 계약 고정 | SSE·델타 규칙 재구현 | 스트림 연결 코드와 복원·계약 시험 | 게이트 3종 |
| T6 | 시간 초과(연결·헤더 대기·스트림 유휴)와 취소(취소 오류·연결 정리) | reqwest `timeout`·`read_timeout`로 구간 묶기, 재시도 대기 중 취소 | 시간 초과·취소 코드와 통합 시험 | 게이트 3종 |
| T7 | 오류 종류와 가리기: 헤더 전 연결 6종 구분, 어림 가리기에 알고 있는 키·호스트 2차 가리기 | 02-4 `decide_retry` 호출, 오류에 키·base_url 원문 담기 | 전송 오류 타입·분류·가리기와 통합 시험 | 게이트 3종 |
| T8 | 사용량과 시각(요청 시작·첫 토큰·마지막 토큰) 반환, `docs/RESEARCH.md`·`docs/research/02-client-transport.md`에 02-5 코드 적용 위치 갱신 | 평균 지표·토큰별 시각 기록 | 사용량·시각 타입과 시험, 조사 적용 위치 갱신 | 게이트 3종 |
| T9 | 미커밋 작업 파일 전수 구조 점검 표 작성, 위반 잔여 이관 | 코드 변경, 파일 삭제, 커밋·푸시 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |
| T10 | T9 잔여 4건 해소(강도 허용값 정본 통합, 상수 자리 이동, 타입·로직·책임 분리, 시험 재료 정본 재사용) | 동작 변경, 파일 삭제, 커밋·푸시 | 분리·이동 파일(`effort.rs`·`consts/status.rs`·`token_times.rs`·`store.rs`·`timeout_connect.rs`·`connect_redact.rs`·`restore_timings.rs`), 잔여 해소 기록 | 게이트 3종 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/plans/02-client/05/plan.md` | 상세 기획 | 완료 |
| `docs/plans/02-client/05/spec.md` | 명세·AC | 완료 |
| `docs/plans/02-client/05/tasks.md` | 작업 계획·DoD | 완료 |
| `docs/research/02-client-transport.md` | 02-5 조사 상세 | 완료 |
| `docs/tasks/02/05/T1-crates-license.md` | T1 위임 문서 | 완료 |
| `docs/tasks/02/05/T2-mock-server.md` | T2 위임 문서 | 완료 |
| `docs/tasks/02/05/T3-client-traits.md` | T3 위임 문서 | 완료 |
| `docs/tasks/02/05/T4-request-body.md` | T4 위임 문서 | 완료 |
| `docs/tasks/02/05/T5-stream-restore.md` | T5 위임 문서 | 완료 |
| `docs/tasks/02/05/T6-timeout-cancel.md` | T6 위임 문서 | 완료 |
| `docs/tasks/02/05/T7-errors-redact.md` | T7 위임 문서 | 완료 |
| `docs/tasks/02/05/T8-usage-timing.md` | T8 위임 문서 | 완료 |
| `docs/tasks/02/05/T9-structure-check.md` | T9 위임 문서 | 완료 |
| `docs/tasks/02/05/T10-structure-fix.md` | T10 위임 문서 | 완료 |

## 문서 영향

- `docs/research/02-client-transport.md`: 신규(02-5 항목 이식, T1 라이선스 확인 결과 반영).
- `docs/RESEARCH.md` 02 절: 02-5 항목 적용 위치(코드 경로) 갱신.
- `docs/DECISIONS.md`: 14번(전송·취소·모의 서버 크레이트) 기록 완료. 15번(`reasoning` 변환과 `extra` 병합)은 T4에서 추가한다.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/05/*` 상태 줄 갱신.

## DoD

[spec.md](spec.md)의 AC 6개를 모두 충족하고, 게이트 3종(Windows·WSL)을 통과하고, 조사 적용 위치가 갱신되고, 작업 단위 전체 리뷰가 PASS한 상태.
