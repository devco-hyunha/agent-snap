# 03-10 단발 run·작업 명세·검증 명령·스모크: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. `agent-snap run` 커맨드를 만든다. 사용자 원문 보존 + 목표·입력·완료 조건 조립. 확인: 명세 조립 단위 테스트(spec AC 1).
- [ ] 2. `--model` 지정·미지정 경로를 만든다. 미지정 시 설정 목록에서 고르고, 입력 불가 시 모델 이름 목록 오류. 확인: 단위 테스트(spec AC 3).
- [ ] 3. 검증 명령 실행 경로를 만든다(03-6 `run_command` 재사용). 종료 코드 0이 아니면 0이 아닌 종료 코드와 이유. 확인: 모의 서버 테스트(spec AC 2).
- [ ] 4. `docs/reports/03-smoke.md`에 과제 3개와 (a) 기능 목록 대조 결과를 기록한다(URL·키 미포함). 확인: 기록 실재(Dod AC 4).
- [ ] 5. `docs/RESEARCH.md` 작업 03 절 적용 위치를 갱신한다.

## 테스트 계획
- 작업 명세 조립·모델 선택·검증 명령 실패는 모의 서버·단위 테스트로 확인한다.
- 스모크는 설정이 있을 때 실제 엔드포인트로 1회 진행하고 기록한다. 출력에 URL과 키를 찍지 않는다.

## DoD
- [ ] [spec.md](spec.md) AC 5개 충족.
- [ ] 검증 명령 3종 통과.
- [ ] `docs/reviews/03/` 리뷰 PASS.

## 문서 영향
- `docs/reports/03-smoke.md` 신규.
- `docs/RESEARCH.md` 작업 03 절.
## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-10-run-once-smoke/T1-run-once-smoke.md` | T1 단발 run·스모크 | planned |
