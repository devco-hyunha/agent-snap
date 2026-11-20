//! `agent` 섹션 읽기·거부 시험: 두 초 값 읽기와 기본값, 모르는 항목 거부.

use std::time::Duration;

use super::support::{
    assert_error_hides_secrets, base_url, config_of, config_with_root_tail, endpoint_object,
    parse_err, parse_ok,
};
use crate::config::{AgentSettings, ConfigError, FieldIssue, SectionIssue, ValidationError};

/// `agent` 섹션의 세 초 값 읽기, 셋 다 적지 않으면 기본값(600초·300초·30초) 사용.
#[test]
fn agent_section_reads_all_values_and_defaults() {
    let with_all = parse_ok(&config_with_root_tail(
        r#","agent":{"run_command_deadline":7,"approval_deadline":3,"wake_interval_seconds":21}"#,
    ));
    assert_eq!(
        with_all.agent,
        AgentSettings {
            run_command_seconds: Duration::from_secs(7),
            approval_seconds: Duration::from_secs(3),
            wake_interval_seconds: Duration::from_secs(21),
        }
    );

    let one = parse_ok(&config_with_root_tail(
        r#","agent":{"approval_deadline":3}"#,
    ));
    assert_eq!(one.agent.run_command_seconds, Duration::from_secs(600));
    assert_eq!(one.agent.approval_seconds, Duration::from_secs(3));

    let empty = parse_ok(&config_of(&[endpoint_object("main", &base_url(), "")]));
    assert_eq!(empty.agent, AgentSettings::default());
    assert_eq!(empty.agent.run_command_seconds, Duration::from_secs(600));
    assert_eq!(empty.agent.approval_seconds, Duration::from_secs(300));
    assert_eq!(empty.agent.wake_interval_seconds, Duration::from_secs(30));
}

/// 0과 음수, 숫자가 아닌 값, 초거대·무한대, 나노초 미만 값을 항목 이름을 밝혀 거부.
/// 0초 무한 대기와 0으로 잘리는 극초값은 허용하지 않음.
#[test]
fn agent_section_rejects_zero_negative_and_non_number() {
    for (item, tail) in [
        (
            "run_command_deadline",
            r#","agent":{"run_command_deadline":0}"#,
        ),
        (
            "run_command_deadline",
            r#","agent":{"run_command_deadline":-1}"#,
        ),
        ("approval_deadline", r#","agent":{"approval_deadline":0}"#),
        ("approval_deadline", r#","agent":{"approval_deadline":-2}"#),
        (
            "approval_deadline",
            r#","agent":{"approval_deadline":"slow"}"#,
        ),
        (
            "run_command_deadline",
            r#","agent":{"run_command_deadline":1e999}"#,
        ),
        (
            "approval_deadline",
            r#","agent":{"approval_deadline":1e999}"#,
        ),
        (
            "run_command_deadline",
            r#","agent":{"run_command_deadline":1e-10}"#,
        ),
        (
            "approval_deadline",
            r#","agent":{"approval_deadline":1e-10}"#,
        ),
        (
            "wake_interval_seconds",
            r#","agent":{"wake_interval_seconds":0}"#,
        ),
        (
            "wake_interval_seconds",
            r#","agent":{"wake_interval_seconds":-3}"#,
        ),
        (
            "wake_interval_seconds",
            r#","agent":{"wake_interval_seconds":1e999}"#,
        ),
    ] {
        let error = parse_err(&config_with_root_tail(tail));
        let expected = format!("agent.{item}");
        assert!(
            matches!(
                &error,
                ConfigError::Validation(ValidationError::Field(FieldIssue::InvalidField { field })) if *field == expected
            ),
            "{tail}: {error}"
        );
        assert_error_hides_secrets(tail, &error);
    }
}

/// 모르는 `agent` 항목은 항목 이름만 밝히고 거부.
/// 값은 오류에 넣지 않음.
#[test]
fn agent_section_rejects_unknown_item_with_name_only() {
    let error = parse_err(&config_with_root_tail(r#","agent":{"idle_timeout":9}"#));
    assert!(
        matches!(
            &error,
            ConfigError::Validation(ValidationError::Section(SectionIssue::UnknownAgentItem { item })) if item == "idle_timeout"
        ),
        "{error}"
    );
    assert!(format!("{error} {error:?}").contains("idle_timeout"));
    assert_error_hides_secrets("모르는 agent 항목", &error);
}
