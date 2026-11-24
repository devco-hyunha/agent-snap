# 01 기반과 개발 규칙

- 상태: 완료 (T1·T2·T3 완료, 5차 리뷰 PASS. 잔여 없음. 커밋 `54d6c3c`, PR #1 병합으로 `develop` 반영 완료)

## 목적
프로젝트 규칙, 개발 규칙, 조사 기록 형식, Cargo 워크스페이스 뼈대, 검사 스크립트를 만든다.

## 범위(In)
- `AGENTS.md`: 하네스 템플릿의 마커 블록을 원문 그대로 둔다. 마커 밖에는 "프로젝트 오버레이"를 두고 `harness/PROJECT_RULES.md`를 가리킨다.
- 프로젝트 규칙
  - 커밋·푸시·병합은 사용자 허락이 필요하다. 커밋 메시지는 한국어로 쓴다. 비밀 값과 `.env`는 커밋하지 않는다.
  - 멈추는 조건(새 권한, 비가역 작업, 보안, 제품 결과 변경, 같은 지적으로 보정 2회 후에도 미달)이 아니면 묻지 않고 추천안으로 진행한다. 그 결정은 `docs/DECISIONS.md`에 결정, 근거, 다시 볼 점으로 기록한다.
  - 라이선스로 Apache-2.0을 적용한다. 소스 파일 헤더 적용 여부는 코드 착수 시 정한다.
  - 이 PC의 하네스 문서는 `harness/`에 있고, 스킬은 `.agents/skills/`와 `.claude/skills/`에 있다.
  - 엔드포인트 주소를 코드와 문서에 넣지 않는다. 키는 사용자 설정 폴더의 `credentials.jsonc`로만 받고 어디에도 값을 남기지 않는다.
  - Hugging Face 모델은 용량·라이선스 목록 승인 후에만 받는다.
  - 에이전트(M1)를 먼저 완성한다. 판정 모델 작업은 M1 이후, 실제 세션 출력으로만 평가한다.
  - 작업마다 논문·자료를 조사해 `docs/RESEARCH.md`에 기록하고, 적용 위치나 미적용 이유를 남긴다. 링크는 직접 열어 실제로 있는지 확인한 것만 적는다.
- 개발 규칙
  - 크레이트: `snap-core`(라이브러리, 모든 로직)와 `snap-cli`(입출력만 맡는 얇은 실행 파일).
  - 모듈은 하나의 책임만 맡는다: config, llm, agent, tools, monitor, session, context, server(04-4에서 추가 예정, HTTP 전용 코드만 다룬다). 도구는 `Tool` 트레이트 구현 하나당 파일 하나.
  - SRP는 액터 기준이다. 파일·모듈·함수는 오직 하나의 액터에 대해서만 책임을 진다. 한 파일에 동작을 쌓지 않고, 한 함수는 100줄을 넘기지 않는다.
  - OS 고유 코드는 `#[cfg]`로 본문에 섞지 않고 OS별 파일로 분리한다. 시험도 같은 책임 기준으로 나눈다. 순수 forwarding·재수출 전용 파일로 과대 분리하지 않는다.
  - 상세 규칙: `harness/PROJECT_RULES.md` 개발 규칙 절과 `dev-principles` leaves(`solid.md`·`separation.md`·`dry.md`·`kiss-yagni.md`)를 따른다.
  - 상수·문자열 배치: 기본값·한도 상수는 상수 모듈 한곳, 화면·도구 결과·승인·오류 문구 텍스트는 로케일 모듈(locale 폴더 구조) 한곳에 모은다. 함수 안에 텍스트를 즉석으로 쓰지 않는다. 언어 전환·문구 변경이 한 파일 수정으로 끝나게 한다.
  - 오류: 라이브러리는 `thiserror`, 실행 파일만 `anyhow`. 테스트 밖 `unwrap`/`expect` 금지.
  - 비밀 값은 Debug 출력에서 가리는 래퍼 타입으로 다룬다.
  - 공개 항목에는 문서 주석을 단다. 주석은 한국어로 쓴다.
  - 모든 기능에 단위 테스트를 두고, 네트워크 경로는 모의 서버 통합 테스트로 확인한다.
  - dev-principles 스킬을 따르며, 지시와 리뷰에는 반드시 경로를 명시해 파일을 읽게 한다: `.agents/skills/dev-principles/SKILL.md` (Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`).
  - AGENTS.md 마커 블록 원본: 에이전트 하네스 저장소 `https://github.com/devco-hyunha/agent-harness`(public)의 템플릿을 원본으로 쓴다. 로컬 경로를 문서에 적지 않는다.
- 문서 뼈대: `CLAUDE.md`(`@AGENTS.md` 한 줄), `.cursor/rules/agent-harness.mdc`(`alwaysApply: true`), `docs/INDEX.md`, `docs/DECISIONS.md`, `docs/RESEARCH.md`(기록 형식: 제목, 링크, 확인 날짜, 핵심, 적용 위치 또는 미적용 이유, 관련 작업), `.gitignore`, 짧은 `README.md`, `LICENSE`(Apache-2.0).
- CI 설정: `.github/workflows/ci.yml`에 게이트 3종(`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`)을 `ubuntu-latest`·`windows-latest` 매트릭스로 돌린다. PR의 초록 여부는 develop 병합 기준이다.
- 린트 설정: 루트에 `clippy.toml`(금지 남용 lint 목록), `deny.toml`(의존성 라이선스·위험 등급), `rust-toolchain.toml`(버전 고정), `Cargo.toml`의 `[lints]`(workspace 공통 lint)를 둔다.
- 설정 파일 자리: `prompts/`(시스템 프롬프트·도구 설명·감시 규칙을 코드와 분리해 두고 git 이력으로 관리)와 `rules/`(규칙 감시 규칙 파일) 폴더를 뼈대에 만든다. 내용물은 01에서는 자리와 README 한 줄만 둔다.
- Cargo 워크스페이스 뼈대: 위 모듈의 빈 파일과 컴파일되는 최소 코드. 상수·로케일 모듈은 파일 하나로 두지 않고 폴더로 만든다.
  - 상수 자리: `crates/snap-core/src/consts/` 폴더(`mod.rs` + `defaults.rs`(설정 기본값) + `limits.rs`(한도·상한)). 책임별로 파일을 나눈다.
  - 텍스트 자리(locale 모듈): `crates/snap-core/src/locale/` 폴더에 `mod.rs`(문구 키 목록·locale 선택·기본 locale 지정)를 두고, 언어마다 `ko/`·`en/` 하위 폴더를 둔다. 각 locale 폴더는 도메인별 파일(`config.rs`, `tools.rs`, `approval.rs`, `error.rs`)로 나눈다. 문구 키는 locale 공통(함수 이름·상수 이름)이고, locale 파일은 같은 키에 자기 언어 문구만 채운다. 언어 전환은 `mod.rs`의 locale 선택 한 줄과 설정 키 한 칸으로 끝난다.
  - 01에서는 `locale/`(mod.rs)·`locale/ko/`·`locale/en/` 자리와 키 목록 뼈대만 만들고, 실제 문구 채우기는 그 모듈을 쓰는 계획(02~)에서 한다.

## 근거(선행 계획 잔여)
없음. 이 계획이 최선행이고, 잔여가 생기면 이 계획 `tasks.md`의 `## 잔여` 절에 남긴다.

## 비범위(Out)
기능 구현.

## 반영할 연구
- 시스템 프롬프트, 도구 설명, 감시 규칙은 코드와 분리한 설정 파일(`prompts/`, `rules/`)로 두고 git 이력으로 관리한다(Harness² 2610.14224).
- `docs/RESEARCH.md`는 기획 단계 조사를 가리키고, 작업별 적용 위치를 이어서 기록한다.

## 작업계획
각 행의 상세 작업 파일은 `docs/tasks/01/foundation/T<n>-<짧은 이름>.md`에 둔다. 파일 이름은 부모 폴더 이름을 되풀하지 않는다. 착수 시 만든다.

| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |
|---|---|---|---|---|
| T1 규칙·문서 뼈대 | AGENTS.md 마커(하네스 저장소 템플릿 원본), CLAUDE.md, `.cursor/rules/agent-harness.mdc`, docs 뼈대(PLAN·INDEX·DECISIONS·RESEARCH), LICENSE, `.gitignore` | 구현 코드 금지 | 규칙 파일·문서 파일 | `git check-ignore` 확인 |
| T2 CI·린트·Cargo 뼈대·폴더 자리 | 워크스페이스·크레이트·모듈 자리와 최소 코드, `[lints]`·`rust-toolchain.toml`·`clippy.toml`·`deny.toml`, `.github/workflows/ci.yml` 매트릭스, `prompts/`·`rules/`·`consts/`·`locale/` 폴더 자리 | 기능 구현·실제 문구·규칙 내용 채우기 금지 (자리와 README 한 줄만) | Cargo 뼈대·CI 설정·린트 설정·폴더 자리 | 게이트 3종, CI 초록 |
| T3 deny.toml 키 교정 (T2 리뷰 보정) | `deny.toml`의 `[bans]`에서 cargo-deny가 모르는 키를 고친다 | `deny.toml` 외 파일 수정·기능 구현 금지 | `deny.toml` 교정 | `cargo deny check` 포함 게이트 4종 |

상세 작업 파일: `docs/tasks/01/foundation/T1-rules-docs.md`, `T2-ci-lint-folders.md`, `T3-deny-toml.md`

## AC
1. AGENTS.md의 마커 블록이 하네스 저장소 템플릿 원본과 같고, 마커 밖에 프로젝트 오버레이가 있다.
2. DECISIONS.md에 결정 인덱스, RESEARCH.md에 기록 형식이 있다.
3. `git check-ignore`로 `.env`, `snap.jsonc`, `target/x`, `models/x`, `logs/x`, `docs/tasks/`, `docs/reviews/`가 무시된다.
4. CLAUDE.md는 `@AGENTS.md` 한 줄이고, `.cursor/rules/agent-harness.mdc`에는 `alwaysApply: true`가 있다.
5. `LICENSE`(Apache-2.0)가 저장소 최상위에 있다.
6. Cargo 워크스페이스 뼈대가 컴파일되고 검사 명령을 통과한다. CI 매트릭스가 초록이다.
7. `consts/`·`locale/`가 폴더(여러 파일)로 잡혀 있고, 함수에 즉석 텍스트를 박지 않는 기준이 AGENTS.md와 PROJECT_RULES에 있다.
8. 린트 설정(clippy.toml·deny.toml·rust-toolchain.toml·`[lints]`)과 CI 워크플로우가 저장소에 있다.
9. `prompts/`·`rules/` 자리가 자리만 갖추고 있다(내용물 채우기 금지).

## 리스크
규칙 문서가 커지면 문서 부담이 된다. AGENTS.md는 짧게 유지한다.

## DoD
AC 충족, 검증 통과, 리뷰 PASS.
