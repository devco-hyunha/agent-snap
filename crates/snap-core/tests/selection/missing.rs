//! T1 선택 키 시험: 대상 없음 책임. 엔드포인트·모델 이름 누락만 봄.

use snap_core::llm::SelectionError;

use crate::material::config;
use crate::material::reject_texts;

#[test]
fn rejects_unknown_endpoint_name() {
    let (display, _) = reject_texts("absent/qwen");
    assert!(
        display.contains("absent"),
        "display names the endpoint: {display}"
    );
    assert!(
        display.contains("qwen"),
        "display names the model: {display}"
    );
}

#[test]
fn rejects_unknown_model_name() {
    let (display, _) = reject_texts("local/absent");
    assert!(
        display.contains("absent"),
        "display names the model: {display}"
    );
}

#[test]
fn target_missing_carries_names_only() {
    let config = config();
    let key = snap_core::llm::parse_selection_key("local/absent").expect("key parses");
    let err = snap_core::llm::find_target(&config, &key).expect_err("model is absent");
    match err {
        SelectionError::TargetMissing { endpoint, model } => {
            assert_eq!(endpoint, "local");
            assert_eq!(model, "absent");
        }
        other => panic!("expected target missing, got: {other:?}"),
    }
}
