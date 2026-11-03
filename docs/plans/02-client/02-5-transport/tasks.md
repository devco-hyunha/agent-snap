# 02-5 클라이언트 트레이트와 스트리밍 전송: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. 서버별 `reasoning` 변환 규칙을 문서로 확인해 plan/spec/D15/RESEARCH에 고정한다(`kind` 여섯 가지, `off`→`none`, OpenRouter 중첩, `lmstudio`는 공통 필드 없음·`extra`만). 확인: RESEARCH·[상세](../../../research/02-client.md#kind별-reasoning-요청-필드-변환-02-5)에 표와 링크가 있다. (기획 반영 완료 2026-11-03; 코드 반영은 4단계)
- [ ] 1-1. 새 외부 크레이트(HTTP 클라이언트, 취소 토큰, 모의 서버용)의 라이선스를 확인하고 `docs/DECISIONS.md`에 적는다. 확인: 결정 기록이 있다.
- [ ] 2. 모의 서버를 만든다. 응답 시점(헤더 지연, 본문 멈춤)과 연결 종료를 테스트가 제어하고 관찰한다. 요청 헤더와 본문을 기록한다. 확인: 모의 서버 자체를 시험하는 통합 테스트.
- [ ] 3. LLM 클라이언트 트레이트를 만든다(요청과 취소 토큰을 받아 이벤트 스트림을 연다). 상위 로직이 트레이트로만 받도록 HTTP 라이브러리는 전송 구현 안에서만 쓴다. 타입 이름은 기존 코드 기준(`SecretValue`, `HttpSettings`). 확인: 코드 품질 리뷰의 경계 확인.
- [ ] 4. 요청 본문과 인증 헤더를 만든다. 보내지 않을 매개변수 제외, spec 변환 표대로 `reasoning` 변환·`extra` 병합, 키 있으면 Bearer, 없으면 Authorization 헤더 없음. 확인: `kind`별 본문(특히 OpenRouter 중첩·`lmstudio` 무필드) 통합 테스트(AC 2).
- [ ] 5. 응답 스트림을 [02-3](../02-3-sse/tasks.md) 복원으로 연결한다. 디코더 실패 후 재호출 계약(D28)을 코드·테스트로 고정한다. 확인: 02-3 AC 1과 같은 형식 + 실패 후 `Ok` 오해 없음(AC 1).
- [ ] 6. 시간 초과를 만든다. 연결은 라이브러리에 맡기고, 헤더 대기와 스트림 유휴를 따로 잰다. 확인: 헤더가 유휴보다 늦게 오는 성공, 헤더 미도착, 본문 멈춤을 각각 시험하는 통합 테스트(AC 3).
- [ ] 7. 취소를 만든다. 스트림 도중 취소하면 취소 오류를 내고 연결을 정리한다. 확인: 취소 후 500ms 안에 오류와 스트림 종료, 3초 안에 서버 쪽 연결 닫힘을 확인하는 통합 테스트(AC 4).
- [ ] 8. 오류 종류와 가리기를 만든다. (a) 헤더 전 연결을 `PreHeaderConnectKind` 여섯 값으로 구분(AC 6, D26). (b) 서버 오류 본문·연결 실패에 어림 가리기 후 **알고 있는 키·호스트로 한 번 더** 가린다(D28). 확인: 세부 종류·Display·Debug 통합 테스트(AC 2·6).
- [ ] 9. 사용량과 시각을 돌려준다. 확인: 서버가 사용량을 주는 경우와 안 주는 경우, 시각 순서 통합 테스트(AC 5).
- [ ] 10. `docs/RESEARCH.md` 작업 02 절 항목에 02-5 **코드** 적용 위치를 갱신한다(변환 표 항목 포함).

## 테스트 계획
- 종류: 로컬 모의 서버 통합 테스트.
- 주입할 것: 취소 토큰, 시험용 시간 값(짧게), 모의 서버의 응답 시점과 연결 종료 제어, 알고 있는 키·호스트 문자열.
- 시간에 의존하는 테스트는 경과 시간을 넉넉한 하한·상한으로만 확인한다.
- 테스트 파일은 관심사별로 나눈다(복원·디코더 계약, 요청 본문·`kind`별 reasoning, 인증과 비밀 값·가리기, 연결 오류 세부, 시간 초과, 취소, 사용량과 시각). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [ ] [spec.md](spec.md)의 AC 6개를 모두 충족한다.

## 문서 영향
- `docs/RESEARCH.md` · `docs/research/02-client.md`: `kind`별 reasoning 변환(확정), Ollama, OpenRouter Streaming/Reasoning, tokio-util, reqwest, vLLM, DistServe, Etalon, Same Model….
- `docs/DECISIONS.md`: D15 변환 표 확정, 새 크레이트 라이선스(구현 시), D26·D28은 본 세부 작업에서 이행.
