# 02-1 비밀 값 래퍼와 자격 파일 읽기: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. JSONC 해석 크레이트가 필요하면 라이선스를 확인하고 `docs/DECISIONS.md`에 적는다. 02-2와 02-8도 이 해석기를 쓴다. 확인: 결정 기록이 있다. → `jsonc-parser` 0.34 선택, `docs/DECISIONS.md` D18에 기록.
- [x] 2. 비밀 값 래퍼를 만든다. Debug와 Display에서 값을 가리고 값은 명시적으로만 꺼낸다. 확인: 값이 든 래퍼를 Debug·Display로 찍은 문자열에 값이 없는 단위 테스트(AC 5). → `secret_value_debug_and_display_hide_the_value`, `secret_value_reveals_only_through_explicit_call`, `secret_value_equality_compares_the_wrapped_value` 통과.
- [x] 3. 자격 파일 읽기의 정상 경로를 만든다. 주석, 마지막 쉼표, `api_keys` 아래 이름별 문자열, `api_keys` 없음, 파일 없음. 확인: 경우마다 이름별 값과 빈 목록을 확인하는 단위 테스트(AC 1, 2). → `parse_credentials_with_comments_and_trailing_comma_finds_key_by_name`, `absent_credentials_has_no_key_and_is_not_an_error`, `missing_or_empty_api_keys_is_empty_list_not_error` 통과.
- [x] 4. 거부 경로를 만든다. 값이 문자열이 아닌 항목(엔드포인트 이름만), `api_keys`가 객체가 아님, JSONC 문법 오류(줄 번호만). 확인: 경우마다 오류를 확인하고 오류 Display·Debug에 값과 줄 내용이 없음을 확인하는 단위 테스트(AC 3, 4, 5). → `non_string_key_value_is_rejected_with_endpoint_name_only`, `jsonc_syntax_error_reports_only_line_number`, `api_keys_not_object_carries_no_value` 통과.
- [x] 5. 키 찾기를 만든다. 이름으로 값을 찾고, 없거나 빈 문자열이거나 파일이 없으면 "없음"을 돌려준다. 대소문자와 `-`·`_`가 다른 이름은 서로 다른 이름이다. 확인: 각 경우를 확인하는 단위 테스트(AC 1, 2). → `parse_credentials_names_are_case_and_symbol_sensitive_and_blank_is_none`, `absent_credentials_has_no_key_and_is_not_an_error` 통과.
- [x] 6. 이 세부 작업의 모든 오류에 대해 Display·Debug에 키 값이 없음을 마지막으로 한 번 모아 확인한다(AC 5). → `all_errors_hide_the_key_value_in_display_and_debug` 통과.
- [x] 7. `docs/RESEARCH.md` 작업 03 절 설정·키 위치 항목의 적용 위치를 확인하고, 구현 중 결정이 있으면 `docs/DECISIONS.md`에 적는다. → `Credentials::parse`(`crates/snap-core/src/credentials.rs`)와 `Credentials::find_key`가 적용 위치이고, `docs/DECISIONS.md` D18(jsonc-parser 0.34) 기록 완료.

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 없음(자격 파일 내용을 문자열로 준다). 테스트가 전역 환경을 쓰지 않는다.
- 파일 입출력이 필요하면 임시 폴더를 쓴다.
- 테스트 파일은 관심사별로 나눈다(자격 파일 읽기, 키 찾기, 비밀 값 가리기). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [x] [spec.md](spec.md)의 AC 5개를 모두 충족한다. → `cargo test -p snap-core`의 11개 테스트(`secrets.rs` 3개, `credentials.rs` 8개) 통과.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절 설정·키 위치 항목: 적용 위치 확인.
- `docs/DECISIONS.md`: 크레이트 라이선스 확인 결과와 구현 중 결정.
