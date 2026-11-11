//! 읽기와 기본값 테스트(명세 AC 1): 정상 설정, 빠진 값, 폴더에서 읽기.

use std::collections::BTreeMap;
use std::time::Duration;

use super::support::{
    TempDir, base_url, config_of, config_with_models, endpoint_object, endpoint_object_with_kind,
    model_object, models_tail, parse_ok,
};
use crate::config::{
    CONFIG_FILE_NAME, CREDENTIALS_FILE_NAME, ConfigError, EndpointKind, ExtraValue, HttpSettings,
    ReadingError, Reasoning, Sampling, ToolCallForm, load_config, load_credentials,
};

const FULL_CONFIG: &str = r#"
// 주석이 든 정상 설정
{
  "target_language": "en",
  "endpoints": [
    {
      "name": "  main  ", /* 이름은 trim해서 저장한다 */
      "base_url": "https://example.invalid/v1",
      "kind": "llamacpp",
      "unknown_field": 1, // 알 수 없는 칸은 무시한다
      "models": [
        {
          "name": "fast",
          "id": " path/to/model.gguf ",
          "reasoning": "medium",
          "sampling": { "temperature": 0.2, "top_p": 0.95, "max_tokens": 4096, },
          "context_window": 8192,
          "tool_call_form": "text",
          "extra": { "keep_alive": 0, "options": { "num_ctx": 8192, "stop": ["a", null] } },
        },
        { "name": "plain", "id": "plain-id" },
      ],
    },
  ],
  "sampling": { "temperature": 1 },
  "http": {
    "retries": 5,
    "backoff_min": 0.5,
    "backoff_max": 20,
    "retry_after_max": 45,
    "connect_seconds": 7,
    "header_wait_seconds": 200,
    "stream_idle_seconds": 90,
  },
}
"#;

/// AC 1: 주석·마지막 쉼표가 든 중첩 설정에서 엔드포인트·모델·샘플링·시간 값이 읽힌다.
#[test]
fn parse_full_config_with_comments_reads_every_value() {
    let config = parse_ok(FULL_CONFIG);

    assert_eq!(config.target_language, "en");
    assert_eq!(config.endpoints.len(), 1);
    let endpoint = &config.endpoints[0];
    assert_eq!(endpoint.name, "main");
    assert_eq!(endpoint.base_url.reveal(), "https://example.invalid/v1");
    assert_eq!(endpoint.kind, EndpointKind::LlamaCpp);
    assert_eq!(endpoint.models.len(), 2);

    let fast = &endpoint.models[0];
    assert_eq!(fast.name, "fast");
    assert_eq!(fast.id, "path/to/model.gguf");
    assert_eq!(fast.reasoning, Some(Reasoning::Medium));
    assert_eq!(fast.context_window, 8192);
    // D50 결정 7: 선언 값(`text`)을 그대로 읽습니다.
    assert_eq!(fast.tool_call_form, ToolCallForm::Text);
    assert_eq!(
        fast.sampling,
        Sampling {
            temperature: Some(0.2),
            top_p: Some(0.95),
            max_tokens: Some(4096),
        }
    );
    let options = ExtraValue::Object(BTreeMap::from([
        ("num_ctx".to_owned(), ExtraValue::Number("8192".to_owned())),
        (
            "stop".to_owned(),
            ExtraValue::Array(vec![ExtraValue::String("a".to_owned()), ExtraValue::Null]),
        ),
    ]));
    assert_eq!(
        fast.extra,
        BTreeMap::from([
            ("keep_alive".to_owned(), ExtraValue::Number("0".to_owned())),
            ("options".to_owned(), options),
        ])
    );

    let plain = &endpoint.models[1];
    assert_eq!(plain.sampling, Sampling::default());
    assert_eq!(plain.reasoning, None);
    assert!(plain.extra.is_empty());
    // D29: `context_window`를 생략하면 기본 65536이 읽힌다.
    assert_eq!(plain.context_window, 65536);
    // D50 결정 7: `tool_call_form`를 생략하면 기본 `openai`가 읽힌다.
    assert_eq!(plain.tool_call_form, ToolCallForm::OpenAi);

    assert_eq!(config.sampling.temperature, Some(1.0));
    assert_eq!(config.sampling.top_p, None);
    assert_eq!(
        config.http,
        HttpSettings {
            retries: 5,
            backoff_min: Duration::from_millis(500),
            backoff_max: Duration::from_secs(20),
            retry_after_max: Duration::from_secs(45),
            connect_seconds: Duration::from_secs(7),
            header_wait_seconds: Duration::from_secs(200),
            stream_idle_seconds: Duration::from_secs(90),
        }
    );
}

/// AC 1: 빠진 값은 기본값이 되고 생략한 `endpoints`는 빈 목록이다.
#[test]
fn parse_empty_object_uses_defaults_and_empty_endpoints() {
    let config = parse_ok("{}");

    assert_eq!(config.target_language, "ko");
    assert!(config.endpoints.is_empty());
    assert_eq!(config.models().count(), 0);
    assert_eq!(config.sampling, Sampling::default());
    assert_eq!(config.http.retries, 3);
    assert_eq!(config.http.backoff_min, Duration::from_secs(1));
    assert_eq!(config.http.backoff_max, Duration::from_secs(30));
    assert_eq!(config.http.retry_after_max, Duration::from_secs(60));
    assert_eq!(config.http.connect_seconds, Duration::from_secs(10));
    assert_eq!(config.http.header_wait_seconds, Duration::from_secs(300));
    assert_eq!(config.http.stream_idle_seconds, Duration::from_secs(120));
}

/// AC 1: `http`의 일부만 적으면 나머지는 기본값이고, 생략한 `models`는 빈 목록이다.
#[test]
fn parse_partial_http_and_missing_models_fill_defaults() {
    let text = format!(
        r#"{{"endpoints":[{}],"http":{{"retries":0}}}}"#,
        endpoint_object("main", &base_url(), "")
    );
    let config = parse_ok(&text);

    assert!(config.endpoints[0].models.is_empty());
    assert_eq!(config.http.retries, 0);
    assert_eq!(config.http.stream_idle_seconds, Duration::from_secs(120));
}

/// AC 1: `endpoints`를 빈 배열로 적어도 통과한다.
#[test]
fn parse_empty_endpoints_array_is_accepted() {
    let config = parse_ok(r#"{"endpoints":[]}"#);

    assert!(config.endpoints.is_empty());
}

/// AC 1·2: 다른 엔드포인트의 같은 모델 별칭과, 대소문자만 다른 엔드포인트 이름은 허용된다.
#[test]
fn parse_allows_same_alias_across_endpoints_and_case_different_names() {
    let model = models_tail(&[model_object("fast", "id-1", "")]);
    let text = config_of(&[
        endpoint_object("Local", &base_url(), &model),
        endpoint_object("local", &base_url(), &model),
    ]);
    let config = parse_ok(&text);

    let pairs: Vec<(&str, &str)> = config
        .models()
        .map(|(endpoint, model)| (endpoint.name.as_str(), model.name.as_str()))
        .collect();
    assert_eq!(pairs, [("Local", "fast"), ("local", "fast")]);
}

/// AC 1: 모델 이름과 서버에 보낼 id는 서로 다를 수 있다.
#[test]
fn parse_keeps_model_name_and_id_separate() {
    let config = parse_ok(&config_with_models(&[model_object(
        "alias",
        "C:/models/a.gguf",
        "",
    )]));

    let (_, model) = config.models().next().expect("모델이 하나 있어야 합니다");
    assert_eq!(
        (model.name.as_str(), model.id.as_str()),
        ("alias", "C:/models/a.gguf")
    );
}

/// AC 1: `kind` 6종과 `reasoning` 4종이 모두 읽힌다.
#[test]
fn parse_accepts_every_kind_and_reasoning_value() {
    let kinds = [
        ("openai", EndpointKind::OpenAi),
        ("openrouter", EndpointKind::OpenRouter),
        ("ollama", EndpointKind::Ollama),
        ("vllm", EndpointKind::Vllm),
        ("llamacpp", EndpointKind::LlamaCpp),
        ("lmstudio", EndpointKind::LmStudio),
    ];
    for (text, expected) in kinds {
        let config = parse_ok(&config_of(&[endpoint_object_with_kind(
            "main",
            &base_url(),
            text,
            "",
        )]));
        assert_eq!(config.endpoints[0].kind, expected, "kind {text}");
    }

    let levels = [
        ("off", Reasoning::Off),
        ("low", Reasoning::Low),
        ("medium", Reasoning::Medium),
        ("high", Reasoning::High),
    ];
    for (text, expected) in levels {
        let model = model_object("fast", "id-1", &format!(r#","reasoning":"{text}""#));
        let config = parse_ok(&config_with_models(&[model]));
        assert_eq!(config.endpoints[0].models[0].reasoning, Some(expected));
    }
}

/// AC 1: 폴더 경로를 받아 `agent-snap.jsonc`와 `credentials.jsonc`를 읽는다(임시 폴더).
#[test]
fn load_reads_config_and_credentials_from_folder() {
    let folder = TempDir::new("load");
    folder.write(CONFIG_FILE_NAME, FULL_CONFIG);
    folder.write(
        CREDENTIALS_FILE_NAME,
        r#"{"api_keys":{"main":"key-from-file"}}"#,
    );

    let config = load_config(folder.path()).expect("설정을 읽어야 합니다");
    let credentials = load_credentials(folder.path()).expect("자격 파일을 읽어야 합니다");

    assert_eq!(config.endpoints[0].name, "main");
    let key = config.endpoints[0]
        .api_key(&credentials)
        .expect("키가 있어야 합니다");
    assert_eq!(key.reveal(), "key-from-file");
}

/// AC 1: 자격 파일이 없으면 키가 없는 상태이고 오류가 아니다.
#[test]
fn load_credentials_without_file_is_absent() {
    let folder = TempDir::new("no-credentials");

    let credentials = load_credentials(folder.path()).expect("파일이 없어도 오류가 아닙니다");

    assert!(credentials.is_empty());
}

/// AC 1·5: 설정 파일이 없으면 폴더 경로와 파일 이름만 담은 접근 오류다.
#[test]
fn load_missing_config_reports_folder_and_file_name_only() {
    let folder = TempDir::new("no-config");

    let error = load_config(folder.path()).expect_err("설정 파일이 없으면 오류여야 합니다");

    assert!(
        matches!(&error, ConfigError::Reading(ReadingError::Io{ file, .. }) if *file == CONFIG_FILE_NAME)
    );
    let text = error.to_string();
    assert!(text.contains(CONFIG_FILE_NAME));
    assert!(text.contains(&folder.path().display().to_string()));
}

/// AC 5: 자격 파일 내용 오류는 자격 파일 오류로 드러난다(키 값 없음).
#[test]
fn load_credentials_with_invalid_content_reports_credentials_error() {
    let folder = TempDir::new("bad-credentials");
    folder.write(CREDENTIALS_FILE_NAME, r#"{"api_keys":"leak-key-value-9"}"#);

    let error = load_credentials(folder.path()).expect_err("객체가 아닌 api_keys는 거부됩니다");

    assert!(matches!(
        error,
        ConfigError::Reading(ReadingError::Credentials(_))
    ));
    assert!(!error.to_string().contains("leak-key-value-9"));
}
