# 02-8 설정 저장: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. 주석을 보존하며 JSONC를 고치는 방식을 정한다. 필요한 크레이트는 라이선스를 확인하고, 파일 원자적 교체와 주석 보존 편집 자료를 조사해 `docs/RESEARCH.md`에 적는다. 확인: `docs/DECISIONS.md`에 결정과 라이선스 확인이 있고 RESEARCH.md에 실제로 열어 본 링크만 있다. (완료 2026-11-05: `jsonc-parser` 0.34.0 `cst` 채택, D40 승인. 작업 1 리뷰는 D40 승인으로 갈음)
- [x] 2. JSONC 쓰기를 만든다. 엔드포인트 추가·수정·삭제와 모델 옵션 쓰기, 키 칸은 쓰지 않음, 주석과 손대지 않은 항목 보존, 파일이 없으면 새로 만듦. 확인: 주석이 든 파일에 저장한 뒤 주석·다른 항목이 그대로이고 다시 읽으면 검증을 통과하는 단위 테스트(AC 1). (완료 2026-11-06: `config/save_config.rs` `save_endpoint` 추가·수정, 단위 테스트 10개. 리뷰 PASS 기록 `docs/reviews/02-client/02-8-config-save-task2-review-1.md`. 엔드포인트 삭제 저장은 3·4에서 키 연동으로 함께)
- [x] 3. 자격 파일 쓰기를 만든다. `api_keys` 아래 키 추가·바꾸기, 주석과 다른 항목 보존, 파일이 없으면 새로 만듦, 이름 변경 시 키 이름 변경, 삭제 시 키 삭제. 확인: 쓴 뒤 02-1 해석으로 다시 읽어 같은 값이고 주석과 다른 항목이 그대로임을 확인하는 단위 테스트(AC 2, 3). (완료 2026-11-06: `config/save_credentials.rs` `save_key`·`rename_key`·`remove_key`와 저장 연동까지, 테스트 16개. 리뷰 PASS `docs/reviews/02-client/02-8-config-save-task3-review-1.md`. 작업 4 리뷰 3차 PASS `docs/reviews/02-client/02-8-config-save-task4-review-3.md`에서 자격 쪽 검증 정합성까지 함께 확인)
- [x] 4. 안전한 쓰기를 만든다. 임시 파일에 쓰고 바꾸기, 쓰기 전 02-2 검증, 두 파일을 함께 바꿀 때의 순서. 확인: 교체 실패를 일으켜 기존 파일이 그대로이고 임시 파일이 남지 않음을 확인하는 단위 테스트, 검증 실패 입력에 파일이 바뀌지 않음을 확인하는 단위 테스트(AC 4). (완료 2026-11-06: `config/save_atomic.rs`·`config/save_store.rs`, 검증 → 자격 교체 → 설정 교체 순서(D41), Windows 열림 실측 시험 포함 테스트 14개)
- [x] 5. 모든 저장 오류의 Display·Debug에 키 값과 base_url이 없음을 모아 확인한다(AC 5). (완료 2026-11-06: `config/save_error.rs` `SaveIo`·`MissingEndpoint`, 가림 확인 테스트 추가, `ConfigError` 변형 30개 전수 확인)
- [x] 6. `docs/RESEARCH.md`에 1단계 조사 항목의 적용 위치를 적고, 구현 중 결정을 `docs/DECISIONS.md`에 적는다. (완료 2026-11-06: RESEARCH 작업 02 절 02-8 항목 2줄의 적용 위치를 실제 코드 경로(`config/save_config.rs`·`save_credentials.rs`·`save_atomic.rs`·`save_store.rs`·`save_error.rs`)로 갱신하고 상세 문서 `research/02-client.md` 02-8 절에 확인 결과를 더했다. DECISIONS D40·D41은 상태 recorded 유지, Windows 공유 위반 실측과 검증 통합 검토 결과를 정본 D40·D41의 「구현 후 확인 메모」에 남겼다. 예시 파일 `agent-snap.example.jsonc`은 02-2 검증 시험(`config/tests/redaction.rs`의 `example_file_passes_validation`·`example_file_contains_placeholders_only`)과 저장 시험(`config/tests/save_config.rs`의 `textVerbosity`·`budgetTokens` `extra`)으로 형식이 확인됨.)

## 잔여: 오류·저장 모듈 분리 (구 02-9, 2026-11-06 흡수)

상세 정본: [srp-residual/](srp-residual/README.md) (`plan` · `spec` · `tasks`). 개발 규칙 점검 뒤 남은 SRP·DRY 부채를 02-8 저장 구현의 잔여로 갚는다. 동작(저장 AC)은 바꾸지 않는다.

- [x] 7. `ConfigError`를 읽기·검증·저장 부분 열거형으로 나눈다. (완료: `docs/reviews/02-client/02-9-error-split-task1-review-1.md` — 파일명은 구 번호, 내용은 02-8 잔여)
- [x] 8. 이름 규칙 매핑·키 칸 지우기 중복을 한 자리로 모은다. (완료: `...-task2-review-1.md`)
- [x] 9. 이름 바꾸기·삭제·검증 책임을 파일로 뺀다. (완료: `...-task3-review-3.md`)
- [x] 10. `config/`를 `error/`·`save/`·`read/`로, 저장 테스트를 `tests/save/`로 나눈다. (완료: `...-task5-review-1.md` · `...-task5-review-2.md`)
- [x] 11. 잔여 문서·의존 정리(D42·RESEARCH)와 **02-8 잔여 단위** 리뷰 PASS. (완료: `docs/reviews/02-client/02-8-config-save-srp-residual-review-1.md`. 상세: [srp-residual/tasks.md](srp-residual/tasks.md) 작업 4). 작업 02 전체 리뷰는 [02-9](../02-9-client-review/plan.md).

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트. 임시 폴더에서 파일을 쓴다.
- 주입할 것: 파일 교체 실패(시험용), 설정 폴더 경로.
- 테스트 파일은 관심사별로 나눈다(설정 파일 쓰기, 자격 파일 쓰기, 안전한 쓰기, 오류 가리기). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [x] [spec.md](spec.md)의 AC 5개를 모두 충족한다. (AC 1~2: 작업 2 리뷰 PASS, AC 2·3: 작업 3 리뷰 PASS, AC 4: 작업 4 리뷰 3차 PASS, AC 5: 작업 5 리뷰 PASS. 기록: `docs/reviews/02-client/02-8-config-save-task2-review-1.md`~`task5-review-2.md`)
- [x] 잔여(SRP) [srp-residual/spec.md](srp-residual/spec.md) AC와 잔여 단위 리뷰 PASS(작업 11).

## 문서 영향
- `docs/RESEARCH.md` 작업 02 절: 파일 원자적 교체, JSONC 주석 보존 편집.
- `docs/DECISIONS.md`: 크레이트 라이선스 확인 결과와 구현 중 결정(쓰기 순서 포함).
