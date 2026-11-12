//! `tool_call` 프로파일 값 검사와 예약 키 거부 시험(D15, AC 7 뒷부분).

use snap_core::agent::ToolCallProfile;
use snap_core::config::{ModelIssue, SectionIssue, ValidationError};
use snap_core::llm::ChatRequest;

use std::collections::BTreeMap;

use snap_core::config::{ExtraValue, Sampling};

/// AC 7: 예약 키를 덮으려는 항목은 설정 오류로 내고, 덮지 않으면 온도만 합칩니다.
#[test]
fn profile_rejects_reserved_keys_and_applies_temperature() {
    let profile = ToolCallProfile::new(0.1).expect("0.1은 프로파일 값");
    let mut request = request_with_extra(BTreeMap::from([(
        "tools".to_owned(),
        ExtraValue::Bool(true),
    )]));
    let error = profile
        .apply(&mut request)
        .expect_err("예약 키가 있으면 설정 오류");
    assert!(matches!(
        error,
        ValidationError::Model(ModelIssue::ExtraReservedKey { .. })
    ));

    let mut clean = request_with_extra(BTreeMap::new());
    profile
        .apply(&mut clean)
        .expect("예약 키가 없으면 합쳐야 합니다");
    assert_eq!(clean.sampling.temperature, Some(0.1));
}

/// AC 7: 범위를 벗난 온도는 프로파일 만들기에서 거부합니다(02-2 규칙 재사용).
#[test]
fn profile_rejects_out_of_range_temperature() {
    let error = ToolCallProfile::new(2.5).expect_err("2.5는 범위 밖");
    assert!(matches!(
        error,
        ValidationError::Section(SectionIssue::SamplingOutOfRange { .. })
    ));
}

/// 요청 골격에 `extra`를 채운 골격을 만듭니다.
fn request_with_extra(extra: BTreeMap<String, ExtraValue>) -> ChatRequest {
    ChatRequest {
        model: "loop-test-model".into(),
        messages: Vec::new(),
        tools: Vec::new(),
        sampling: Sampling::default(),
        reasoning: None,
        extra,
    }
}
