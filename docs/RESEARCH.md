# 조사 인덱스

- 이 파일은 조사 인덱스입니다. 항목당 핵심 한 줄, 적용 위치(파일·함수) 또는 미적용 이유, 상태, 상세 링크를 적습니다.
- 상세는 `research/<기획 순번>-<주제>.md`에 둡니다. 링크는 직접 열어 확인한 것만 적습니다.

| 기획 | 주제 | 핵심 | 적용 위치 / 미적용 이유 | 상태 |
|---|---|---|---|---|
| 01 | 코딩 에이전트 조사 (Codex CLI·OpenCode·Orca) | 승인 요청을 동작 단위로 쪼개고, 도구·승인 설계의 참고 자료로 쓴다 | `docs/research/01-coding-agent-survey.md`. 01은 조사 파일 자리만 반영. 승인 동작 단위·모델 목록 자동 발견은 02-7·03 범위(계획 문서)에 반영 | 반영 |
| 02 | llama.cpp 서버 README | 모델 `id`가 파일 경로일 수 있어 `name`과 `id`를 따로 둔다 | 02-2 적용: `crates/snap-core/src/config/types.rs`의 `ModelConfig`(name·id 분리), `crates/snap-core/src/config/validate/names.rs`(id trim 검증), `crates/snap-core/tests/validation/values/`(R4 분리 후: endpoint·sampling·http). 서버별 요청 필드 변환은 02-5 예정 | 반영 |
| 02 | Ollama OpenAI 호환 문서 | 로컬 서버는 키를 받지 않으므로 키 없는 엔드포인트를 허용한다 | 02-2 적용: `crates/snap-core/src/config/keys.rs`(자격 파일에 이름 없으면 키 없음), `crates/snap-core/tests/keys/resolve.rs`. `kind` 허용값 6종은 `crates/snap-core/src/consts/limits.rs`. 예시 안내는 `snap.example.jsonc` | 반영 |
