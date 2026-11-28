# 02-1 작업 계획

- 상태: 완료 (T1~T5 완료, 6차 5차 리뷰 PASS. 커밋 `1f535fb`, PR #2 병합으로 `feature/client` 반영 완료)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow.
- 작업 방식: 세부 작업 하나는 한 번 위임하고 한 번 코드 리뷰로 끝낸다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종).

## 단계

### T1 비밀 값 래퍼
- 범위: `crates/snap-core/src/config/`에 `SecretString`(Debug 가리기·`expose()` 하나).
- 금지: `Display`·`PartialEq`·`Hash` 추가.
- 확인: Debug 출력에 평문이 없고, 값 접근이 한 메서드로 통일하는지.
- 검증: 게이트 3종.

### T2 snap 설정 폴더 경로
- 범위: `config/` 안에 OS별 파일(`mod.rs`+`windows.rs`+`unix.rs`)로 경로 결정.
- 금지: `mod.rs`에 `#[cfg]` 본문.
- 확인: base 경로 주입으로 테스트, `windows.rs`·`unix.rs` 분리. (구현 완료: base 주입 `config_dir_in`, 폴더 이름 상수는 `consts/defaults.rs`)
- 검증: 게이트 3종.

### T3 자격 파일 읽기
- 범위: `credentials.jsonc` 읽기·`api_keys` 이름 조회·형식 검증·locale 오류 문구 연결(`locale/ko/credentials.rs`, `locale/en/credentials.rs`).
- 금지: 자격 파일 쓰기·`snap.jsonc` 해석.
- 확인: trim 조회, None 경로, 거부 3종(파일 없음 포함)의 오류 문구에 값이 없는지. (구현 완료: `config/credentials.rs`가 읽기·조회·검증 담당, 상수는 `consts/defaults.rs`, locale 문구는 `locale/ko|en/credentials.rs`)
- 검증: 게이트 3종.

### T4 시험 폴더 묶기
- 범위: 같은 접두 시험 파일을 접두 폴더로 이동 (`tests/credentials/`, `tests/secret_string/`와 `main.rs`).
- 금지: 제품 소스 수정·기능 추가.
- 확인: 접두 되풀 파일 잔여 없음, 시험 개수 보존. (구현 완료: `tests/credentials/`·`tests/secret_string/`에 `main.rs` 포함 이동, `#[test]` 13건 유지, 옛 접두 파일 잔여 0건)
- 검증: T5가 02-1 전체로 1회 실행 (T4 단독 로그 없음).

### T5 게이트·문서 정리
- 범위: T4까지 반영한 02-1 전체에 게이트 3종 1회, 게이트 로그 기록, 새 크레이트 라이선스 → `docs/DECISIONS.md`, 상태 갱신.
- 금지: 계획 재설계.
- 확인: 게이트 로그·DECISIONS·상태 줄 반영.
- 검증: 게이트 3종 (02-1 전체 1회).

## 테스트 계획
T1~T3는 네트워크 없이 임시 폴더·주입 값으로 확인한다. 책임별 시험 파일을 나누고, 실제 키 값 같은 값은 로그에 남기지 않는다.

## DoD
AC 1~7 충족, 게이트 3종 통과, `docs/reviews/02/`에 리뷰 PASS 기록, 문서 연쇄 반영 목록 완료.