# 02-8 잔여 — 설정 오류·저장 모듈 분리: 명세 (구 02-9)

- 흡수: 2026-11-06. 안내: [README.md](README.md).
- 상위 문서: [plan.md](plan.md)
- 상태: done (잔여 단위 리뷰 PASS 2026-11-06)

## 입력과 출력

이 작업은 순수 구조 정리(재배치)다. 제품 동작(입력·출력·오류 문구·담는 값·저장 순서)을 바꾸지 않는다.

- 유지 대상: 02-2 검증 규칙, 02-8 저장 규칙(주석 보존, 원자적 교체, D41 순서: 검증 → 자격 교체 → 설정 교체), 오류 표시 규칙(키 값·base_url·경로 전체 비노출).
- 바뀌는 것: 파일·모듈 경계, 함수 위치, 열거형 구조(부분 열거형 도입).

## 동작 규칙 (유지 대상 재확인)

### 오류 타입 구조

- `ConfigError`는 설정 모듈 전체가 쓰는 오류 타입으로 남는다. 공개 경로 `snap_core::config::ConfigError`를 그대로 둔다.
- 안을 도메인별로 나눈다.
  - 읽기·해석(`JsoncSyntax`, `RootNotObject`, `Io`, `ConfigDirUnknown`, `Credentials`): 파일 읽기·환경 변경 이유로만 바뀌는 변형.
  - 검증(`LegacyTopLevelModels`~`BackoffRangeInverted`, 21종): 02-2 규칙 변경 이유로만 바뀌는 변형.
  - 저장(`EndpointRenameAfterKeySide`, `EndpointRemoveAfterKeySide`, `SaveIo`, `MissingEndpoint`, 4종): 02-8 저장 규칙 변경 이유로만 바뀌는 변형.
- 감싸는 열거형은 부분 열거형(한 덩어리별 세부 열거형)을 `#[from]`(표준 변환 트레이트)으로 받아 감싼다. Display·Debug 커스텀(AC 5 가리기)은 부분 열거형마다 적용하고, 합쳐서 드러난 표시가 이전과 같게 한다.
- 부분 열거형의 변형 이름은 호출부에서 쓸 수 있게 재수출한다. 180건 참조(측정: 20개 파일)를 최소한으로 고쳐 공개 이름과 변형 이름을 유지한다.

### 저장 쪽 이름 바꾸기·삭제 분리

- 설정 파일 쪽 이름 바꾸기·삭제 편집(`prepared_rename_text`·`prepared_remove_text`·`write_checked_text`)을 `config/`의 새 파일로 분리한다(이름 예시: `save_rename.rs`).
- 자격 파일 쪽 이름 바꾸기·삭제(`rename_key`·`remove_key`와 그 지원 함수)를 `config/`의 새 파일로 분리한다(이름 예시: `save_key_rename.rs`).
- 이름 바꾸기·삭제 저장을 묶는 자리(`save_store.rs`의 `rename_endpoint`·`remove_endpoint`)는 그대로 두고, 호출 대상만 새 파일의 공개 함수로 바꾼다.

### 중복 합치기

- 이름 규칙 매핑(`NameRejection` → `EmptyEndpointName`·`EndpointNameHasSlash`·`EmptyModelName`·`ModelNameHasSlash`)을 한 파일의 한 함수로 모는다(이름 예시: `save_names.rs`). `save_config.rs`의 `endpoint_name_error`, `save_credentials.rs`의 `check_endpoint_name`, `CheckedModel::check` 안 매핑 — 같은 지식을 세 곳이 각자 적고 있다.
- 자격 키 칸 지우기 루프(`CstObjectProp`를 받아 `remove()`를 도는 루프)를 `save_cst.rs`의 보조 함수 한 자리로 모는다. `rename_key_with`·`remove_key_with`·`set_key`의 세 자리 중복이다.
- `read_root`(설정 쪽)와 `read_checked_root`(자격 쪽)는 `save_read.rs`의 파일 읽기 공유를 유지한다. CST 해석 실패 매핑 차이는 합치지 않고 그대로 둔다(매핑 규칙이 도메인별로 다르다).

## 금지 사항

- 오류 문구·담는 값·저장 순서 변경 금지 (AC 4·AC 5 준수 유지).
- 파일 삭제 금지 (신규 파일 추가만).
- 새 의존성 추가 금지.
- `credentials`·`jsonc`·`secrets`·`validate` 모듈 구조 변경 금지.

## AC (인수 조건)

1. Given 현행 브랜치에서 통과하던 테스트가 있을 때, When 이 작업 뒤에 `cargo test`를 실행하면, Then 기존 테스트 전부가 그대로 통과한다(테스트 본문 이동 외에는 실패 없음).
2. Given `ConfigError`를 참조하는 호출부(20개 파일 180건)가 있을 때, When 부분 열거형으로 바꾸면, Then 오류 문구·담는 값이 이전과 같음을 확인하는 테스트(`config/tests/redaction.rs`·`config/tests/save_redaction.rs`)가 그대로 통과한다.
3. Given AC 5(키 값·base_url 비노출) 확인 테스트가 있을 때, When 부분 열거형마다 Display·Debug 커스텀을 적용하면, Then 가리기 확인 테스트가 새 타입 경로에서도 통과한다.
4. Given 이름 바꾸기·삭제 저장이 있을 때, When 분리된 파일로 실행하면, Then D41 순서와 D40 규칙 3(설정 교체 실패 시 자격 파일 변경 상태 표시)을 확인하는 테스트가 그대로 통과한다.
5. Given 중복 대상(이름 매핑 3곳, 키 칸 지우기 루프 3곳)이 있을 때, When 한 자리로 모으면, Then 남은 중복 정의가 사라지고(호출만 남음) clippy·test가 통과한다.
6. Given `cargo fmt --check`와 `cargo clippy --all-targets -- -D warnings`가 있을 때, When 이 작업 뒤에 실행하면, Then 경고 0건으로 통과한다.
7. Given 작업 단위 전체 리뷰가 있을 때, When dev-principles 2~11절과 하네스 10절(파일 단위 단일 책임)로 점검하면, Then 제품 파일 하나의 책임이 둘 이상인 자리가 0건이 된다.

## 테스트 계획

- 이동: `save_config.rs`·`save_credentials.rs`의 이름 바꾸기·삭제 테스트 → `config/tests/save_rename.rs` 등 새 테스트 파일로 이동(본문 이동만, 동작 검증 유지).
- 신규: 부분 열거형 가리기 확인 테스트 추가(`config/tests/`).
- 실행: `cargo test -p snap-core save_` 먼저 → `cargo test` 전체.
- 네트워크 없는 단위 테스트, 임시 폴더 사용(02-8 관례 유지).

## 문서 영향

- `docs/plans/02-client/02-9-error-split/`(신규, 본 문서).
- `docs/DECISIONS.md`: 부분 열거형 채택 결정을 새 결정 문서로 남긴다(예시: `docs/decisions/D42-config-error-partial.md`).
- `docs/RESEARCH.md` 02 절: 적용 위치 메모를 02-9 항목으로 갱신한다.
- 02-8 `tasks.md`: 완료 상태 유지(추가 갱신 없음).

## 리스크

- 부분 열거형 재수출 누락으로 공개 경로 유지 실패 → AC 2로 잡는다.
- 가리기 커스텀 재적용 누락 → AC 3으로 잡는다.
- 동일 지적 보정 2회 시 정지 규칙(하네스 5절 2번) 적용.

## 상태

planned (승인 대기)
