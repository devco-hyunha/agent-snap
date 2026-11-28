# 02-9 설정 저장 조사

- 조사 시점: 2026-11-28 (읽기 전용 리서치 서브)
- 주제: Rust에서 JSONC 파일의 주석과 포맷을 보존하며 값을 편집·저장하는 방법

## 1. jsonc-parser CST 기능 (채택)

- 저장소는 `jsonc-parser` 0.34.0을 `features = ["serde"]`로 쓰고 있다(`crates/snap-core/Cargo.toml`). `cst` 기능은 켜지 않은 상태다.
- CST는 "모든 주석과 모든 공백을 유지해 작성자가 쓴 그대로 다시 쓴다"고 문서에 명시한다. `CstRootNode` → `CstObject`의 `get`·`set_value`·`append`·`insert`·`remove`·`sort_properties`(주석이 항목과 함께 이동)를 쓰고 `to_string()`으로 다시 쓴다.
- 주의점: 루트 노드를 드롭하면 조상 약참조가 사라져 패닉이 난다. 노드 수명을 함수 안에 한정한다.
- 근거 링크: [docs.rs CST 모듈](https://docs.rs/jsonc-parser/latest/jsonc_parser/cst/index.html) · [CstRootNode](https://docs.rs/jsonc-parser/latest/jsonc_parser/cst/struct.CstRootNode.html) · [CstObject](https://docs.rs/jsonc-parser/latest/jsonc_parser/cst/struct.CstObject.html)
- 결정 5번(D5)이 같은 해석기 통일을 정했으므로, 기능 한 줄 추가만 필요하다.

## 2. 대안 크레이트 (기각)

| 크레이트 | 라이선스 | 판정 | 근거 |
|---|---|---|---|
| jsonkit 0.3.0 | MIT | 기각 | `strip_comments`·`minify`·`prettify`·`validate`만 제공. 주석을 지우는 도구 |
| json-patch 5.0.0 | MIT/Apache-2.0 | 기각 | `serde_json::Value`에 RFC 6902 적용. 주석 개념 없음 |
| jsonformat 2.1.0 | MIT | 기각 | JSON 포맷 전용 |
| monochromatic-jsonc-edit 0.1.3 | LGPL-3.0 | 기각 | 주석 보존 편집은 제공하지만 결정 4번 allow 목록(OSI·FSF 관례 10종, copyleft 거부)에 걸림 |
| jsonc 0.1.0 | MIT | 기각 | `jsonc-parser` 0.26 래퍼. 오래된 버전 의존 |

- 근거 링크: [jsonkit](https://docs.rs/jsonkit/latest/jsonkit/) · [json-patch](https://docs.rs/json-patch/latest/json_patch/) · [jsonformat](https://crates.io/api/v1/crates/jsonformat) · [monochromatic-jsonc-edit](https://crates.io/api/v1/crates/monochromatic-jsonc-edit/0.1.3)

## 3. 자체 텍스트 위치 편집 (기각)

- 주석 안 키 이름 오탐(`// base_url을 지칭`), 문자열 안 `//`, 배열 안 중복 키(`endpoints` 안 `name`), CRLF·BOM·마지막 쉼표로 치환이 어긋난다.
- 방어는 직접 찾기가 아니라 `parse_to_ast`의 `CollectOptions { tokens: true }`로 토큰 범위를 잡고, 결과를 한 번 더 파싱해 검증하는 왕복이다. CST가 있으면 필요하지 않다(YAGNI).

## 4. 파일 교체 (채택)

- `tempfile::NamedTempFile::new_in(원본 폴더)` + `persist()`가 같은 볼륨을 보장하고 원자적으로 교체한다. 실패 시 `PersistError`로 임시 파일을 되돌린다.
- `std::fs::rename`도 마운트 지점이 다르면 오류다. copy 대체 경로는 원본을 지우거나 반쯤 쓴 상태를 남길 수 있어 쓰지 않는다.
- Windows는 다른 프로세스가 파일을 열면 교체 실패, 유닉스는 성공하지만 열려 있던 쪽은 옛 inode에 남는다.
- `persist`는 내용·폴더를 동기화하지 않으므로 flush가 필요하다.
- 근거 링크: [tempfile NamedTempFile::persist](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html) · [std::fs::rename](https://doc.rust-lang.org/std/fs/fn.rename.html)

## 5. 적용 위치

- 02-9 기획: `docs/plans/02-client/09/plan.md`의 `## 리서치 및 설계 근거`.
- 결정 기록: `docs/DECISIONS.md` 21번(`cst` 편집과 `tempfile` 원자 저장, `preserve_order` 미사용).
- 결정 6번 다 볼 점(`preserve_order` 필요 여부) 해소: CST 편집은 텍스트 위치에서 값만 고르므로 키 순서가 저절로 보존된다. 기능 켜지 않음.
