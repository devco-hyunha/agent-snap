# 01 기반과 개발 규칙 — 위임 1

## 목적
docs/tasks/01-foundation.md의 범위(In)를 구현해 AC 1~5를 충족한다.

## 채택한 근거
- 템플릿 파일은 마커 블록(`<!-- agent-harness:begin -->` ~ `<!-- agent-harness:end -->`)만 있고, CLAUDE.md 템플릿은 `@AGENTS.md` 한 줄이다(템플릿 폴더 조사 결과).
- 이 PC: rustc/cargo 1.99.1, MSVC Build Tools 있음(Rust 도구 확인 결과).
- Harness²(arXiv 2610.14224): 시스템 프롬프트·도구 설명·감시 규칙을 코드와 분리한다 → `prompts/`, `rules/` 폴더.

## 작업 범위
01-foundation.md 범위(In) 전체. 단 `prompts/`, `rules/`는 용도를 적은 짧은 README.md만 둔다(내용은 작업 03, 05에서 채움).

## 금지 사항
- 범위 밖 파일 수정 금지(docs/PLAN.md, 다른 작업 문서, docs/research/ 수정 금지). 01-foundation.md는 상태 줄만 `in-progress`로 바꾼다.
- 커밋·푸시 금지.
- 외부 크레이트 의존성 추가 금지(이번 작업은 의존성 없이 컴파일).
- 엔드포인트 주소, 키 값 기록 금지.

## 산출물
AGENTS.md, CLAUDE.md, .cursor/rules/agent-snap.mdc, docs/INDEX.md, docs/DECISIONS.md, docs/RESEARCH.md, .gitignore, README.md, prompts/README.md, rules/README.md, scripts/check.sh, Cargo.toml(워크스페이스), Cargo.lock, crates/snap-core, crates/snap-cli.

## 검증 방법
- `bash scripts/check.sh` 통과, 로그 경로 확인
- 501줄 임시 파일 실패 시험 후 임시 파일 삭제
- 마커 블록 diff
- `git check-ignore -v .env agent-snap.toml target/x models/x logs/x`

## 보고 형식
만든 파일 목록, 검증 명령별 결과(출력 요지와 로그 경로), AC 1~5 각각의 충족 근거, 남은 문제.
