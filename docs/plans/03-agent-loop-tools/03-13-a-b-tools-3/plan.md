# 03-13 (a)(b) 배치 이관 3차: 심볼 도구·훅 연결 지점·MCP 연결

- 상태: planned
- 상위 문서: [../03-agent-loop-tools.md](../03-agent-loop-tools.md), 선행: 03-12
- 근거: [기능 목록](../../../research/agent-feature-inventory-2026-11-07.md) (a) LSP 심볼, (i) 훅, (j) MCP, DECISIONS D43 2차 개정(2026-11-09)·D33. 03-13 범위는 D33 후보 7종 전체(2026-11-09 사용자 확정)다.

## 목적
Serena 동작 참고 기능 7종(심볼 찾기·파일 심볼 개요·참조·선언·심볼 단위 교체·이름 바꾸기·파일 진단)을 네이티브 `Tool`로 03에 전부 넣는다. 훅 연결 지점, MCP 연결 골격을 함께 둔다. 04 확장으로 넘기던 (a)(b)(i)(j) 잔여와 D33 Serena 후보를 03에서 마무리한다.

## 범위(In)
- 심볼 도구 7종(D33 후보 전부, 백엔드는 rust-analyzer 등 LSP 서버를 `Tool` 경로로 감싼다):
  - `find_symbol`: 이름으로 심볼 위치를 찾는다.
  - `file_symbols`: 파일 안 심볼 개요 목록을 낸다.
  - `find_refs`: 심볼의 참조 지점 목록을 낸다.
  - `find_declaration`: 심볼의 선언 위치를 낸다.
  - `replace_symbol`: 심볼 단위로 코드를 교체한다(승인 게이트 통과, 체크포인트(03-12)와 연동).
  - `rename_symbol`: 심볼 이름을 바꾼다(교체와 같은 승인 게이트).
  - `file_diagnostics`: 파일의 진단 목록(컴파일·정적 진단)을 낸다.
- 훅 연결 지점: 05 `Monitor` 트레이트가 훅을 걸 수 있는 지점(호출 전·결과 후 한 줄 훅)을 인터페이스로 남긴다.
- MCP 연결 골격: MCP 서버의 도구 목록을 `Tool` 등록 목록에 붙이는 자리. 등록 시 승인·작업 폴더 경계 규칙을 같은 경로로 적용한다.

## 비범위(Out)
- 훅 스크립트 실행 규칙·규칙 감시: 05.
- MCP 서버 자체 구현·포함: 제품 확장. Serena(MCP)를 붙이지 않고 D33 정본대로 네이티브 `Tool`로만 구현한다(동작 참고만, 의존 금지).
- Serena 비범위 항목 유지(D33): JetBrains·유료 백엔드, Serena REPL, 대시보드·온보딩·Serena 메모리 도구, 외부 프로젝트 질의 MCP.

## AC
1. 심볼 조회 4종(`find_symbol`·`file_symbols`·`find_refs`·`find_declaration`)이 설정에 등록한 백엔드에서 결과를 반환하는 것을 테스트로 확인한다.
2. 심볼 교체·이름 바꾸기(`replace_symbol`·`rename_symbol`)가 승인 게이트를 지나고, 거부 시 코드가 바뀌지 않음을 테스트로 확인한다.
3. `file_diagnostics`가 파일 진단 목록을 반환하는 것을 테스트로 확인한다.
4. 훅 연결 지점(인터페이스)이 루프의 왕복마다 기록되는 것을 단위 테스트로 확인한다.
5. MCP 등록 도구가 `Tool` 목록에 붙고 같은 승인 규칙을 지나는 것을 확인한다.
6. 검증 명령 3종 통과.

## 리스크
- LSP 백엔드 기동·수명(Windows): 03-7 프로세스 트리 규칙과 겹친다. 기동 전 확인 게이트를 둔다.
- MCP 도구 이름이 등록 목록과 겹치면 03-2의 중복 이름 검사를 지난다.
- 심볼 교체·이름 바꾸기는 파일 편집 도구(03-3)보다 범위가 넓다. 승인 게이트·체크포인트(03-12)·작업 폴더 경계를 같은 경로로 지킨다.
- Serena의 이름 바꾸기는 심볼 참조 전부를 고치는 동작이다. 참조 목록(`find_refs`)을 먼저 만들고 사람이 확인하는 순서를 권고한다.

## 작업계획

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 | 선행 작업 |
|---|---|---|---|---|---|
| T1 조회·진단 골격 | `find_symbol`·`file_symbols`·`find_refs`·`find_declaration`·`file_diagnostics`, 훅 인터페이스, MCP 등록 골격 | 훅 실행 규칙·MCP 서버 구현, Serena 결합 | 도구 모듈·인터페이스·테스트 | cargo test, cargo clippy, cargo fmt | 03-12 |
| T2 교체·이름 바꾸기 | `replace_symbol`·`rename_symbol` (승인 게이트·체크포인트 연동) | Serena REPL·JetBrains 백엔드 | 도구 모듈·테스트 | cargo test, cargo clippy, cargo fmt | 03-13 T1 |

## DoD
AC 충족, 검증 명령 통과, 리뷰 PASS(`docs/reviews/03/`).

## 작업 파일

| 파일 | 작업 | 상태 |
|---|---|---|
| `docs/tasks/03/03-13-a-b-tools-3/T1-symbol-hooks-mcp.md` | 조회·진단·훅·MCP 골격 | 계획 |
| `docs/tasks/03/03-13-a-b-tools-3/T2-symbol-edit-rename.md` | 교체·이름 바꾸기 | 계획 |
