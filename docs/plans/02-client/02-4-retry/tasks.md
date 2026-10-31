# 02-4 재시도 판단: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. 재시도 대상 판별을 만든다(429, 500/502/503/504, 헤더 전 연결 오류 대상, 헤더 대기 시간 초과 제외, 그 밖의 4xx 포기. D14). 확인: 상태 코드별 판별 단위 테스트(AC 3).
- [ ] 2. `Retry-After` 해석을 만든다. 초와 HTTP 날짜 두 형식, 해석 불가와 음수는 헤더 없음, 0과 과거 날짜는 대기 0초(D14). 현재 시각은 주입받는다. 확인: 두 형식과 해석 불가·음수·0·과거 날짜 입력의 단위 테스트(AC 1).
- [ ] 3. 상한 비교를 만든다. 상한 이하는 그 시간만큼 대기, 초과는 대기 초를 밝힌 포기. 확인: 경계값 단위 테스트(AC 1).
- [ ] 4. 지수 백오프와 Full Jitter를 만든다. 난수는 주입받고 최대 횟수를 넘으면 포기한다. 확인: 난수 고정으로 대기 시간이 0 이상이고 상한과 `기본값 × 2^(시도-1)` 중 작은 값 이하임을 시도 횟수별로 확인하는 단위 테스트(AC 2).
- [ ] 5. `docs/RESEARCH.md` 작업 02 절의 RFC 9110, RFC 6585, OpenRouter API Rate Limits, AWS, Google SRE 항목에 02-4 적용 위치를 갱신한다.

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 난수 공급, 현재 시각. 실제 대기는 하지 않는다.
- 테스트 파일은 관심사별로 나눈다(대상 판별, `Retry-After` 해석, 백오프). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [ ] [spec.md](spec.md)의 AC 3개를 모두 충족한다.

## 문서 영향
- `docs/RESEARCH.md` 작업 02 절: RFC 9110, RFC 6585, llama.cpp(503), OpenRouter API Rate Limits, AWS, Google SRE.
- `docs/DECISIONS.md`: 구현 중 결정이 있으면 추가한다.
