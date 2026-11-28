# 02-8 모델 `context_window` 읽기·검증·예시: 작업 계획

- 상태: 완료 (기획 승인 2026-11-28. T1~T4 구현·개별 리뷰 PASS. 작업 단위 전체 리뷰 r1 미달(minor 4건 문서·주석 정합) → 보정 → r2 PASS(`docs/reviews/02/02-8-unit-r2.md`). PR #14로 `feature/client` 병합 2026-11-28)
- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 하네스 전달: T-파일 `읽을 파일`에 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md`, dev-principles 색인(`.agents/skills/dev-principles/SKILL.md`, Cursor·Claude는 `.claude/skills/…`)을 넣는다.
- 지정 잎(build·review): solid·dry·kiss-yagni·separation·naming·errors·security·testing·comments·workflow·constants-locale.
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(게이트 3종). Windows 로컬과 WSL(Ubuntu) 양쪽.
- 브랜치: `feature/client`에서 나눈 `feature/client-context-window`에서 진행한다.

## 단계

### T1 `context_window` 읽기·기본값
- 상태: 완료 (r1 미달(minor 3건 문서 정합) → 문서 정합 보정. 코드·시험·AC·게이트는 r1 통과, `docs/reviews/02/02-8-T1-r1.md`)
- 범위: `ModelConfig`에 `context_window: u64`를 더하고, 생략 시 65536 기본값 창구를 둔다. 기본값 상수는 `consts/` 상수 모듈에 둔다.
- 금지: 요청 본문에 이 키 싣기, 다른 칸 추가, 상한 상수 도입.
- 확인: 값 있음·생략 읽기 시험(spec AC 1·2·4).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T2 값 검증·거부 오류·locale 문구
- 상태: 완료 (r1 미달 4건(major 1, minor 3) → 보정 → r2 PASS(지적 0건), `docs/reviews/02/02-8-T2-r2.md`)
- 범위: 1 이상 정수만 받아들인다. 0·음수·소수·문자열은 모델 이름만 밝힌 설정 오류로 거부한다. `is_integer_at_least`를 재사용한다. 거부 문구를 `locale/ko/validate.rs`·`locale/en/validate.rs`에 둔다.
- 금지: 입력 값·주소·키를 오류에 담기, 상한 도입, 검증 규칙을 두 곳에 두기.
- 확인: 0·음수·소수·문자열 거부와 모델 이름만 노출 시험(spec AC 3).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T3 예시 파일 주석
- 상태: 완료 (r1 미달 minor 3건 → 보정 → r2 PASS(지적 0건), `docs/reviews/02/02-8-T3-r2.md`)
- 범위: `snap.example.jsonc`에 `context_window` 안내 주석을 추가한다. 값은 넣지 않는다.
- 금지: 예시 파일에 값 넣기, 빈 파일(`agent-snap.example.jsonc`) 손대기.
- 확인: 예시 검증·자리표시자 시험 통과와 주석 존재(spec AC 5).
- 검증: 게이트 3종 Windows·WSL(Ubuntu) 양쪽.

### T4 구조 점검 (미커밋 작업 파일 전수)
- 상태: 완료 (대상 파일 전수 표 작성, 위반 0건, 100줄 초과 3종 SRP 검토 기록 포함, `docs/tasks/02/08/T4-structure-audit.md`)
- 범위: 커밋되지 않은 모든 작업 파일을 열어 항목·책임 묶음·책임 개수·위반 여부를 표로 남긴다. 100줄 초과 파일 전부에 SRP 검토 기록을 남긴다.
- 금지: 코드 변경, 파일 삭제, 커밋·푸시.
- 확인: 표가 대상 파일 전부를 담는다. 위반을 잔여로 이관한다.
- 검증: `.md` 전용 변경이면 주석 전용 경량 게이트(`cargo fmt --check`만). 코드 변경이 있으면 게이트 3종.

## 테스트 계획

- 종류: 값 주입 설정 읽기·거부 단위 시험, 예시 파일 검증 시험.
- 주입할 것: `context_window` 값(정상·0·음수·소수·문자열), 모델 이름.
- 네트워크·파일 쓰기·시간 값은 쓰지 않는다.
- 시험 파일은 관심사별로 나눈다(읽기 값, 거부 형식, 가리기, 예시).

## DoD

- [spec.md](spec.md)의 AC 5개를 모두 충족한다.
- 게이트 3종 Windows·WSL 통과.
- `docs/RESEARCH.md` 02 절의 02-8 항목 적용 위치 갱신.
- `docs/DECISIONS.md` 20번 결정 기록.
- 코드 품질 리뷰 PASS 기록이 `docs/reviews/02/`에 있다.

## 문서 영향

- `docs/DECISIONS.md`: 20번(`context_window` 규칙) 추가.
- `docs/RESEARCH.md` 02 절: 02-8 적용 위치 갱신.
- `docs/PLAN.md`·`docs/plans/02-client.md`·`docs/plans/02-client/08/*` 상태 줄 갱신.
- `snap.example.jsonc`: 주석 추가.
