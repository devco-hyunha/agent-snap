# 02-8 잔여 — 설정 오류·저장 모듈 분리: 작업 계획 (구 02-9)

- 흡수: 2026-11-06. 상위 체크리스트: [../tasks.md](../tasks.md) 잔여 절. 안내: [README.md](README.md).
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 상태: done (잔여 단위 리뷰 PASS 2026-11-06 `docs/reviews/02/08-config-save/srp-residual-review-1.md`)

## 구현 단계

### 작업 1. `ConfigError`를 부분 열거형으로 나눈다 (완료 2026-11-06: 리뷰 PASS `docs/reviews/02/08-config-save/srp-task1-review-1.md`)

- 범위: `config/error.rs`(신규 부분 열거형 3개 파일 포함), `config.rs` 재수출, `ConfigError`를 참조하는 호출부 20개 파일 180건(`config/` 제품 9개, 테스트 11개).
- 금지 사항: 오류 문구·담는 값 변경, 파일 삭제, 새 의존성. 검증 규칙(02-2) 추가·제거 금지.
- 산출물: `config/error.rs`(감싸는 열거형 + Display·Debug), `config/error_reading.rs`·`config/error_validation.rs`·`config/error_save.rs`(부분 열거형마다 Display·Debug 가리기), 호출부 참조 갱신, 가리기 확인 테스트 갱신(`config/tests/`).
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -p snap-core`, `cargo test` (AC 2·3·6).

### 작업 2. 중복을 한 자리로 모은다 (완료 2026-11-06: 리뷰 PASS `docs/reviews/02/08-config-save/srp-task2-review-1.md`)

- 범위: `config/save_config.rs`·`config/save_credentials.rs`의 이름 규칙 매핑 3자리(`endpoint_name_error`, `check_endpoint_name`, `CheckedModel::check` 안), 자격 키 칸 지우기 루프 3자리(`rename_key_with`·`remove_key_with`·`set_key`)를 새 파일(`config/save_names.rs`)과 `config/save_cst.rs` 보조 함수로 합친다.
- 금지 사항: 동작 변경(이름 trim 규칙·중복 칸 처리 규칙 유지), 파일 삭제, 새 의존성. `save_read.rs`의 읽기 공유는 유지하고 해석 실패 매핑은 합치지 않는다.
- 산출물: `config/save_names.rs`(이름 규칙 매핑), `config/save_cst.rs`(칸 지우기 보조 함수), 호출부 갱신.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -p snap-core config::tests::save`, `cargo test` (AC 5·6).

### 작업 3. 이름 바꾸기·삭제 책임을 파일로 뺀다 (완료 2026-11-06: 3차 PASS `docs/reviews/02/08-config-save/srp-task3-review-3.md`)

- 범위: `config/save_config.rs`(추가·수정만 남긴다), `config/save_credentials.rs`(키 쓰기만 남긴다). 이름 바꾸기·삭제 편집을 `config/save_rename.rs`(설정 쪽)와 `config/save_key_rename.rs`(자격 쪽)로 옮긴다. `config/save_store.rs`는 호출 대상만 바꾼다.
- 검증영역 분리 추가(사용자 지시 2026-11-06): `save_config.rs`의 검증 코드(`impl EndpointSave`의 `validate`·`check` L44-83, `CheckedEndpoint`·`CheckedModel`과 그 `check` L103-155)를 `config/save_check.rs`로 옮긴다. 검증은 02-2 규칙 변경 이유로만 바뀌는 별도 책임이다. 타입 `EndpointSave`·`ModelSave`는 `save_config.rs`에 두고 impl만 옮긴다(Rust는 같은 크레이트 내 impl 위치 분리를 허용). `CheckedEndpoint`·`CheckedModel`은 `pub(super)`로 열어 `merge_endpoint`가 쓰게 한다.
- 금지 사항: 저장 순서(D41)·D40 규칙 3 상태 표시 변경 금지, 파일 삭제, 테스트 본문 동거(제품 소스에 테스트 본문 금지).
- 산출물: `config/save_rename.rs`, `config/save_key_rename.rs`, `config/save_check.rs`, `config/save_store.rs` 호출 갱신, `config/tests/save_rename.rs`(이동), 기존 저장 테스트 파일 정리.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -p snap-core save_`, `cargo test` (AC 1·4).

### 작업 4. 문서·의존 설정을 정리하고 잔여 단위 리뷰를 PASS로 마친다

- 범위: `docs/RESEARCH.md` 02 절에 이 잔여(구 02-9) 적용 위치, `docs/DECISIONS.md`에 부분 열거형 채택 기록, `crates/snap-core/Cargo.toml`의 `jsonc-parser` 유지 확인.
- 금지 사항: 저장 기능 AC·02-8 본 작업 1~6 완료 표시를 되돌리지 않음. 커밋은 02-8 잔여 단위 리뷰 PASS·보고 후 사용자 허락 시에만(작업 02 전체 리뷰·커밋 게이트는 [02-9](../../02-09-client-review/plan.md)).
- 산출물: 결정 문서(`docs/decisions/D42-config-error-partial.md`), RESEARCH 갱신, 잔여 단위 리뷰 기록(`docs/reviews/02/08-config-save/srp-residual-review-1.md` 권장. 기존 `02-09-error-split-task*` 조각 리뷰는 그대로 인용).
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- 참고: 이 절의 「단위 리뷰」는 **02-8 잔여(SRP) 묶음**만이다. 작업 02 전체(02-1~02-8) 리뷰는 02-9다.

### 작업 5. `config/` 폴더 분리 (오류·저장·읽기 무 + 테스트 save) (완료 2026-11-06: 2차 PASS `docs/reviews/02/08-config-save/srp-task5-review-2.md`)

- 사용자 승인 2026-11-06 "지금 이동"으로 추가. 상세: `docs/tasks/02-09-error-split-5-folder-split.md`.
- 범위: `config/error/`·`config/save/`·`config/read/`(승인 상세 문서 기준), 부모 모듈·`config.rs` 재수출, `config/tests/save/`(저장 테스트 다섯 파일).
- `load.rs`·`location.rs`와 비저장 테스트는 `config/`·`config/tests/`에 유지.
- 금지: 파일 내용 변경(동작·문구·가리기), 파일 삭제(git mv로 추적 유지), 커밋.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (AC 1).

## 테스트 계획

- 네트워크 없는 단위 테스트, 임시 폴더 사용(02-8 관례 유지).
- 이름 바꾸기·삭제 테스트를 새 테스트 파일로 이동한다(본문만 이동).
- 부분 열거형 가리기 확인 테스트를 추가한다.
- 실행 순서: `cargo test -p snap-core save_` → `cargo test` 전체.

## DoD

- [x] spec.md의 AC 7개를 모두 충족한다.
- [x] 제품 파일 하나의 책임이 둘 이상인 자리가 0건이다(dev-principles 2절 SRP, 하네스 10절).
- [x] 중복 6자리가 한 자리씩으로 합쳐지고 호출만 남는다(dev-principles 3절 DRY).
- [x] 검증 3종 통과 + 잔여 단위 리뷰 PASS 기록(`02-08-config-save-srp-residual-review-1.md`).

## 문서 영향

- `docs/plans/02-client/02-08-config-save/srp-residual/`(이 폴더, 구 02-9).
- `docs/decisions/D42-config-error-partial.md`(신규), `docs/DECISIONS.md` 색인.
- `docs/RESEARCH.md` 02 절에 02-8 잔여(SRP) 적용 위치 메모.

## 리스크

- 재수출 누락으로 공개 경로 유지 실패 → AC 2로 잡는다.
- 가리기 커스텀 재적용 누락 → AC 3으로 잡는다.
- 동일 지적 보정 2회 정지(하네스 5절 2번)를 지킨다.

## 상태

in-progress (기획 승인 완료 2026-11-06, 작업 1 착수)
