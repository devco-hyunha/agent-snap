# 02-5 클라이언트 트레이트와 스트리밍 전송: 상세 기획

- 상태: planned
- 상위 문서: [02-client.md](../../02-client.md)

## 목적
OpenAI 호환 엔드포인트에 스트리밍 요청을 보내고 [02-3](../02-3-sse/plan.md)으로 복원한 이벤트를 돌려준다. 이 세부 작업에서는 재시도하지 않는다. 에이전트 루프(작업 03)가 트레이트로만 받고, 감시기(작업 05)가 스트림을 중간에 끊을 수 있게 취소를 지원한다.

## 범위(In)
- LLM 클라이언트 트레이트: 요청과 취소 토큰을 받아 이벤트 스트림을 연다.
- OpenAI 호환 구현
  - 요청 본문: 모델 id, 메시지, 도구, 샘플링. `tool_choice`, `n`, `stream_options`는 보내지 않는다.
  - 모델의 `reasoning`(`off`, `low`, `medium`, `high`)을 엔드포인트 `kind`에 맞는 요청 필드로 바꿔 싣고, 모델의 `extra`를 요청 본문에 그대로 합친다(D15). 서버별 변환 규칙은 첫 단계에서 서버 문서로 확인해 확정한다.
  - 인증: 자격 파일에서 정해진 키가 있으면 `Authorization: Bearer`, 없으면 헤더를 보내지 않는다.
  - 시간 초과: 연결, 헤더 대기, 스트림 유휴.
  - 취소 시 연결 정리.
  - 오류 가리기.
  - 사용량과 시각(요청 시작·첫 토큰·마지막 토큰).
- 모의 서버: 응답 시점과 연결 종료를 테스트가 제어하고 관찰할 수 있어야 한다.

## 비범위(Out)
- 재시도: [02-6](../02-6-retry-apply/plan.md).
- 모델 이름으로 클라이언트 만들기와 모델 목록 조회: [02-7](../02-7-model-select/plan.md).
- 스트림 해석 규칙: 02-3.
- `stream_options`, 비스트리밍 요청.
- `reasoning`과 `extra`의 검증(값 범위, 예약 키 거부): [02-2](../02-2-config/plan.md). 이 세부 작업은 검증을 통과한 값을 싣기만 한다.
- 토큰마다 도착 시각 기록과 지표 계산(필요하면 작업 06).
- 평균 지표 계산(요청별 원시 시각만 돌려준다).

## 선행 세부 작업
[02-4 재시도 판단](../02-4-retry/plan.md). 작업 순서상 앞선다. 이 세부 작업은 [02-1](../02-1-secrets/plan.md)의 비밀 값과 [02-3](../02-3-sse/plan.md)의 복원을 쓴다.

## 참고 자료
- RESEARCH.md 작업 02 절
  - Ollama OpenAI compatibility: `tool_choice`, `n` 미지원. 요청에 보내지 않는다. `stream_options`는 서버마다 지원이 달라 보내지 않는다. `reasoning_effort`로 생각을 조절하므로 `kind`가 `ollama`일 때의 `reasoning` 변환에 쓴다. 키를 무시하는 로컬 서버가 있다.
  - 서버별 문서(llama.cpp, vLLM, LM Studio, OpenRouter, OpenAI): `reasoning` 변환 규칙을 구현 첫 단계에서 확인해 RESEARCH.md에 기록한다.
  - OpenRouter Streaming: 연결을 끊으면 지원하는 공급자에서 생성과 과금이 멈춘다. 이것이 취소 시 연결 정리의 근거다.
  - tokio-util CancellationToken: 복제본이 취소 상태를 공유하고 `cancelled()`는 cancel-safe.
  - reqwest ClientBuilder: `connect_timeout`만 라이브러리에 맡기고 헤더 대기와 유휴는 따로 잰다. `timeout`과 `read_timeout`은 두 구간을 구분하지 못해 쓰지 않는다.
  - vLLM OpenAI 호환 서버: 키가 있으면 항상 Bearer 헤더를 보낸다.
  - DistServe, Etalon, When Is the Same Model Not the Same Service?: 요청별 원시 시각(요청 시작·첫 토큰·마지막 토큰)을 평균 없이 돌려준다. 서버 구조 내용은 적용하지 않는다.
- DECISIONS D13: 재시도는 한 층에서만(이 세부 작업은 재시도하지 않음).
- DECISIONS D15: `reasoning` 변환과 `extra` 병합.
- 확인된 사실: 오류에 base_url과 키가 들어가면 안 된다. 헤더 대기와 스트림 유휴를 나눠야 한다.

## 리스크
- 서버마다 `reasoning` 필드 이름이 달라 문서로 확인하기 전에는 변환 규칙을 정할 수 없다. 변환할 수 없는 `kind`가 나오면 처리를 정한다(D15 다시 볼 점).
- 시간에 의존하는 모의 서버 테스트가 흔들릴 수 있다. 시험용 시간 값을 짧게 주고 경과 시간은 넉넉한 하한·상한으로만 확인한다.
- 서버가 오류 본문에 키를 되돌려 줄 수 있다. 본문을 가린 뒤 요약하고 길이를 제한해 담는다.
- 연결 실패 오류는 HTTP 라이브러리가 주소를 오류 문자열에 넣을 수 있다. 주소를 담지 않는 오류로 바꿔 보고한다.
- 취소 후 서버 쪽 연결이 닫히는지 모의 서버가 관찰할 수 있어야 한다.
- 새 외부 크레이트(HTTP 클라이언트, 취소 토큰, 모의 서버 등)는 처음 쓰는 이 세부 작업에서 라이선스를 확인하고 `docs/DECISIONS.md`에 적는다.
