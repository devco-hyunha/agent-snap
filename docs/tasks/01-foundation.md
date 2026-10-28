# 01 기반과 개발 규칙

- 상태: done

## 목적
프로젝트 규칙, 개발 규칙, 조사 기록 형식, Cargo 워크스페이스 뼈대, 검사 스크립트를 만든다.

## 범위(In)
- `AGENTS.md`: 템플릿(`C:\workspace\agent-harness\templates\AGENTS.harness.md`)의 마커 블록을 원문 그대로 넣는다. 마커 밖에는 "프로젝트 규칙"과 "개발 규칙" 절을 둔다.
- 프로젝트 규칙
  - 작업 문서 하나의 리뷰가 PASS하면 자동으로 커밋한다. 푸시와 병합은 사용자 허락이 필요하다.
  - 커밋 메시지는 한국어로 쓴다. 비밀 값과 `.env`는 커밋하지 않는다.
  - 멈추는 조건(새 권한, 비가역 작업, 보안, 제품 결과 변경, 같은 지적으로 보정 2회 후에도 미달)이 아니면 묻지 않고 추천안으로 진행한다. 그 결정은 `docs/DECISIONS.md`에 결정, 근거, 다시 볼 점으로 기록한다.
  - 서브에이전트 모델: 단순 작업은 haiku, 구현은 sonnet, 어려운 설계는 opus.
  - 이 PC의 하네스 문서는 `~/.claude/harness/GLOBAL_HARNESS.md`, 스킬은 `~/.claude/skills/`에 있다.
  - 엔드포인트 주소를 코드와 문서에 넣지 않는다. 키는 환경 변수로만 받고 어디에도 값을 남기지 않는다.
  - Hugging Face 모델은 용량·라이선스 목록 승인 후에만 받는다.
  - 에이전트(M1)를 먼저 완성한다. 판정 모델 작업은 M1 이후, 실제 세션 출력으로만 평가한다.
  - 작업마다 논문·자료를 조사해 `docs/RESEARCH.md`에 기록하고, 적용 위치나 미적용 이유를 남긴다. 링크는 직접 열어 실제로 있는지 확인한 것만 적는다.
- 개발 규칙
  - 크레이트: `snap-core`(라이브러리, 모든 로직)와 `snap-cli`(입출력만 맡는 얇은 실행 파일).
  - 모듈은 하나의 책임만 맡는다: config, llm, agent, tools, monitor, session, context. 도구는 `Tool` 트레이트 구현 하나당 파일 하나.
  - 소스 파일은 300줄을 목표로 하고, 500줄을 넘으면 나눈다(테스트 모듈 제외). 함수는 60줄 이하를 목표로 한다.
  - 오류: 라이브러리는 `thiserror`, 실행 파일만 `anyhow`. 테스트 밖 `unwrap`/`expect` 금지.
  - 비밀 값은 Debug 출력에서 가리는 래퍼 타입으로 다룬다.
  - 공개 항목에는 문서 주석을 단다. 주석은 한국어로 쓴다.
  - 모든 기능에 단위 테스트를 두고, 네트워크 경로는 모의 서버 통합 테스트로 확인한다.
  - `dev-principles` 스킬을 따른다.
- `scripts/check.sh`: 단계마다 echo로 표시하고, 로그를 `logs/check-<시각>.log`에 남긴다. 검사 항목은 `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, 소스 파일 줄 수 검사(500줄 초과 시 실패)다.
- `CLAUDE.md`(`@AGENTS.md` 한 줄), `.cursor/rules/agent-snap.mdc`(`alwaysApply: true`, AGENTS.md 안내와 핵심 규칙 요약), `docs/INDEX.md`, `docs/DECISIONS.md`(PLAN.md의 D1~D6), `docs/RESEARCH.md`(기록 형식: 제목, 링크, 확인 날짜, 핵심, 적용 위치 또는 미적용 이유, 관련 작업), `.gitignore`(`.env`, `*.env`, `agent-snap.toml`, `target/`, `models/`, `logs/`, `.agent-snap/`), 짧은 `README.md`.
- Cargo 워크스페이스 뼈대: 위 모듈의 빈 파일과 컴파일되는 최소 코드.

## 비범위(Out)
기능 구현.

## 반영할 연구
- 시스템 프롬프트, 도구 설명, 감시 규칙은 코드와 분리한 설정 파일(`prompts/`, `rules/`)로 두고 git 이력으로 관리한다(Harness² 2610.14224).
- `docs/RESEARCH.md`는 `docs/research/2026-10-28-planning.md`를 가리키고, 작업별 적용 위치를 이어서 기록한다.

## AC
1. AGENTS.md의 마커 블록이 템플릿과 바이트 단위로 같고, 마커 밖에 프로젝트 규칙과 개발 규칙 절이 있다.
2. `scripts/check.sh`가 뼈대에서 통과한다. 501줄짜리 임시 소스 파일을 넣으면 실패한다(시험 후 임시 파일은 지운다).
3. DECISIONS.md에 D1~D6, RESEARCH.md에 기록 형식이 있다.
4. `git check-ignore`로 `.env`, `agent-snap.toml`, `target/x`, `models/x`, `logs/x`가 무시된다.
5. CLAUDE.md는 `@AGENTS.md` 한 줄이고, `.cursor/rules/agent-snap.mdc`에는 `alwaysApply: true`가 있다.

## 테스트 계획
마커 블록 diff, `scripts/check.sh` 실행 로그, `git check-ignore -v`.

## 리스크
규칙 문서가 커지면 이전처럼 문서 부담이 된다. AGENTS.md는 짧게 유지한다.

## DoD
AC 충족, 검증 통과, 리뷰 PASS.
