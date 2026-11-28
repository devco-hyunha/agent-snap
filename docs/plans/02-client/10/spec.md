# 02-10 작업 02 전체 리뷰: 명세

- 상태: 진행 (기획 승인 2026-11-28)
- 상위 문서: [plan.md](plan.md)

## 입력과 출력

- 입력: 02-1~02-9 계획 문서(`docs/plans/02-client/01`~`09`), 02 계획 범위 코드(`crates/snap-core/src/config/`, `crates/snap-core/src/jsonc/`, `crates/snap-core/src/llm/`, `crates/snap-core/src/locale/`, `crates/snap-core/src/consts/`), 시험 트리, `harness/HARNESS.md`, `harness/PROJECT_RULES.md`.
- 출력: 정합 감사 표·규칙 감사 표·코드 리뷰 표·AC 확인 표, 잔여 이관 목록, `docs/reviews/02/` 기록, 02 기획 상태 확정.
- 실패: 확인되지 않은 계획 항목, 규칙 위반, 코드 지적, AC 미충족 → 심각도와 대상 경로로 기록.

## 동작 규칙

- 감사는 읽기 전용으로 합니다. 코드를 고치지 않고 문제만 기록합니다.
- 대상의 참값은 02 계획 범위로 정하고, 표에 없는 항목은 미점검으로 봅니다(PROJECT_RULES 구조 점검 규칙).
- 리뷰 좌표는 파일 경로·식별자·원문으로 적고 줄 번호는 쓰지 않습니다(HARNESS 7절).
- 판정은 지적 0건만 PASS, minor만 있어도 미달입니다(HARNESS 7절).
- 결함은 02-10 잔여로 이관하고 같은 계획 안에서 보정한 뒤 재리뷰합니다(HARNESS 9절).

## 감사 항목

| 항목 | 확인 방법 |
|---|---|
| 계획-코드 정합 | 계획 문서의 범위·AC 항목마다 코드 경로와 시험 경로를 적고, 없으면 차이로 기록 |
| 계획 간 경계 | 02-2 검증 규칙과 02-9 저장 검증 재사용, 02-5 전송과 02-6 재시도, 02-7 선택 키와 02-8 `context_window` 규칙 |
| 하네스 준수 | 1절 언어·eli5, 5절 승인·안전 6원칙, 7절 리뷰 규격 |
| 규칙 준수 | PROJECT_RULES 개발 규칙(상수·locale 자리, 책임별 파일 분리, 100줄 트리거 기록, OS 분기 분리, 타입과 로직 분리, 시험 관심사 분리) |
| 코드 리뷰 | review-harness 체크리스트 7항목, dev-principles 2~11절 |
| 비밀 가리기 | 키 값·`base_url` 호스트가 Display·Debug·오류에 없는지 시험 경로로 확인 |
| 게이트·CI | 게이트 4종 Windows·WSL, PR CI 매트릭스(`ubuntu-latest`·`windows-latest`) |

## AC

plan.md의 AC 6개를 그대로 따른다.

## 금지

- 코드·시험·계획 정본 수정(T5 잔여 보정 전).
- 새 기능 추가, 새 계획 번호 분리, 푸시·병합.
- 집계 명령(grep·줄수·개수)만으로 감사를 끝내기.
- 감사 표를 채우지 않은 채 판정하기.
