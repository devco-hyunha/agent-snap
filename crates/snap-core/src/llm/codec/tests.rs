//! `GET /models` 목록 해석의 형식 오류 종류 테스트(02-7 AC 3).
//!
//! `data` 없음·배열 아님·`id` 없음·`id` 문자열 아님을 모두 형식 오류로 알리는지,
//! 모르는 필드·`null` 필드 무시는 그대로 유지하는지 본다.

use crate::llm::error::models_format::{ModelsListFormatError, ModelsListFormatKind};
use crate::llm::selection::models_list::{ModelsListIds, parse_models_ids};

fn parse(json: &str) -> Result<ModelsListIds, ModelsListFormatError> {
    parse_models_ids(json)
}

fn kind_of(result: Result<ModelsListIds, ModelsListFormatError>) -> ModelsListFormatKind {
    result.expect_err("형식 오류여야 합니다").kind
}

#[test]
fn reads_ids_in_order_and_drops_unknown_fields() {
    let json = r#"{"object":"list","data":[
        {"id":"a","object":"model","owned_by":"mock"},
        {"id":"b","created":123,"meta":null,"spec":{"max_length":4096}}
    ]}"#;
    let ModelsListIds(ids) = parse(json).expect("해석 성공");
    assert_eq!(ids, ["a", "b"]);
}

#[test]
fn reports_json_syntax_failure_as_format_error() {
    assert_eq!(
        kind_of(parse("data: [DONE]")),
        ModelsListFormatKind::InvalidJson
    );
}

#[test]
fn reports_missing_data_as_format_error() {
    assert_eq!(
        kind_of(parse(r#"{"object":"list"}"#)),
        ModelsListFormatKind::DataMissing
    );
}

#[test]
fn reports_data_not_array_as_format_error() {
    assert_eq!(
        kind_of(parse(r#"{"data":"one-id"}"#)),
        ModelsListFormatKind::DataNotArray
    );
}

#[test]
fn reports_missing_id_as_format_error() {
    assert_eq!(
        kind_of(parse(r#"{"data":[{"object":"model"}]}"#)),
        ModelsListFormatKind::IdMissing
    );
    assert_eq!(
        kind_of(parse(r#"{"data":[{"id":null}]}"#)),
        ModelsListFormatKind::IdMissing
    );
}

#[test]
fn reports_id_not_string_as_format_error() {
    assert_eq!(
        kind_of(parse(r#"{"data":[{"id":42}]}"#)),
        ModelsListFormatKind::IdNotString
    );
    assert_eq!(
        kind_of(parse(r#"{"data":[{"id":{"name":"deep"}}]}"#)),
        ModelsListFormatKind::IdNotString
    );
}

#[test]
fn format_error_display_hides_body_and_carries_only_kind() {
    let error = parse(r#"{"data":[{"id":127}],"api_key":"leak-key-value-9"}"#)
        .expect_err("형식 오류여야 합니다");
    let display = error.to_string();
    let debug = format!("{error:?}");

    assert!(display.contains("형식 오류"));
    // 종류 표기에는 `data`·`id` 같은 이름이 정상적으로 들어가므로, 본문 값 조각과 키 값만 가리는지 봅니다.
    for forbidden in ["leak", "127", "object", "api_key"] {
        assert!(
            !display.contains(forbidden) && !debug.contains(forbidden),
            "Display·Debug에 본문 조각 `{forbidden}`가 있습니다: {display} / {debug}"
        );
    }
}
