# 02-1 비밀 값 래퍼와 자격 파일 읽기

- 상태: 완료 (T1~T5 완료, 6차 5차 리뷰 PASS 2026-11-23. 잔여 없음. 커밋 `1f535fb`, PR #2 병합으로 `feature/client` 반영 완료)

## 근거
- 기획 정본: `docs/plans/02-client.md` 범위(In) 02-1과 설계 요약의 자격 파일·비밀 값 항목, AC 3.
- 01 잔여 없음 (01 완료, 5차 리뷰 PASS). 01 산출물 계승: `crates/snap-core/src/config/` 모듈 자리, `consts/`·`locale/` 자리.
- 조사: 로컬 서버·서버별 문헌 조사는 02-5·02-7에서 다룬다. 02-1은 계획 정본이 정한 형식(자격 파일 이름·`api_keys` 구조)을 따른다.

## 목적
비밀 값을 가리는 래퍼 타입을 만들고, 사용자 설정 폴더의 자격 파일 `credentials.jsonc`를 읽어 엔드포인트 이름으로 키를 찾는다. 02-2(설정 읽기)와 02-5(전송)이 이 타입을 그대로 쓰게 하여, 키 값이 로그·오류·Debug에 남지 않는 기반을 만든다.

## 범위(In)
- `snap-core/src/config/`에 비밀 값 래퍼 타입을 둔다. Debug·Display·로그에 값을 내지 않고, 값이 필요한 곳(전송·저장)에서만 값을 꺼내게 한다.
- snap 설정 폴더 경로를 OS별로 결정한다. Windows는 `%APPDATA%\snap`, Linux는 `$XDG_CONFIG_HOME/snap`(미설정 시 `~/.config/snap`). OS별 파일(`mod.rs`+`windows.rs`+`unix.rs`)로 분리한다.
- 자격 파일 `credentials.jsonc` 읽기: 경로를 주입받고, JSONC로 해석하고, `api_keys` 표에서 trim한 엔드포인트 이름으로 키를 찾는다. 자격 파일에 이름이 없으면 키 없음(None)으로 돌려준다.
- 자격 파일 검증: 최상위 `api_keys` 객체가 없거나 키 값이 문자열이 아니면 오류로 돌려준다. 오류 문구는 locale 모듈(`locale/ko/config.rs`)에서 끌어오고, 파일 경로만 담으며 자격 파일에 담긴 값은 담지 않는다.
- 읽기 전용이다. 자격 파일 쓰기는 02-9 범위다.

## 금지
- 자격 파일 쓰기·수정(02-9 범위).
- HTTP 전송·`snap.jsonc` 해석·`api_key` 칸 거부·`.gitignore` 갱신(02-2 범위).
- 테스트 밖 `unwrap`/`expect`/`panic`.
- 키 값과 자격 파일 내용가 base_url을 Debug·Display·로그·오류 문구에 남기기.
- 기능 요구 없는 타입·함수 추가(YAGNI).

## 산출물
- `crates/snap-core/src/config/` 안의 비밀 값 래퍼·설정 폴더 경로·자격 파일 읽기 코드와 단위 테스트.
- 자격 파일 읽기에 새 크레이트를 처음 쓰면 라이선스를 확인해 `docs/DECISIONS.md`에 결정·근거·다시 볼 점을 남긴다.

## 검증 명령
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 비밀 값 래퍼 | `config/`에 `SecretString`을 두고 Debug 가리기·값 접근 메서드 하나로 정리 | Display 구현·`PartialEq`·`Hash` 추가 금지 | 래퍼 타입과 단위 테스트 | 게이트 3종 |
| T2 설정 폴더 경로 | OS별 파일로 `%APPDATA%`·`XDG_CONFIG_HOME` 경로를 결정하고 base 경로를 주입받는 형태로 정리 | `mod.rs`에 `#[cfg]` 본문 섞기 금지 | `windows.rs`·`unix.rs`와 단위 테스트 | 게이트 3종 |
| T3 자격 파일 읽기 | `credentials.jsonc`를 JSONC로 읽고 `api_keys` 표에서 이름으로 키 찾기·형식 검증·오류 문구 locale 연결 | 파일 쓰기·`snap.jsonc` 해석 금지 | 읽기 코드와 단위 테스트 | 게이트 3종 |
| T4 시험 폴더 묶기 | 같은 접두를 공유하는 시험 파일을 접두 폴더로 옮긴다 (PROJECT_RULES 접두 묶기 규칙) | 제품 소스 수정·기능 추가 금지 | `tests/credentials/`·`tests/secret_string/`와 `main.rs` | 게이트 3종 (T5와 합산, T4 단독 실행 없음) |
| T5 게이트·문서 정리 | T4까지 반영한 02-1 전체에 게이트 3종 1회 실행, 라이선스 확인·DECISIONS 갱신, RESEARCH·상태 갱신 | 계획 문서 내용 재설계 금지 | 게이트 기록·문서 갱신 | 게이트 3종 (02-1 전체 1회) |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/02/01/T1-secret-string.md` | T1 | 완료 |
| `docs/tasks/02/01/T2-config-dir.md` | T2 | 완료 |
| `docs/tasks/02/01/T3-credentials-read.md` | T3 | 완료 |
| `docs/tasks/02/01/T4-tests-folders.md` | T4 | 완료 |
| `docs/tasks/02/01/T5-gate-docs.md` | T5 (최종 게이트 1회) | 완료 |

## AC
1. `SecretString`의 `Debug` 출력에 평문 키가 나오지 않고 가렸음을 표시한다. 테스트로 확인한다.
2. 자격 파일 읽기가 `credentials.jsonc`의 `api_keys`에서 trim한 엔드포인트 이름으로 키를 찾는다는 것을 테스트로 확인한다. 이름 앞뒤 공백이 있어도 같은 이름으로 찾는다.
3. 자격 파일에 없는 엔드포인트 이름을 요청하면 키 없음(None)을 돌려준다.
4. `api_keys`가 없거나 키 값이 문자열이 아니면 파일 경로는 밝히고 값은 가린 오류를 돌려준다. 오류 문구가 locale 모듈에 있음을 확인한다.
5. 설정 폴더 경로 결정이 OS별 파일에 있고, `mod.rs`에 `#[cfg]` 본문이 없다. 경로는 주입으로 테스트하고, OS 실제 경로는 각 OS에서 시험으로 확인한다.
6. 게이트 3종 통과, 테스트 밖 `unwrap`/`expect`/`panic` 0건, 공개 항목에 한국어 문서 주석.
7. 새 크레이트를 썼다면 `docs/DECISIONS.md`에 라이선스 확인이 있고, 리뷰 기록이 `docs/reviews/02/`에 있다.

## 테스트 계획
- 네트워크 없이 임시 폴더에서 확인한다. 자격 파일 본문은 테스트 안에서 만들고, 실제 키 값 같은 값은 테스트 로그에 남기지 않는다.
- `api_keys` 정상 읽기, 이름 trim, 이름 없음, `api_keys` 누락, 값 타입 오류를 각각의 책임으로 나눈 테스트 파일에서 확인한다.
- 경로 결정은 base 경로를 주입해 확인하고, OS 실제 경로는 각 OS 시험(`windows_config_dir_under_appdata`·`unix_config_dir_under_xdg_config_home`)으로 확인한다. 로컬 검증은 Windows 게이트 4종 + WSL(Ubuntu) 게이트 3종이고, CI는 최종 확인으로 둔다.

## 문서 영향
- `docs/DECISIONS.md`: 새 크레이트 라이선스 확인 기록 (적용 시).
- `docs/RESEARCH.md`: 자격 파일 형식 결정 근거가 서버 문헌 조사와 연결되므로 02-5·02-7 조사 반영 시점에 적용 위치를 함께 갱신한다.
- `docs/plans/02-client.md`: 02-1 상태 줄 반영 (세부 작업 종료 시).
- `crates/snap-core/src/locale/ko/config.rs`: 자격 파일 오류 문구 키 추가. `locale/en/`에도 같은 키 뼈대를 둔다.

## 리스크
- JSONC 해석 크레이트 선택이 파서 동작(주석 처리, 후행 쉼표 허용 범위)을 가른다. 계획 정본은 주석 허용 JSON만 요구하고, 후행 쉼표는 허용하지 않는다.
- 자격 파일이 없는 첫 실행은 키 없음이 정상 경로다. 설정 읽기(02-2)가 없는 파일과 없는 이름을 구분하므로, 02-1은 없는 파일도 명시적인 오류로 돌려준다.

## DoD
AC 충족, 게이트 3종 통과, `docs/reviews/02/`에 리뷰 PASS 기록.