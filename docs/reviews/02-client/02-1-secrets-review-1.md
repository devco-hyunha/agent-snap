# 02-1 비밀 값 래퍼와 자격 파일 읽기 — 리뷰 1

- 리뷰 세션: 구현 맥락 없는 읽기 전용 리뷰 (구현 세션과 분리)
- 리뷰 대상: 워스페이스의 비커밋 변경 (git status 기준 신규 `secrets.rs`, `credentials.rs`, 수정 `lib.rs`, `Cargo.toml`, `Cargo.lock`, plan.md 상태 in-progress)
- 판정: **미달** (major 1건)

## 1. 문제 목록

| # | 파일:라인 | 심각도 | 내용 |
|---|---|---|---|
| 1 | `docs/DECISIONS.md` (전체) | major | `jsonc-parser` 라이선스 확인 기록이 없다. `docs/plans/02-client/02-1-secrets/tasks.md` 단계 1의 완료 기준("확인: 결정 기록이 있다")과 위임 브리프(`docs/tasks/02-1-secrets.brief-1.md`) 보고 형식의 "크레이트 라이선스 확인 근거" 요구를 충족하지 못했다. `docs/plans/02-client.md` 61행의 "새 외부 크레이트는 처음 쓰는 세부 작업에서 라이선스를 확인하고 `docs/DECISIONS.md`에 적는다" 규칙도 위반이다. |
| 2 | `docs/plans/02-client/02-1-secrets/tasks.md:6-13` | minor | 단계 1~7 체크리스트가 모두 `- [ ]`로 남아 있어 구현 반영 체크 기록이 없다. 단계 2~6은 코드와 검증 로그로 구현 결과가 확인되므로 코드에는 반영됨. 단 단계 1은 문제 1과 같이 미완료. |
| 3 | `crates/snap-core/src/credentials.rs:53-56` | minor | 최상위가 객체가 아닌 경우(예: 최상위 전체가 문자열)에 `ApiKeysNotObject`(값 없는 오류 종류)로 처리한다. 명세 오류 표의 "`api_keys`가 객체가 아님" 항목을 최상위 객체 검증까지 넓혀 적용한 것으로, 값은 노출되지 않아 안전하다. 문서 주석에 최상위 객체 검증에 대한 설명을 한 줄 더 두면 명확하다. |
| 4 | `crates/snap-core/src/credentials.rs:63` | minor | `api_keys_object.clone().take_inner()`는 객체를 복제한 뒤 소비하는 경로로, 복제 없이 값만 소비하는 경로를 우선 검토할 수 있다 (KISS·성능 관점의 개선 제안). |

## 2. 6항목별 결과

1. **AC 충족 여부**: 충족 (AC 1~5 전부 테스트로 확인, 아래 3절 참조).
2. **금지 규격 위반**: 위반 없음.
   - 지정 파일(secrets.rs, credentials.rs, lib.rs, Cargo.toml) 외 수정 없음 (git status: `Cargo.lock`은 의존 추가에 따른 자동 갱신, `plan.md`는 상태 갱신 허용 범위).
   - 커밋·푸시 없음 (`git log` 최신 커밋 3건 모두 이번 작업 이전 것).
   - 임시 스크립트 파일 없음.
   - `config.rs`·`llm.rs`·`snap-cli`에 `SecretValue`·`credentials` 참조 없음 (grep 확인).
   - 02-2 범위인 설정 파일 `api_key` 거부 로직 없음. 자격 파일 위치 계산 없음 (`absent()`는 텍스트 없이 빈 상태로 만드는 것이지 경로 계산이 아님).
3. **검증 명령 결과와 로그 근거**: 전부 통과.
   - `cargo fmt --check`: 종료 부호 0 (출력 없음)
   - `cargo clippy --all-targets -- -D warnings`: `Finished ...dev profile` 후 경고 0, 종료 부호 0
   - `cargo test -p snap-core`: `test result: ok. 11 passed; 0 failed` (doc-test 0 passed 포함)
4. **에러 처리와 보안 기본 (dev-principles 2~10절)**: 충족.
   - 관심사 분리: `SecretValue`는 가리기만, `Credentials`는 해석·검색만 담당. 파일 경로 계산·쓰기 없음.
   - DRY: 가림 로직과 오류 생성이 각 파일 한 곳에 있고 중복 정의 없음 (`reveal()` 재정의 없음).
   - KISS·YAGNI: 트레이스·직렬화·다시 쓰기 등 요청받지 않은 기능 없음.
   - 에러 처리: 라이브러리이므로 `thiserror` 사용, 공개 API는 `Result`, 테스트 밖 `unwrap`/`expect` 없음 (`secrets.rs`·`credentials.rs`의 테스트 코드 블록 밖 grep 결과 0건).
   - 보안 기본: 비밀 값을 `Debug`/`Display`에서 가리고 `reveal()`로만 원문, 모든 오류 Display/Debug 테스트로 값 미노출 확인.
   - 문서 주석: 모든 공개 항목(`SecretValue`, `SecretValue::new/reveal`, `Credentials`, `CredentialsError` 변형, `parse/absent/find_key/is_empty`)에 한국어 문서 주석 있음.
   - 단위 테스트: AC 1~5와 각 경로에 테스트 11개.
5. **문서 동기화**: 미달 (문제 1·2). `docs/RESEARCH.md` 작업 03 절 "설정·키 위치와 비밀 파일 차단" 항목의 적용 위치 줄에 "작업 02 D10: 설정 파일과 자격 파일 분리(02-1, 02-2, 02-8)" (RESEARCH.md:261) 반영됨. `docs/DECISIONS.md`에 `jsonc-parser` 라이선스 확인 기록 없음. tasks.md 체크리스트 미반영.
6. **범위 밖 변경**: 없음 (git diff --stat: `Cargo.lock`, `crates/snap-core/Cargo.toml`, `crates/snap-core/src/lib.rs`, `plan.md`, 신규 `credentials.rs`·`secrets.rs`, `docs/tasks/`는 하네스 브리프 산출물).

## 3. AC 1~5별 근거

| AC | 판정 | 근거 (파일:라인, 테스트) |
|---|---|---|
| AC 1 | 충족 | `credentials.rs:99-118` `parse_credentials_with_comments_and_trailing_comma_finds_key_by_name` — 주석·마지막 쉼표, `api_keys` 밖 항목(`"endpoint": []`) 무시, `main-ep`→`main-key-111`, `stage_ep`→`stage-key-222` 확인. `credentials.rs:120-131` `parse_credentials_names_are_case_and_symbol_sensitive_and_blank_is_none` — `Main-Ep` 있음 / `main-ep`, `Main_Ep` 없음(대소문자·`-`·`_` 구분), `empty_ep`(빈 문자열)·`missing-ep` 없음. 구현: `credentials.rs:46-71` (`ParseOptions::default`), `credentials.rs:83-87` (`find_key`의 `filter`로 빈 문자열 걸러냄). |
| AC 2 | 충족 | `credentials.rs:74-76` `absent()` 구현, `credentials.rs:133-140` `absent_credentials_has_no_key_and_is_not_error` — `is_empty` true, 어떤 이름도 `None`. `credentials.rs:142-150` `missing_or_empty_api_keys_is_empty_list_not_error` — `api_keys` 없음·빈 경우 오류 아님(명세 추가 규칙). |
| AC 3 | 충족 | `credentials.rs:21-22` `KeyNotString { endpoint }` (값 미포함), `credentials.rs:63-68` (문자열 아닌 항목만 이름 담아 거부), `credentials.rs:153-166` `non_string_key_value_is_rejected_with_endpoint_name_only` — `bad-ep` 확인, Display·Debug에 `12345`·`good-key-999` 없음. |
| AC 4 | 충족 | `credentials.rs:14-16` `JsoncSyntax { line }` (줄 번호만, 줄 내용·값 미포함), `credentials.rs:48-52` (`line_display()` 매핑), `credentials.rs:168-182` `jsonc_syntax_error_reports_only_line_number` — `line: 2` 확인, Display·Debug에 값·`api_keys` 없음. |
| AC 5 | 충족 | `secrets.rs:28-37` (`Debug`→`SecretValue(가림)`, `Display`→`가림`), `secrets.rs:45-56` `secret_value_debug_and_display_hide_the_value`. `credentials.rs:192-215` `all_errors_hide_the_key_value_in_display_and_debug` — 세가지 거부 경로(KeyNotString·ApiKeysNotObject·JsoncSyntax)의 Display·Debug에 키 값 없음 확인. |

## 4. 판정

- 심각도 critical: 0건.
- 심각도 major: 1건 (`docs/DECISIONS.md`의 `jsonc-parser` 라이선스 확인 기록 부재).
- 심각도 minor: 3건.
- **판정: 미달** (하네스 문서 7절: critical/major 유무로 판정).
- 조치 권고: `docs/DECISIONS.md`에 `jsonc-parser`(crates.io 표기 라이선스 MIT/Apache-2.0-1.0) 라이선스 확인 결정을 D 번호로 기록하고, tasks.md 체크리스트에 구현 반영. 이후 재리뷰는 하네스 문서 9절 범위대로 changed files 기준으로 진행한다.
