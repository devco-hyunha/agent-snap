//! `agent` 섹션의 시간 칸 두 개(`run_command_deadline`·`approval_deadline`)를 읽음.

use std::time::Duration;

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, SectionIssue};
use super::types::AgentSettings;
use super::validate;
use crate::jsonc::{WrongType, parse_number, pick_object, take_optional};

/// 루트의 `agent`를 읽음. 칸이 없으면 기본값이고, 빠진 항목은 기본값이 됨.
pub(super) fn read_agent(root: &mut JsonObject<'_>) -> Result<AgentSettings, ConfigError> {
    let mut agent = AgentSettings::default();
    let Some(object) = take_optional(root, "agent", pick_object)
        .map_err(|WrongType| ConfigError::invalid_field("agent"))?
    else {
        return Ok(agent);
    };
    for (key, value) in object {
        let item = key.as_ref();
        let invalid = || ConfigError::invalid_field(format!("agent.{item}"));
        let slot =
            duration_slot(&mut agent, item).ok_or_else(|| SectionIssue::UnknownAgentItem {
                item: item.to_owned(),
            })?;
        *slot = parse_seconds(&value).ok_or_else(invalid)?;
    }
    Ok(agent)
}

/// 시간 항목 이름으로 값을 담을 칸을 고름. 시간 항목이 아니면 `None` 반환.
fn duration_slot<'a>(agent: &'a mut AgentSettings, item: &str) -> Option<&'a mut Duration> {
    match item {
        "run_command_deadline" => Some(&mut agent.run_command_seconds),
        "approval_deadline" => Some(&mut agent.approval_seconds),
        _ => None,
    }
}

/// 초 단위 시간 해석. 0·음수·무한대 거부.
fn parse_seconds(value: &JsonValue<'_>) -> Option<Duration> {
    Duration::try_from_secs_f64(parse_number(value)?)
        .ok()
        .filter(validate::is_positive_duration)
}
