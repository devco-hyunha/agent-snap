# FigCraft 어댑터 (API 포인터)

언제: 흐름 검증에서 사용자가 그림·보드를 FigCraft로 쓰기로 골랐을 때만. 본체 칸·STOP·발명 거부를 바꾸지 않는다.

원문(필요 시 읽기, 본체·여기로 복붙 금지):
- `docs/research/planning-harness-sources/midasmatterhorn-artcraft-skills/figcraft/SKILL.md`
- 제작 파이프는 1차 Out 참고만: `docs/research/planning-harness-sources/dnviti-artcraft-skills/skills/figcraft-design/SKILL.md`

쓰는 면: 원문에 적힌 MCP·CLI 표면(`list_commands`, `run_command`, `batch`, `create_node`, `set_properties`, `inspect_document`, `get_node`, `screenshot` 등)과 `board.*`만. 원문에 없는 API를 지어내지 않는다. FigCraft용 PRD→보드 전용 원문이 없으면 공용 글·표·STOP으로 메운다.

금지: 본체에 도구 절차 복붙. ui·컴포넌트 사람 확인 전 디자인 제작 파이프·토큰 확정.
