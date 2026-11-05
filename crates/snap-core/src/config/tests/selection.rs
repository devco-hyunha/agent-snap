//! 선택 키 찾기 테스트(02-7 AC 2): 없는 키·형식 오류·모델 이름만 입력을 입력 키만 밝힌 오류로 확인합니다.

use super::support::{HOST, KEY_VALUE};
use super::support::{base_url, config_of, endpoint_object, model_object, models_tail, parse_ok};
use crate::config::{Config, SelectionError};

/// 선택 키 오류에서 입력 키만 밝히고 호스트·키 값은 밝히지 않는지 보는 기준 문자열.
const INPUT_MARK: &str = "selection-key-mark";

/// 오류의 Display·Debug에 입력 키는 있고 가짜 호스트·가짜 키 값은 없는지 확인합니다.
fn assert_only_key_leaks(key: &str, error: &SelectionError) {
    for (kind, text) in [
        ("Display", error.to_string()),
        ("Debug", format!("{error:?}")),
    ] {
        assert!(text.contains(key), "{kind}에 입력 키가 없습니다: {text}");
        for secret in [HOST, KEY_VALUE] {
            assert!(
                !text.contains(secret),
                "선택 키 오류 AC 2: {kind}에 `{secret}`가 있습니다: {text}"
            );
        }
    }
}

/// 선택 키 `pc1/fast`와 `pc2/fast`가 함께 있는 설정입니다. 같은 별칭 `fast`가 다른 엔드포인트에 있습니다.
fn two_endpoint_config() -> Config {
    let pc1 = endpoint_object(
        "pc1",
        &base_url(),
        &models_tail(&[model_object("fast", "id-one", "")]),
    );
    let pc2 = endpoint_object(
        "pc2",
        &base_url(),
        &models_tail(&[model_object("fast", "id-two", "")]),
    );
    parse_ok(&config_of(&[pc1, pc2]))
}

/// AC 2 기준: 정상 선택 키는 엔드포인트와 그 아래 모델을 짝으로 돌려준다.
#[test]
fn select_finds_endpoint_and_model() {
    let config = two_endpoint_config();

    let (endpoint, model) = config.select("pc1/fast").expect("정상 선택 키");

    assert_eq!(endpoint.name, "pc1");
    assert_eq!(model.id, "id-one");
}

/// AC 1: 같은 모델 별칭이어도 선택 키가 다르면 다른 엔드포인트의 모델을 고른다.
#[test]
fn select_splits_same_alias_by_endpoint() {
    let config = two_endpoint_config();

    let (_, first) = config.select("pc1/fast").expect("pc1/fast");
    let (second_endpoint, second_model) = config.select("pc2/fast").expect("pc2/fast");

    assert_eq!(second_endpoint.name, "pc2");
    assert_eq!(second_model.id, "id-two");
    assert_ne!(first.id, second_model.id);
}

/// AC 2: 설정에 없는 선택 키는 키만 밝힌 오류다. Display·Debug에 입력 키만 있고 호스트·키 값은 없다.
#[test]
fn select_unknown_key_carries_only_input_key() {
    let config = two_endpoint_config();
    let key = format!("{INPUT_MARK}/fast");

    let error = match config.select(&key) {
        Ok(_) => panic!("설정에 없는 키가 통과하면 안 됩니다"),
        Err(error) => error,
    };

    assert!(
        matches!(error, SelectionError::UnknownKey { key: ref found } if *found == key),
        "설정에 없는 키 오류가 아니라서 이상합니다: {error:?}"
    );
    assert_only_key_leaks(&key, &error);
}

/// AC 2: 구분자가 없는 입력(모델 이름만 넘긴 것)은 구분자 없음 오류다.
#[test]
fn select_model_name_only_is_no_separator() {
    let config = two_endpoint_config();
    let key = format!("{INPUT_MARK}-alone");

    let error = match config.select(&key) {
        Ok(_) => panic!("모델 이름만 넘긴 입력은 거부해야 합니다"),
        Err(error) => error,
    };

    assert!(
        matches!(error, SelectionError::NoSeparator { ref input } if *input == key),
        "구분자 없는 입력이 구분자 없음 오류가 아니라서 이상합니다: {error:?}"
    );
    assert_only_key_leaks(&key, &error);
}

/// AC 2: 구분자가 둘인 입력과 빈 이름은 형식 오류다.
#[test]
fn select_malformed_key_is_malformed() {
    let config = two_endpoint_config();

    for key in [
        format!("{INPUT_MARK}/a/b"),
        "/fast".to_owned(),
        format!("{INPUT_MARK}/"),
    ] {
        let error = match config.select(&key) {
            Ok(_) => panic!("형식이 틀린 선택 키 `{key}`는 거부해야 합니다"),
            Err(error) => error,
        };
        assert!(
            matches!(error, SelectionError::Malformed { ref input } if *input == key),
            "구분자 둘·빈 이름 `{key}`가 형식 오류여야 합니다"
        );
        assert_only_key_leaks(&key, &error);
    }
}

/// AC 2: `endpoints`가 빈 설정은 고를 모델 없음 오류다. 안내만 담고 키와 주소는 담지 않는다.
#[test]
fn select_empty_endpoints_is_empty_list() {
    let config = parse_ok(&config_of(&[]));

    let error = match config.select("pc1/fast") {
        Ok(_) => panic!("고를 모델이 없어야 합니다"),
        Err(error) => error,
    };

    assert!(matches!(error, SelectionError::EmptyList));
    let text = format!("{error:?}");
    assert!(!text.contains(HOST), "Debug에 호스트가 있습니다: {text}");
    assert!(
        !text.contains(KEY_VALUE),
        "Debug에 키 값이 있습니다: {text}"
    );
}
