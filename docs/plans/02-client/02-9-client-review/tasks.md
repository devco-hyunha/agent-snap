# 02-9 작업 02 전체 리뷰: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 상태: planned (2026-11-06. 구 SRP 02-9는 02-8 잔여로 이동)

## 구현 단계

### 작업 1. 검수 범위와 체크리스트를 고정한다

- 범위: 이 폴더의 plan·spec과 `docs/tasks/02-9-client-review-0-overview.md`(없으면 작성), 증거 목록(세부 리뷰 경로·모듈 목록).
- 금지: 제품 코드 변경, 커밋.
- 산출물: 상세 작업 문서·체크리스트(정합 / 하네스 / 코드 리뷰 / 검증).
- 검증: 문서만. 사용자 승인 후 작업 2.

### 작업 2. 독립 세션으로 작업 02 전체 리뷰를 한다

- 범위: 02-1~02-8(잔여 SRP 포함) 합본. review-harness 7항목, 하네스 7·9·10절, dev-principles 2~11절, [02-client.md](../../02-client.md) AC.
- 금지: 수정(문제만 기록), 조각 PASS로 전체 대체, 임시 스크립트, 커밋.
- 산출물: `docs/reviews/02-client/02-client-full-review-1.md`, 검증 로그 `.local/`.
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

### 작업 3. 상위 DoD를 갱신하고 커밋 허락을 받는다

- 범위: `docs/plans/02-client.md` 상태·DoD, 필요 시 세부 tasks 완료 표시.
- 금지: 허락 전 커밋·푸시.
- 산출물: 상위 문서 갱신, 사용자 보고(결과 1줄 / 불릿≤5 / 경로 / 다음 1줄).

## 테스트 계획

- 검수 게이트. 새 테스트 없음. 검증 3종 재실행만.

## DoD

- [x] spec.md AC 5개 충족 (3차 리뷰 PASS `docs/reviews/02-client/02-client-full-review-3.md`).
- [x] 작업 02 전체 리뷰 PASS 기록이 `docs/reviews/`에 있다 (1차 미달 → 02-2b 보정 → 2차 미달 → fmt 보정 → 3차 PASS).
- [x] [02-client.md](../../02-client.md) DoD를 닫을 수 있는 상태다.

## 문서 영향

- `docs/plans/02-client/02-9-client-review/`
- `docs/reviews/02-client/02-client-full-review-*.md`
- `docs/plans/02-client.md`

## 리스크

- 02-8 잔여 미마감 → 작업 2 전에 잔여 작업 11을 닫는다.
- 동일 지적 보정 2회 정지(하네스 5절 2번).
