# 02-2 작업 계획

- 상태: 완료 (T1~T5 완료·전체 리뷰 PASS 2026-11-24. R 값 검증·자격 파일·이름 규칙 SRP 분리 R1~R4 리뷰 PASS. T6 구조 점검 완료. 커밋 `0090f3a`, PR #4 병합으로 `feature/client` 반영 완료)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow.
- 작업 방식: 세부 작업 하나는 한 번 위임하고 한 번 코드 리뷰로 끝낸다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.

## 단계

### T1 해석기 통일
- 상태: 완료 (2차 재리뷰 PASS 2026-11-24, 게이트 3종 Windows 로컬 통과, 로그 `.local/gate-02-2-t1.log`)
- 범위: `jsonc-parser`를 도입하고 `crates/snap-core/src/jsonc.rs` 공용 모듈로 파서·줄 번호 오류를 묶는다. `config/credentials.rs`가 그 해석기를 쓰게 교체한다.
- 금지: 자격 파일 읽기 규칙(형식·trim·None 경로) 변경. 자격 파일 쓰기.
- 확인: 02-1 계승 시험 13개가 그대로 통과하는지. JSONC 문법 오류가 줄 번호만 밝히는지.
- 검증: 게이트 3종.

### T2 설정 타입·기본값
- 상태: 완료 (3차 재리뷰 PASS 2026-11-24. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate-02-2-t2.log`)
- 범위: `Config`·`EndpointConfig`·`ModelConfig`·`Sampling`·`HttpSettings` 타입과 기본값. `language` 기본 `ko`. `base_url`을 `SecretString`으로 보관. 생략한 `endpoints`·`models`는 `[]`로 정규화.
- 금지: `context_window`·`agent`·`tool_call_form` 타입 추가. 검증 규칙 구현.
- 확인: 정상 설정과 빠진 값·빈 목록의 읽은 값과 기본값.
- 검증: 게이트 3종.

### T3 검증 규칙
- 상태: 완료 (5차 재리뷰 PASS 2026-11-24. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate-02-2-t3.log`)- 범위: 이름 trim·`/`·중복·빈 값, `kind` 6종, `reasoning` 4종, `base_url` 형식, `sampling` 범위·모르는 항목, `http` 7키·값, `extra` 예약 키, 거부 칸 4종.
- 금지: 값·주소·키를 오류에 담기. locale 밖 즉석 문자열.
- 확인: 경우마다 거부되고 오류에 이름만 있음.
- 검증: 게이트 3종.

### T4 키 정하기
- 상태: 완료 (2차 재리뷰 PASS 2026-11-24. 게이트 3종 Windows 로컬 통과, 로그 `.local/gate-02-2-t4.log`)
- 범위: 검증된 엔드포인트와 자격 파일 값으로 키 매핑(이름 있음·빈 문자열·이름 없음·파일 없음).
- 금지: 자격 파일 해석 규칙 변경. HTTP 전송.
- 확인: 각 경우의 매핑 결과.
- 검증: 게이트 3종.

### R 값 검증·자격 파일·이름 규칙 SRP 분리
- 상태: 완료 (R1 리뷰 PASS 2026-11-24(2차). R2 리뷰 PASS 2026-11-24(2차). R3 리뷰 PASS 2026-11-24(5차). R4 리뷰 PASS 2026-11-24(2차). 커밋 `0090f3a`, PR #4 병합으로 `feature/client` 반영 완료)
- 범위·검증·산출물: [refactor-srp.md](refactor-srp.md) R1~R4 절을 따른다.
- 확인: 이동만(동작·오류 문구 무변경), 기존 공개 경로 유지, 게이트 3종 Windows·WSL.
- 검증: 게이트 3종. R1 로그 `.local/gate-02-2-r1.log`·`.local/gate-02-2-r1-wsl.log`, R2 로그 `.local/gates/r2-gates.md`, R3 로그 `.local/gate-02-2-r3.log`·`.local/gate-02-2-r3-wsl.log`. R4 로그 `.local/r4-win-{fmt,clippy,test}.log`·`.local/r4-wsl-{fmt,clippy,test}.log`.

### T5 예시·문서 정리
- 상태: 완료 (5차 리뷰 PASS 2026-11-24. 게이트 3종 Windows·WSL 통과, 로그는 PASS 확정 후 삭제)
- 범위: `snap.example.jsonc`(자리표시자만), `.gitignore`에 `credentials.jsonc`, 게이트 3종 Windows·WSL, `docs/RESEARCH.md`·`docs/DECISIONS.md`·상태 갱신.
- 금지: 계획 문서 재설계.
- 확인: 예시 파일이 검증을 통과하는지. 게이트 기록·DECISIONS·RESEARCH·상태 줄 반영.
- 검증: 게이트 3종 (02-2 전체 1회, Windows + WSL).

### T6 구조 점검
- 상태: 완료 (6차 시험 분리 리뷰 PASS → 7차 잔여 2건 보정 PASS → 8차 잔여 1건(해석 시험 2개 이관) 해소 PASS 2026-11-24. T6 잔여 없음. 로그는 PASS 확정 후 삭제)
- 범위: 저장소 전체를 파일·함수·폴더 단위로 `harness/PROJECT_RULES.md` 개발 규칙에 견줘 확인.
- 확인: 책임 둘 이상이 몰린 파일, 요구가 몰린 시험 파일, 순수 forwarding·순수 재수출·함수 1개 전용 파일, 접두 되풀, 파일 하나 폴더 감싸기, OS 분기 동거, `mod.rs`의 `#[cfg]` 본문 함수.
- 검증: 게이트 3종.
- 점검 결과 (독립 리뷰 3b426cd8 판정 미달, 지적 4건):
  - 분리 완료: `validate/values/`·`credentials/`·`validate/normalize.rs`·`tests/validation/values/` (R1~R4). `validate/error.rs`·`types.rs`는 열거형 단일 축·설정 스키마 응집이라 유지 (PROJECT_RULES 과소분리 금지).
  - major 1 `src/config/reader.rs:21·94`: `ConfigError`·읽기·`check_language`·`display_path` 동거 → `config/error.rs` 분리, `check_language`는 검증 자리로 이전 잔여.
  - major 2 `tests/config_dir_path.rs:16·27`: `#[cfg(windows)]`·`#[cfg(unix)]` 시험 한 파일 동거 (HARNESS 10절) → `tests/config_dir_path/`의 `windows.rs`·`unix.rs` 분리 잔여.
  - minor 3 `src/config/reader.rs:94`·`src/config/credentials/read.rs:46`: `display_path` 중복 (dry) → 단일 정본 통합 잔여.
  - minor 4 `src/consts/limits.rs:60·63`: `SAMPLING_SCOPE`·`HTTP_BLOCK` 표시 값이 `consts/`에 있음 (constants-locale) → `locale/` 이동 잔여.
  - 위반 아님: `keys.rs`·`credentials/lookup.rs`·`validate/normalize.rs`의 pub fn 1개 파일 (R2·R3 승인 골격, `mod.rs` 재수출 응집).
  - 잔여: `tests/credentials/lookup.rs`의 jsonc 해석 시험 2개(`allows_comments_in_jsonc`·`rejects_trailing_comma`)가 왕복·trim 책임과 몰려 있음 → `tests/credentials/` 안 해석 책임 파일로 이관 (refactor-srp.md `## 잔여 메모`).
- 보정 결과 (리뷰 1차 지적 4건, 2026-11-24. 동작·오류 문구·시험 단정 변경 없음):
  - major 1 반영: `ConfigError`를 `src/config/error.rs`로 분리 (읽기·검증·경로 표시 제거). `check_language`는 `src/config/validate/language.rs`로 옮겨 `check_config` 흐름(마지막 단계)에 합침. 언어 오류는 `ValidationError::Language`로 통합 — 문구 `CONFIG_LANGUAGE_REJECTED`·`rejects_unknown_language` 단정(`language` 이름 표시, 입력 값 미포함)은 그대로.
  - major 2 반영: `tests/config_dir_path.rs`를 `tests/config_dir_path/`로 교체 (`main.rs` 공통 시험 + OS별 `mod` 선언, `windows.rs`, `unix.rs`). 시험 2개+공통 1개 그대로.
  - minor 3 반영: `display_path` 정본을 `src/config/paths.rs` 한 파일로 통합하고 `reader.rs`·`credentials/read.rs`·`credentials/shape.rs`가 import로 재사용.
  - minor 4 반영: `SAMPLING_SCOPE`·`HTTP_BLOCK`을 `locale/ko/validate.rs`·`locale/en/validate.rs`로 이동 (값·이름 유지). `consts/limits.rs`에서 제거, `validate/values/sampling.rs`·`validate/values/http.rs`가 import.
- 보정 결과 (리뷰 2차 신규 지적 1건, 2026-11-24. 동작·오류 문구·기본값·시험 단정 변경 없음):
  - `src/config/paths.rs` 제거 (함수 1개 전용 파일 금지). `display_path` 대신 호출부 7곳(`config/reader.rs` 3곳, `config/credentials/read.rs` 3곳, `config/credentials/shape.rs` 1곳)이 `path.display().to_string()`을 직접 쓴다. PROJECT_RULES "호출부가 직접 부를 쓰게 한다".
  - `src/config/validate/language.rs` 제거 (함수 1개 전용 파일 금지). `check_language`를 `validate/mod.rs`의 `check_config` 흐름 한 칸(마지막 단계)으로 합친다. `ValidationError::Language`·`CONFIG_LANGUAGE_REJECTED` 문구·입력 값 미포함 규칙 그대로.
  - 삭제 파일 2건: `crates/snap-core/src/config/paths.rs`·`crates/snap-core/src/config/validate/language.rs`. 이유: T6 리뷰 2차 지적 1 (PROJECT_RULES 개발 규칙 "순수 forwarding·순수 재수출·함수 1개 전용 파일을 새 파일로 만들지 않는다"). 근거: `docs/reviews/02/02-2-t6-r2.md`.
  - 게이트 3종 Windows·WSL 통과, 로그 `.local/t6fix3-win-{fmt,clippy,test}.log`·`.local/t6fix3-wsl-{fmt,clippy,test}.log`. 시험 70개 유지.
- 보정 결과 (리뷰 4차 이후 시험 파일 분리, 2026-11-24. 시험 본문·단정·재료 호출 그대로 옮김. 제품 소스 `src/` 미수정. 새 시험·단정 변경 없음):
  - `tests/validation/names.rs`(10) → `tests/validation/names/`의 `duplicate.rs`·`empty.rs`·`separator.rs`·`missing.rs` + `mod.rs`.
  - `tests/validation/values/endpoint.rs`(8) → `tests/validation/values/endpoint/`의 `kind.rs`·`reasoning.rs`·`base_url.rs`·`extra.rs` + `mod.rs`.
  - `tests/validation/values/http.rs`(4) → `tests/validation/values/http/`의 `keys.rs`·`backoff.rs` + `mod.rs`.
  - `tests/validation/values/sampling.rs`(3) → `tests/validation/values/sampling/`의 `range.rs`·`items.rs` + `mod.rs`.
  - `tests/validation/fields.rs`(5) → `tests/validation/fields/`의 `key_fields.rs`·`legacy.rs`·`unknown.rs` + `mod.rs`.
  - `tests/jsonc/parse.rs`(4) → `tests/jsonc/parse/`의 `commented.rs`·`line.rs`·`hide.rs` + `mod.rs`. `parses_commented_jsonc` 파일은 `parse.rs`가 아니라 `commented.rs`로 둔다: clippy `module_inception`(모듈 이름 되풀 금지)이 `-D warnings`에서 막히고, PROJECT_RULES는 파일 이름에 부모 접두 되풀을 금지한다.
  - `tests/jsonc/reader.rs`(2) → `tests/jsonc/reader/`의 `parse_failure.rs`·`trim_none.rs` + `mod.rs`.
  - `tests/config/reject.rs`(4) → `tests/config/reject/`의 `missing_file.rs`·`language.rs`·`parse.rs`·`shape.rs` + `mod.rs`.
  - `tests/config/read.rs`(3) → `tests/config/read/`의 `nested.rs`·`values.rs`·`file.rs` + `mod.rs`.
  - `tests/config/defaults.rs`(5) → `tests/config/defaults/`의 `values.rs`·`normalize.rs` + `mod.rs`. 기본값 책임 파일은 `defaults.rs`가 아니라 `values.rs`(부모 폴더 이름 되풀 금지).
  - `tests/example.rs`(3) → `tests/example/`의 `validation.rs`·`placeholders.rs` + `main.rs`. `example_path` 헬퍼는 `validation.rs`에 두고 `placeholders.rs`는 경로를 직접 쓴다.
  - 분리 골격: 하위 폴더는 `mod.rs`, 최상위 시험 폴더는 `main.rs` (기존 `tests/config/main.rs`·`tests/jsonc/main.rs` 관례와 동일). `tests/validation/main.rs`의 `mod material;`은 유지 (`material.rs` 파일 유지).
  - 삭제 파일 11건: 위 표의 원래 파일 11개 (`tests/validation/names.rs`·`values/endpoint.rs`·`values/http.rs`·`values/sampling.rs`·`fields.rs`, `tests/jsonc/parse.rs`·`reader.rs`, `tests/config/reject.rs`·`read.rs`·`defaults.rs`, `tests/example.rs`). 이유: 서로 다른 책임이 한 시험 파일에 몰려 있음 (PROJECT_RULES "한 시험 파일에 왕복·쓰기 규칙·거부·보존·가리기 같은 서로 다른 요구를 몰아넣지 않는다"). 근거: `docs/reviews/02/02-2-t6-r4.md` 정성 검사 표.
  - 잔여 (미반영): `tests/credentials/lookup.rs`의 jsonc 해석 시험 2개(`allows_comments_in_jsonc`·`rejects_trailing_comma`) 이관. 이번 지시 범위(분리 표 11건)에 없어 그대로 둠. → 8차에서 해소 (아래 보정 결과 참조).
  - 잔여 (미반영, 5차 전수 점검 지적): ① `src/config/unix.rs:21`·`src/config/windows.rs:14`의 `config_dir_in` 동일 로직 사본 (dry). ② `src/config/validate/mod.rs:16`의 `check_config` 흐름 조립과 `check_language` 값 규칙 동거 (separation). 두 건 모두 제품 소스라 이번 지시(시험 파일만 분리) 범위 밖. 근거: `docs/reviews/02/02-2-t6-r5.md`.
  - 게이트 3종 Windows·WSL 통과 (대상 SHA `b4836dced125760d72817539141de5d85add95db`), 로그 `.local/t6split-win-{fmt,clippy,test}.log`·`.local/t6split-wsl-{fmt,clippy,test}.log`. Windows 16:15:48~16:16:17, WSL 07:17:30 UTC. 시험 70개 유지 (Windows 13+2+8+2+6+5+3+31=70, WSL 합계 70).
- 보정 결과 (5차 잔여 2건, 2026-11-24. 사용자 지시 "제품 소스 2건 보정". 동작·오류 문구·기본값·시험 단정 변경 없음):
  - 잔여 ① 해소 (dry): `config_dir_in` 사본 2개를 없애고 정본을 `src/config/mod.rs`의 `pub fn config_dir_in` 정의 한 곳으로 합쳤다. `src/config/unix.rs`·`src/config/windows.rs`는 OS 고유 값(base 경로 계산)만 남기고 `use crate::config::config_dir_in;` import로 재사용한다. `config/mod.rs`의 `#[cfg(windows)] pub use windows::config_dir_in`·`#[cfg(all(unix, not(windows)))] pub use unix::config_dir_in` 재수출 2줄을 없애고, 같은 경로 `snap_core::config::config_dir_in`를 본문 정의 한 곳으로만 내보낸다. 공개 경로가 그대로라 `tests/config_dir_path/main.rs`의 import와 시험 1개는 그대로 통과한다. `config_dir_in`은 OS와 무관한 코드라 `#[cfg]`를 쓰지 않은 본문 함수이고, PROJECT_RULES의 "`mod.rs`의 `#[cfg]` 본문 함수 금지"에 걸리지 않는다. 새 파일은 만들지 않았다 (PROJECT_RULES "함수 1개 전용 파일 금지·호출부가 직접 부를 쓰게 한다"는 두 OS가 함께 쓰는 공유 값이라 호출부가 직접 쓰기 어려운 자리다).
  - 잔여 ② 해소 (separation): `check_language`를 `src/config/validate/mod.rs`에서 빼서 `src/config/validate/values/mod.rs`로 옮기고 `check_values` 흐름의 마지막 칸에 넣었다. 값 규칙 정본 자리(`validate/values/`)에 두는 지시대로이며 새 파일은 만들지 않았다 (`values/mod.rs` 흐름 한 칸에 합침). `check_config`는 거부 칸·이름·값 흐름 3칸만 남기는 조립 골격이 됐다. `ValidationError::Language`·`CONFIG_LANGUAGE_REJECTED` 문구·입력 값 미포함 규칙·허용값 `ALLOWED_LANGUAGES`·`rejects_unknown_language` 단정 그대로.
  - 게이트 3종 Windows·WSL(Ubuntu) 통과, 로그 `.local/t6fix4-win.log`·`.local/t6fix4-wsl.log` (대상 SHA256 5종·종료 코드·시작/끝 시각 기록). Windows 16:52:45~16:52:49, WSL 16:53:07~16:53:23 (KST). 대상 SHA256: `config/mod.rs` 39D5F177…, `config/unix.rs` 5869846D…, `config/windows.rs` 7BF1757D…, `validate/mod.rs` C83043C0…, `validate/values/mod.rs` 28711E8E… (Windows·WSL 동일). 시험 70개 유지 (Windows 13+2+8+2+6+5+3+31=70, WSL 합계 70).
  - 문서 연쇄 반영: `docs/DECISIONS.md` D8의 언어 값 확인 자리 경로를 `config/validate/values/mod.rs`의 `check_values` 마지막 칸으로 갱신.
- 보정 결과 (6차 잔여 1건 해소, 2026-11-24. 시험 파일만 분리. 제품 소스 `src/` 미수정. 시험 본문·단정·재료 호출 그대로 옮김. 새 시험·단정 변경 없음):
  - 해소 (separation·testing): `tests/credentials/lookup.rs`에 몰려 있던 jsonc 해석 시험 2개(`allows_comments_in_jsonc`·`rejects_trailing_comma`)를 `tests/credentials/parse.rs`로 옮겼다. 파일 이름은 부모 접두(`credentials`)를 되풀하지 않고 책임만 밝힌다 (PROJECT_RULES "파일 이름에는 접두를 되풀하지 않는다"). `lookup.rs`에는 조회 책임 시험 3개(`reads_api_keys_table`·`trims_name_on_both_sides`·`missing_name_returns_none`)만 남는다. `tests/credentials/main.rs`에 `mod parse;`를 추가했다 (기존 `mod lookup;`·`mod reject;`과 같은 골격). 새 파일은 하나(`parse.rs`)이고 폴더로 감싸지 않았다.
  - 파일→책임 맵: `tests/credentials/parse.rs` = jsonc 해석 책임(주석 허용·마지막 쉼표 거부), `tests/credentials/lookup.rs` = 이름 조회 책임(정상 읽기·trim·이름 없음), `tests/credentials/main.rs` = 시험 골격(mod 선언).
  - 삭제 파일 없음 (파일 이동만. `lookup.rs`에서 2개 시험 본문을 빼고 `parse.rs`로 옮김).
  - 게이트 3종 Windows·WSL(Ubuntu, `~/.cargo/bin` PATH) 통과, 로그 `.local/t6split2-win.log`·`.local/t6split2-wsl.log` (대상 SHA256 3종·종료 코드·시작/끝 시각 기록). Windows 17:15:36~17:15:46, WSL 17:16:29~17:16:35 (KST). 대상 SHA256: `parse.rs` 67A2EA29…, `lookup.rs` 68D0A1DB…, `main.rs` 1E15B747… (Windows·WSL 동일). 시험 70개 유지 (Windows 13+2+8+2+6+5+3+31=70, WSL 합계 70).
  - 문서 연쇄 반영: `docs/plans/02-client/02/refactor-srp.md` `## 잔여 메모`의 해석 시험 이관 항목을 해소로 갱신. `docs/plans/02-client/02/plan.md` 상태 줄의 잔여 1건 기록을 해소로 갱신. `docs/plans/02-client.md` 상태 줄의 잔여 1건 기록을 해소로 갱신. `docs/PLAN.md` 02 항목 상태 줄을 T6 잔여 해소 상태로 갱신.

## 테스트 계획
T1~T4는 네트워크 없는 단위 테스트다. 설정 읽기 시험은 임시 폴더를 쓰고, 자격 파일 값과 운영체제 관례 값은 주입한다. 시험 파일은 책임별로 나누고 같은 접두는 접두 폴더로 묶는다. 실제 키 값과 주소는 로그에 남기지 않는다.

## DoD
AC 1~7 충족, 게이트 3종 Windows·WSL 통과, `docs/reviews/02/`에 리뷰 PASS 기록, 문서 연쇄 반영 목록 완료.