# D42 설정 오류는 읽기·검증·저장 부분 열거형으로 나눈다 (2026-11-06)

- 상태: recorded (2026-11-06 사용자 승인·02-8 잔여 작업 1~5 반영)
- 요약 인덱스: [docs/DECISIONS.md](../DECISIONS.md#d42-설정-오류는-읽기검증저장-부분-열거형으로-나눈다)
- 이 문서가 해당 결정의 상세 정본. DECISIONS.md에는 요약과 링크만.
- 관련: [D40](D40-save-editor.md), [D41](D41-save-order-validation.md), [02-8 잔여 spec](../plans/02-client/02-8-config-save/srp-residual/spec.md), [잔여 tasks](../plans/02-client/02-8-config-save/srp-residual/tasks.md)

## 결정

- 결정: `ConfigError`는 공개 경로 `snap_core::config::ConfigError`를 유지한 채, 안을 읽기·검증·저장 **부분 열거형**(한 덩어리별 세부 열거형)으로 나눈다.
  - 읽기: `ReadingError` (`JsoncSyntax`, `RootNotObject`, `Io`, `ConfigDirUnknown`, `Credentials`)
  - 검증: `ValidationError` (02-2 규칙 변형)
  - 저장: `SaveError` (`EndpointRenameAfterKeySide`, `EndpointRemoveAfterKeySide`, `SaveIo`, `MissingEndpoint`)
- 감싸는 `ConfigError`는 세 부분 열거형을 `#[from]`으로만 받는다. Display·Debug 가리기(키 값·base_url·경로 전체 비노출)는 부분 열거형마다 두고, 합쳐서 드러나는 문구는 이전과 같게 한다.
- 파일 위치(폴더 분리 후): `crates/snap-core/src/config/error.rs`(감싸기·재수출), `config/error/reading.rs`·`validation.rs`·`save.rs`. 저장·읽기 제품 코드는 `config/save/`·`config/read/`, 저장 시험은 `config/tests/save/`에 둔다.

## 근거

- 근거: 한 파일이 읽기·검증·저장이라는 서로 다른 변경 이유를 함께 가지면 하네스 10절·dev-principles 2절 SRP를 깬다. 부분 열거형으로 나누면 변경 이유가 파일마다 하나이고, 공개 이름은 재수출로 유지해 호출부 180건을 최소로 고친다(잔여 AC 2·3).
- 동작·오류 문구·담는 값·저장 순서(D41)는 바꾸지 않는 구조 정리만 한다(잔여 AC 1·4).

## 다시 볼 점

- 다시 볼 점: 부분 열거형 변형을 더 쪼개거나 공개 경로를 줄이는 일은 호출부 영향이 크므로, 새 오류 종류가 생길 때만 잔여 spec·AC와 함께 본다.

## 구현 후 확인 메모 (2026-11-06, 02-8 잔여)

- 작업 1~3·5 리뷰 PASS 기록: `docs/reviews/02-client/02-9-error-split-task1-review-1.md` ~ `task5-review-2.md`(파일명은 구 번호, 내용은 02-8 잔여).
- `jsonc-parser` 0.34.0 `cst`는 D40대로 `crates/snap-core/Cargo.toml`에 유지한다. 잔여 작업에서 버전·기능을 바꾸지 않았다.
