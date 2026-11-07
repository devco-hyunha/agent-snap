# 02-9 작업 02 전체 리뷰: 명세

- 상위 문서: [plan.md](plan.md)
- 상태: planned (2026-11-06)

## 입력과 출력

- 입력: 작업 02의 승인된 plan·spec·tasks, 구현된 코드·테스트, 기존 세부·잔여 리뷰 기록, 하네스·dev-principles.
- 출력: 작업 02 전체 리뷰 PASS(또는 미달 지적 목록), `docs/reviews/` 기록, 상위 DoD 충족 여부. 제품 동작은 바꾸지 않는다.

## 검수 축

1. **기획·코드 정합**: 02-client 설계 요약·각 세부 AC와 실제 모듈·공개 API·테스트가 모순 없이 맞는지.
2. **하네스 준수**: HARNESS·PROJECT_HARNESS·승인 안전, 파일 단위 SRP, 테스트 비동거, 비밀·가리기, 리뷰 없이 PASS하지 않음.
3. **코드 품질**: review-harness 7항목, dev-principles 2~11절을 작업 02 합본에 적용.
4. **검증**: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` 재실행.

## 금지 사항

- 기능 추가·동작·오류 문구 변경을 리뷰 세션에서 하지 않는다(수정 금지, 문제만 기록).
- 세부 작업 조각 PASS만으로 작업 02 전체를 PASS로 대체하지 않는다.
- 커밋·푸시 금지(사용자 허락 전).

## AC (인수 조건)

1. Given 02-1~02-8(잔여 포함) 산출물이 있을 때, When 작업 02 전체 리뷰를 하면, Then [02-client.md](../../02-client.md) AC 1~5가 증거(테스트·문서·리뷰)로 충족됨이 기록된다.
2. Given 세부 plan·spec과 코드가 있을 때, When 정합을 점검하면, Then 설계 요약·경계(트레이트만 노출, HTTP는 전송 안, 키·base_url 비가리기 등)와 어긋난 critical·major가 0건이다.
3. Given 하네스·dev-principles가 있을 때, When 작업 02 코드를 보면, Then 신규 책임 혼재·테스트 동거 확대·승인 위반 critical·major가 0건이다.
4. Given 검증 3종이 있을 때, When 독립 리뷰 세션이 재실행하면, Then 모두 exit 0이다.
5. Given 리뷰가 끝나면, When 기록을 남기면, Then `docs/reviews/02-client/`에 작업 02 전체 리뷰 PASS 파일이 있고 상위 DoD를 닫을 수 있다.

## 테스트 계획

- 새 단위 테스트 추가 없음(검수 게이트).
- 검증 명령은 공통 기준 3종을 리뷰어가 직접 재실행한다.

## 문서 영향

- `docs/reviews/02-client/02-client-full-review-1.md`(또는 동등 이름).
- `docs/plans/02-client.md` 상태·DoD.
- 필요 시 RESEARCH/DECISIONS의 빈 칸만 지적(본 작업에서 대량 재작성하지 않음).

## 리스크

- 조각 리뷰 PASS에 기대 전체를 성급히 PASS → AC 2·3으로 막는다.
- 02-8 잔여 미마감 상태로 02-9 착수 → 선행 조건으로 잔여 작업 11 완료를 권장한다.
