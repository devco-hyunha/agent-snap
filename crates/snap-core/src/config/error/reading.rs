//! 설정 파일 읽기·해석의 실패 이유만 담는 열거형입니다.

use std::fmt;
use std::path::PathBuf;

use crate::credentials::CredentialsError;
use crate::jsonc::JsoncError;

/// 설정 파일을 찾고, 읽고, 해석하는 단계가 실패했을 때의 오류입니다.
///
/// 읽기 전용이며, 파일 내용을 따지지 않고 단계가 실패한 이유만 담습니다(검증과 저장은
/// 각각 `error/validation.rs`·`error/save.rs`가 맡습니다). Display·Debug에 base_url 호스트와
/// 키 값, 경로 전체를 담지 않습니다(명세 AC 2·4·5).
#[derive(thiserror::Error)]
pub enum ReadingError {
    /// JSONC 문법 오류입니다. 줄 번호만 담고 줄 내용과 값은 담지 않습니다.
    #[error("설정 파일 해석 실패: JSONC 문법 오류 (줄 {line}번)")]
    JsoncSyntax { line: usize },
    /// 설정 파일이 최상위 객체가 아닙니다.
    #[error("설정 파일 해석 실패: 최상위가 객체가 아닙니다")]
    RootNotObject,
    /// 설정 폴더에서 파일을 읽지 못했습니다. 폴더 경로와 파일 이름만 담습니다.
    #[error("설정 파일 접근 실패: `{folder}`의 `{file}`")]
    Io {
        folder: PathBuf,
        file: &'static str,
        #[source]
        source: std::io::Error,
    },
    /// 설정 폴더 위치를 계산할 환경 정보가 부족합니다. 경로는 담지 않습니다.
    #[error("설정 폴더 위치를 정할 수 없습니다: 환경 정보가 부족합니다")]
    ConfigDirUnknown,
    /// 자격 파일 해석 오류를 그대로 드러냅니다(키 값은 담지 않음).
    #[error(transparent)]
    Credentials(#[from] CredentialsError),
}

impl From<JsoncError> for ReadingError {
    fn from(error: JsoncError) -> Self {
        match error {
            JsoncError::Syntax { line } => Self::JsoncSyntax { line },
            JsoncError::RootNotObject => Self::RootNotObject,
        }
    }
}

impl fmt::Debug for ReadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display와 같은 공개 문구만 남겨 실수로 내부 값이 드러나지 않게 합니다.
        write!(f, "ReadingError({})", self)
    }
}
