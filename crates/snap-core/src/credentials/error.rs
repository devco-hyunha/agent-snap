//! 자격 파일 해석 오류 타입만 두는 자리입니다(03-6 R1 감사 분리).
//!
//! 해석과 조회는 `mod.rs`가 맡고, 이 파일은 그 결과로 나오는 오류 타입만 담습니다
//! (dev-principles 2절 SRP, `config/error/`와 같은 자리 규칙).

/// 자격 파일 해석에 실패했을 때 나오는 오류입니다.
///
/// 모든 오류는 키 값(원문)을 담지 않습니다. Display와 Debug에 값이 드러나지 않음을
/// 단위 테스트로 확인합니다(명세 AC 5).
#[derive(Debug, thiserror::Error)]
pub enum CredentialsError {
    /// JSONC 문법 오류입니다. 줄 번호만 담고 줄 내용과 값은 담지 않습니다.
    #[error("자격 파일 해석 실패: JSONC 문법 오류 (줄 {line}번)")]
    JsoncSyntax { line: usize },
    /// 자격 파일이 최상위 객체가 아닙니다.
    #[error("자격 파일 해석 실패: 최상위가 객체가 아닙니다")]
    RootNotObject,
    /// `api_keys` 자체가 객체가 아닙니다. 어떤 값도 담지 않습니다.
    #[error("자격 파일 해석 실패: `api_keys`가 객체가 아닙니다")]
    ApiKeysNotObject,
    /// `api_keys` 아래 항목의 값이 문자열이 아닙니다. 엔드포인트 이름만 담습니다.
    #[error("자격 파일 해석 실패: 엔드포인트 `{endpoint}`의 키 값이 문자열이 아닙니다")]
    KeyNotString { endpoint: String },
}

impl From<crate::jsonc::JsoncError> for CredentialsError {
    fn from(error: crate::jsonc::JsoncError) -> Self {
        match error {
            crate::jsonc::JsoncError::Syntax { line } => Self::JsoncSyntax { line },
            crate::jsonc::JsoncError::RootNotObject => Self::RootNotObject,
        }
    }
}
