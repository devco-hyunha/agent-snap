//! 요청에 실리는 값 중 샘플링·추가 값 타입입니다.
//!
//! 값은 읽는 시점에 검증되므로 이후 단계는 그대로 믿고 씁니다.

use std::collections::BTreeMap;

/// 샘플링 값입니다. 적지 않은 항목은 `None`이며 요청 본문에서 빼 서버 기본을 따릅니다(D14).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sampling {
    /// 0 이상 2 이하.
    pub temperature: Option<f64>,
    /// 0 초과 1 이하.
    pub top_p: Option<f64>,
    /// 1 이상 정수.
    pub max_tokens: Option<u64>,
}

/// 모델 `extra`에 적힌 JSON 값입니다. 하위 객체까지 그대로 담습니다(D15).
#[derive(Debug, Clone, PartialEq)]
pub enum ExtraValue {
    /// `null`.
    Null,
    /// 불리언.
    Bool(bool),
    /// 숫자입니다. 적힌 모양 그대로 문자열로 보관합니다.
    Number(String),
    /// 문자열.
    String(String),
    /// 배열.
    Array(Vec<ExtraValue>),
    /// 객체.
    Object(BTreeMap<String, ExtraValue>),
}
