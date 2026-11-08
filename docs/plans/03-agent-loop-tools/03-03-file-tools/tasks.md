# 03-3 파일 도구·glob·grep·list_dir: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../03-agent-loop-tools.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. 경로 정규화·경계 검사·심볼릭 링크 검사 헬퍼를 만든다(도구마다 호출). 확인: 작업 폴더 밖 경로 거부 단위 테스트(spec AC 2).
- [ ] 2. 비밀 파일 차단 목록을 한 곳에 두고 도구가 공유한다(D17, DRY). 확인: `agent-snap.jsonc`·`credentials.jsonc` 읽기·쓰기·목록·검색 거부 단위 테스트(spec AC 4).
- [ ] 3. `read_file` 도구를 만든다(크기 상한·잘림·바이너리 거부). 확인: 크기·잘림·바이너리 단위 테스트(spec AC 1).
- [ ] 4. `write_file`을 만든다. 덮어쓰기 표시. 확인: 임시 폴더 단위 테스트.
- [ ] 5. `edit_file`을 만든다(일치 1개·일치 없음·다중 일치). 확인: 3종 단위 테스트(spec AC 3).
- [ ] 6. `list_dir`·`glob`을 만든다(경계·비밀 파일 차단 적용). 확인: 단위 테스트.
- [ ] 7. `grep`을 만든다. ripgrep 경로 우선·없으면 내장 검색·결과 200줄·잘림·비밀 파일 차단. 확인: 단위 테스트(spec AC 5·6).
- [ ] 8. `docs/RESEARCH.md` 작업 03 절 적용 위치 갱신.

## 테스트 계획
- 임시 폴더에 파일·링크·비밀 파일을 만들고, 읽기·쓰기·검색을 확인한다.
- 심볼릭 링크는 OS 생성이 가능하면 테스트로 만들고, Windows에서 막히면 링크 검사 코드 단위 테스트로 대체한다.
- 잘림·크기 상한·200줄 제한은 값만 바꿔 짧게 확인한다.

## DoD
- [ ] [spec.md](spec.md) AC 7개 충족.
- [ ] 검증 명령 3종 통과.
- [ ] `docs/reviews/03/` 리뷰 PASS.

## 문서 영향
- `docs/RESEARCH.md` 작업 03 절.
- 필요하면 `docs/DECISIONS.md`(경계 규칙 세부 결정 시).
## 작업 파일
| 파일 | 작업 | 상태 |
| `docs/tasks/03/03-03-file-tools/T1-file-write-tools.md` | T1 읽기·쓰기·편집 | planned |
| `docs/tasks/03/03-03-file-tools/T2-list-search-tools.md` | T2 목록·검색 | planned |
| `docs/tasks/03/03-03-file-tools/T3-path-guard.md` | T3 경계·비밀 차단 | planned |
