//! 요청 본문 조립 자리. 본문 객체를 만드는 한 책임만 담음.

use serde_json::Map;
use serde_json::Value;

use crate::config::types::Sampling;
use crate::consts::request::BODY_CONTENT_KEY;
use crate::consts::request::BODY_MAX_TOKENS_KEY;
use crate::consts::request::BODY_MESSAGES_KEY;
use crate::consts::request::BODY_MODEL_KEY;
use crate::consts::request::BODY_ROLE_KEY;
use crate::consts::request::BODY_STREAM_KEY;
use crate::consts::request::BODY_TEMPERATURE_KEY;
use crate::consts::request::BODY_TOOLS_KEY;
use crate::consts::request::BODY_TOP_P_KEY;
use crate::llm::client::kind::EndpointKind;
use crate::llm::client::reasoning::attach_reasoning;
use crate::llm::client::request::ChatMessage;
use crate::llm::client::request::ChatRequest;

/// 채팅 요청 하나를 OpenAI 호환 본문 객체로 조립함.
///
/// `tool_choice`·`n`·`stream_options`는 넣지 않는다. `context_window`는 본문에 담지 않는다.
pub fn build_body(request: &ChatRequest, kind: EndpointKind) -> Value {
    let mut body = Map::new();
    body.insert(
        BODY_MODEL_KEY.to_owned(),
        Value::String(request.model_id.clone()),
    );
    body.insert(
        BODY_MESSAGES_KEY.to_owned(),
        messages_value(&request.messages),
    );
    attach_tools(&mut body, &request.tools);
    attach_sampling(&mut body, &request.sampling);
    attach_reasoning(&mut body, kind, request.reasoning);
    attach_extra(&mut body, &request.extra);
    body.insert(BODY_STREAM_KEY.to_owned(), Value::Bool(true));
    Value::Object(body)
}

/// 메시지를 본문용 배열로 옮김.
fn messages_value(messages: &[ChatMessage]) -> Value {
    let items: Vec<Value> = messages.iter().map(message_value).collect();
    Value::Array(items)
}

/// 메시지 하나를 본문용 객체로 옮김.
fn message_value(message: &ChatMessage) -> Value {
    let mut object = Map::new();
    object.insert(
        BODY_ROLE_KEY.to_owned(),
        Value::String(message.role.clone()),
    );
    object.insert(
        BODY_CONTENT_KEY.to_owned(),
        Value::String(message.content.clone()),
    );
    Value::Object(object)
}

/// 도구 목록이 있을 때만 본문에 넣음. 빈 목록은 서버 기본을 따름.
fn attach_tools(body: &mut Map<String, Value>, tools: &[Value]) {
    if tools.is_empty() {
        return;
    }
    body.insert(BODY_TOOLS_KEY.to_owned(), Value::Array(tools.to_vec()));
}

/// 샘플링 값 중 있는 항목만 본문에 넣음.
fn attach_sampling(body: &mut Map<String, Value>, sampling: &Sampling) {
    if let Some(temperature) = sampling.temperature {
        body.insert(BODY_TEMPERATURE_KEY.to_owned(), Value::from(temperature));
    }
    if let Some(top_p) = sampling.top_p {
        body.insert(BODY_TOP_P_KEY.to_owned(), Value::from(top_p));
    }
    if let Some(max_tokens) = sampling.max_tokens {
        body.insert(BODY_MAX_TOKENS_KEY.to_owned(), Value::from(max_tokens));
    }
}

/// 모델 `extra` 원문을 본문에 그대로 합침. 예약 키는 설정 검증이 이미 거부함.
fn attach_extra(body: &mut Map<String, Value>, extra: &Option<Map<String, Value>>) {
    let Some(extra) = extra else {
        return;
    };
    for (key, value) in extra {
        body.insert(key.clone(), value.clone());
    }
}
