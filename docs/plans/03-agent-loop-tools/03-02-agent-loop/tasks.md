# 03-2 에이전트 루프: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. `Tool` 트레이트를 정의한다(이름·설명·인자 형식·실행 한 자리, 도구 구현 하나당 파일 하나). 확인: 등록 목록이 OpenAI `tools` 배열로 직렬화되는 단위 테스트. (T1 완료: `tools/tool.rs` 골격, `agent/registry.rs` 변환)
- [x] 2. 루프 골격을 만든다(전송 → `tool_calls` 처리 → 결과 붙임 → 반복 → `stop`). 02의 클라이언트를 재사용하고 재시도는 02-6 한 층에만 둔다(D13). 확인: 모의 서버로 `tool_calls`→결과→`stop` 왕복 테스트(spec AC 1). (T1 완료: `agent/runner.rs`, `tests/agent_loop/reply.rs`)
- [x] 3. 반복 상한(기본 32·설정 가능)과 종료 조건을 나눈다(상한 초과, `stop`, 도구 호출 불가 모델). 확인: 상한 초과와 모델 미지원 오류 테스트(spec AC 2, AC 4). (T1 완료: `tests/agent_loop/turns.rs`·`reply.rs`)
- [x] 4. 도구 인자 JSON 해석·복구·실패 시 짧은 오류를 넣는다(2609.07370). 확인: 깨진 JSON·미등록 이름·형식 어긋남을 도구 결과 오류로 돌려주는 단위 테스트(spec AC 3). (T1 완료: `agent/args.rs`·`executor.rs`, 접두는 `ToolOutcome::error` 단일 정본)
- [x] 5. `tool_call` 낮은 온도 프로파일을 모델 `extra`에 합치는 경로를 만든다(D15 매핑). 확인: 요청 본문에 프로파일 값이 드는지 모의 서버로 확인(spec AC 7). (T1 완료: `agent/profile.rs`, `tests/agent_loop/prompt_extra.rs`)
- [x] 6. 도구 결과 4,000자 잘라 넣기(`…[truncated]`)를 만든다. 확인: 긴 출력이 잘림을 확인하는 단위 테스트(spec AC 5). (T1 완료: `agent/truncate.rs`)
- [x] 7. `agent/system_prompt.txt` 앞머리 조립을 만든다(AGENTS.md 합치기는 03-5이 채운다). 확인: 프롬프트 조립 순서가 테스트로 고정(spec AC 6 앞부분). (T1 완료: `agent/prompt.rs`)
- [x] 8. `docs/RESEARCH.md` 작업 03 절의 적용 위치를 갱신하고, 구현 중 결정은 `docs/DECISIONS.md`에 남긴다. (T1 완료: RESEARCH.md 03절 `03-2 루프 골격 구현 완료`로 갱신, DECISIONS.md D47 델타 누적 교정 추가)

## 테스트 계획
- 종류: 모의 서버 통합 테스트(02-5·02-6 패턴 재사용)와 네트워크 없는 단위 테스트.
- 도구 결과는 가짜(문자열 고정)로 두고, 시간 값은 짧게 주고 경과 시간은 하한·상한으로만 확인한다.
- 테스트 파일은 `crates/snap-core/tests/`에 `agent_loop/`로 두되, 같은 관심사 파일이 둘 이상일 때만 하위 폴더(D25).

## DoD
- [x] [spec.md](spec.md) AC 7개를 모두 충족한다.
- [x] 검증 명령 3종 통과.
- [x] `docs/reviews/03/`에 리뷰 PASS 기록.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절 적용 위치.
- `docs/DECISIONS.md`(구현 중 결정 시).
## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-02-agent-loop/T1-agent-loop.md` | T1 루프 골격 | 완료 |
| `docs/tasks/03/03-02-agent-loop/T2-fallback-point.md` | T2 폴백 지점 | 계획 |
