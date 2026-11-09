# 03-3 파일 도구·glob·grep·list_dir: 상세 기획

- 상태: planned
- 상위 문서: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md)
- 순서 정정(2026-11-08): 트레이트 설계는 [03-1](../03-01-tool-trait/plan.md)로 분리됐고, 이 문서는 파일 도구 구현(03-3)만 다룬다. 순서는 03-1 → 03-2(루프) → 03-3(이 문서).
## 범위(In)
- read_file: 텍스트 읽기, 줄 수·크기 제한, 긴 파일은 `offset`·`limit` 구간 읽기, 길면 잘라 표시.
- write_file: 새로 쓰기·덮어쓰기. 작업 폴더 안 쓰기는 승인 대상이 아니라 표시로 알린다(읽기·쓰기·편집 승인 기준 = D49).
- edit_file: 정확한 문자열 교체. 일치가 하나가 아니면 이유(다중 일치·불일치)를 담아 실패.
- list_dir·glob·grep: 목록·이름 패턴 검색·본문 검색(ripgrep 경로 우선, 없으면 내장 검색). 검색 결과에 비밀 파일·작업 폴더 밖 경로를 내보내지 않는다(D17).
- 작업 폴더 경계: `..`·절대 경로·심볼릭 링크로 작업 폴더 밖 쓰기·읽기·목록을 거부한다.
- 파일 도구 공통: 비밀 파일(`agent-snap.jsonc`·`credentials.jsonc`) 읽기·쓰기·목록·검색 결과 차단(D17).

## 비범위(Out)
- `run_command`: [03-6](../03-06-run-command/plan.md).
- AGENTS.md 읽기: [03-5](../03-05-agents-md/plan.md).
- 바이너리 파일 편집·인코딩 변환: 범위 밖(필요하면 04 이후 별도).
- LSP·심볼 도구: 03-13 배치(D43 2차 개정, 2026-11-09). 04 배치에서 03-13으로 이관(D33·D43·D45).

## 선행 세부 작업
[03-1](../03-01-tool-trait/plan.md)의 `Tool` 트레이트, [03-2](../03-02-agent-loop/plan.md)의 루프.

## 참고 자료
- SWE-agent 2405.15793(편집 최소·실패 시 짧은 오류), 2604.27296(검색-치환 형식 실패율 → 06에서 잰다).
- OpenCode grep/glob(ripgrep) 설계.
- DECISIONS D17(비밀 파일 차단), D9(작업 폴더 경계·격리 보류), D22(승인), D49(읽기 상한·쓰기 승인·구간 읽기, 2026-11-09).
- 번역 전문: `.local/research/03/arxiv-2405.15793.kr.md`, `.local/research/03/arxiv-2604.27296.kr.md`.

## 리스크
- 심볼릭 링크가 작업 폴더 안 파일을 가리키는 경우와 밖을 가리키는 경우를 나누는 기준이 애매하다. 착수 시 "작업 폴더 안에서 생성된 링크만 허용" 같은 명시 규칙으로 정한다.
- grep 출력이 길면 03-2의 잘라 넣기 규칙을 공유한다(DRY).

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 읽기·쓰기·편집 도구 | read_file(줄·크기 제한·잘림·`offset`·`limit` 구간 읽기), write_file(덮어쓰기 표시, D49 승인 아님), edit_file(정확한 문자열 교체·다중 일치 실패), 경로 정규화·경계·비밀 검사 기본 자리(path_guard 1차) 포함 | run_command(03-6)·AGENTS.md(03-5)·바이너리 편집 | 파일 도구 3개·경계 검사 1차·단위 테스트 | cargo test(경계·다중 일치·잘림·구간), cargo clippy, cargo fmt | 03-1·03-2 |
| T2 목록·검색 도구 | list_dir·glob·grep(ripgrep 우선·내장 대체), 결과에 비밀 파일·밖 경로 제외, list_dir·glob 숨김 기본 제외 | `.gitignore` 규칙 해석(미적용) | 검색 도구 3개·단위 테스트 | cargo test(고정 제외·비밀 제거·숨김), cargo fmt | T1 |
| T3 경로 경계·비밀 차단 | 심볼릭 링크 거부, `path_guard` 헬퍼 공통 적용과 검사 순서 고정, 비밀 파일 차단 확장(D17) | 링크 기준 임의 확정, run_command 우회 확장(06 한계) | `path_guard` 완성·단위 테스트 | cargo test(경계·비밀 차단) | T1·T2 |

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-03-file-tools/T1-file-write-tools.md` | 읽기·쓰기·편집 | 진행 |
| `docs/tasks/03/03-03-file-tools/T2-list-search-tools.md` | 목록·검색 | 계획 |
| `docs/tasks/03/03-03-file-tools/T3-path-guard.md` | 경로 경계·비밀 차단 | 계획 |
