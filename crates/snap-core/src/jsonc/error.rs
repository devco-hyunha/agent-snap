//! JSONC 해석 오류 타입만 두는 자리.
//!
//! 해석 헬퍼는 `mod.rs`가 맡고, 이 파일은 그 결과로 나오는 오류 타입만 담음
//! (dev-principles 2절 SRP, `config/error/`·`credentials/error.rs`와 같은 자리 규칙).

/// JSONC 텍스트를 객체로 읽지 못했을 때의 오류. 입력 값은 담지 않음.
#[derive(Debug)]
pub(crate) enum JsoncError {
    /// 문법 오류. 줄 번호만 담고 줄 내용과 값은 담지 않음.
    Syntax { line: usize },
    /// 최상위가 객체가 아닌 상태(빈 텍스트 포함).
    RootNotObject,
}

/// 칸이 있지만 기대한 JSON 타입이 아닌 상태.
#[derive(Debug)]
pub(crate) struct WrongType;
