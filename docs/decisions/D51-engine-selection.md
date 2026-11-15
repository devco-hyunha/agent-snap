# D51 03-8 엔진 선택: rquickjs (정본)

- 상태: recorded (2026-11-15 조사 확정, 사용자 승인 대기)
- 요약 인덱스: [docs/DECISIONS.md](../DECISIONS.md#d51-03-8-엔진-선택-rquickjs-2026-11-15)
- 조사 상세·비교표·직접 열람 링크: [docs/tasks/03/03-08-execute-bridge/T1-engine-selection.md](../tasks/03/03-08-execute-bridge/T1-engine-selection.md)
- 이 문서가 해당 결정의 상세 정본. DECISIONS.md에는 요약과 링크만 둔다.

## 결정

- 채택 엔진: `rquickjs` 0.14.0, 기능 `std`(기본) + `futures`(ES6 Promise ↔ Rust future 왕복).
- 추가할 의존: `crates/snap-core/Cargo.toml` `[dependencies]`에 `rquickjs` 한 항목. 그 아래 `rquickjs-core` → `rquickjs-sys`(QuickJS-NG C). 1차에서 `macro`·`loader`·`chrono`·`indexmap`·`either`·`parallel`은 켜지 않는다.
- 라이선스는 MIT 한 개만 들어간다. D5·D18·D24·D30이 정한 확인 방식(라이선스 허용형만 채택)을 따라 crates.io의 라이선스 표기와 README의 라이선스 절을 직접 열람해 확인했다.
- `bindgen` 기능은 두 OS 표적(`x86_64-pc-windows-msvc`·`x86_64-unknown-linux-gnu`)에 사전 생성 바인딩이 있어 필요하지 않다(2026-11-15 crates.io·docs.rs 직접 열람).

## 기각 후보와 근거

| 후보 | 기각 근거 (2026-11-15 crates.io·docs.rs 직접 열람) |
|---|---|
| `boa_engine` 0.22.0 | 라이선스 `Unlicense OR MIT`로 허용형이 없는 것은 아니지만, 직접 필수 의존 43개(선택 31) + MSRV 1.91.0. 커스텀 `JobExecutor`를 붙여야 승인 대기 future와 만난다. M1 저사양이 목표인 snap에 무겁다 |
| `deno_core` 0.413.0 | MIT이나 V8(`deno_v8`)과 직접 의존 33개를 깔고, `JsRuntime::run_event_loop`를 tokio 위에서 돌린다. plan.md:17의 "V8, 무거움" 평가 그대로 |
| `mlua` 0.12.2 | MIT·직접 의존 7개. await는 Lua coroutine 기반 `async`이고, 모델의 코드 작성 학습성이 JS보다 뒤진다. `Lua::sandbox`는 Luau 전용이라 스크립트 단위 승인 축에 바로 쓸 수 없다 |
| JSON 호출 체인(자체) | 새 의존 0개. 03-4의 `recover_arguments`·스키마 검증 재사용으로 얇다. 분기·반복 표현이 없어 plan.md:13-15 표현력 요건과 1차 읽기 전용 조합이 성립하지 않는다 |
| Rust 임의 코드·wasm | eval 크레이트가 사실상 없고 컴파일 시간 부담. wasm은 모델 생성 언어로 부적합(plan.md·spec.md:15-16) |

## 승인 게이트 결합 (AC 2)

- `AsyncRuntime`·`AsyncContext`는 Rust future를 ES6 Promise로, ES6 Promise를 Rust future로 왕복한다(2026-11-15 docs.rs `futures` 기능 설명). 스크립트 `read_file` 함수가 async Rust 클로저로 등록되고, 쓰기·명령 호출은 03-6 승인 게이트 future를 기다린다.
- `for (const f of files) write_file(f)`가 반복마다 게이트를 거치는 AC 5는 함수 등록 위치에서 지켜진다.

## M1 저사양 우선 근거

- 엔진은 병목이 아니다. plan.md 리스크 39행: 병목은 스크립트 기동이 아니라 도구 I/O이고, 엔진 선택 기준은 의존성 크기를 앞세운다(T1 메모 3절).
- rquickjs는 QuickJS-NG C를 정적 컴파일하고(라이선스 MIT), 엔진 자체 의존이 사전 의존 1개이며, QuickJS는 "작고 쉽게 심어져야 함"(embedable), 저 시작 시간을 명시한다(2026-11-15 crates.io README).

## 1차 허용 도구 목록 (spec.md:19 대조 확인)

- `read_file`·`grep`·`glob`·`list_dir`: 1차 허용 목록이 spec.md:19와 같다. plan.md 범위(In) 14행과 일치.
- `write_file`·`edit_file`·`run_command`: 1차에서 닫는다. `run_command`만 L1 승인(D49). 개방 순서는 03-4 안정화 뒤(plan.md Out).

## D46 "다시 볼 점" 마감

- D46의 다시 볼 점 "03-8 엔진 선정(boa/deno_core/rquickjs)과 라이선스는 착수 전 확정"이 이 결정으로 마감한다. rquickjs를 고르고 라이선스 MIT를 확인했다.

## 다시 볼 점

- 엔진 기동 시간·최종 바이너리 크기 실측치 없음. rquickjs가 Windows MSVC에서 QuickJS-NG를 붙이는 "experimental" 표시를 crates.io 표에서 두는 만큼, 우리 CI 매트릭스(`windows-latest`) 빌드로 T2에서 실측한다. `windows-latest`와 `ubuntu-latest` 둘 다에 실패하면 `bindgen` 기능을 열고, 그 뒤에도 막히면 엔진을 JSON 호출 체인 경로를 유지하는 쪽으로 바꾼다.
- MSRV 확인: 워크스페이스 `Cargo.toml`에 `rust-version`을 적지 않았다. rquickjs 0.14.0의 MSRV 1.87을 우리 개발 환경이 넘는지 03-8 T2 착수 때 `rust-toolchain.toml`을 두지 않고 CI 매트릭스로 확인한다.
