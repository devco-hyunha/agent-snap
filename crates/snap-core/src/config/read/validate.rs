//! 값 하나에 대한 순수 검증 규칙입니다(D14, D15, D19, D20, D21). 키 목록 중 객체에 있는 칸 확인
//! (`first_present_key`, 값은 읽지 않음)도 이 모듈이 맡습니다. 구조 읽기는 `parse`·`sampling`·
//! `http` 모듈이 맡습니다.

use std::time::Duration;

use jsonc_parser::JsonObject;

/// 모델 `extra`에 적을 수 없는, 클라이언트가 채우는 요청 키입니다(D15).
pub(crate) const RESERVED_EXTRA_KEYS: [&str; 4] = ["model", "messages", "stream", "tools"];

/// 엔드포인트에 적으면 거부하는 키 칸입니다(D10, D20).
pub(in crate::config) const ENDPOINT_KEY_FIELDS: [&str; 2] = ["api_key", "api_key_env"];

/// 엔드포인트·모델 이름이 규칙을 지키지 못한 이유입니다. 오류 종류는 호출한 쪽이 정합니다.
pub(in crate::config) enum NameRejection {
    /// trim 후 비어 있습니다.
    Empty,
    /// trim 후 `/`가 들어 있습니다. 오류에 밝힐 trim한 이름을 담습니다.
    HasSlash { name: String },
}

/// `keys` 중 객체에 칸이 있는 첫 키를 돌려줍니다. 값은 읽지 않고 칸이 있는지만 봅니다.
pub(in crate::config) fn first_present_key<'k>(
    object: &JsonObject<'_>,
    keys: &[&'k str],
) -> Option<&'k str> {
    keys.iter().copied().find(|key| object.get(key).is_some())
}

/// 앞뒤 공백을 자르고 비어 있지 않으면 잘라낸 값을 돌려줍니다(D20).
pub(in crate::config) fn trimmed_non_empty(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

/// 엔드포인트·모델 이름 규칙을 확인하고 trim한 이름을 돌려줍니다(D20).
///
/// trim 후 비어 있지 않아야 하며 `/`를 포함하면 안 됩니다. `/`는 선택 키
/// `엔드포인트이름/모델이름`의 구분자입니다.
pub(in crate::config) fn checked_name(raw: &str) -> Result<String, NameRejection> {
    let name = trimmed_non_empty(raw).ok_or(NameRejection::Empty)?;
    if name.contains('/') {
        return Err(NameRejection::HasSlash { name });
    }
    Ok(name)
}

/// `base_url`이 D19 규칙을 지키는지 봅니다.
///
/// scheme은 `http`/`https`만, 빈 문자열·앞뒤 공백·끝 슬래시는 거부합니다. 호스트와 경로(`/v1`
/// 포함)는 검사하지 않습니다.
pub(in crate::config) fn is_valid_base_url(raw: &str) -> bool {
    if raw.is_empty() || raw != raw.trim() || raw.ends_with('/') {
        return false;
    }
    let Some((scheme, _)) = raw.split_once("://") else {
        return false;
    };
    scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https")
}

/// `temperature`는 0 이상 2 이하입니다(D14).
/// 03-2가 도구 호출 턴 온도를 같은 규칙으로 검사하게 `pub(crate)`로 열린다(02-2 규칙 재사용).
pub(crate) fn is_valid_temperature(value: &f64) -> bool {
    (0.0..=2.0).contains(value)
}

/// `top_p`는 0 초과 1 이하입니다(D14).
pub(in crate::config) fn is_valid_top_p(value: &f64) -> bool {
    *value > 0.0 && *value <= 1.0
}

/// `max_tokens`는 1 이상입니다(D14). 정수 여부는 읽을 때 정수로 해석하며 확인합니다.
pub(in crate::config) fn is_valid_max_tokens(value: &u64) -> bool {
    *value >= 1
}

/// `context_window`는 1 이상입니다(D29). 0은 창이 없다는 뜻이라 쓰지 못하게 막습니다.
pub(in crate::config) fn is_valid_context_window(value: &u64) -> bool {
    *value >= 1
}

/// `http` 시간 값은 0보다 커야 합니다(D21). 0초 대기·0초 제한은 요청을 즉시 실패시킵니다.
pub(in crate::config) fn is_positive_duration(value: &Duration) -> bool {
    !value.is_zero()
}

/// 백오프 시작값은 상한보다 클 수 없습니다(D13, D21).
pub(in crate::config) fn is_valid_backoff_range(min: Duration, max: Duration) -> bool {
    min <= max
}
