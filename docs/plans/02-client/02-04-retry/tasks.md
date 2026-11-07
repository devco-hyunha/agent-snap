# 02-4 재시도 판단: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 세부 작업 총괄: [docs/tasks/02/04-retry/T0-overview.md](../../../tasks/02/04-retry/T0-overview.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 상태: done

## 구현 단계
세부 작업 문서는 `docs/tasks/`에 두고, 아래 단계는 그 파일과 1:1로 대응한다(구현 brief·하네스 주입 사본은 이 단계가 아님).

- [x] 1. 재시도 대상 판별 — [02-4-retry-1-classify.md](../../../tasks/02/04-retry/T4-api.md)  
  상태 코드 화이트/블랙리스트(429·500/502/503/504 vs 402·408·425·목록 밖 5xx·기타 4xx), 헤더 전 연결 오류 후보, 헤더 대기 초과 비재시도(D14, D26). 확인: 판별 단위 테스트(spec AC 5·6).
- [x] 2. `Retry-After` 해석·상한 — [02-4-retry-2-retry-after.md](../../../tasks/02/04-retry/T4-api.md) — 상태: done (리뷰 PASS: [review-2](../../../reviews/02/04-retry/2-retry-after-review-2.md))  
  초·HTTP 날짜, 0/과거→0초, 음수·해석불가→헤더 없음, 상한 이하 대기·초과 포기(D14, D26). 현재 시각 주입. 확인: 단위 테스트(spec AC 1·2).
- [x] 3. Full Jitter 백오프·횟수 — [02-4-retry-3-backoff.md](../../../tasks/02/04-retry/T4-api.md) — 상태: done (리뷰 PASS: [review-2](../../../reviews/02/04-retry/3-backoff-review-2.md))  
  난수 주입, 시도 횟수(첫 요청=1), 횟수 초과를 `Retry-After` 상한 초과보다 먼저(D13, D26). 확인: 난수 고정 단위 테스트(spec AC 3·4).
- [x] 4. 공개 API·모듈 배치 — [02-4-retry-4-api.md](../../../tasks/02/04-retry/T4-api.md) — 상태: done (리뷰 PASS: [review-2](../../../reviews/02/04-retry/4-api-review-2.md))  
  `llm/retry/`(D25), `decide_retry`·`RetrySettings`(Duration/`u32`), 포기 이유 필드 제한, 관심사별 테스트(`retry_api.rs`). 확인: API·포기 필드 단위 테스트(spec AC 7) 및 통합 판정 스모크. review-1 PASS(minor3) → minor1·2 보정 → review-2 PASS.
- [x] 5. `docs/RESEARCH.md`·`docs/research/02-client.md`의 RFC 9110, RFC 6585, OpenRouter Rate Limits(402 미적용 D26 포함), AWS, Google SRE 항목에 02-4 적용 위치를 코드 반영 후 갱신한다.

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 난수 공급, 현재 시각. 실제 대기는 하지 않는다.
- 테스트 파일은 관심사별로 나눈다(대상 판별, `Retry-After` 해석, 백오프, API). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [x] [spec.md](spec.md)의 갱신된 AC(7개)를 모두 충족한다.
- [x] 검증: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`(인라인·프로젝트 공통 명령).
- [x] review-harness 기준 코드리뷰 **PASS** 기록: [02-04-retry-review-1](../../../reviews/02/04-retry/review-1.md), [api-review-2](../../../reviews/02/04-retry/4-api-review-2.md).

## 문서 영향
- `docs/RESEARCH.md`·`docs/research/02-client.md` 작업 02 절: RFC 9110, RFC 6585, llama.cpp(503), OpenRouter API Rate Limits(D26), AWS, Google SRE.
- `docs/DECISIONS.md`·`docs/decisions/D26-retry-exclusions-mapping.md`: 채택 결정.
- `docs/tasks/02-04-retry*.md`: 세부 작업 분할.
