//! 선택 키 클라이언트의 `GET /models` 정상 조회 통합 시험(02-7 AC 3).
//!
//! 모의 서버가 준 목록 JSON에서 id만 순서대로 읽는지, 모르는 필드·`null`을 무시하는지,
//! 설정 `models`와 자동 동기화하지 않는지, base_url 경로 결합을 한 번만 하는지 본다.
//! 형식 오류와 본문 가리기는 `format_error.rs`가 따로 다룬다(SRP).

use snap_core::config::Config;
use snap_core::llm::CancellationToken;

use crate::common::mock_server::{MockResponseScript, MockServer};

use super::support::{
    BEARER_PREFIX, DEFAULT_LIST_IDS, EXTRA_IDS, FAST_SELECTION_KEY, FIRST_ID_MARK, KEEPING_ID,
    KEY_HEADER_NAME, KEY_MARK, SECOND_ID_MARK, ac3_config_text, ac3_credentials, list_script,
};

/// AC 3: 모의 서버가 준 응답의 id 목록을 순서대로 읽는다.
///
/// 기본 목록 스크립트(`models_list()`)로 조회한다. 조회는 설정의 모델 id를 건드리지 않는다.
#[tokio::test]
async fn list_models_reads_ids_from_server_response() {
    let server = MockServer::spawn(MockResponseScript::models_list()).await;
    let config = Config::parse(&ac3_config_text(server.base_url())).expect("AC 3 설정");
    let credentials = ac3_credentials();

    let client = config
        .client_for(FAST_SELECTION_KEY, &credentials)
        .expect("선택 키 클라이언트");
    let ids = client
        .list_models(CancellationToken::new())
        .await
        .expect("목록 조회 성공");
    assert_eq!(ids, DEFAULT_LIST_IDS);

    let records = server.recorded_requests().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].method, "GET");
    assert_eq!(records[0].path, "/models");
    assert_eq!(
        records[0]
            .headers
            .get(KEY_HEADER_NAME)
            .expect("pc1에는 키 헤더가 있어야 합니다")
            .to_str()
            .expect("헤더 값"),
        format!("{BEARER_PREFIX}{KEY_MARK}")
    );
}

/// AC 3: 조회 결과를 설정의 모델 목록과 자동으로 맞추지 않음을 설정 값으로 확인한다.
#[tokio::test]
async fn list_models_does_not_sync_with_config_models() {
    let server = MockServer::spawn(MockResponseScript::models_list()).await;
    let config = Config::parse(&ac3_config_text(server.base_url())).expect("AC 3 설정");
    let credentials = ac3_credentials();

    let client = config
        .client_for(FAST_SELECTION_KEY, &credentials)
        .expect("선택 키 클라이언트");
    let ids = client
        .list_models(CancellationToken::new())
        .await
        .expect("목록 조회 성공");

    // 서버가 알려 준 id와 설정의 손쓴 id가 다르고, 조회 뒤에도 설정 값은 그대로다.
    assert_eq!(ids, DEFAULT_LIST_IDS);
    assert!(
        !ids.iter().any(|id| id == KEEPING_ID),
        "조회에 설정 id가 섞여선 안 됩니다"
    );
    let (_, model) = config
        .select(FAST_SELECTION_KEY)
        .expect("조회 뒤 설정 선택");
    assert_eq!(model.id, KEEPING_ID);
    assert_ne!(model.id, FIRST_ID_MARK);
    assert_ne!(model.id, SECOND_ID_MARK);
}

/// AC 3: 모르는 필드·`null` 필드가 섞인 응답에서도 id만 읽고 무시한다.
#[tokio::test]
async fn list_models_ignores_unknown_and_null_fields() {
    let body = r#"{
        "object":"list",
        "data":[
            {"id":"alpha","object":"model","owned_by":"mock","created":1},
            {"id":"beta","meta":null,"spec":{"max_length":4096},"parent":null,"tags":["x","y"]}
        ]
    }"#;
    let server = MockServer::spawn(list_script(body)).await;
    let config = Config::parse(&ac3_config_text(server.base_url())).expect("AC 3 설정");
    let credentials = ac3_credentials();

    let client = config
        .client_for(FAST_SELECTION_KEY, &credentials)
        .expect("선택 키 클라이언트");
    let ids = client
        .list_models(CancellationToken::new())
        .await
        .expect("모르는 필드·null 필드 응답");
    assert_eq!(ids, EXTRA_IDS);
}

/// 작업 3(N5): base_url에 `/v1`이 있어도 `/models`를 한 번만 붙여 `/v1/models`로 요청한다.
///
/// `/v1/v1`처럼 경로가 이중으로 결합되면 모의 서버가 그 경로를 몰라 요청이 닿지 않는다.
#[tokio::test]
async fn list_models_joins_path_once_when_base_url_has_v1() {
    let server = MockServer::spawn_models_only(MockResponseScript::models_list()).await;
    let base_url = format!("{}/v1", server.base_url());
    let config = Config::parse(&ac3_config_text(&base_url)).expect("AC 3 설정");

    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");
    let ids = client
        .list_models(CancellationToken::new())
        .await
        .expect("/v1이 붙은 base_url로 목록 조회 성공");
    assert_eq!(ids, DEFAULT_LIST_IDS);

    let records = server.recorded_requests().await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].method, "GET");
    assert_eq!(records[0].path, "/v1/models");
}
