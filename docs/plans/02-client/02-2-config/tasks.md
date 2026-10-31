# 02-2 설정 읽기와 검증: 작업 계획

- 상위 문서: [plan.md](plan.md), [spec.md](spec.md)
- 검증 명령과 공통 DoD는 [상위 문서](../../02-client.md)의 '세부 작업 공통 기준'을 따른다.

## 구현 단계
- [ ] 1. `docs/DECISIONS.md`의 D18을 쓴다(jsonc-parser 0.34). 확인: 결정 기록이 02-1에서 이미 기록됐음을 확인.
- [ ] 2. 설정 항목과 기본값을 만든다(언어, 엔드포인트, 모델, 샘플링 기본값, 시간·재시도 값). base_url은 비밀 값 래퍼로 담고, 주석이 든 JSONC를 읽는다. 확인: 정상 설정과 빠진 값에 대해 읽은 값과 기본값을 확인하는 단위 테스트(AC 1).
- [ ] 3. 검증을 만든다. 엔드포인트 이름 중복(정확히 같은 문자열만 중복, 대소문자·`-`·`_`는 서로 다른 이름)과 빈 이름, 모델 이름 중복, 없는 엔드포인트 참조, `kind` 값, `reasoning` 값, `base_url` 값(D19: scheme `http`/`https` 한정, 빈 문자열·공백·끝 슬래시 금지), 샘플링 범위와 모르는 샘플링 항목(D14). 확인: 경우마다 거부되고 오류에 이름만 있음을 확인하는 단위 테스트(AC 2).
- [ ] 4. `extra`의 예약 키(`model`, `messages`, `stream`, `tools`)와 엔드포인트의 `api_key` 칸을 거부한다. 확인: 예약 키는 모델 이름과 키 이름만, `api_key`는 엔드포인트 이름과 안내만 밝히고 값이 없는 단위 테스트(AC 2, 4).
- [ ] 5. 키 정하기를 만든다. 자격 파일([02-1](../02-1-secrets/tasks.md))에 엔드포인트 이름이 있으면 그 키, 없거나 빈 문자열이거나 파일이 없으면 키 없음. 확인: 각 경우를 확인하는 단위 테스트(AC 3).
- [ ] 6. JSONC 문법 오류를 줄 번호만 알리는 오류로 바꾼다. 확인: 문법이 틀린 입력의 오류에 줄 번호만 있고 값이 없는 단위 테스트(AC 5).
- [ ] 7. 사용자 설정 폴더 위치 계산과, 폴더 경로를 받아 읽는 기능을 나눠 만든다. 환경 변수 `AGENT_SNAP_CONFIG_DIR`이 우선하고(D19), 없으면 운영체제 관례를 따른다. 확인: 환경 정보를 주입해 `AGENT_SNAP_CONFIG_DIR` 우선순위와 Windows·XDG 경로를 확인하고, 임시 폴더로 읽기를 확인하는 단위 테스트(AC 1).
- [ ] 8. 설정의 Debug와 모든 설정 오류의 Display·Debug에 base_url 호스트와 키가 없음을 확인한다(AC 5의 Debug·Display 가림 확인).
- [ ] 9. 저장소 루트 `agent-snap.example.jsonc`에 자리표시자만으로 예시 파일을 만들고, 옛 형식 예시 파일 `agent-snap.example.toml`을 삭제하고(D10), `.gitignore`의 `agent-snap.toml`을 `agent-snap.jsonc`로 바꾼다. 확인: 예시 파일을 읽는 단위 테스트(AC 5), 예시 파일이 `.jsonc` 1개로 통일됨.
- [ ] 10. `docs/RESEARCH.md` 작업 02 절에 기록한 llama.cpp 서버 README와 Ollama OpenAI compatibility·Authentication 항목의 02-2 적용 위치(base_url 검증, 키 정하기)를 갱신한다.

## 테스트 계획
- 종류: 네트워크 없는 단위 테스트.
- 주입할 것: 운영체제 관례에 필요한 환경 정보, 키 파일에서 읽은 이름별 값. 설정 읽기 시험은 임시 폴더를 쓴다.
- 테스트 파일은 관심사별로 나눈다(읽기와 기본값, 검증, 키 정하기, 오류 가리기, 폴더 위치). 측정은 dev-principles 3절(DRY)·6절(이름과 함수)로 한다(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).

## DoD
- [ ] [spec.md](spec.md)의 AC 5개를 모두 충족한다.

## 문서 영향
- `agent-snap.example.jsonc`: 새로 만든다(자리표시자만).
- `agent-snap.example.toml`: 삭제한다(D10 다시 볼 점, `.jsonc` 예시 1개로 통일).
- `.gitignore`: 설정 파일 이름을 바꾼다.
- `docs/RESEARCH.md` 작업 02 절: llama.cpp, Ollama 모델 목록 항목.
- `docs/DECISIONS.md`: 크레이트 라이선스 확인 결과와 구현 중 결정.
