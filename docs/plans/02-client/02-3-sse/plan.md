# 02-3 SSE 파싱과 델타 복원: 상세 기획

- 상태: planned
- 상위 문서: [02-client.md](../../02-client.md)

## 목적
네트워크 없이 바이트 조각을 이벤트로 바꾸고 하나의 응답으로 합친다. 전송([02-5](../02-5-transport/plan.md))은 이 결과를 그대로 쓰므로, 서버별 스트림 차이를 이 세부 작업에서 모두 흡수한다.

## 범위(In)
- SSE 줄 처리: 줄 끝 CRLF·LF·CR, `:`로 시작하는 주석 줄(`: keep-alive`, `: OPENROUTER PROCESSING`), 여러 `data` 줄 결합, 맨 앞 BOM, 조각 경계에서 잘린 줄과 멀티바이트 문자, `[DONE]`, 빈 줄 없이 끝난 마지막 이벤트(D12).
- 청크 JSON 해석: content, reasoning(`reasoning_content`와 `reasoning` 모두 생각으로), tool_calls를 index별로 누적(병렬 교차 포함).
- 모르는 델타 필드는 원문 JSON으로 보존한다.
- 최상위 `error` 청크는 스트림 오류로 끝낸다.
- usage는 사용량으로 읽는다.

## 비범위(Out)
- HTTP 요청과 응답 헤더 처리: [02-5](../02-5-transport/plan.md).
- 요청 시작·첫 토큰·마지막 토큰 시각 기록: 02-5.
- `<think>` 태그를 content에서 떼어 내는 일(서버 설정으로 해결하고 감시는 작업 05).
- OpenRouter `reasoning_details` 배열의 구조 해석(모르는 필드로 원문 보존만 한다. 필요해질 때 해석한다).
- 토큰마다 도착 시각 기록.

## 선행 세부 작업
[02-2 설정 읽기와 검증](../02-2-config/plan.md). 이 세부 작업은 설정 값을 직접 쓰지 않지만 작업 순서상 앞선다(02-client.md의 선행 표).

## 참고 자료
- RESEARCH.md 작업 02 절
  - WHATWG HTML Standard, Server-sent events: 줄 끝 세 종류, 주석, data 결합, 빈 줄에서 이벤트 내보냄, UTF-8, 맨 앞 BOM 하나 버림. 명세는 파일 끝의 미완 이벤트를 버리지만 D12로 내보낸다.
  - llama.cpp server README: `reasoning_content`.
  - vLLM Reasoning Outputs: 생각 필드가 `reasoning`이고 예전 이름이 `reasoning_content`. 한쪽만 읽으면 빈 값을 읽는다.
  - LM Studio OpenAI Compatibility Endpoints: 별도 처리 없음. reasoning 필드가 오면 두 이름 처리 또는 모르는 필드 보존으로 받는다.
  - OpenRouter Streaming: 주석 줄 무시, 헤더 이후 오류가 HTTP 200 안의 최상위 `error` 청크로 오고 그 뒤 스트림이 끝난다.
  - OpenRouter Reasoning Tokens: `reasoning`은 생각으로, `reasoning_details`는 모르는 필드로 원문 보존.
- DECISIONS D12: 빈 줄 없이 끝난 마지막 이벤트를 내보낸다. 잘린 JSON이면 형식 오류로 끝난다.
- 확인된 사실(02-client.md): keep-alive 주석 줄과 reasoning 필드가 오고, CRLF와 청크 경계에서 잘리는 줄이 있다.

## 리스크
- 서버마다 스트림 형식이 다르다. 모르는 델타 필드를 보존하고 reasoning은 두 이름을 모두 받는다.
- 바이트 조각 경계가 멀티바이트 문자나 CRLF 중간에 걸리면 결과가 달라질 수 있다. 한 바이트씩 나눠 넣는 테스트로 막는다.
- D12에 따라 잘린 마지막 이벤트를 받아들이므로, 잘린 JSON은 형식 오류로 끝나야 한다. 이 경계를 테스트로 확인한다.
