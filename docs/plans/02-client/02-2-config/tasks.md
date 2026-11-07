# 02-2 설정 읽기와 검증: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [x] 1. `docs/DECISIONS.md`의 D18을 쓴다(jsonc-parser 0.34). 확인: 결정 기록이 02-1에서 이미 기록됐음을 확인.
- [x] 2. 설정 항목과 기본값을 만든다(언어, `endpoints` 배열과 각 엔드포인트 아래 `models` 배열, 샘플링 기본값, `http` 시간·재시도 값). base_url은 비밀 값 래퍼로 담고, 주석이 든 JSONC를 읽는다. 생략한 `endpoints`/`models`는 `[]`로 정규화한다. 확인: 정상 설정과 빠진 값·빈 목록에 대해 읽은 값과 기본값을 확인하는 단위 테스트(AC 1).
- [x] 3. 검증을 만든다. 엔드포인트 이름 중복(trim 후 정확히 같은 문자열만 중복, 대소문자·`-`·`_`는 서로 다른 이름)과 trim 후 빈 name, trim 후 `/`가 든 엔드포인트·모델 이름(D20), 같은 엔드포인트 안 모델 이름 중복·빈 name/id, `kind` 값, `reasoning` 값, `base_url` 값(D19: scheme `http`/`https` 한정, 빈 문자열·공백·끝 슬래시 금지; `/v1` 경로는 검사하지 않음), `http` 키(명시된 7개만, 옛 toml 이름 호환 없음)와 시간 값(0 초과 유한)·`backoff_min <= backoff_max`(D21), 샘플링 범위와 모르는 샘플링 항목(D14). 다른 엔드포인트의 같은 모델 별칭은 허용한다. 확인: 경우마다 거부되고 오류에 이름만 있음을 확인하는 단위 테스트(AC 2).
- [x] 4. 거부 칸을 처리한다. `extra`의 예약 키(`model`, `messages`, `stream`, `tools`), 엔드포인트의 `api_key`·`api_key_env`(값은 읽지도 출력하지도 않음), 최상위 옛 `models`, 모델의 `endpoint` 칸을 거부한다. 그 밖 알 수 없는 칸은 무시한다. 확인: 예약 키는 모델 이름과 키 이름만, `api_key`·`api_key_env`는 엔드포인트 이름과 안내만, 옛 `models`·모델 `endpoint`는 안내·이름만 밝히고 값이 없는 단위 테스트(AC 2, 4).
- [x] 5. 키 정하기를 만든다. 자격 파일([02-1](../02-1-secrets/tasks.md))에 trim한 엔드포인트 이름이 있으면 그 키, 없거나 빈 문자열이거나 파일이 없으면 키 없음. 확인: 각 경우를 확인하는 단위 테스트(AC 3).
- [x] 6. JSONC 문법 오류를 줄 번호만 알리는 오류로 바꾼다(공용 모듈 `crates/snap-core/src/jsonc.rs`가 자격 파일과 설정 파일에 함께 쓰인다, D21). 확인: 문법이 틀린 입력의 오류에 줄 번호만 있고 값이 없는 단위 테스트(AC 5).
- [x] 7. 사용자 설정 폴더 위치 계산과, 폴더 경로를 받아 읽는 기능을 나눠 만든다. 환경 변수 `AGENT_SNAP_CONFIG_DIR`이 우선하고(D19), 없으면 운영체제 관례를 따른다. 확인: 환경 정보를 주입해 `AGENT_SNAP_CONFIG_DIR` 우선순위와 Windows·XDG 경로를 확인하고, 임시 폴더로 읽기를 확인하는 단위 테스트(AC 1).
- [x] 8. 설정의 Debug와 모든 설정 오류의 Display·Debug에 base_url 호스트와 키가 없음을 확인한다(AC 5의 Debug·Display 가림 확인).
- [x] 9. 저장소 루트 `agent-snap.example.jsonc`에 자리표시자만으로 예시 파일을 만들고(중첩 `endpoints[].models` 형태, `/v1`은 안내만), 옛 형식 예시 파일 `agent-snap.example.toml`을 삭제하고(D10), `.gitignore`의 `agent-snap.toml`을 `agent-snap.jsonc`로 바꾼다. 확인: 예시 파일을 읽는 단위 테스트(AC 5), 예시 파일이 `.jsonc` 1개로 통일됨.
- [x] 10. `docs/RESEARCH.md` 작업 02 절에 기록한 llama.cpp 서버 README와 Ollama OpenAI compatibility·Authentication 항목의 02-2 적용 위치(base_url 검증, 키 정하기)를 갱신한다.

## 02-2b 후속 보강(D29 `context_window`)
- [ ] 11. `ModelConfig`에 `context_window`(`u64`)를 읽고, 생략 시 65536으로 정규화하고 1 이상 정수만 받아들이며 0·음수·소수·문자열·범위 초과는 모델 이름만 밝힌 설정 오류로 거부한다. 확인: 읽기·거부·값 비가리기 단위 테스트, 02-8 저장 왕복 테스트. 구현 완료(2026-11-07), 검증 3종 통과. 02-9 전체 리뷰 재리뷰 남아 있음(여기 `[x]`는 재리뷰 PASS 뒤에 올린다).

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 운영체제 관례에 필요한 환경 정보, 키 파일에서 읽은 이름별 값. 설정 읽기 시험은 임시 폴더를 쓴다.
- 테스트 파일은 관심사별로 나눈다(읽기와 기본값, 검증, 키 정하기, 오류 가리기, 폴더 위치). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [x] [spec.md](spec.md)의 AC 5개를 모두 충족한다.

## 문서 영향
- `agent-snap.example.jsonc`: 새로 만든다(자리표시자만, 중첩 모델).
- `agent-snap.example.toml`: 삭제한다(D10 다시 볼 점, `.jsonc` 예시 1개로 통일).
- `.gitignore`: 설정 파일 이름을 바꾼다.
- `docs/RESEARCH.md` 작업 02 절: llama.cpp, Ollama 모델 목록 항목.
- `docs/DECISIONS.md`: 크레이트 라이선스 확인 결과와 구현 중 결정.
