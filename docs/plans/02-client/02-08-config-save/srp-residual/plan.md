# 02-8 잔여 — 설정 오류·저장 모듈 분리: 기획 (구 02-9)

- 흡수: 2026-11-06. 옛 번호 02-9. 안내: [README.md](README.md).
- 검증 명령과 공통 DoD는 [상위 문서](../../../02-client.md)의 '세부 작업 공통 기준'을 따른다.
- 상세 계획과 인수 조건: [tasks.md](tasks.md), [spec.md](spec.md)
- 상태: done (잔여 단위 리뷰 PASS 2026-11-06)

## 목적

02-8 개발 규칙 점검에서 남은 SRP(Single Responsibility Principle, 한 파일 = 한 책임) 부채를 갚는다. 설정 오류 타입 하나가 읽기·검증·저장 변경 이유를 함께 갖고, 설정 파일 쓰기와 자격 파일 쓰기가 각각 항목 추가·이름 바꾸기·삭제 세 책임을 한 파일에 담고 있다. 이를 책임별 파일로 나누고, 두 쪽에 중복으로 적힌 이름 규칙 매핑과 키 칸 지우기 루프를 한 자리로 모은다.

## 리서치 및 설계 근거

- 조사 생략: 순수 구조 정리(리팩터링) 작업이라 학술·기술 근거가 설계 결론을 바꾸지 않는다(하네스 3절 Research Gate 생략 조건).
- 근거 측정(2026-11-06, grep `ConfigError::`): 제품 파일 9개, 테스트 파일 11개, 크레이트 루트 파일 4개 — 20개 파일 180건 참조.
- 근거 규칙: 하네스 10절 파일 단위 단일 책임, dev-principles 2절 SRP·3절 DRY(중복 금지)·6절 이름과 함수, harness/PROJECT_RULES.md 개발 규칙.
- 직전 리뷰 기록: `docs/reviews/02/08-config-save/devrules-audit-review-2.md` (PASS). 남은 SRP 논의는 이 작업 문서로 이관한다.

## 범위 (In)

`crates/snap-core/src/config/` 제품 파일(`config.rs`, `error.rs`, `save_config.rs`, `save_credentials.rs`, `save_cst.rs`, `save_read.rs`, `save_values.rs`, `save_error.rs`, `save_atomic.rs`, `save_store.rs`)과 `config/tests/` 전체, `ConfigError`를 참조하는 호출부 20개 파일.

## 범위 밖 (Out)

- 02-2·02-8 요구 동작 변경(오류 문구·담는 값·저장 순서): 현행 그대로 유지.
- 공개 이름 유지: `ConfigError`를 쓰는 호출부는 부분 열거형(한 덩어리별 세부 열거형)을 감싸는 변형으로 재수출해 이름을 그대로 둔다.
- 이미 분리된 공용 자리(`validate::RESERVED_EXTRA_KEYS`, `checked_name`, `extra_json`).
- 파일 삭제, 새 의존성, 범위 밖 모듈.

## 설계 요약

- 작업 1: `ConfigError`를 읽기·검증·저장 세 부분 열거형으로 나누고 `ConfigError`가 감싼다.
- 작업 2: 두 저장 쪽에 중복된 이름 규칙 매핑과 키 칸 지우기 루프를 한 자리로 모은다.
- 작업 3: `save_config.rs`·`save_credentials.rs`에서 이름 바꾸기·삭제 책임을 별도 파일로 뺀다.
- 작업 4: 문서 정리와 작업 단위 전체 리뷰.
