# 03-3 파일 도구·glob·grep·list_dir: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계 (T1 → T2 → T3 순서, plan.md 작업계획 표와 1:1 대응)
- [x] 1. (T1) `path_guard` 1차를 만든다: 경로 정규화·작업 폴더 경계·비밀 파일 목록 검사까지 한 파일 한 책임으로 채운다(도구마다 호출). 확인: 작업 폴더 밖 경로 거부 단위 테스트(spec AC 2)와 `agent-snap.jsonc`·`credentials.jsonc` 읽기·쓰기 거부 단위 테스트(spec AC 4).
- [x] 2. (T1) `read_file` 도구를 만든다(크기 상한·잘림·바이너리 거부·`offset`·`limit` 구간 읽기·거부 시 안내 문구). 확인: 크기·잘림·바이너리·구간 단위 테스트(spec AC 1).
- [x] 3. (T1) `write_file`을 만든다. 덮어쓰기 표시. 승인 대상 아님(D49). 확인: 임시 폴더 단위 테스트.
- [x] 4. (T1) `edit_file`을 만든다(일치 1개·일치 없음·다중 일치). 확인: 3종 단위 테스트(spec AC 3).
- [x] 5. (T2) `list_dir`·`glob`을 만든다(경계·비밀 파일 차단, 숨김 기본 제외·opt 포함). 확인: 단위 테스트.
- [x] 6. (T2) `grep`을 만든다. ripgrep 경로 우선·없으면 내장 검색·결과 200줄(3형식 공통)·잘림·비밀 파일 차단. 확인: 단위 테스트(spec AC 5·6).
- [x] 7. (T3) `path_guard` 확장: 심볼릭 링크 검사 추가와 검사 순서 고정(경로 정규화 → 경계 → 링크 → 비밀 목록). 확인: 링크 포함 경계·비밀 차단 단위 테스트(spec AC 2). 완료 메모(2026-11-10, Windows): `path_guard.rs` 한 자리로 순서 확정, 링크는 "작업 폴더 안에서 생성된 링크만 허용"(OS별 `#[cfg]` 게이트), `walk`·ripgrep 순회 기준 통일, 게이트 `gate03_t3_t3_cargo` fmt=0·clippy=0·test=0.
- [x] 8. `docs/RESEARCH.md` 작업 03 절 적용 위치 갱신.
- [x] 9. (T4) `is_secret_name` 확장: `.env` 계열(`.env`, `.env`로 시작하는 파일)을 읽기·쓰기·목록·검색 차단 목록에 추가한다. 확인: `.env`·`.env.local` 읽기·쓰기 거부와 목록·검색 결과 제외 단위 테스트(spec AC 8). 완료 메모(2026-11-10): `is_env_family` 한 곳 정본(Windows 대소문자 무시), grep 인자 거르기 `!.env`·`!.env*` 2줄, 리뷰 5차 PASS(`docs/reviews/03/03-03-file-tools/T4-review-5.md`, 지적 0건, 게이트 fmt=0·clippy=0·test=0).

## 테스트 계획
- 임시 폴더에 파일·링크·비밀 파일을 만들고, 읽기·쓰기·검색을 확인한다.
- 심볼릭 링크는 OS 생성이 가능하면 테스트로 만들고, Windows에서 막히면 링크 검사 코드 단위 테스트로 대체한다.
- 잘림·크기 상한·200줄 제한은 값만 바꿔 짧게 확인한다.
- 구간 읽기는 256KB 넘는 파일을 50KiB 이하 구간으로 읽고, 기본 읽기 거부 안내(`offset`·`limit`·grep)가 결과에 남는지를 확인한다(D49, spec AC 1a).

## DoD
- [x] [spec.md](spec.md) AC 7개(AC 1a 구간 읽기 포함) 충족. (AC 1·1a·3 = T1, AC 5 = T2, AC 2·4·6 = T1 1차→T2 목록·검색→T3 링크·순회 확정. 03-3 AC 1~7는 T1·T2·T3 각 리뷰 2차 PASS로 확정)
- [x] 검증 명령 3종 통과. (로그 gate03_t3_t3_cargo·gate03_t3_t3_fasttrack: fmt=0·clippy=0·test=0)
- [x] `docs/reviews/03/` 리뷰 PASS. (T1·T2·T3 각 2차 PASS, T3는 `docs/reviews/03/03-03-file-tools/T3-review-2.md`에 지적 0건 확정)

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절.
- 필요하면 `docs/DECISIONS.md`(경계 규칙 세부 결정 시).
## 작업 파일
| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-03-file-tools/T1-file-write-tools.md` | T1 읽기·쓰기·편집 | 완료 |
| `docs/tasks/03/03-03-file-tools/T2-list-search-tools.md` | T2 목록·검색 | 완료 |
| `docs/tasks/03/03-03-file-tools/T3-path-guard.md` | T3 경계·비밀 차단 | 완료 |
| `docs/tasks/03/03-03-file-tools/T4-env-block.md` | T4 `.env` 계열 차단 | 완료(리뷰 5차 PASS)·SRP 분할 완료(분할 리뷰 4차 PASS) |
