//! `agent` 섹션 칸(`run_command_deadline`·`approval_deadline`) 읽기입니다(03-6 T1, spec AC 3).
//!
//! 02-6 `http` 읽기가 쓰던 같은 도우미(`take_optional`·`parse_number`·`parse_seconds`)를
//! 재사용합니다(DRY). 인정하는 칸은 두 개이고, 다른 칸은 거부합니다.

use std::time::Duration;

use jsonc_parser::{JsonObject, JsonValue};

use super::super::error::{ConfigError, SectionIssue};
use super::types::AgentSettings;
use super::validate;
use crate::jsonc::{WrongType, parse_number, pick_object, take_optional};

/// 루트의 `agent`를 읽습니다. 칸이 없으면 기본값이고, 빠진 항목은 기본값이 됩니다.
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

/// 시간 항목 이름으로 값을 담을 칸을 고릅니다. 시간 항목이 아니면 `None`입니다.
fn duration_slot<'a>(agent: &'a mut AgentSettings, item: &str) -> Option<&'a mut Duration> {
    match item {
        "run_command_deadline" => Some(&mut agent.run_command_seconds),
        "approval_deadline" => Some(&mut agent.approval_seconds),
        _ => None,
    }
}

/// `http` 읽기의 같은 초 단위 해석 규칙을 그대로 씁니다(D21: 0·음수·무한대 거부).
fn parse_seconds(value: &JsonValue<'_>) -> Option<Duration> {
    Duration::try_from_secs_f64(parse_number(value)?)
        .ok()
        .filter(validate::is_positive_duration)
}
