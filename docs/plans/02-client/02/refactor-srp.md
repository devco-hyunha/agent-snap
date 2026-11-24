# 02-2-R SRP 파일 분리

- 상태: 완료 (R1 리뷰 PASS 2026-11-24, R2 리뷰 PASS 2026-11-24, R3 리뷰 PASS 2026-11-24(5차), R4 리뷰 PASS 2026-11-24. 커밋 허락 대기)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md), [tasks.md](tasks.md)
- 근거: 02-2 T3·T4 리뷰에서 파일 내 중복만 고치고 파일 분리는 범위 밖이었다. 값 규칙 검증이 02-5·02-7로 커지기 전에 책임 단위로 나눈다.
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.

## 목적

값 규칙 검증·자격 파일·이름 규칙 코드를 "한 파일 = 한 책임"으로 나눠, 규칙을 바꿀 때 한 파일만 고치게 한다. 동작은 바꾸지 않는다.

## 범위(In)

- `src/config/validate/values.rs`를 책임 4+1로 분리 (엔드포인트 값/모델 값/샘플링/http/숫자 헬퍼)
- `src/config/credentials.rs`를 책임 4로 분리 (오류/읽기·해석/구조 검증/키 조회)
- `src/config/validate/names.rs`에서 trim 정규화를 `normalize.rs`로 분리
- 시험 파일 대응 분리: `tests/validation/values.rs` 3분, `tests/validation/names.rs` 2분
- 문서 연쇄: `tasks.md`에 R 단계 추가, `docs/DECISIONS.md`에 파일 레이아웃 결정 1건

## 비범위(Out)

- 검증 규칙·기본값·오류 문구의 동작 변경
- `validate/error.rs` 분리 (Rust는 열거형 한 축을 여러 파일로 못 나눈다)
- `types.rs` 분리 (설정 스키마 한 책임으로 응집. 기본값 창구 분리는 KISS에 걸려 보류)
- 퍼지는 경로 변경 (`pub use` 재수출로 기존 경로 유지)

## 설계 요약

### R1 값 검증 분리
- 범위: `validate/values.rs`를 `validate/values/` 디렉터리로 교체.
  - `values.rs` (진입점 `check_values`, 기존 그대로)
  - `values/endpoint.rs` (`kind`·`base_url`·`reasoning`·`extra`)
  - `values/sampling.rs` (`check_sampling`·`check_sampling_items`)
  - `values/http.rs` (`check_http`·`check_http_items`·`check_backoff_range`)
  - `values/num.rs` (`is_number_between`·`is_integer_at_least`·`is_seconds`)
- 함수 이동만. 시그니처·동작 그대로. `pub` 여부는 필요한 최소로.
- 산출물: 위 5개 파일, 기존 `values.rs`는 진입점만 남김.
- 검증: 게이트 3종.

### R2 자격 파일 분리
- 범위: `credentials.rs`를 `credentials/` 디렉터리로 교체.
  - `credentials.rs` (진입점, 기존 pub 4종 재수출)
  - `credentials/error.rs` (`CredentialsError`)
  - `credentials/read.rs` (`read_credentials`·`credentials_from_str`·`credentials_path`·`display_path`)
  - `credentials/shape.rs` (`api_keys_from_value`·`invalid_shape`)
  - `credentials/lookup.rs` (`find_api_key`)
- 외부 경로 `config::credentials::*`는 그대로.
- 검증: 게이트 3종.

### R3 이름 trim 분리
- 범위: `validate/names.rs`에서 `trim_names`·`trim_field`를 `validate/normalize.rs`로 이동. `names.rs`는 이름 검증만 남김.
- `validate/mod.rs`의 `pub use names::trim_names`는 `normalize::trim_names`로 갱신 (외부 경로 `validate::trim_names`는 유지).
- 검증: 게이트 3종.

### R4 시험 파일 분리
- 범위: `tests/validation/values.rs`를 `values/endpoint.rs`·`values/sampling.rs`·`values/http.rs`로, `tests/validation/names.rs`를 `names.rs`(검증)·`normalize.rs`(trim)로 나눔.
- `values/` 진입점은 `values/mod.rs`가 맡고 `mod endpoint; mod sampling; mod http;`을 선언.
- `tests/validation/main.rs`에 `mod normalize;` 추가. `material.rs`는 그대로 사용.
- 시험 개수: 26개 유지 (이동만, 추가 없음).
- 검증: 게이트 3종 Windows·WSL.

## AC

1. Given 게이트 3종, When Windows·WSL에서 실행, Then 전부 통과 (시험 70개 유지).
2. Given 분리 전 함수, When 코드 비교, Then 검증 규칙·기본값·오류 문구의 동작 변경 없음 (이동만).
3. Given 기존 공개 경로(`validate::check_config`·`validate::trim_names`·`credentials::*`), When 시험·호출 코드, Then 경로 변경 없음.
4. Given 분리된 각 파일, When 책임 목록, Then 변경 사유가 하나이고 그 사유로만 바뀌는 파일임.
5. Given 새 파일의 독스트링·주석, When 언어 점검, Then 명사형 한 줄(한다·된다·입니다 금지).
6. Given `validate/error.rs`·`types.rs`, When 분리 시도, Then 그대로 유지 (분리 없음 기록).

## 시험 계획

네트워크 없는 기존 단위 시험을 그대로 옮긴다. 새 시험은 만들지 않는다. 게이트 3종을 Windows 로컬과 WSL(Ubuntu) 양쪽에서 돌리고 로그는 `.local/`에 남긴 뒤 PASS 확정 후 삭제한다.

## 문서 영향

- `docs/plans/02-client/02/tasks.md`: R 단계 추가·상태 갱신
- `docs/DECISIONS.md`: SRP 분리 결정 1건
- `docs/INDEX.md`: 문서 추가 반영
- 리뷰 기록: `docs/reviews/02/02-2-r{1,2,3,4}-r{n}.md`

## 잔여 메모

- `crates/snap-core/tests/credentials/lookup.rs`의 `allows_comments_in_jsonc`·`rejects_trailing_comma`는 왕복·trim·이름없음 책임과 다른 jsonc 해석 책임 시험이다 (02-1 T3 레거시 기원, R2 변경분 아님). 후속에서 `tests/credentials/` 안 해석 책임 파일로 분리한다 (R2 리뷰 1차 지적 2, 2026-11-24). → 해소: 2026-11-24 8차 보정에서 해석 시험 2개를 `crates/snap-core/tests/credentials/parse.rs`로 옮기고, 조회 책임 시험 3개만 `lookup.rs`에 남겼다 (게이트 3종 Windows·WSL 통과, 시험 70개 유지).
- R2가 `credentials/read.rs`에 둔 `display_path`는 T6 보정(2026-11-24)에서 `crates/snap-core/src/config/paths.rs`로 모았다가, 2차 보정에서 그 파일을 없애고 호출부가 `path.display().to_string()`을 직접 쓰게 했다 (PROJECT_RULES "함수 1개 전용 파일 금지"). 위 R2 절의 경로 목록은 그 시점 스냅샷으로 남긴다 (HARNESS 7절).

## 리스크

- 파일 이동 중 import 누락 → 게이트 3종이 잡음.
- 시험 파일 분리 시 `mod` 선언 누락 → `cargo test`가 잡음.
- 과분리: 127줄 `names.rs` 분리는 이득이 가장 작음 → R3는 승인 후 취소 가능.

## DoD

AC 1~6 충족, 게이트 3종 Windows·WSL 통과, `docs/reviews/02/`에 리뷰 PASS 기록, 문서 연쇄 반영 목록 완료.
