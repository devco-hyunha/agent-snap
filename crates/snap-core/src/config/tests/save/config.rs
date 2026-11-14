//! 설정 파일 쓰기 단위 테스트(02-8 작업 2 AC 1~4).
//!
//! 주석이 든 파일에 저장하면 주석과 손대지 않은 항목이 그대로 남고, 02-2 해석·검증으로 다시
//! 읽혀 새 값이 읽히는지 봅니다. 키 값이 설정 파일에 안 들어가는 것도 함께 봅니다.
//! 저장 견본 조립은 지원부(`support/save_samples.rs`)로 옮겨 여러 시험이 함께 쓰게 합니다.

use crate::config::{
    CONFIG_FILE_NAME, ConfigError, EndpointKind, EndpointSave, ModelIssue, Reasoning, ToolCallForm,
    ValidationError, save_endpoint,
};

use super::super::support::{
    ANNOTATED_FILE, BROKEN_TEXT, EMPTY_ENDPOINTS_FILE, EXISTING_MAIN_FILE, HOST, HTTP_TEXT,
    KEY_VALUE, LOCAL_ENDPOINT_TEXT, ROOT_EXTRA_TEXT, TempDir, assert_error_hides_secrets, base_url,
    full_extra, full_save, model_save, model_with_extra_key, parse_ok, reasoning_only_save,
    save_named, validate_reserved_keys,
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
            matches!(&error, ConfigError::Validation(ValidationError::Model(ModelIssue::ExtraReservedKey{ model, key: found }))
                if model == "fast" && found == key),
            "오류가 모델과 키 이름만 밝혀야 합니다: {error}"
        );
        assert_error_hides_secrets("extra 예약 키", &error);
    }
}

/// AC 1(D29): 저장한 `context_window`를 02-2 읽기로 다시 읽으면 같은 값이 읽힌다.
#[test]
fn saved_context_window_roundtrips_to_same_value() {
    let folder = TempDir::new("맥락-창-왕복");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    save_endpoint(folder.path(), &full_save("main", &base_url())).expect("저장이 성공해야 합니다");

    let saved = parse_ok(&folder.read(CONFIG_FILE_NAME));
    // `full_save`의 모델이 32768를 쓰므로, 읽기도 같은 값을 돌려줘 왕복이 성립한다.
    assert_eq!(saved.endpoints[0].models[0].context_window, 32768);
}

/// AC 1(D29): `context_window`를 생략한 저장은 파일에 칸을 만들지 않고, 다시 읽으면 65536이다.
#[test]
fn omitted_context_window_roundtrips_to_default() {
    let folder = TempDir::new("맥락-창-생략-왕복");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    // `reasoning`만 준 저장은 `context_window`를 쓰지 않는다(AC 4의 생략 규칙과 같은 입력).
    save_endpoint(folder.path(), &reasoning_only_save()).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    assert!(
        !text.contains("\"context_window\""),
        "생략한 칸은 쓰면 안 됩니다: {text}"
    );
    assert_eq!(parse_ok(&text).endpoints[0].models[0].context_window, 65536);
}

/// AC 1(D50 결정 7): 저장한 `tool_call_form`를 02-2 읽기로 다시 읽으면 같은 값이 읽힌다.
#[test]
fn saved_tool_call_form_roundtrips_to_same_value() {
    let folder = TempDir::new("호출-형식-왕복");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    save_endpoint(folder.path(), &full_save("main", &base_url())).expect("저장이 성공해야 합니다");

    let saved = parse_ok(&folder.read(CONFIG_FILE_NAME));
    // `full_save`의 모델이 `auto`를 쓰므로, 읽기도 같은 값을 돌려줘 왕복이 성립한다.
    assert_eq!(
        saved.endpoints[0].models[0].tool_call_form,
        ToolCallForm::Auto
    );
}

/// AC 1(D50 결정 7): `tool_call_form`를 생략한 저장은 파일에 칸을 만들지 않고, 다시 읽으면 `openai`다.
#[test]
fn omitted_tool_call_form_roundtrips_to_default() {
    let folder = TempDir::new("호출-형식-생략-왕복");
    folder.write(CONFIG_FILE_NAME, EMPTY_ENDPOINTS_FILE);

    // `reasoning`만 준 저장은 `tool_call_form`를 쓰지 않는다(D29 `context_window`와 같은 자리 규칙).
    save_endpoint(folder.path(), &reasoning_only_save()).expect("저장이 성공해야 합니다");

    let text = folder.read(CONFIG_FILE_NAME);
    assert!(
        !text.contains("\"tool_call_form\""),
        "생략한 칸은 쓰면 안 됩니다: {text}"
    );
    assert_eq!(
        parse_ok(&text).endpoints[0].models[0].tool_call_form,
        ToolCallForm::OpenAi
    );
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
    // D50 결정 7: `full_save`가 준 호출 형식 선언(`auto`)이 그대로 읽혀야 합니다.
    assert_eq!(model.tool_call_form, ToolCallForm::Auto);
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
