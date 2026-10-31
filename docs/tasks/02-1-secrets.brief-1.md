# 작업 02-1 위임 브리프 1: 비밀 값 래퍼와 자격 파일 읽기

- 상태: planned (구현 위임 1호, 완료 시 서브가 in-progress로 갱신)
- 상위 기획: `docs/plans/02-client/02-1-secrets/plan.md`, 명세: `docs/plans/02-client/02-1-secrets/spec.md`

## 목적
`snap-core`에 비밀 값 래퍼를 추가하고, 자격 파일 `credentials.jsonc`를 해석해 엔드포인트 이름별 키를 찾는 경로를 만든다. 기획 승인 상태: plan.md가 승인되어 구현 단계로 들어간다.

## 서브 보고에서 채택한 근거(출처 포함)
- 결정 근거: `docs/DECISIONS.md`의 D10(JSONC 형식·자격 파일 모양·이름 그대로 비교(D10는 plan.md가 인용), D14(빈 키는 '없음'), D3(Debug 가림 래퍼)).
- 개발 규칙: `AGENTS.md`의 개발 규칙 절(크레이트 구분, thiserror는 라이브러리만, 비밀 값 Debug 가림, 공개 항목 문서 주석 한국어, 모든 기능에 단위 테스트).

## 작업 범위
1. `crates/snap-core/src/secrets.rs` 신규: 비밀 값 래퍼 `SecretValue`(가칭)
   - `Debug`·`Display`에서 값을 가린다. 값은 명시적 접근 함수(예: `reveal()`)로만 원문을 꺼낸다.
   - 문자열 값 생성, 비교·직렬화 시 원문 노출 경로만 둔다.
2. `crates/snap-core/src/credentials.rs` 신규: 자격 파일 텍스트 해석
   - 입력: 자격 파일 텍스트 문자열(파일 없음 표기 지원). 파일이 없으면 빈 목록.
   - JSONC 해석(주석 `//`, `/* */`, 마지막 쉼표 허용). 크레이트 선택: `jsonc-parser`(MIT/Apache-2.0-1.0, 라이선스 확인을 테스트·보고에 근거로 기록).
   - 최상위 객체 아래 `api_keys` 객체의 `"<엔드포인트 이름>": "<키>"`만 읽는다. `api_keys` 밖 최상위 항목은 무시.
   - `api_keys`가 없거나 비어 있으면 빈 목록(오류 아님). 이름은 적힌 그대로, 대소문자·`-`·`_` 구분.
   - 거부 규칙: 값이 문자열이 아니면 그 엔드포인트 이름만 담은 오류, `api_keys`가 객체가 아니면 값 없이 오류 종류만, JSONC 문법 오류는 줄 번호만(줄 내용·값 금지).
   - 키 찾기: 이름 있고 빈 문자열 아니면 값 반환, 없거나 빈 문자열이면 `None`('없음', 오류 아님). 파일 없으면 `None`.
   - 오류 타입은 `thiserror`, 모든 오류 Display·Debug에 키 값 금지.
3. `crates/snap-core/src/lib.rs`에 두 모듈 등록(`pub mod secrets; pub mod credentials;`).
4. `crates/snap-core/Cargo.toml`에 선택 크레이트 의존 추가.
5. 단위 테스트: AC 1~5 전부를 `#[cfg(test)]`로 (임시 파일 금지, 인라인 검증만).

## 금지 사항
- 위 지정 파일(위 1~4)과 그 파일의 테스트 외 파일 수정·생성 금지 (예: `config.rs`, `llm.rs`, `snap-cli` 손대지 않기).
- 커밋·푸시 금지.
- 검증은 임시 스크립트 파일 없이 cargo 명령 한 줄로만 실행.
- 요청받지 않은 코드 샘플·문서 파일 생성 금지. 문서 문서는 수정하지 않는다(plan.md 상태 갱신만 허용: in-progress).
- 자격 파일 파기 위치 계산·설정 파일 읽기(`config.rs` 책임) 구현 금지 — 02-2 범위의 설정 파일 `api_key` 거부 로직 금지.

## 산출물
- `crates/snap-core/src/secrets.rs`, `crates/snap-core/src/credentials.rs`
- `crates/snap-core/src/lib.rs`(모듈 등록), `crates/snap-core/Cargo.toml`(의존 추가)
- 테스트 코드 2개 파일 내부 (AC 1~5)
- 검증 결과 요약 (fmt/clippy/test 통과)

## 검증 방법
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test -p snap-core` (워크스페이스 루드 `C:\workspace\agent-snap`)

## 보고 형식
- 추가·수정 파일 목록과 각각의 요약
- AC 1~5별 테스트 이름과 결과
- 크레이트 라이선스 확인 근거(선택 크레이트와 라이선스 표기)
- 바뀐 줄 요약(신규 파일 총 줄 수), 검증 명령 결과 요약
- 상태 갱신: plan.md의 `- 상태:` 줄을 in-progress로 변경했는지
