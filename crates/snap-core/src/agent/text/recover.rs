//! 2안 텍스트 프로토콜 응답 복원 경로를 한곳에서 관리
//!
//! 짝 찾기·닫힘 기호 복구는 `fallback/args.rs`를 그대로 사용(중복 구현 금지)

use serde_json::Value;

use crate::agent::args::{balanced_object_end, recover_arguments};

/// 2안 응답에서 복원한 도구 호출 한 개
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredCall {
    /// 모델이 쓴 도구 이름
    pub name: String,
    /// 모델이 쓴 인자 JSON 글자. 객체이면 문자열로 바꾼 값
    pub arguments: String,
}

/// 2안 응답 복원 결과가 가는 자리(짧은 오류 글자는 `notice`가 만듦)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoverOutcome {
    /// 이름·인자 칸을 찾은 호출. `extra_calls`는 첫 호출 뒤에 글자로 쓰인 호출 후보
    /// (2개 이상은 범위 밖 — 첫 호출만 실행하고 나머지는 안내로 넘김)
    Found {
        /// 글자에서 뽑은 첫 호출
        call: RecoveredCall,
        /// 첫 호출 뒤에 남은 글자에서 뽑은 추가 호출 후보
        extra_calls: Vec<RecoveredCall>,
    },
    /// assistant 내용에서 JSON 객체를 찾지 못함
    Missing,
    /// JSON 객체는 있었지만 해석하지 못함. 1회 복구(`recover_arguments`)도 실패한 자리
    Unparsable,
}

/// 2안 assistant 내용 글자에서 도구 호출을 복원함
///
/// Markdown 둘레를 벗기고 첫 중괄호부터 짝까지 잘라 파싱하며, 실패 시 `recover_arguments`의 1회 복구만 사용
#[must_use]
pub fn recover_text_call(content: &str) -> RecoverOutcome {
    recover_json_text(&strip_markdown_fences(content))
}

/// 둘레를 벗겨 본문만 남김
///
/// 둘레가 없으면 잘라낸 글자를 그대로 반환
/// 표기 한 줄(언어 이름)을 버리고 본문을 읽음
fn strip_markdown_fences(text: &str) -> String {
    let trimmed = text.trim();
    let Some(rest) = trimmed.strip_prefix("```") else {
        return trimmed.to_owned();
    };
    // 한 줄 언어 표기(json·jsonc 등)를 버리고 본문부터 읽음
    let body = &rest[rest.find('\n').map_or(rest.len(), |newline| newline + 1)..];
    body.strip_suffix("```").unwrap_or(body).trim().to_owned()
}

/// 글자에서 JSON 객체를 찾아내는 자리
///
/// 여러 객체가 섞인 글자는 객체마다 확인하고, 남은 객체는 추가 후보로 넘김
fn recover_json_text(text: &str) -> RecoverOutcome {
    let candidates = collect_object_texts(text);
    if candidates.is_empty() {
        return RecoverOutcome::Missing;
    }
    let mut calls: Vec<RecoveredCall> = Vec::new();
    for candidate in &candidates {
        let Some(value) = parse_with_one_repair(candidate) else {
            // 잘린 JSON을 1회 복구로도 못 읽음. 이미 찾은 호출이 있으면 그 결과로 종료
            if calls.is_empty() {
                return RecoverOutcome::Unparsable;
            }
            break;
        };
        let Some(call) = call_fields(&value) else {
            // 이름·인자 칸이 없는 모호한 객체는 걸러냄(거부 규칙)
            continue;
        };
        calls.push(call);
    }
    if calls.is_empty() {
        // 객체는 있었지만 이름·인자 칸이 없음. 해석은 성공했고 스키마 대상이 없으므로
        // 복원 실패로 보고 짧은 오류 자리로 넘김(모호 시 거부)
        return RecoverOutcome::Unparsable;
    }
    let extra_calls = calls.drain(1..).collect();
    RecoverOutcome::Found {
        call: calls.remove(0),
        extra_calls,
    }
}

/// 글자에서 중괄호로 묶인 객체 텍스트를 순서대로 뽑음
///
/// 각 객체는 첫 중괄호부터 짝에 맞는 닫힘 기호까지 자르고, 짝이 안 맞는 잘린 객체가 나오면 앞 객체와
/// 합쳐 한 번만 더 확인함(1회 복구가 처리할 수 있게 넘기기 위함)
fn collect_object_texts(text: &str) -> Vec<String> {
    let mut objects: Vec<String> = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('{') {
        let tail = &rest[start..];
        match balanced_object_end(tail) {
            Some(end) => {
                objects.push(tail[..=end].to_owned());
                rest = &tail[end + 1..];
            }
            // 닫힘 기호가 없는 잘린 객체. 앞 객체가 있으면 합쳐 복구 대상으로 넘김
            None => {
                let joined = match objects.pop() {
                    Some(previous) => format!("{previous}{tail}"),
                    None => tail.to_owned(),
                };
                objects.push(joined);
                break;
            }
        }
    }
    objects
}

/// JSON을 파싱하고, 실패하면 `recover_arguments`(닫힘 기호 1회 복구)에 넘겨 확인
fn parse_with_one_repair(text: &str) -> Option<Value> {
    if let Ok(value) = serde_json::from_str::<Value>(text) {
        return Some(value);
    }
    let repaired = recover_arguments(text).ok()?;
    serde_json::from_str::<Value>(&repaired).ok()
}

/// JSON 객체에서 도구 이름(`name`)과 인자(`arguments`) 칸을 뽑음. 모호하면 None
fn call_fields(value: &Value) -> Option<RecoveredCall> {
    let name = value.get("name").and_then(Value::as_str)?;
    let arguments = value.get("arguments")?;
    let arguments_text = match arguments {
        Value::Object(_) | Value::Array(_) => arguments.to_string(),
        Value::String(text) => text.to_owned(),
        _ => return None,
    };
    Some(RecoveredCall {
        name: name.to_owned(),
        arguments: arguments_text,
    })
}
