# 02 설정과 스트리밍 클라이언트

- 상태: planned

## 목적
설정을 읽고 OpenAI 호환 엔드포인트와 스트리밍으로 대화하는 클라이언트를 만든다. 이후 감시기가 스트림을 중간에 끊을 수 있어야 한다.

## 범위(In)
- config: `agent-snap.toml`(base_url, model, api_key_env, target_language 기본 `ko`, 샘플링 기본값). `agent-snap.example.toml`에는 자리표시자만 둔다.
- llm: chat completions 스트리밍(SSE 직접 파싱). content, reasoning(`reasoning_content`와 `reasoning` 필드), tool_calls 델타를 복원한다. `:`로 시작하는 주석 줄(`: keep-alive`)은 건너뛴다. 모르는 필드는 원문으로 보존한다.
- 취소 토큰으로 스트림을 즉시 중단할 수 있게 한다. 시간 초과와 재시도(연결 오류만)를 지원한다.

## 비범위(Out)
에이전트 루프와 도구.

## 조사
- 주제: 서버별 OpenAI 호환 스트리밍 차이(llama.cpp server, Ollama, vLLM, LM Studio), reasoning 필드 관례.
- 기록: RESEARCH.md. 논문이 없는 주제는 공식 문서로 대신한다.

## AC
1. 모의 SSE 서버로 content, reasoning, tool_call 델타가 순서대로 복원되고, keep-alive 줄은 무시된다.
2. 키가 Authorization 헤더에만 들어가고, Debug 출력과 로그에는 키 값이 없다(테스트).
3. api_key_env를 지정했는데 변수가 없으면 변수 이름만 밝히고 종료한다. 지정하지 않으면 키 없이 요청한다.
4. 취소하면 스트림이 끊기고 연결이 정리된다(테스트).
5. RESEARCH.md에 이 작업의 조사 항목과 적용 위치가 있다. `scripts/check.sh`를 통과한다.

## 테스트 계획
`scripts/check.sh`. 실제 엔드포인트에 한 번 요청해 스트림 복원을 확인한다(설정이 있을 때만).

## 리스크
서버마다 스트림 형식이 다르다. 모르는 필드를 보존해 나중에 처리한다.

## DoD
AC 충족, 검증 통과, 리뷰 PASS.
