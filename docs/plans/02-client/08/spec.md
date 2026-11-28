# 02-8 모델 `context_window` 읽기·검증·예시: 명세

- 상태: 완료 (기획 승인 2026-11-28. T1~T4 구현·개별 리뷰 PASS. 작업 단위 전체 리뷰 r1 미달(minor 4건 문서·주석 정합) → 보정 → r2 PASS(`docs/reviews/02/02-8-unit-r2.md`). PR #14로 `feature/client` 병합 2026-11-28)
- 상위 문서: [plan.md](plan.md)
- 선행 결정: 15번(요청 본문 키 규칙), 20번(`context_window` 규칙, 이 작업에서 기록)

## 입력과 출력

- 입력: 설정 파일의 `endpoints[].models[]` 항목 안 `context_window` 칸(있을 수 있음).
- 출력: `ModelConfig.context_window: u64`. 생략 시 65536.
- 실패: 1 이상 정수가 아닌 값(0·음수·소수·문자열·숫자 아닌 것) → 모델 이름만 밝힌 설정 오류.

## 동작 규칙

### 읽기

- 칸이 없으면 65536으로 정규화한다. 기본값은 상수 모듈(`consts/`)의 상수 한곳에서 받고, 기본값 창구 함수로 타입에 연결한다.
- 값은 `max_tokens`와 별도 칸으로 유지한다. 같은 칸을 쓰지 않는다.

### 검증

- 1 이상 정수만 받아들인다. `is_integer_at_least` 헬퍼를 재사용한다(1 이상 기준).
- 0·음수·소수·문자열·객체·배열은 거부한다.
- 거부 오류는 모델 이름만 담는다. 입력 값·엔드포인트 주소·키는 담지 않는다.
- 상한은 두지 않는다.

### 예시 파일

- `snap.example.jsonc`에 `context_window` 안내 주석만 둔다. 값은 넣지 않는다(생략 시 65536임을 주석으로 밝힘).
- 예시 파일은 기존 검증 규칙을 그대로 통과한다.

## 오류 종류와 담는 것

| 오류 | 담는 것 | 담지 않는 것 |
|---|---|---|
| `context_window` 값 거부 | 모델 이름 | 입력 값, 엔드포인트 주소, 키 |

## AC

1. Given 모델에 `context_window`이 없을 때, When 설정을 읽으면, Then 값은 65536이다.
2. Given `context_window: 32768`일 때, When 설정을 읽으면, Then 값은 32768이다.
3. Given `context_window`이 0·음수·소수·문자열일 때, When 설정을 읽으면, Then 설정 오류가 나고 Display·Debug에 모델 이름만 있고 입력 값은 없다.
4. Given `context_window`과 `max_tokens`가 함께 있을 때, When 설정을 읽으면, Then 두 값이 별도 칸으로 유지된다.
5. Given 예시 파일 `snap.example.jsonc`일 때, When 예시 검증 시험을 돌리면, Then 검증·타입 해석을 통과하고 `context_window` 안내 주석이 있다.

## 시험 계획

- `crates/snap-core/tests/config/`에 값 주입 읽기·거부 시험을 추가한다. 설정 문자열을 직접 만들어 쓰고 네트워크·파일 쓰기는 쓰지 않는다.
- 기존 예시 시험(`crates/snap-core/tests/example/`)에 주석 확인을 더한다.
- 거부 시험은 입력 값이 오류에 안 나오는지를 문자열 비교로 확인한다.
