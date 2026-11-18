# 코드 주석 짧게 정리 (독립 정리)

- 상태: 완료
- 성격: 기획 순번(01~09·03 세부)과 무관한 유지 정리. 순번표·세부 표에 행을 넣지 않는다.
- 브랜치: `feature/agent-comment-shorten` (`feature/agent` 최신 위에서 분기)
- 작성일: 2026-11-17

## 목적

개발 코드 주석이 필요 이상으로 길다. 코드로 드러나지 않는 의도·이유·근거만 결론 먼저 1~2문장으로 남기고, 리뷰 라운드·보정 이력 표기는 코드에서 뺀다.

## 리서치 및 설계 근거

단순 주석 정리라 Research Gate 생략.

| 근거 | 반영 |
|---|---|
| HARNESS.md 1절 언어와 우선순위 | 주석은 의도·이유·근거를 결론 먼저 1~2문장, 외국어·한자 금지 |
| eli5-clarity | 코드로 자명한 줄은 설명하지 않음 |
| PROJECT_RULES 공개 항목 한국어 주석 | 공개 항목 문서 주석은 한국어 유지 |
| 사용자 지시 2026-11-17 | `crates/**/*.rs` 전체, 주석만, 동작 불변 |

측정(2026-11-17, Windows): `crates/**/*.rs` 332종, 25,795줄 중 주석 4,640줄(18%).

## 범위(In)

- `crates/**/*.rs` 전체 주석·독스트링 정리.
- 남길 것: 코드에 안 드러나는 제약·수명 요구, 지금 동작의 의도·이유(1~2문장), 근거 포인터(정본 파일:줄).
- 지울 것: 리뷰 라운드 번호·보정 이력, 규칙 인용 문구 나열, `AC n`·`spec`·`할 일 n`·`라운드 n`·`T2`·세부 작업 번호(03-8) 같은 작업 문서 참조, 코드 대상 밖에 있는 경위·설계 이동 설명.
- 주석 성격: 대상(타입·함수·필드)이 하는 일을 밝히는 설명만 둔다. 파일 머리(`//!`)는 파일 책임 한 줄로만 좁히고, 같은 내용의 마디를 여러 개 늘리지 않는다.
- 제거 전: 이력 문장의 정본이 `docs/reviews/`·T-파일에 있는지 확인 후 제거.
- 모듈별 T(아래 작업계획). T마다 구현 서브 1 → 독립 리뷰 1.

## 비범위(Out)

- 코드 동작·공개 API 시그니처·시험 본문 로직 변경.
- 기획 순번표(`docs/PLAN.md` M1·M2)·`docs/plans/03-agent-loop-tools.md` 세부 표에 행 추가.
- 새 기능·리팩터·파일 이동(주석 문장만).

## AC

1. Given `crates/**/*.rs`를 전수 볼 때, When 주석을 고치면, Then 남는 주석은 의도·이유·근거 1~2문장(또는 근거 포인터)이고 라운드·보정 이력 표기가 코드에 없다.
2. Given 주석만 고친 뒤, When 게이트 3종을 Windows와 WSL Ubuntu(`PATH=$HOME/.cargo/bin`)에서 각각 돌리면, Then 모두 exit 0.
3. Given 착수 전 `#[test]`·`#[tokio::test]` 개수, When 작업이 끝나면, Then 개수가 같다(시험 수 불변).
4. Given 모듈별 T, When 그 T 범위 파일만 고치면, Then 범위 밖 `.rs`는 손대지 않는다.
5. Given 제거한 이력 문장, When 검색하면, Then 정본이 `docs/reviews/` 또는 T-파일에 있다(없으면 제거하지 않거나 정본을 먼저 남긴다).

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 config | `crates/snap-core/src/config/**`, `credentials/**`, `jsonc/**`, `secrets/**` 주석 | 동작·시험 로직 변경, 범위 밖 파일 | 짧아진 주석 | 게이트 3종(Win+WSL) |
| T2 llm | `crates/snap-core/src/llm/**` 주석 | 동일 | 짧아진 주석 | 동일 |
| T3 agent | `crates/snap-core/src/agent/**` 주석 | 동일 | 짧아진 주석 | 동일 |
| T4 tools | `crates/snap-core/src/tools/**` 중 `execute_bridge` 제외 주석 | 동일 | 짧아진 주석 | 동일 |
| T5 execute_bridge | `execute_bridge/**`, `execute.rs`, `execute_bridge.rs` 주석 | 동일 | 짧아진 주석 | 동일 |
| T6 tests | `crates/snap-core/tests/**`·루트 `tests/*.rs` 주석 | 동일 | 짧아진 주석 | 동일 |
| T7 cli_other | `crates/snap-cli/**`, 나머지 `snap-core/src` 루트·기타 모듈 주석 | 동일 | 짧아진 주석 | 동일 |

## DoD

AC 충족, 모듈별 리뷰 PASS(`docs/reviews/comment-shorten/`), 시험 수 불변, 기획 순번표 무변경.

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/comment-shorten/T1-config.md` | T1 config | 완료 |
| `docs/tasks/comment-shorten/T2-llm.md` | T2 llm | 완료 |
| `docs/tasks/comment-shorten/T3-agent.md` | T3 agent | 완료 |
| `docs/tasks/comment-shorten/T4-tools.md` | T4 tools | 완료 |
| `docs/tasks/comment-shorten/T5-execute-bridge.md` | T5 execute_bridge | 완료 |
| `docs/tasks/comment-shorten/T6-tests.md` | T6 tests | 완료 |
| `docs/tasks/comment-shorten/T7-cli-other.md` | T7 cli_other | 완료 |

## 착수·병합

1. `git checkout feature/agent && git pull origin feature/agent`
2. `feature/agent-comment-shorten` 생성·이동(미커밋 `03-08` plan 병합 사실 1줄 함께 반영)
3. T1부터 모듈별 구현→리뷰. 기본 브랜치에 붙이지 않음
4. 전체 PASS 후 로컬 커밋(허락) → 푸시 → `feature/agent` PR → CI 4검사 초록 → 병합(PR 번호·브랜치 이름만 기록)
