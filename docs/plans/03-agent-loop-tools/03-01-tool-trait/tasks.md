# 03-1 `Tool` 트레이트 설계: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.
- 03-1은 설계 확정(문서) 작업이다. 코드 구현은 03-2·03-3·03-6에서 하고, 그 서명을 이 문서가 고정한다.

## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-01-tool-trait/T1-tool-trait-design.md` | T1 설계 확정 | 완료 |

## 구현 단계
- [x] 1. `Tool` 트레이트 서명을 spec.md에 고정한다(이름·설명·parameters_schema·executes_writes·call, `ToolContext`, `ToolOutcome`). 확인: spec.md에 서명 블록이 있고 이름 목록이 표로 적혀 있다(AC 1).
- [x] 2. 등록 규칙을 정한다. 트레이트 객체 → OpenAI `tools` 자리 변환 예제 한 개(grep 예제)를 spec.md에 넣는다. 확인: spec.md 예제가 03-agent-loop-tools.md OpenAI 절의 `type:"function"`·`function.name`·`function.parameters`와 같은 자리인지 대조(AC 2).
- [x] 3. 결과 규격을 고정한다. `Done`/`ApprovalRequired`/`Error` 3자리와 "오류: 이유 한 줄" 규칙을 spec.md에 적는다. `set.rs`의 한 줄 변환과 같은 형식임을 근거로 적는다(AC 1 연장).
- [x] 4. 승인자 자리와 경로 검사 헬퍼 이름을 정한다. `ToolContext`의 승인자 자리, `ApprovalRequired` 흐름, 헬퍼 이름 `tools::path_guard`를 spec.md에 적는다(AC 3). 03-6(승인자)·03-3(경계)·03-8(쓰기 게이트)이 이 자리를 쓴다는 문장도 함께 적는다.
- [x] 5. 서명 규칙 문서를 AGENTS.md 규칙과 맞춘다. "도구는 구현 하나당 파일 하나" 선언을 spec.md에 명시한다(PROJECT_RULES 개발 규칙 근거).
- [x] 6. 참조 문서를 갱신한다. 03-2 plan의 선행이 03-1을 가리키는지, 03-3·03-8 plan이 이 서명을 참조하는지 확인한다. 어긋나면 그 문서의 참조 줄만 고친다.
- [x] 7. `docs/RESEARCH.md` 작업 03 절 "도구 인터페이스와 작은 모델 논문" 행의 적용 위치에 `tools/tool.rs`가 03-1 설계로 확정됨을 한 줄로 갱신한다.

## 테스트 계획
- 대상: 문서 AC 1~3. 코드 테스트는 없다(설계 확정 단계).
- 확인 방법: spec.md의 서명 블록·예제·자리 표를 리뷰 서브가 03-agent-loop-tools.md OpenAI 절·D9·D17·D22·D46과 대조해 판정.
- 검증 명령은 대상 코드가 없어 `cargo fmt --check`·`cargo test`는 통과 대상 아님(스킵 사유를 리뷰 기록에 적는다).

## DoD
- [x] spec.md AC 3개가 문서로 충족된다.
- [x] 참조 문서(03-2·03-3·03-8 plan) 정합 확인.
- [x] `docs/reviews/03/03-01/`에 설계 리뷰 PASS 기록(`T1-tool-trait-design-review.md`, 2026-11-08 PASS).

## 문서 영향
- `docs/RESEARCH.md` 작업 03 적용 위치 한 줄.
- 03-2·03-3·03-8 plan의 선행·참조 줄(어긋나면).
- `docs/DECISIONS.md`: 서명 확정으로 새 결정이 생기면 남긴다(예정: 없음).
