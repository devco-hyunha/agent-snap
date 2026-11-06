//! 설정 파일 쓰기 단위 테스트(02-8 작업 2 AC 1~4).
//!
//! 주석이 든 파일에 저장하면 주석과 손대지 않은 항목이 그대로 남고, 02-2 해석·검증으로 다시
//! 읽혀 새 값이 읽히는지 봅니다. 키 값이 설정 파일에 안 들어가는 것도 함께 봅니다.

use std::collections::BTreeMap;

use crate::config::{
    CONFIG_FILE_NAME, ConfigError, EndpointKind, EndpointSave, ExtraValue, ModelSave, Reasoning,
    Sampling, ValidationError, save_endpoint,
};

use super::super::support::{
    HOST, KEY_VALUE, TempDir, assert_error_hides_secrets, base_url, parse_ok,
};

/// AC 1: `extra` 예약 키는 02-2 읽기 규칙을 재사용해 저장 전에 거부합니다(AC 1 보존과 함께).
#[test]
fn reserved_extra_key_rejects_before_touching_file() {
    for key in validate_reserved_keys() {
        let folder = TempDir::new("extra-예약-키");
        folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);
        let save = EndpointSave {
            models: vec![model_with_extra_key("fast", key)],
            ..save_named("main", &base_url())
        };

        let error =
            save_endpoint(folder.path(), &save).expect_err(&format!("`{key}`는 거부되어야 합니다"));

        assert_eq!(
            folder.read(CONFIG_FILE_NAME),
            EMPTY_ENDPOINTS_FILE,
            "`{key}`: 파일이 바뀌었습니다"
        );
        assert!(
            matches!(&error, ConfigError::Validation(ValidationError::ExtraReservedKey{ model, key: found })
                if model == "fast" && found == key),
            "오류가 모델과 키 이름만 밝혀야 합니다: {error}"
        );
        assert_error_hides_secrets("extra 예약 키", &error);
    }
}

/// AC 1: 저장한 파일을 02-2로 다시 읽으면 새 값이 읽히고, 주석과 손대지 않은 항목은 그대로입니다.
#[test]
fn added_endpoint_keeps_comments_and_untouched_entries() {
    let folder = TempDir::new("주석-보존");
    folder.write(CONFIG_FILE_NAME, ANNOTATED_FILE);

    save_endpoint(folder.path(), &full_save("main", &base_url())).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    for marker in [
        "// 사용자 주석입니다.",
        "/* 블록 주석 */",
        "// 배열 안 주석",
        "// 항목 뒤 주석",
        "// http 위 주석",
        LOCAL_ENDPOINT_TEXT,
        HTTP_TEXT,
        ROOT_EXTRA_TEXT,
    ] {
        assert!(
            text.contains(marker),
            "원문 그대로 남아야 할 항목이 없습니다({marker}): {text}"
        );
    }

    let saved = parse_ok(&text);
    assert_eq!(saved.target_language, "ko");
    assert_eq!(
        saved.endpoints.len(),
        2,
        "새 엔드포인트가 추가되어야 합니다"
    );
    let added = &saved.endpoints[1];
    assert_eq!(added.name, "main");
    assert_eq!(added.base_url.reveal(), base_url());
    assert_eq!(added.kind, EndpointKind::LlamaCpp);
    assert_eq!(added.models.len(), 1);
    let model = &added.models[0];
    assert_eq!(model.name, "fast");
    assert_eq!(model.id, "id-1");
    assert_eq!(model.reasoning, Some(Reasoning::Low));
    assert_eq!(model.sampling.temperature, Some(0.2));
    assert_eq!(model.sampling.max_tokens, Some(1024));
    assert_eq!(model.extra, full_extra());
    assert_eq!(saved.http.retries, 5, "http 칸 값이 바뀌면 안 됩니다");
}

/// 같은 이름 엔드포인트와 모델은 항목을 새로 만들지 않고 값만 바꿉니다.
#[test]
fn same_names_update_in_place_without_new_entries() {
    let folder = TempDir::new("같은-이름-수정");
    folder.write(CONFIG_FILE_NAME, EXISTING_MAIN_FILE);
    let saved_before = parse_ok(&folder.read(CONFIG_FILE_NAME));

    // trim한 같은 이름이므로 기존 엔드포인트·모델을 고칩니다.
    save_endpoint(folder.path(), &full_save(" main ", &base_url()))
        .expect("수정 저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    let saved = parse_ok(&text);
    assert_eq!(
        saved.endpoints.len(),
        saved_before.endpoints.len(),
        "새 항목이 늘면 안 됩니다"
    );
    assert_eq!(
        saved.endpoints[0].base_url.reveal(),
        base_url(),
        "주소가 바뀌어야 합니다"
    );
    assert_eq!(
        saved.endpoints[0].kind,
        EndpointKind::LlamaCpp,
        "kind가 바뀌어야 합니다"
    );
    assert_eq!(
        saved.endpoints[0].models.len(),
        1,
        "모델 이름이 같으면 하나여야 합니다"
    );
    assert_eq!(saved.endpoints[0].models[0].id, "id-1");
    assert!(
        text.contains("// 모델 안 주석"),
        "수정할 때 주석이 남아야 합니다: {text}"
    );
}

/// AC 2: 파일이 없으면 새로 만들어 02-2 검증을 통과합니다.
#[test]
fn saving_creates_missing_config_file() {
    let folder = TempDir::new("새-파일");

    save_endpoint(folder.path(), &full_save("main", &base_url()))
        .expect("파일이 없으면 새 파일을 만들어야 합니다");

    let saved = parse_ok(&folder.read(CONFIG_FILE_NAME));
    assert_eq!(saved.endpoints.len(), 1);
    assert_eq!(saved.endpoints[0].name, "main");
    assert_eq!(saved.endpoints[0].models[0].name, "fast");
}

/// AC 3: 키 값이 저장 입력에 주어져도 설정 파일에는 키 칸이 없습니다(저장 API는 키를 받지 않습니다, D10).
#[test]
fn saved_file_never_contains_key_field() {
    let folder = TempDir::new("키-안-씀");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    // 02-8 저장 API는 키 값을 받지 않습니다. 화면(UI)이 키를 별도 자격 파일 저장으로 넘기는 설계입니다(D16).
    save_endpoint(folder.path(), &full_save("main", &base_url())).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    // `base_url`에는 정당한 주소가 들어가므로, 가리기 대상은 키 칸과 키 값만 봅니다.
    for forbidden in ["api_key", "api_key_env", KEY_VALUE] {
        assert!(
            !text.contains(forbidden),
            "설정 파일에 `{forbidden}`이 있습니다: {text}"
        );
    }
    assert_eq!(parse_ok(&text).endpoints[0].base_url.reveal(), base_url());
    assert_eq!(parse_ok(&text).endpoints.len(), 1);
}

/// AC 4: 옵션을 일부만 준 저장은 기본값을 파일에 강제로 쓰지 않습니다.
#[test]
fn partial_model_options_skip_default_fields() {
    let folder = TempDir::new("기본값-생략");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    save_endpoint(folder.path(), &reasoning_only_save()).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    for skipped in [
        "\"sampling\"",
        "\"context_window\"",
        "\"extra\"",
        "\"temperature\"",
        "\"top_p\"",
        "\"max_tokens\"",
    ] {
        assert!(
            !text.contains(skipped),
            "{skipped} 칸이 쓰면 안 됩니다: {text}"
        );
    }
    assert!(
        text.contains("\"reasoning\": \"low\""),
        "reasoning 칸: {text}"
    );
}

/// 모델 옵션을 준 저장은 값이 생긴 칸만 파일에 씁니다.
#[test]
fn given_model_options_are_written_as_fields() {
    let folder = TempDir::new("옵션-쓰기");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    save_endpoint(folder.path(), &full_save("main", &base_url())).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    for written in [
        "\"reasoning\": \"low\"",
        "\"context_window\": 32768",
        "\"keep_alive\": 0",
        "\"textVerbosity\": \"high\"",
        "\"budgetTokens\": 2048",
        "\"temperature\": 0.2",
        "\"max_tokens\": 1024",
    ] {
        assert!(text.contains(written), "{written} 가 없습니다: {text}");
    }
    assert!(
        !text.contains("\"top_p\""),
        "안 준 샘플링 칸은 안 씁니다: {text}"
    );
}

/// 검증 규칙에 맞지 않는 이름·주소는 파일을 건드리지 않고 02-2와 같은 오류만 냅니다.
#[test]
fn invalid_input_rejects_without_touching_file() {
    for (label, save) in [
        ("빈 이름", save_named("   ", &base_url())),
        ("슬래시 이름", save_named("a/b", &base_url())),
        ("잘못된 주소", save_named("main", "ftp://plain.invalid/v1")),
        (
            "주소 끝 슬래시",
            save_named("main", &format!("https://{HOST}/v1/")),
        ),
    ] {
        let folder = TempDir::new("거부");
        folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

        let error =
            save_endpoint(folder.path(), &save).expect_err(&format!("{label}은 거부되어야 합니다"));

        assert_eq!(
            folder.read(CONFIG_FILE_NAME),
            EMPTY_ENDPOINTS_FILE,
            "{label}: 파일이 바뀌었습니다"
        );
        assert_error_hides_secrets(label, &error);
    }
}

/// 모델을 준 저장은 모델 이름·id 규칙 확인에서 거부되면 파일을 건드리지 않습니다.
#[test]
fn invalid_model_names_rejects_without_touching_file() {
    for (label, models) in [
        ("빈 모델 이름", vec![model_save("   ", "id-1")]),
        ("슬래시 모델 이름", vec![model_save("a/b", "id-1")]),
        ("빈 모델 id", vec![model_save("fast", "  ")]),
    ] {
        let folder = TempDir::new("모델-거부");
        folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);
        let save = EndpointSave {
            name: "main".to_owned(),
            base_url: base_url(),
            kind: EndpointKind::OpenAi,
            models,
        };

        let error =
            save_endpoint(folder.path(), &save).expect_err(&format!("{label}은 거부되어야 합니다"));

        assert_eq!(
            folder.read(CONFIG_FILE_NAME),
            EMPTY_ENDPOINTS_FILE,
            "{label}: 파일이 바뀌었습니다"
        );
        assert_error_hides_secrets(label, &error);
    }
}

/// 파일이 있는데 내용을 읽지 못하면 오류로 끝나고 파일을 그대로 둡니다.
#[test]
fn unreadable_file_reports_error_and_keeps_file() {
    let folder = TempDir::new("읽을-수-없음");
    folder.write(CONFIG_FILE_NAME, BROKEN_TEXT);

    let error = save_endpoint(folder.path(), &full_save("main", &base_url()))
        .expect_err("내용 오류는 거부되어야 합니다");

    assert_eq!(folder.read(CONFIG_FILE_NAME), BROKEN_TEXT);
    assert_error_hides_secrets("내용 오류", &error);
}

/// AC 1에 쓰는 파일: 주석 3종, 손대지 않은 엔드포인트 항목, 손대지 않은 칸이 있습니다.
const ANNOTATED_FILE: &str = r#"// 사용자 주석입니다.
{
  /* 블록 주석 */
  "target_language": "ko",
  "endpoints": [
    // 배열 안 주석
    {"name": "local", "base_url": "http://local.invalid/v1", "kind": "ollama"} // 항목 뒤 주석
  ],
  // http 위 주석
  "http": { "retries": 5 },
  "unknown_root_key": { "keep_alive": 1 }
}"#;

/// AC 1 원문 비교에 쓰는 항목들입니다.
const LOCAL_ENDPOINT_TEXT: &str =
    r#"{"name": "local", "base_url": "http://local.invalid/v1", "kind": "ollama"}"#;
const ROOT_EXTRA_TEXT: &str = "\"unknown_root_key\": { \"keep_alive\": 1 }";
const HTTP_TEXT: &str = "\"retries\": 5";

/// 저장 전에 `main` 엔드포인트와 `fast` 모델이 있는 파일 텍스트입니다(수정 검증용).
const EXISTING_MAIN_FILE: &str = r#"{
  "endpoints": [
    {
      "name": "main",
      "base_url": "http://old.invalid/v1", // 주소 설명 주석
      "kind": "openai",
      "notes": " 그대로 두는 칸",
      "models": [
        // 모델 안 주석
        {"name": "fast", "id": "first-id", "reasoning": "high"}
      ]
    }
  ]
}"#;

const EMPTY_ENDPOINTS_FILE: &str = "{\"endpoints\":[]}";
const BROKEN_TEXT: &str = "{\"endpoints\":[}";

/// 이름과 주소를 고른 저장 정보(모델 `fast` 하나, 모델 옵션 전부).
fn full_save(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::LlamaCpp,
        models: vec![full_model_save()],
    }
}

/// `reasoning`만 준 저장 정보(AC 4).
fn reasoning_only_save() -> EndpointSave {
    EndpointSave {
        name: "main".to_owned(),
        base_url: base_url(),
        kind: EndpointKind::Ollama,
        models: vec![ModelSave {
            name: "fast".to_owned(),
            id: "id-1".to_owned(),
            reasoning: Some(Reasoning::Low),
            ..Default::default()
        }],
    }
}

/// `name`과 `base_url`만 고른 저장 정보(모델 없음).
fn save_named(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::OpenAi,
        models: Vec::new(),
    }
}

/// 모델 옵션을 다 준 저장 정보(trim한 이름 ` fast `를 씁니다).
fn full_model_save() -> ModelSave {
    ModelSave {
        name: " fast ".to_owned(),
        id: "id-1".to_owned(),
        reasoning: Some(Reasoning::Low),
        extra: full_extra(),
        context_window: Some(32768),
        sampling: Sampling {
            temperature: Some(0.2),
            top_p: None,
            max_tokens: Some(1024),
        },
    }
}

/// 이름과 id만 고르고 나머지는 옵션 다 준 값에서 채웁니다.
fn model_save(name: &str, id: &str) -> ModelSave {
    ModelSave {
        name: name.to_owned(),
        id: id.to_owned(),
        ..full_model_save()
    }
}

/// `extra`에 예약 키 `key`를 넣은 모델 저장 정보입니다.
fn model_with_extra_key(name: &str, key: &str) -> ModelSave {
    let mut extra = full_extra();
    extra.insert(key.to_owned(), ExtraValue::Number("1".to_owned()));
    ModelSave {
        extra,
        ..model_save(name, "id-1")
    }
}

/// 02-2 읽기 규칙이 거부하는 `extra` 예약 키 목록입니다(구현 상수를 쓰면 순환 검증이 됩니다).
fn validate_reserved_keys() -> [&'static str; 4] {
    ["model", "messages", "stream", "tools"]
}

/// 벤치마크 결론(작업 1, D40)에 맞는 `extra`: 생각 길이·`textVerbosity`는 새 키가 아니라 `extra`로 흡수합니다.
fn full_extra() -> BTreeMap<String, ExtraValue> {
    let mut extra = BTreeMap::new();
    extra.insert("keep_alive".to_owned(), ExtraValue::Number("0".to_owned()));
    extra.insert(
        "textVerbosity".to_owned(),
        ExtraValue::String("high".to_owned()),
    );
    extra.insert(
        "thinking".to_owned(),
        ExtraValue::Object(
            [(
                "budgetTokens".to_owned(),
                ExtraValue::Number("2048".to_owned()),
            )]
            .into_iter()
            .collect(),
        ),
    );
    extra
}
