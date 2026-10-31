# 02-3 SSE 파싱과 델타 복원: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. SSE 줄 처리를 만든다. 줄 끝 세 종류, 주석 줄, 여러 data 줄 결합, BOM, 조각 경계 잘림과 멀티바이트 문자, `[DONE]`, 빈 줄 없이 끝난 마지막 이벤트(D12). 확인: 형식별 입력과 조각 경계 시험을 하는 단위 테스트(AC 1, 2).
- [ ] 2. 조각 경계 불변성을 확인한다. 위 형식이 섞인 바이트를 임의 경계와 한 바이트씩 나눠 넣어 한 번에 넣은 결과와 같은지 본다. 확인: 단위 테스트(AC 1).
- [ ] 3. 청크 JSON에서 content, 생각(두 이름), tool_calls를 복원한다. index별 누적과 병렬 교차 포함. 확인: 순서 보존과 교차 델타를 확인하는 단위 테스트(AC 2, 3).
- [ ] 4. 모르는 델타 필드 원문 보존, 최상위 `error` 청크의 스트림 오류, 잘못된 JSON의 형식 오류, 잘린 마지막 이벤트의 형식 오류를 만든다. 확인: 오류 Display·Debug에 키·주소가 없음을 포함한 단위 테스트(AC 4).
- [ ] 5. usage 읽기(있으면 값, 없으면 "없음")를 만든다. 확인: 단위 테스트(AC 5).
- [ ] 6. `docs/RESEARCH.md` 작업 02 절의 WHATWG, llama.cpp, vLLM, LM Studio, OpenRouter Streaming, OpenRouter Reasoning Tokens 항목에 02-3 적용 위치를 갱신한다.

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 없음(바이트 조각을 직접 넣는다).
- 테스트 입력은 확인된 사실의 형식(keep-alive 주석, reasoning 필드, CRLF, 청크 경계에서 잘린 줄)을 반영한다.
- 테스트 파일은 관심사별로 나눈다(줄 처리, 조각 경계, 델타 복원, 오류). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [ ] [spec.md](spec.md)의 AC 5개를 모두 충족한다.

## 문서 영향
- `docs/RESEARCH.md` 작업 02 절: WHATWG, llama.cpp, vLLM, LM Studio, OpenRouter Streaming, OpenRouter Reasoning Tokens.
- `docs/DECISIONS.md`: 구현 중 결정이 있으면 추가한다.
