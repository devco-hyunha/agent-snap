//! 선택 키 클라이언트의 `GET /models` 형식 오류·가리기 통합 시험(02-7 AC 3, 작업 3).
//!
//! 목록 응답이 정한 형식이 아닌 경우 형식 오류 종류만 알리고, Display·Debug에 키·주소·본문
//! 원문이 없는지, 200 본문이 비면 빈 목록 성공이 아니라 형식 오류로 내는지 본다.
//! 정상 조회는 `ids_read.rs`가 다룬다(SRP).

use snap_core::config::Config;
use snap_core::llm::CancellationToken;
use snap_core::llm::TransportError;

use crate::common::mock_server::{MockResponseScript, MockServer};

use super::support::{FAST_SELECTION_KEY, KEY_MARK, ac3_config_text, ac3_credentials, list_script};

/// 작업 3(AC 3): 형식 오류마다 종류만 알리고, Display·Debug에 키·호스트·본문 조각이 없다.
#[tokio::test]
async fn list_models_format_errors_hide_key_host_and_body() {
    // 본문에 자격 키 마킹 조각이 섞여 있어도 형식 오류 표기에선 가려지는지 본다.
    // 본문 JSON을 온전하게 유지한 채로 키 마킹 키 한 개만 뒤에 붙여, 형식 오류 5종이 각 라벨대로
    // 나는지(InvalidJson으로 뭉개지지 않는지)까지 함께 확인한다.
    let cases = [
        ("data 없음", "data 없음", r#"{"object":"list"}"#),
        ("data 배열 아님", "data가 배열 아님", r#"{"data":"one-id"}"#),
        ("id 없음", "id 없음", r#"{"data":[{"object":"model"}]}"#),
        ("id null", "id 없음", r#"{"data":[{"id":null}]}"#),
        (
            "id 문자열 아님",
            "id가 문자열 아님",
            r#"{"data":[{"id":42}]}"#,
        ),
    ];

    for (label, expected_kind, body_json) in cases {
        // 본문 JSON의 끝 `}`를 잘라 낸 뒤에 키 마킹 키 한 개를 더 넣어, 본문은 유효한 JSON으로
        // 유지하고 키 값만 섞인 상태로 만든다(형식 오류 5종이 각 라벨대로 나게 확인하기 위함).
        let head = body_json
            .strip_suffix('}')
            .expect("본문 JSON은 }로 닫혀야 합니다");
        let body_with_secret = format!(r#"{head},"{KEY_MARK}":"leak-value"}}"#);
        let server = MockServer::spawn(list_script(&body_with_secret)).await;
        let config = Config::parse(&ac3_config_text(server.base_url())).expect("AC 3 설정");

        let client = config
            .client_for(FAST_SELECTION_KEY, &ac3_credentials())
            .expect("선택 키 클라이언트");
        let error = client
            .list_models(CancellationToken::new())
            .await
            .expect_err("형식 오류여야 합니다");
        drop(server);

        let display = error.to_string();
        let debug = format!("{error:?}");
        assert!(
            matches!(error, TransportError::ModelsListFormat(_)),
            "{label}: 목록 형식 오류 종류여야 합니다: {display}"
        );
        assert!(
            display.contains("모델 목록 형식 오류"),
            "{label}: 안내 말이 Display에 있어야 합니다"
        );
        // 라벨마다 형식 오류 종류가 골고루 나야 한다(모두 InvalidJson으로 뭉개지면 안 됨).
        let actual_kind = error
            .to_string()
            .rsplit_once(": ")
            .expect("종류 표기가 있어야 합니다")
            .1
            .to_owned();
        assert_eq!(
            actual_kind, expected_kind,
            "{label}: 형식 오류 종류가 `{expected_kind}`여야 합니다: {display}"
        );
        // 종류 표기 말에는 `data`·`id` 이름이 정상적으로 들어가므로, 본문 값과 키·주소 값만 가리는지 봅니다.
        for forbidden in [KEY_MARK, "leak", "127.0.0.1", "42", "object"] {
            assert!(
                !display.contains(forbidden) && !debug.contains(forbidden),
                "{label}: Display·Debug에 본문·키·주소 조각 `{forbidden}`이 있습니다: {display} / {debug}"
            );
        }
    }
}

/// 작업 3(AC 3): 200 응답의 본문이 비면 빈 목록 성공이 아니라 형식 오류로 낸다.
#[tokio::test]
async fn list_models_rejects_empty_body_as_format_error() {
    let server = MockServer::spawn(MockResponseScript::empty_ok()).await;
    let config = Config::parse(&ac3_config_text(server.base_url())).expect("AC 3 설정");

    let client = config
        .client_for(FAST_SELECTION_KEY, &ac3_credentials())
        .expect("선택 키 클라이언트");
    let error = client
        .list_models(CancellationToken::new())
        .await
        .expect_err("빈 본문은 형식 오류여야 합니다");

    assert!(
        matches!(error, TransportError::ModelsListFormat(_)),
        "빈 본문은 목록 형식 오류로 내야 합니다: {error}"
    );
    assert!(error.to_string().contains("모델 목록 형식 오류"));
}
