# 10 문서 규칙 정착과 문서 정리

- 상태: 진행 (세부 1·2·4는 2026-11-07 완료, 세부 3 추가 요구로 `update-harness` 폴더·중첩 기획 순번 구조 도입 완료. 세부 5는 2026-11-09 규칙 추가 완료 → 2026-11-13 PASS 기준 규칙을 확인 단위 규칙으로 교체하고 그 자리 폐기 메모를 남김. 2차 보정: 기획 문서의 "작업 파일" 표 채우기 남아 있음)

## 목적
strata-shift의 문서 관리 방식에서 좋은 부분을 agent-snap에 이식하고, 규칙에 안 맞는 기존 문서를 정리한다. 코드 변경은 없다.

## 리서치 및 설계 근거
strata-shift의 실사용 규칙(`C:\workspace\strata-shift\docs\PROJECT_RULES.md` 2절·3절·8절·9절)을 근거로 삼는다. 근거 파일은 저장소 밖에 있어 이 작업 산출물에 복사하지 않고, 규칙 본문만agent-snap 정본에 쓴다.

| 근거 | 반영할 것 | 이식하지 않을 것 |
|---|---|---|
| PROJECT_RULES.md 2절 문서 규격 | PLAN 순번표의 상태 한 행 갈아쓰기, 라운드 보고 누적 금지, 작업 파일 한 개에 라운드 섹션 하나 | 상태 셀 2,000자 상한(검증 방식 없음) |
| PROJECT_RULES.md 3절 커밋 게이트 | 리뷰 PASS → 상태 갱신 → 커밋 → 다음 착수 순서 고정, 커밋 메시지에 게이트 결과 한 줄 | — |
| PROJECT_RULES.md 8절 검증 재사용 | 게이트 이름·exit 코드·로그 경로·SHA256·시각 기록, 대상 해시 불변이면 재실행 금지 | "게이트 하트비트" 진행 가시성 규격(빌드 시간이 긴 프로젝트용) |
| PROJECT_RULES.md 9절 기획→작업 | 작업 파일 명명 `T<번호>-<짧은 이름>.md`, 기획 문서의 "작업 파일" 표, 상태 둘 동시 갱신 | `.local` 산출물 전용 규칙 |

## 범위(In)
1. `harness/PROJECT_RULES.md`에 `## 문서 규격` 절을 새로 두고 위 표의 반영 항목을 규칙으로 적는다.
2. 기획 문서에 "작업 파일" 표를 만드는 형식을 규칙에 담고, `docs/plans/02-client/` 세부 문서에 표를 채운다(02-2~02-9, 02-8 잔여).
3. `docs/tasks/` 문서 이름을 `docs/tasks/<순번>-<작업명>/T<번호>-<짧은 이름>.md`로 통일한다. 대상: 02-04-retry(4개), 02-08-config-save(7개), 02-09-client-review(3개), 02-09-error-split(5개).
4. 문서에 남은 옛 번호·경로 참조를 정리한다: `srp-residual` 폴더 설명의 02-9 혼선 메모를 유지하되 참조 경로를 새 명명으로 맞춘다.

## 비범위(Out)
- 코드·테스트 변경, CI 변경, git 추적 대상 변경(`docs/plans/`, `docs/tasks/` 추적 방식은 지금대로).
- strata-shift 저장소 자체 수정.
- PLAN.md 순번표의 번호 체계 갈아치기(작업 01~09는 지금 번호를 그대로 쓴다).

## 설계 요약
### 세부 1: 문서 규격 규칙 추가 (harness/PROJECT_RULES.md)
- 범위: `harness/PROJECT_RULES.md`에 `## 문서 규격` 절 1개 추가(20줄 안).
- 산출물: 규칙 4줄(PLAN 상태 한 행 갈아쓰기 / 라운드 보고는 tasks 파일에만 / 작업 파일당 라운드 섹션 하나 / 검증 재사용 기록 규격).
- 금지: 하네스 마커 블록·HARNESS.md 수정, AGENTS.md 본문 되풀이(정본 포인터만).
- 검증: 파일 존재 확인, 마커 블록 무변경 확인(`rg -n "agent-harness:" harness/PROJECT_RULES.md` 결과 없음), 절 순서 확인.

### 세부 2: 작업 파일 표와 명명 통일
- 범위: `docs/plans/02-client/*/tasks.md`에 "작업 파일" 표 추가, `docs/tasks/` 파일 이동·개명.
- 산출물: 표가 채워진 tasks.md, 새 경로 작업 문서.
- 금지: 문서 본문 내용 재생산(기존 내용 이동이 원칙), 결정 문서 본문 복제.
- 검증: `rg --files docs/tasks`로 옛 중첩 이름(`02-4-retry-1-classify.md` 같은 flat 이름) 사라짐 확인, 이동 후 각 파일에 라운드 섹션 하나만 있는지 확인.

### 세부 5: 리뷰 PASS 기준 강화와 notes 로컬 전용 규칙 (2026-11-09 추가, 2026-11-13 규칙 폐기)
- 범위: `harness/PROJECT_RULES.md` 개발 규칙 절에 "PASS는 지적 0건 확인 후 판정" 규칙을 새 줄로 추가(Fast-track 셀프 체크·라운드 로그 기록은 예외로 본문에 명시, `[안전 예외 승인: 5번]`). `docs/notes/` 규칙 줄(38번 줄)에 로컬 전용 문서 성질 예시 보강.
- 산출물: 규칙 1줄(판정) + 38번 줄 예시 문장. 하네스 7절·9절과 겹치는 자리는 두지 않고 정본을 개발 규칙 절 한 곳에 둔다.
- 금지: HARNESS.md·마커 블록 수정, PROJECT_RULES.md 40번 줄(측정 기준)·45번 줄(경로 명시)과 새 규칙(46번 줄)의 합치기 금지(별개 규칙이라 현행 유지).
- 검증: `rg -n "지적 0건" harness/PROJECT_RULES.md`, 마커 블록 무변경 확인.
- 폐기 메모(2026-11-13): "PASS는 지적 0건 확인 후 판정" 규칙(46번 줄)은 하네스 9절(재리뷰 범위) 정본과 확인 단위 표를 정하는 프로젝트 규칙으로 교체됐다. 교체 근거와 정본: `harness/PROJECT_RULES.md` 개발 규칙 46~49행, [docs/decisions/D71-review-verification-unit.md](../decisions/D71-review-verification-unit.md) 결정 6(미측정·미기록 항목은 major, critical은 없다), 작업 문서 [docs/tasks/update-harness/03-harness-review-check-8/T1-rule-sync.md](../tasks/update-harness/03-harness-review-check-8/T1-rule-sync.md). 이유: 지적 0건만을 PASS 기준으로 삼으면 03-6 T5 통합 리뷰에서 발견된 주석 잔여(minor 2건)처럼 "측정 항목이 문서에 존재하는가"를 확인하는 단위가 없어 같은 지적이 3라운드 반복됐다. 그래서 판정 기준을 확인 단위 표(정합 항목 목록) 기준으로 바꿨고, 위 검증 명령 `rg -n "지적 0건" harness/PROJECT_RULES.md`는 성립하지 않으므로 폐기한다. 이 절의 38번 줄 예시 문장 보강(로컬 전용 문서 성질)은 그대로 유효하다.

## AC(인수 조건)
1. `harness/PROJECT_RULES.md`에 `## 문서 규격` 절이 있고, 그 절이 라운드 누적 금지·검증 재사용 기록·작업 파일 표를 담는다.
2. PLAN.md의 각 순번 행에는 상태 한 행만 남고 라운드 문장이 없다.
3. `docs/tasks/` 작업 문서가 `docs/tasks/<순번>-<작업명>/T<번호>-*.md` 형식이 되고, 같은 작업 문서에 라운드 보고 섹션이 하나다.
4. 기획 문서 `tasks.md`의 "작업 파일" 표가 작업 파일 경로와 상태를 담고, 둘이 같은 상태를 맞춘다.
5. 리뷰 확인 단위 규칙(`harness/PROJECT_RULES.md` 개발 규칙 46~49행)과 그 정본([docs/decisions/D71-review-verification-unit.md](../decisions/D71-review-verification-unit.md))이 문서 전체에 반영된다. 이 기획이 넣은 "PASS는 지적 0건" 규칙은 2026-11-13 교체로 폐기됐고, 폐기한 자리에 폐기 표시·근거·이유를 남긴다(세부 5 폐기 메모).
6. 코드·테스트는 그대로다(`git diff`에 코드 파일 변경 없음).

## 테스트 계획
- 코드 없음. 문서 실재 검사: `rg --files docs`, `rg -n "T[0-9]+-" docs/tasks`, `rg -n "작업 파일" docs/plans/02-client`.
- 마커 블록 무변경: `git diff harness/PROJECT_RULES.md`로 마커 안 라인을 훑는다.

## 리스크
- 파일 개명(이동)은 되돌리기 번거로움: `docs/tasks/`는 git 미추적이라 이동 시 원본이 지워진다 → 승인받을 때 원본 복사 백업(`.local/docs-backup/`)을 남긴다.
- 옛 리뷰 기록(`docs/reviews/`, 미추적)의 파일명 참조가 새 명명과 어긋난다 → README 메모로 대응(설계 요약 4번).

## 문서 영향
`harness/PROJECT_RULES.md`, `docs/plans/02-client/**/tasks.md`, `docs/tasks/**`, `docs/INDEX.md`.

## 상태
planned
