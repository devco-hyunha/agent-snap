# D40 02-8 설정·자격 저장 편집 도구와 원자적 교체 (2026-11-05)

- 상태: recorded (2026-11-05 사용자 승인 완료)
- 관련: [02-8 plan](../plans/02-client/02-8-config-save/plan.md), [spec](../plans/02-client/02-8-config-save/spec.md), [작업 1](../tasks/02-8-config-save-1-jsonc-editor.md), [RESEARCH 02-8](../research/02-client.md)

## 결정(제안)

1. 주석 보존 편집: `jsonc-parser` 0.34.0의 `cst` 기능을 켠 것으로 쓴다. CST(원문 구조를 주석·공백까지 보관하는 트리) 편집 API로 엔드포인트·모델 항목만 고치고, 손대지 않은 항목과 사용자가 단 주석은 원문 그대로 둔다.
2. 파일 원자적 교체: 같은 폴더의 임시 파일에 모두 쓴 뒤 `std::fs::rename`(Windows는 `MoveFileExW` 경로)으로 기존 파일을 바꾼다. 교체 전에 `sync_all`(디스크 반영)을 하고, 교체 실패 시 기존 파일은 그대로 두고 임시 파일만 지운다.
3. 두 파일을 함께 바꿀 때 순서: 자격 파일(`credentials.jsonc`)을 먼저 쓰고 교체한 뒤, 설정 파일(`agent-snap.jsonc`)을 쓴다. 이름 바꾸기·삭제에서 키 쪽이 먼저 정렬되어야 설정 파일 쪽 이름 변경이 그 뒤를 따르기 때문이다. 설정 파일 교체가 실패하면 자격 파일은 이미 바뀐 상태이고, 오류에 두 파일 중 어느 쪽이 바뀌었는지 밝힌다(어긋난 상태를 사용자가 알 수 있게). 이 순서를 정본으로 둔다.
4. 라이선스: `jsonc-parser` 0.34.0은 MIT다(crates.io 라이선스 표기 MIT, 저장소 LICENSE도 MIT). `cst`는 선택 기능이라 의존을 더 붙이지 않는다.

## 근거

- `jsonc-parser`는 이미 02-2·02-8 해석기로 쓰고 있는 크레이트(D18)와 같은 저장소이고, `cst` 기능을 켜면 주석·공백을 보존한 채 값만 고칠 수 있다. 새 의존을 더하지 않고 같은 크레이트 기능만 켜는 선택이라 dev-principles 3절 재사용 사다리 3번(이미 설치된 의존성 사용)에 맞는다.
- `monochromatic-jsonc-edit` 0.1.2는 주석 보존 편집을 제공하지만(crates.io 설명), 라이선스가 LGPL-3.0-or-later라 프로젝트의 허용형 기준(D5)에 맞지 않아 기각한다.
- npm `jsonc-parser` 3.3.1(MIT)의 `modify`·`applyEdits`는 주석을 보존하는 오프셋 편집을 만든다. Rust 프로젝트에는 쓰지 않고 방식 참고만 한다(동작 확인용).
- 원자적 교체는 `std::fs::rename` 문서가 "기존 파일을 바꾸며, 다른 볼륨에서는 실패한다"를 밝힌다. Linux `rename(2)`는 실패 시 대상 파일이 그대로 남음을 보장하고 볼륨 다름을 `EXDEV`로 알린다. Windows는 `MoveFileExW`의 `MOVEFILE_REPLACE_EXISTING` 문서가 대상 파일 교체 규칙과 ACL 요건을 밝힌다.

## 확인 링크(2026-11-05 직접 열림)

- [crates.io jsonc-parser 0.34.0 버전 정보](https://crates.io/api/v1/crates/jsonc-parser/0.34.0) — 라이선스 MIT, `cst` 기능 존재
- [dprint/jsonc-parser Cargo.toml](https://raw.githubusercontent.com/dprint/jsonc-parser/main/Cargo.toml) — license = "MIT", `cst = []`
- [dprint/jsonc-parser LICENSE](https://raw.githubusercontent.com/dprint/jsonc-parser/main/LICENSE) — MIT 전문
- [dprint/jsonc-parser src/lib.rs](https://raw.githubusercontent.com/dprint/jsonc-parser/main/src/lib.rs) — CST 편집 예시(`set_value`·`append`)
- [npm registry jsonc-parser](https://registry.npmjs.org/jsonc-parser) — 최신 3.3.1 라이선스 MIT
- [microsoft/node-jsonc-parser README](https://raw.githubusercontent.com/microsoft/node-jsonc-parser/main/README.md) — `modify`·`applyEdits` 설명, MIT 표기
- [crates.io monochromatic-jsonc-edit 0.1.2](https://crates.io/api/v1/crates/monochromatic-jsonc-edit/0.1.2) — 라이선스 LGPL-3.0-or-later
- [std::fs::rename](https://doc.rust-lang.org/stable/std/fs/fn.rename.html) — 플랫폼 동작(`MoveFileExW` 경로, 볼륨 다름)
- [man7 rename(2)](https://man7.org/linux/man-pages/man2/rename.2.html) — 원자적 교체, 실패 시 대상 보존, `EXDEV`
- [MoveFileExW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw) — `MOVEFILE_REPLACE_EXISTING` 세부
- [Moving and Replacing Files](https://learn.microsoft.com/en-us/windows/desktop/FileIO/moving-and-replacing-files) — 열려 있는 파일 이동 불가 안내

## 확인하지 못한 것

- Windows에서 열려 있는 파일(다른 프로세스가 쓰기 중)을 교체할 때의 실패 코드 세부(`ERROR_SHARING_VIOLATION` 등)는 위 문서들에 명시로 적히지 않아 확인하지 못했습니다. 구현 단계인 02-8 작업 4에서 직접 확인.
- `monochromatic-jsonc-edit`의 저장소 LICENSE 파일과 Cargo.toml 표기는 crates.io에 등록된 라이선스 표기(LGPL-3.0-or-later)만 확인했고 개별 파일은 열지 않았습니다. 기각 근거로 쓰기 충분한 수준입니다.

## 구현 후 확인 메모 (2026-11-06, 02-8 작업 2~5)

- 임시 파일 이름 규칙: `{파일 이름}-{프로세스 번호}-{번호}.tmp`이며 같은 폴더에 둔다(`save_atomic.rs`의 `temp_name`). 프로세스 번호와 번호를 붙여, 다른 프로세스·다른 저장이 만든 임시 파일과 이름이 겹치지 않게 한다.
- 교체 실패 처리: `std::fs::rename`이 실패하면 기존 파일은 그대로 두고, 이번 저장이 만든 임시 파일만 지운다(`write_replacing`).
- Windows 실측 결과: 공유 설정 없이(`share_mode(0)`) 열어 둔 대상 파일은 `std::fs::rename`이 실패하고, 그 오류는 원시 번호 32(공유 위반)로 난다. 읽기 전용 열림은 교체를 막지 못한다. 확인한 코드는 `save_error.rs`의 `SHARING_VIOLATION`(32)과 `SaveIoKind::SharingViolation`이고, 시험은 `config/tests/save_atomic.rs`의 `windows_open_target_file_fails_standard_rename_and_keeps_file`·`windows_read_only_open_allows_standard_rename`다.
- 편집 결과의 주석 보존은 시험으로 확인했다: `config/tests/save_config.rs`·`config/tests/save_credentials.rs`가 저장이 끝나도 주석과 손대지 않은 항목이 원문 그대로임을 본다. 우회 방식(오프셋 편집)은 쓸 필요가 없었다.
- 규칙 3(자격 먼저→설정 나중)에 검증을 앞으로 옮긴 것은 D41로 남겼다.

## 다시 볼 점

- `cst` 기능의 편집 결과가 주석 위치를 어떻게 옮기는지(주석 보존·들여쓰기 유지)는 02-8 작업 2 단위 테스트로 확인한다. 기대와 다르면 원문 텍스트에 오프셋으로 편집을 넣는 방식(`modify`·`applyEdits`와 같은 방식)으로 우회한다.
- 버전 상한: `jsonc-parser`는 0.34.0 고정(D18). `cst` 편집 API가 0.x에서 바뀌면 D18과 함께 버전 올리기 검토.
- 자격 파일 교체 후 설정 파일 실패 시 상태를 오류 안내로만 받아들일지, 되돌리기를 붙일지는 06 백업 기획과 함께 판단한다.
