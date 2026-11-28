# 02-9 설정 저장: 작업 계획

- 상태: 완료 (기획 승인 2026-11-28. T1~T5 구현·개별 리뷰 PASS, 잔여 R1~R3 보정 완료. 작업 단위 전체 리뷰 r1 미달(major 1·minor 5) → 보정 → r2 미달(minor 1) → HARNESS 9절로 메인 보정 후 닫음. PR #15로 `feature/client` 병합 2026-11-28)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.
- 브랜치: `feature/client`에서 나눈 `feature/config-save`에서 진행한다.

## 단계

### T1 저장 골격(주석 보존 편집·원자 저장·저장 오류)
- 상태: 완료 (구현 완료·게이트 Windows·WSL 통과. 독립 리뷰 r1 minor 3건 문서 보정 후 PASS)
- 범위: `jsonc-parser`에 `cst` 기능을 켜고, `tempfile`을 제품 의존으로 추가한다. 설정 폴더가 없으면 만들고, 원본 폴더에 임시 파일을 써서 원본과 교체하는 원자 저장을 둔다. 저장 실패 오류 타입과 locale 문구를 둔다.
- 금지: 파일 전체 갈아쓰기, copy 대체 경로, `serde_json` `preserve_order` 켜기, 오류에 키·주소 담기.
- 확인: 임시 폴더 왕복 시험(spec AC 7), 교체 실패 시 원본 보존 시험.
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T2 엔드포인트 추가·수정·삭제
- 상태: 완료 (구현 완료·리뷰 r1 minor 3건 보정 완료·게이트 3종 Windows·WSL 통과. r2 minor 1건 문서 보정 후 닫음)
- 범위: 엔드포인트 이름 기준으로 `snap.jsonc`의 `endpoints` 배열에 추가·수정·삭제한다. 저장 직전에 `check_config`를 재사용해 통과해야만 쓴다.
- 금지: 키 칸을 설정에 쓰기, 검증 규칙을 두 곳에 두기, 오류에 키·주소 담기.
- 확인: 왕복·주석 보존·다른 항목 보존·거부 시험(spec AC 1·2·3·6).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T3 모델 추가·수정·삭제
- 상태: 완료 (구현 완료·리뷰 r1 minor 3건 보정 완료·게이트 3종 Windows·WSL 통과. r2 PASS)
- 범위: 엔드포인트 이름·모델 이름 기준으로 `endpoints[].models` 안 항목을 추가·수정·삭제한다. `context_window`는 1 이상 정수만 쓴다(결정 20번).
- 금지: 요청 본문 키 규칙 위반, `context_window` 상한 도입, 검증 규칙을 두 곳에 두기.
- 확인: 모델 왕복·엔드포인트 보존·주석 보존 시험(spec AC 2·4).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T4 자격 파일 키 추가·수정·삭제
- 상태: 완료 (구현 완료·게이트 3종 Windows·WSL 통과. 리뷰 r1 minor 2건 Fast-track 보정 후 닫음)
- 범위: 엔드포인트 이름 기준으로 `credentials.jsonc`의 `api_keys` 표에 키를 추가·수정·삭제한다. 자격 파일이 없으면 최상위 `api_keys` 객체로 새로 만든다. 저장 직전에 자격 구조 검사를 재사용한다.
- 금지: 키를 `snap.jsonc`에 쓰기, 오류에 키 값 담기.
- 확인: 키 왕복·`snap.jsonc` 무변경·주석 보존·가리기 시험(spec AC 2·5·6).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T5 구조 점검 (미커밋 작업 파일 전수)
- 상태: 완료 (표 산출물 작성 완료. 리뷰 r1 표 정합 6건 보정. 작업 단위 리뷰 r1 major 1건 해소로 대상 104건 갱신)
- 범위: 커밋되지 않은 모든 작업 파일을 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일 전부에 SRP 검토 기록을 남긴다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.
- 산출물: `docs/tasks/02/09/T5-structure-audit.md`

## 잔여 (T5 구조 점검 이관분)

- 상태: 완료 (R1~R3 보정 구현 완료·게이트 3종 Windows·WSL 통과. 리뷰 r1 minor 3건(주석·계획 문서 참조·오류 함수 자리) 보정 완료·게이트 3종 양쪽 통과. 작업 문서: `docs/tasks/02/09/R1-R3-structure-fixes.md`)
- 이관 시점: 계획 (T5 표에서 이관. 보정 구현은 승인 후 `feature/config-save`에서 순차로 잇는다)

| 번호 | 대상 | 위반 사유 | 할 일 |
|---|---|---|---|
| R1 | `crates/snap-core/src/config/save/select.rs` | 120줄 초과, 오류 생성 함수 4개(`endpoints_not_array`·`models_not_array`·`endpoint_not_found`·`model_not_found`)가 선택 규칙과 다른 변경 이유 | 오류 조립을 별도 파일로 분리하고 참조를 정리한다 (완료: `config/save/errors.rs`로 이동. r1 보정: `config/save/credentials/select.rs`의 `api_keys_not_object`를 같은 기준으로 `config/save/errors.rs`로 옮기고, `key_not_found`는 없애며 기존 `endpoint_not_found`를 재사용) |
| R2 | `crates/snap-core/src/locale/en/config.rs`·`crates/snap-core/src/locale/ko/config.rs` | 124줄, 자격·설정 검증·저장 문구가 한 파일에 동거 (constants-locale의 도메인별 파일 분리 미준수) | locale 도메인별 파일로 나누고 참조를 함께 고친다 (완료: `locale/<lang>/credentials.rs`·`validate.rs`·`save.rs`로 분할) |
| R3 | `crates/snap-core/src/config/mod.rs` | `config_dir_in`·`snap_file_in`이 모듈 골격과 동거 | 경로 조립 함수를 별도 파일로 옮긴다 (완료: `config/paths.rs`로 이동, `mod.rs`는 선언·재수출 골격만) |

- 이관에서 뺀 항목(리뷰 r1 반영): `check.rs`(순수 forwarding 아님 — `ConfigSaveError` 변환 책임을 가짐), `credentials/` 접두 되풀(파일 이름에 접두 없음), `jsonc.rs` 삭제 자리(이미 정리 완료). `agent-snap.example.jsonc` 빈 파일은 02-8 계획이 "손대지 말 것"으로 정한 자리라 이관하지 않는다.

## 테스트 계획

- 종류: 임시 폴더 왕복 저장 시험, 주석 보존 시험, 거부 시 원본 보존 시험, 키 가리기 시험.
- 주입할 것: 설정·자격 텍스트(문자열), 연산 요청(엔드포인트·모델·키 이름과 값).
- 네트워크는 쓰지 않는다. 시간은 쓰지 않는다.
- 시험 파일은 관심사별로 나눈다(왕복, 주석 보존, 거부·원본 보존, 가리기, 파일 생성).

## DoD

- [spec.md](spec.md)의 AC 7개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-9 항목 적용 위치 갱신.
- `docs/DECISIONS.md` 21번 결정 기록.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/DECISIONS.md`: 21번(`cst` 편집과 `tempfile` 원자 저장, `preserve_order` 미사용) 추가.
- `docs/RESEARCH.md` 02 절: 02-9 적용 위치 갱신.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/09/*` 상태 줄 갱신.
