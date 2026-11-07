//! 설정 테스트 공용 도구: 입력 텍스트 조립, 임시 폴더, 거부 사례 표.
//!
//! 주소와 키는 값이 새는지 확인하기 위한 가짜 표지만 씁니다.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::config::{
    Config, ConfigEnv, ConfigError, EndpointKind, EndpointSave, HostOs, ReadingError, SaveError,
    ValidationError,
};
use crate::credentials::Credentials;

/// 설정 파일 쪽 가리기 검사용 가짜 호스트입니다. `base_url`이 들어간 검사의 가리기 대상으로만
/// 씁니다(자격 파일 저장은 주소를 다루지 않으므로 파일 이름·키 값 검사에 쓰지 않습니다).
pub(crate) const HOST: &str = "leak-check-host.invalid";
/// 가리기 검사용 가짜 키 값입니다. 오류와 파일에서 키 값이 사라지는지 보는 검사에 씁니다.
pub(crate) const KEY_VALUE: &str = "leak-key-value-9";

/// 사례 라벨: 최상위 옛 `models` 거부입니다. 가림 테스트가 같은 사례를 찾을 때 씁니다.
pub(crate) const LEGACY_MODELS_LABEL: &str = "최상위 옛 models";
/// 사례 라벨: 모델의 `endpoint` 칸 거부입니다. 가림 테스트가 같은 사례를 찾을 때 씁니다.
pub(crate) const MODEL_ENDPOINT_FIELD_LABEL: &str = "모델의 endpoint 칸";

/// 모든 환경 정보가 비어 있는 설정 폴더 계산 입력입니다.
pub(crate) fn empty_env(os: HostOs) -> ConfigEnv {
    ConfigEnv {
        os,
        config_dir_override: None,
        appdata: None,
        xdg_config_home: None,
        home: None,
    }
}

/// 정상 `base_url` 값입니다.
pub(crate) fn base_url() -> String {
    format!("https://{HOST}/v1")
}

/// `kind`를 정해 만든 엔드포인트 객체 하나입니다. `tail`은 `kind` 뒤에 그대로 붙는 칸들입니다
/// (`,"x":1` 형태).
pub(crate) fn endpoint_object_with_kind(
    name: &str,
    base_url: &str,
    kind: &str,
    tail: &str,
) -> String {
    format!(r#"{{"name":"{name}","base_url":"{base_url}","kind":"{kind}"{tail}}}"#)
}

/// `kind`가 `openai`인 엔드포인트 객체 하나입니다. `tail`은 `kind` 뒤에 그대로 붙는 칸들입니다.
pub(crate) fn endpoint_object(name: &str, base_url: &str, tail: &str) -> String {
    endpoint_object_with_kind(name, base_url, "openai", tail)
}

/// 모델 객체 하나입니다. `tail`은 `id` 뒤에 그대로 붙는 칸들입니다.
pub(crate) fn model_object(name: &str, id: &str, tail: &str) -> String {
    format!(r#"{{"name":"{name}","id":"{id}"{tail}}}"#)
}

/// 모델 객체들을 `models` 칸으로 담은 엔드포인트 `tail`입니다(`,"models":[…]` 형태).
pub(crate) fn models_tail(models: &[String]) -> String {
    format!(r#","models":[{}]"#, models.join(","))
}

/// 엔드포인트 객체들(쉼표로 이은 것)을 `endpoints`에 담고 루트 칸 `root_tail`(`,"x":1` 형태)을
/// 더한 설정 텍스트입니다.
fn config_with_endpoints_and_root_tail(endpoints: &[String], root_tail: &str) -> String {
    format!(r#"{{"endpoints":[{}]{root_tail}}}"#, endpoints.join(","))
}

/// 엔드포인트 객체들(쉼표로 이은 것)을 `endpoints`에 담은 설정 텍스트입니다.
pub(crate) fn config_of(endpoints: &[String]) -> String {
    config_with_endpoints_and_root_tail(endpoints, "")
}

/// 엔드포인트 `main` 하나에 모델 객체들을 담은 설정입니다.
pub(crate) fn config_with_models(models: &[String]) -> String {
    config_of(&[endpoint_object("main", &base_url(), &models_tail(models))])
}

/// 모델 하나 `fast`가 든 엔드포인트 `main` 객체입니다. `endpoint_tail`은 엔드포인트 객체에
/// 더하는 칸입니다.
fn main_endpoint_with_fast_model(endpoint_tail: &str) -> String {
    let models = models_tail(&[model_object("fast", "id-1", "")]);
    endpoint_object("main", &base_url(), &format!("{models}{endpoint_tail}"))
}

/// 모델 하나 `fast`가 든 엔드포인트 `main` 하나의 설정입니다. `endpoint_tail`은 엔드포인트
/// 객체에 더하는 칸입니다.
pub(crate) fn config_with_endpoint_tail(endpoint_tail: &str) -> String {
    config_of(&[main_endpoint_with_fast_model(endpoint_tail)])
}

/// 엔드포인트 `main`의 `kind` 칸이 없는 설정입니다.
pub(crate) fn config_without_kind() -> String {
    config_of(&[format!(r#"{{"name":"main","base_url":"{}"}}"#, base_url())])
}

/// 엔드포인트 `main` 아래 모델 `fast`에 `id` 칸이 없는 설정입니다.
pub(crate) fn config_without_model_id() -> String {
    config_with_models(&[r#"{"name":"fast"}"#.to_owned()])
}

/// 엔드포인트 `main` 아래 모델 `fast` 하나가 있고, 모델 객체에 `model_tail` 칸을 더한 설정입니다.
pub(crate) fn config_with_model_tail(model_tail: &str) -> String {
    config_with_models(&[model_object("fast", "id-1", model_tail)])
}

/// 정상 엔드포인트에 루트 칸(`root_tail`, `,"x":1` 형태)을 더한 설정입니다.
pub(crate) fn config_with_root_tail(root_tail: &str) -> String {
    config_with_endpoints_and_root_tail(&[main_endpoint_with_fast_model("")], root_tail)
}

/// 정상 설정을 읽습니다.
pub(crate) fn parse_ok(text: &str) -> Config {
    Config::parse(text).unwrap_or_else(|error| panic!("정상 설정이어야 합니다: {error}"))
}

/// 거부되어야 하는 설정을 읽어 오류를 돌려줍니다.
pub(crate) fn parse_err(text: &str) -> ConfigError {
    match Config::parse(text) {
        Ok(config) => panic!("거부되어야 합니다: {config:?}"),
        Err(error) => error,
    }
}

/// 오류의 Display·Debug에 지정한 값들이 없는지 확인합니다. 가리기 확인 순서를 여러 파일에
/// 나눠 적지 않게 하려는 자리입니다.
pub(crate) fn assert_error_hides(label: &str, error: &ConfigError, secrets: &[&str]) {
    for (kind, text) in [
        ("Display", error.to_string()),
        ("Debug", format!("{error:?}")),
    ] {
        for secret in secrets {
            assert!(
                !text.contains(secret),
                "{label}: {kind}에 `{secret}`가 있습니다: {text}"
            );
        }
    }
}

/// 오류의 Display·Debug에 호스트와 키 값이 없는지 확인합니다.
pub(crate) fn assert_error_hides_secrets(label: &str, error: &ConfigError) {
    assert_error_hides(label, error, &[HOST, KEY_VALUE]);
}

/// 키 값 없이 저장하는 엔드포인트 저장 정보입니다(모델 없음). 설정 파일 저장 쪽 견본에서
/// 여러 파일이 같은 조립을 다시 적지 않게 하려는 자리입니다.
pub(crate) fn endpoint_save(name: &str, base_url: &str) -> EndpointSave {
    EndpointSave {
        name: name.to_owned(),
        base_url: base_url.to_owned(),
        kind: EndpointKind::OpenAi,
        models: Vec::new(),
    }
}

/// 02-1 해석을 왕복용으로 다시 읽습니다.
pub(crate) fn parse_credentials(text: &str) -> Credentials {
    Credentials::parse(text).expect("02-1 해석이 통과해야 합니다")
}

/// 02-1 해석이 읽은 키 값(trim한 이름 기준)을 문자열로 꺼냅니다.
pub(crate) fn key_value(credentials: &Credentials, endpoint: &str) -> Option<String> {
    credentials
        .find_key(endpoint)
        .map(|secret| secret.reveal().to_owned())
}

/// 자격 파일 텍스트에서 특정 키 이름으로 쓰인 칸의 개수를 셉니다(줄 시작 기준).
pub(crate) fn count_key_entries(text: &str, endpoint: &str) -> usize {
    let prefix = format!(r#""{endpoint}":"#);
    text.lines()
        .filter(|line| line.trim_start().starts_with(&prefix))
        .count()
}

/// 검증·저장 경로가 표준대로 파일을 옮기는 교체 지점입니다.
pub(crate) fn standard_replacement(from: &Path, target: &Path) -> std::io::Result<()> {
    std::fs::rename(from, target)
}

/// 두 파일을 함께 고치는 저장이 밝히는 어긋난 상태 변형들을 견본으로 만듭니다(D40 규칙 3).
/// 가림 테스트 여러 파일에서 같은 견본을 다시 적지 않게 하려는 자리입니다.
pub(crate) fn side_effect_errors() -> Vec<(&'static str, ConfigError)> {
    vec![
        (
            "이름 바꾸기 어긋난 상태",
            ConfigError::Save(SaveError::EndpointRenameAfterKeySide { source: None }),
        ),
        (
            "삭제 어긋난 상태",
            ConfigError::Save(SaveError::EndpointRemoveAfterKeySide { source: None }),
        ),
    ]
}

/// `target`의 파일 이름이 `blocked`와 같으면 교체를 막고, 나머지는 표준대로 옮깁니다.
/// 교체 실패를 보는 테스트가 여러 파일에서 같은 지점을 다시 적지 않게 하려는 자리입니다.
pub(crate) fn blocking_replacement(
    blocked: &'static str,
) -> impl Fn(&Path, &Path) -> std::io::Result<()> {
    move |from: &Path, target: &Path| {
        // `file_name()`는 `OsStr`을 돌려주므로 같은 종류로 비교합니다.
        let blocked_name = Path::new(blocked).as_os_str();
        if target.file_name() == Some(blocked_name) {
            return Err(std::io::Error::other("교체 시험"));
        }
        std::fs::rename(from, target)
    }
}

/// 거부 사례 하나입니다. 같은 표를 검증 테스트와 가림 테스트가 함께 씁니다.
pub(crate) struct RejectionCase {
    pub(crate) label: &'static str,
    pub(crate) text: String,
    pub(crate) is_expected: Box<dyn Fn(&ConfigError) -> bool>,
}

fn case(
    label: &'static str,
    text: String,
    is_expected: impl Fn(&ConfigError) -> bool + 'static,
) -> RejectionCase {
    RejectionCase {
        label,
        text,
        is_expected: Box::new(is_expected),
    }
}

fn endpoint_with_url(base_url: &str) -> String {
    config_of(&[endpoint_object("main", base_url, "")])
}

/// 모델 `fast`의 샘플링 항목 `item`에 `value`(JSON 값 텍스트)를 적은 범위 위반 사례입니다.
/// 오류가 같은 항목 이름을 밝히는지 확인합니다.
fn sampling_case(label: &'static str, item: &'static str, value: &str) -> RejectionCase {
    case(
        label,
        config_with_model_tail(&format!(r#","sampling":{{"{item}":{value}}}"#)),
        move |error| {
            matches!(error, ConfigError::Validation(ValidationError::SamplingOutOfRange{ item: found, model })
                if found == item && model == "fast")
        },
    )
}

/// 모델 `fast`의 `extra`에 예약 키 `key`를 적은 사례입니다. 오류가 같은 키를 밝히는지 확인합니다.
fn reserved_key_case(key: &'static str) -> RejectionCase {
    case(
        "extra 예약 키",
        config_with_model_tail(&format!(r#","extra":{{"{key}":"{KEY_VALUE}"}}"#)),
        move |error| {
            matches!(error, ConfigError::Validation(ValidationError::ExtraReservedKey{ model, key: found })
                if model == "fast" && found == key)
        },
    )
}

/// 명세 AC 2의 거부 항목을 한 줄씩 담은 표입니다. 모든 입력에 가짜 호스트가 들어 있습니다.
pub(crate) fn rejection_cases() -> Vec<RejectionCase> {
    let two_main = config_of(&[
        endpoint_object("main", &base_url(), ""),
        endpoint_object("main", &base_url(), ""),
    ]);
    let two_main_trimmed = config_of(&[
        endpoint_object("main", &base_url(), ""),
        endpoint_object("  main  ", &base_url(), ""),
    ]);
    let two_fast = config_with_models(&[
        model_object("fast", "id-1", ""),
        model_object(" fast ", "id-2", ""),
    ]);
    let mut cases = vec![
        case(
            "엔드포인트 이름 중복",
            two_main,
            |e| matches!(e, ConfigError::Validation(ValidationError::DuplicateEndpoint{ endpoint }) if endpoint == "main"),
        ),
        case(
            "엔드포인트 이름 중복(trim 후)",
            two_main_trimmed,
            |e| matches!(e, ConfigError::Validation(ValidationError::DuplicateEndpoint{ endpoint }) if endpoint == "main"),
        ),
        case(
            "같은 엔드포인트의 모델 이름 중복",
            two_fast,
            |e| matches!(e, ConfigError::Validation(ValidationError::DuplicateModel{ endpoint, model }) if endpoint == "main" && model == "fast"),
        ),
        case(
            "엔드포인트 api_key 칸",
            config_with_endpoint_tail(&format!(r#","api_key":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::EndpointKeyField{ endpoint }) if endpoint == "main"),
        ),
        case(
            "엔드포인트 api_key_env 칸",
            config_with_endpoint_tail(&format!(r#","api_key_env":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::EndpointKeyField{ endpoint }) if endpoint == "main"),
        ),
        case(
            LEGACY_MODELS_LABEL,
            config_with_root_tail(&format!(
                r#","models":[{{"name":"x","id":"{KEY_VALUE}","endpoint":"main"}}]"#
            )),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::LegacyTopLevelModels)
                )
            },
        ),
        case(
            MODEL_ENDPOINT_FIELD_LABEL,
            config_with_model_tail(&format!(r#","endpoint":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::ModelEndpointField{ model }) if model == "fast"),
        ),
        case(
            "빈 엔드포인트 이름",
            config_of(&[endpoint_object("   ", &base_url(), "")]),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::EmptyEndpointName)
                )
            },
        ),
        case(
            "빈 모델 이름",
            config_with_models(&[model_object("  ", "id-1", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::EmptyModelName{ endpoint }) if endpoint == "main"),
        ),
        case(
            "빈 모델 id",
            config_with_models(&[model_object("fast", "  ", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::EmptyModelId{ endpoint, model }) if endpoint == "main" && model == "fast"),
        ),
        case(
            "엔드포인트 이름의 슬래시",
            config_of(&[endpoint_object(" a/b ", &base_url(), "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::EndpointNameHasSlash{ endpoint }) if endpoint == "a/b"),
        ),
        case(
            "모델 이름의 슬래시",
            config_with_models(&[model_object("a/b", "id-1", "")]),
            |e| matches!(e, ConfigError::Validation(ValidationError::ModelNameHasSlash{ endpoint, model }) if endpoint == "main" && model == "a/b"),
        ),
        case(
            "지원하지 않는 kind",
            config_of(&[endpoint_object_with_kind(
                "main",
                &base_url(),
                KEY_VALUE,
                "",
            )]),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidKind{ endpoint }) if endpoint == "main"),
        ),
        case(
            "지원하지 않는 reasoning",
            config_with_model_tail(&format!(r#","reasoning":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidReasoning{ model }) if model == "fast"),
        ),
        // D29: `context_window`는 1 이상 정수만 받아들입니다. 값은 오류에 밝히지 않습니다.
        case(
            "context_window가 0",
            config_with_model_tail(r#","context_window":0"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidContextWindow{ model }) if model == "fast"),
        ),
        case(
            "context_window가 음수",
            config_with_model_tail(r#","context_window":-1"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidContextWindow{ model }) if model == "fast"),
        ),
        case(
            "context_window가 정수 아님",
            config_with_model_tail(r#","context_window":1.5"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidContextWindow{ model }) if model == "fast"),
        ),
        case(
            "context_window가 숫자 아님",
            config_with_model_tail(&format!(r#","context_window":"{KEY_VALUE}""#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidContextWindow{ model }) if model == "fast"),
        ),
        case(
            "context_window가 범위 초과",
            config_with_model_tail(r#","context_window":18446744073709551616"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidContextWindow{ model }) if model == "fast"),
        ),
        sampling_case("temperature 범위 초과", "temperature", "2.5"),
        sampling_case("temperature 음수", "temperature", "-0.1"),
        sampling_case("top_p가 0", "top_p", "0"),
        sampling_case("top_p가 1 초과", "top_p", "1.01"),
        sampling_case("max_tokens가 0", "max_tokens", "0"),
        sampling_case("max_tokens가 정수 아님", "max_tokens", "1.5"),
        sampling_case("max_tokens가 숫자 아님", "max_tokens", r#""many""#),
        case(
            "모르는 샘플링 항목",
            config_with_model_tail(&format!(r#","sampling":{{"seed":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::UnknownSamplingItem{ item }) if item == "seed"),
        ),
        case(
            "루트 sampling 범위 초과",
            config_with_root_tail(r#","sampling":{"top_p":5}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::SamplingOutOfRange{ item, model }) if item == "top_p" && model == "기본"),
        ),
        case(
            "모르는 http 항목(옛 이름)",
            config_with_root_tail(&format!(r#","http":{{"max_retries":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::UnknownHttpItem{ item }) if item == "max_retries"),
        ),
        case(
            "http 값이 음수",
            config_with_root_tail(r#","http":{"connect_seconds":-1}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.connect_seconds"),
        ),
        case(
            "http 값이 0",
            config_with_root_tail(r#","http":{"connect_seconds":0}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.connect_seconds"),
        ),
        case(
            "http 값이 무한대",
            config_with_root_tail(r#","http":{"stream_idle_seconds":1e999}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.stream_idle_seconds"),
        ),
        case(
            "http 값이 나노초 미만(0으로 취급)",
            config_with_root_tail(r#","http":{"connect_seconds":1e-10}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.connect_seconds"),
        ),
        case(
            "http retries가 음수",
            config_with_root_tail(r#","http":{"retries":-1}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.retries"),
        ),
        case(
            "http 값이 숫자 아님",
            config_with_root_tail(&format!(r#","http":{{"backoff_min":"{KEY_VALUE}"}}"#)),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.backoff_min"),
        ),
        case(
            "backoff_min이 backoff_max보다 큼",
            config_with_root_tail(r#","http":{"backoff_min":40}"#),
            |e| {
                matches!(
                    e,
                    ConfigError::Validation(ValidationError::BackoffRangeInverted)
                )
            },
        ),
        case(
            "최상위가 객체가 아님",
            format!(r#"["{HOST}"]"#),
            |e| matches!(e, ConfigError::Reading(ReadingError::RootNotObject)),
        ),
        case("빈 파일", String::new(), |e| {
            matches!(e, ConfigError::Reading(ReadingError::RootNotObject))
        }),
        case(
            "http retries가 정수 아님",
            config_with_root_tail(r#","http":{"retries":1.5}"#),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidField{ field }) if field == "http.retries"),
        ),
    ];
    // 명세 AC 2의 기대값이라 구현 상수와 따로 적습니다(구현 상수를 쓰면 순환 검증이 됩니다).
    cases.extend(
        ["model", "messages", "stream", "tools"]
            .into_iter()
            .map(reserved_key_case),
    );
    for (label, url) in [
        ("base_url scheme이 ftp", format!("ftp://{HOST}/v1")),
        ("base_url에 scheme 없음", format!("{HOST}/v1")),
        ("base_url이 빈 문자열", String::new()),
        ("base_url 앞 공백", format!(" https://{HOST}/v1")),
        ("base_url 뒤 공백", format!("https://{HOST}/v1 ")),
        ("base_url 끝 슬래시", format!("https://{HOST}/v1/")),
    ] {
        cases.push(case(
            label,
            endpoint_with_url(&url),
            |e| matches!(e, ConfigError::Validation(ValidationError::InvalidBaseUrl{ endpoint }) if endpoint == "main"),
        ));
    }
    cases
}

/// 테스트가 끝나면 지워지는 임시 폴더입니다.
pub(crate) struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// 이 프로세스 안에서 겹치지 않는 임시 폴더를 만듭니다.
    pub(crate) fn new(label: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "agent-snap-test-{}-{unique}-{label}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|error| panic!("임시 폴더를 만들지 못했습니다: {error}"));
        Self { path }
    }

    /// 폴더 경로입니다.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// 폴더 안의 텍스트 파일을 읽어 돌려줍니다.
    pub(crate) fn read(&self, file: &str) -> String {
        std::fs::read_to_string(self.path.join(file))
            .unwrap_or_else(|error| panic!("임시 파일을 읽지 못했습니다: {error}"))
    }

    /// 폴더 안에 파일을 씁니다.
    pub(crate) fn write(&self, file: &str, text: &str) {
        std::fs::write(self.path.join(file), text)
            .unwrap_or_else(|error| panic!("임시 파일을 쓰지 못했습니다: {error}"));
    }

    /// 폴더에 남은 `.tmp` 파일이 하나도 없는지 확인합니다.
    pub(crate) fn assert_no_temp_files(&self) {
        let leftovers: Vec<String> = std::fs::read_dir(self.path())
            .expect("임시 폴더를 읽을 수 있어야 합니다")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".tmp"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "임시 파일이 남으면 안 됩니다: {leftovers:?}"
        );
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // 정리 실패는 테스트 결과에 영향이 없으므로 무시합니다.
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
