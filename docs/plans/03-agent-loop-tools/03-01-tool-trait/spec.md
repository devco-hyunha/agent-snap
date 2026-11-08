# 03-1 `Tool` 트레이트 설계: 명세

- 상위 문서: [plan.md](plan.md), 상위 기획: [../../03-agent-loop-tools.md](../../03-agent-loop-tools.md)
- 역할: 설계 확정 문서. 이 문서가 03-2(루프)·03-3(파일 도구)·03-6(run_command)·03-8(execute)가 쓰는 트레이트 서명의 정본이다.
- 상태: 확정(2026-11-08). 아래 서명·이름은 03-2·03-3·03-6·03-8이 그대로 쓴다.

## 입력과 출력
- 입력: 도구 트레이트 객체 하나(구현체는 03-3·03-6에서 채운다).
- 출력(등록): OpenAI `tools` 배열 한 자리 — `{ "type": "function", "function": { "name", "description", "parameters"(JSON Schema) } }`(03-agent-loop-tools.md OpenAI 절 규칙).
- 출력(실행): 결과 문자열 하나. 03-2가 `role:"tool"`, `tool_call_id`를 붙여 messages에 넣는다.

## `Tool` 트레이트 서명 (AC 1)
```rust
// tools/tool.rs (설계 서명, 구현은 03-3·03-6에서)
pub trait Tool {
    fn name(&self) -> &str;                      // 등록 이름, tool_calls 이름과 비교
    fn description(&self) -> &str;               // 모델에게 보이는 한 줄 설명
    fn parameters_schema(&self) -> serde_json::Value;  // 인자 JSON Schema
    fn executes_writes(&self) -> bool { false }  // 쓰기·명령 도구 여부(03-8 승인 구분용)
    async fn call(&self, args_json: &str, ctx: &ToolContext) -> ToolOutcome;
}

pub struct ToolContext { /* 작업 폴더, 승인자(CommandApprover 자리), 잘림 상한 */ }

pub enum ToolOutcome {
    Done(String),                    // 결과 문자열(잘림 규칙은 03-2가 적용)
    ApprovalRequired { /* 승인자에게 넘길 명령 요약 */ },
    Error(String),                  // `오류: 이유` 한 줄(set.rs 형식 재사용)
}
```
- 필드·메서드 이름 위가 서명의 정본이다. 매개체·라이브타임·비대화형 async 여부는 03-2 착수 시 확정하되, 이름은 바꾸지 않는다.

- 서명 구성 항목: `name`·`description`·`parameters_schema`·`executes_writes`·`call`, `ToolContext`, `ToolOutcome`(`Done`·`ApprovalRequired`·`Error`) — 위 코드가 정본이고 AC 1의 근거다.
- 도구 구현은 파일 하나당 하나(PROJECT_RULES 개발 규칙). 트레이트 정의는 `tools/tool.rs`, 경로 검사는 `tools/path_guard.rs`처럼 트레이트·구현·헬퍼를 책임별 파일로 나눈다(SRP).

## 등록 규칙 (AC 2)
- 트레이트 객체 목록 → `tools` 배열 변환은 03-2 한 곳에서만 한다(DRY: 변환기는 `agent/runner`에 두고, 03-8·03-3·03-6은 등록만 한다).
- 이름 규칙: 소문자·언어 연결(기존 7개: read_file, write_file, edit_file, list_dir, glob, grep, run_command). 등록 이름과 `tool_calls` 이름이 같은지 검사(03-2 규칙).
- 예제 한 자리(정본):
```json
{ "type": "function", "function": {
    "name": "grep",
    "description": "본문 검색. 검색어·경로를 받아 결과를 한 줄씩 돌려준다.",
    "parameters": { "type": "object", "properties": {
        "pattern": { "type": "string" },
        "path": { "type": "string" } },
      "required": ["pattern"] } } }
```

## 결과 규격 (AC 1 연장)
- 결과 3갈래: `Done`(문자열)·`ApprovalRequired`(승인 요청)·`Error`(한 줄 오류). 각각 위 열거형이 정본이다.
- `Done` 문자열은 UTF-8 4,000자 넘으면 03-2가 자른다(03-1은 자르지 않는다 — 자르는 자리는 03-2가 정본).
- 오류는 `오류: 이유` 한 줄만 모델에 간다(추가 설명·여러 줄 금지, set.rs 규칙).

## 승인자·경로 검사 자리 (AC 3)
- `ToolContext`에 승인자 인터페이스가 들어온다. 승인이 필요한 도구는 `ApprovalRequired`로 돌려 승인자에게 넘긴다(run_command·03-8의 쓰기 게이트가 이 자리를 쓴다).
- 승인자는 D22의 `CommandApprover`(가칭)를 가리키며, CLI가 주입한다. L1(명령마다 확인) 기본, 승인 대기 300초·만료 시 거부(D22). 승인자 이름·주입 방식 확정 시 D22 표의 이름과 맞춘다.
- 경로 검사 헬퍼 이름은 `tools::path_guard`로 정한다(03-3·03-6·03-8이 호출, D17·D9 경계). 03-1은 이름만 정하고 구현은 03-3이 한다.

## AC
1. 위 서명이 이 문서에 있고 필드·메서드 이름이 정해진다. → 근거: `## Tool 트레이트 서명` 코드 블록과 구성 항목 목록.
2. 등록 규칙과 OpenAI `tools` 자리 예제 한 개가 이 문서에 있다. → 근거: `## 등록 규칙`의 grep 예제(`"type": "function"`·`function.name`·`function.parameters`).
3. `ToolContext`에 승인자 자리가, 실행 결과에 승인 요청(`ApprovalRequired`)이 자리로 있음을 이 문서가 밝힌다. → 근거: `## 승인자·경로 검사 자리`와 `ToolOutcome`의 `ApprovalRequired`.

## 비범위
- 구현 코드(tool.rs 본문 작성): 03-2가 골격을, 03-3·06이 도구를 만든다.
- 호출 형식 경로 선택(모델 프로필): 03-2·03-4.
