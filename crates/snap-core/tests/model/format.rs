//! 02-7 T3 목록 형식 오류 시험. `data` 없음·`id` 비문자열만 봄(spec AC 4·5).

use serde_json::json;

use snap_core::llm::ModelsError;
use snap_core::llm::ModelsFormatProblem;

use crate::materials::KEY;
use crate::materials::json_script;
use crate::materials::models_of;
use crate::server::MockServer;

/// 시험용 서버 본문 표시. 원문이 오류에 들어가지 않는지 보는 기준 문자열.
const BODY_MARK: &str = "vendorBodyMark";

/// `data` 배열이 없는 응답은 형식 오류. 응답 원문 전체가 오류에 담기지 않는다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_format_error_when_data_missing() {
    let server = MockServer::start().await.unwrap();
    let body = json!({"object": "list", "total_count": 1, BODY_MARK: "qwen-id"}).to_string();
    server.enqueue(json_script(200, &body));

    let error = models_of(&server)
        .await
        .expect_err("data missing is format error");

    assert_format(&error, ModelsFormatProblem::DataMissing);

    server.stop();
}

/// `data` 항목의 `id`가 문자열이 아니면 형식 오류.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_format_error_when_id_not_string() {
    let server = MockServer::start().await.unwrap();
    let body = json!({"data": [{"id": 7, "object": "model"}]}).to_string();
    server.enqueue(json_script(200, &body));

    let error = models_of(&server)
        .await
        .expect_err("id type is format error");

    assert_format(&error, ModelsFormatProblem::IdNotString { index: 0 });

    server.stop();
}

/// `data`가 배열이 아니면 형식 오류로 본다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_format_error_when_data_not_array() {
    let server = MockServer::start().await.unwrap();
    let body = json!({"data": "qwen-id"}).to_string();
    server.enqueue(json_script(200, &body));

    let error = models_of(&server)
        .await
        .expect_err("data type is format error");

    assert_format(&error, ModelsFormatProblem::DataMissing);

    server.stop();
}

/// 형식 오류의 Display·Debug에 응답 원문·키가 없고 세부 사실만 밝힌다.
fn assert_format(error: &ModelsError, problem: ModelsFormatProblem) {
    let ModelsError::Format(format) = error else {
        panic!("test expects format error: {error}");
    };
    assert_eq!(format.problem(), problem, "format problem carries fact");

    let display = error.to_string();
    let debug = format!("{error:?}");
    for secret in [BODY_MARK, KEY, "127.0.0.1"] {
        assert!(
            !display.contains(secret) && !debug.contains(secret),
            "format error hides response text: {display}"
        );
    }
}
