# 02-8 모델 `context_window` 읽기·검증·예시: 상세 기획

- 상태: 완료 (기획 승인 2026-11-28. 02-7까지 반영된 `feature/client`에서 `feature/client-context-window`를 나눠 착수했다. T1~T4 구현·개별 리뷰 PASS(T1 r1 문서 정합 보정, T2 r2 PASS, T3 r2 PASS, T4 구조 점검 위반 0건). 작업 단위 전체 리뷰 r1 미달(minor 4건 문서·주석 정합) → 보정 → r2 PASS(`docs/reviews/02/02-8-unit-r2.md`). PR #14로 `feature/client` 병합 2026-11-28)
- 상위 문서: [02-client.md](../../02-client.md)
- 선행 세부 작업: [02-7 모델 선택과 모델 목록 조회](../07/plan.md) 완료(PR #13으로 `feature/client` 병합 2026-11-28).
- 선행 결정: 20번(`context_window` 규칙)을 이 세부 작업에서 기록한다. 백업 결정 D29가 같은 규칙이고, 현재 저장소 번호로 새로 적는다.

## 목적

모델 항목의 선택 칸 `context_window`(토큰 수)를 읽고 검증한다. 클라이언트가 맞출 맥락 한도를 사용자가 설정할 수 있게 하기 위함이다. 생략하면 65536(64K)으로 정규화하고, 1 이상 정수만 받아들인다. 이 값은 요청 본문에 싣지 않는다(02-5 결정 15번과 같은 규칙). 한도를 실제로 쓰는 일은 작업 04가 맡는다.

## 리서치 및 설계 근거

- 백업 결정 D29(`C:\Workspace\agent-snap-backup\docs\DECISIONS.md`의 "D29 모델별 맥락 창"): 선택 `context_window`(토큰, 1 이상 정수), 생략 시 65536, `max_tokens`와 별도, 요청 본문에 이 키로 실지 않음. 같은 규칙을 현재 저장소 결정 20번으로 새로 적는다.
- 백업 조사(`C:\Workspace\agent-snap-backup\docs\RESEARCH.md`): OpenRouter 모델 목록이 `context_length`를 주지만 자동 채우기는 하지 않는다. 서버 값으로 채우는 일은 미적용(D29).
- 사용자 결정(2026-11-28): 예시 파일에는 값 대신 주석으로만 안내한다. 상한은 두지 않는다(1 이상 정수만). 범위는 읽기·검증·예시 파일로 좁히고 파일 저장은 02-9가 맡는다.
- OpenRouter `context_length` 자동 채우기 미적용 근거: 설정이 정본이고, 서버 목록 값은 02-7에서 읽기만 한다. 목록 값으로 설정을 자동 채우면 사용자가 손으로 정한 한도가 사라진다.

## 범위(In)

- `ModelConfig`에 `context_window: u64`를 더하고, 생략 시 65536 기본값 창구를 둔다.
- 값 검증: 1 이상 정수만 받아들인다. 0·음수·소수·문자열·숫자가 아닌 값은 모델 이름만 밝힌 설정 오류로 거부한다.
- 기본값 상수(65536)는 `consts/` 상수 모듈에 둔다.
- 거부 문구(한국어·영어)를 `locale/ko/validate.rs`·`locale/en/validate.rs`에 둔다.
- 예시 파일 `snap.example.jsonc`에 `context_window` 안내 주석을 추가한다(값은 넣지 않는다).

## 비범위(Out)

- 설정 파일에 값을 쓰는 저장: 02-9.
- 서버 목록의 `context_length`로 설정을 자동 채우기: 미적용(D29).
- 한도를 세어 맥락을 자르는 일: 작업 04.
- 요청 본문에 `context_window` 키를 싣기: 미적용(02-5 결정 15번 규칙).
- 상한 상수 도입: 미적용(사용자 결정 2026-11-28).

## 설계 요약

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 | `ModelConfig.context_window` 읽기·기본값 창구·상수(65536) | 요청 본문 키로 싣기, 타입 변경 외 칸 추가 | 타입 칸·기본값 창구·상수와 읽기 시험 | 게이트 3종 |
| T2 | 값 검증(1 이상 정수만, 0·음수·소수·문자열 거부)·거부 오류·locale 문구 | 입력 값 노출, 상한 도입 | 검증 함수·오류 종류·locale 상수와 거부 시험 | 게이트 3종 |
| T3 | 예시 파일 `snap.example.jsonc` 주석 안내 | 예시 파일에 값 넣기, 빈 파일(`agent-snap.example.jsonc`) 손대기 | 예시 파일 주석과 예시 검증 시험 | 게이트 3종 |
| T4 | 미커밋 작업 파일 전수 구조 점검 표 | 코드 변경, 파일 삭제, 커밋·푸시 | 구조 점검 표 산출물 | 주석 전용 경량 게이트 |

## 리스크

- 검증이 `serde_json` 값 단계(`check_values`)와 타입 단계(`serde_json::from_value`)에 나뉜다. 검증 단계에서 거부하면 타입 단계까지 가지 않으므로, 거부 경로를 시험으로 확인해야 한다.
- 기본값을 타입 기본값 창구로 두면 검증 단계의 `Value`에는 값이 없다. 검증은 값이 있을 때만 범위를 보므로 생략 정규화는 타입 단계에서 일어난다. 이 두 지점을 시험으로 나눠 확인한다.

## AC

1. Given 모델에 `context_window`이 없을 때, When 설정을 읽으면, Then 값은 65536이다.
2. Given 모델에 `context_window: 32768`일 때, When 설정을 읽으면, Then 값은 32768이다.
3. Given `context_window`이 0·음수·소수·문자열일 때, When 설정을 읽으면, Then 설정 오류가 나고 오류에 모델 이름만 담기고 입력 값은 담지 않는다.
4. Given `context_window`이 `max_tokens`와 함께 있을 때, When 설정을 읽으면, Then 두 값이 별도 칸으로 유지된다.
5. Given 예시 파일 `snap.example.jsonc`을 읽을 때, When 예시 검증 시험을 돌리면, Then 예시가 설정 검증과 타입 해석을 통과하고 `context_window` 안내 주석이 있다.

## 시험 계획

- 기존 설정 시험 트리(`crates/snap-core/tests/config/`)에 값을 주입하는 읽기·거부 시험을 추가한다. 네트워크·파일 쓰기 없이 문자열로 만든다.
- 예시 파일 시험(`crates/snap-core/tests/example/`)은 기존 검증·자리표시자 시험을 재사용한다.
- 시간에 의존하는 시험은 없다.

## 문서 영향

- `docs/DECISIONS.md`: 20번 결정(`context_window` 규칙) 추가.
- `docs/RESEARCH.md` 02 절: `context_window` 항목 적용 위치 갱신.
- `docs/plans/02-client.md`·`docs/PLAN.md`: 02-8 상태 갱신.
- `docs/plans/02-client/02/plan.md`: 비범위 표기 정합(02-8로 옮긴 상태).
- `snap.example.jsonc`: 주석 추가.

## DoD

[spec.md](spec.md)의 AC 5개를 모두 충족하고, 게이트 3종(Windows·WSL)을 통과하고, 조사 적용 위치가 갱신되고, 작업 단위 전체 리뷰가 PASS한 상태.
