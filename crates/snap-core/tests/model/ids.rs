//! 02-7 T3 정상 id 추출 시험. 섞인 필드 중 `id`만 나오는지만 봄(spec AC 3).

use serde_json::json;

use crate::materials::json_script;
use crate::materials::models_of;
use crate::server::MockServer;

/// 서버가 섞어 보낸 필드 중 `id` 문자열 목록만 결과에 남는다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_list_keeps_only_ids() {
    let server = MockServer::start().await.unwrap();
    let body = json!({
        "object": "list",
        "data": [
            {"id": "qwen-id", "object": "model", "created": 1700000000, "owned_by": "library",
             "pricing": {"prompt": 0.0}, "context_length": 4096},
            {"id": "llama-id", "object": "model", "meta": {"architecture": "llama"}},
        ],
        "total_count": 2,
    })
    .to_string();
    server.enqueue(json_script(200, &body));

    let ids = models_of(&server).await.unwrap();

    assert_eq!(ids, vec!["qwen-id".to_owned(), "llama-id".to_owned()]);

    server.stop();
}

/// 빈 `data` 배열은 빈 목록으로 정상 처리한다.
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn models_list_keeps_empty_data_as_empty_list() {
    let server = MockServer::start().await.unwrap();
    let body = json!({"object": "list", "data": []}).to_string();
    server.enqueue(json_script(200, &body));

    let ids = models_of(&server).await.unwrap();

    assert!(ids.is_empty(), "empty data is an empty list");

    server.stop();
}
