//! 코드를 스트림 이벤트로 디코딩하는 파일들: 청크 해석·델타 조립·스트림 디코더·HTTP-date 읽기.
//!
//! 같은 관심사 파일 둘 이상을 한 폴더로 모았습니다(D25). 공개 타입은 `llm.rs`가 그대로
//! 재수출해 공개면을 기존 그대로 유지합니다(D28).

pub(in crate::llm) mod chunk;
pub(in crate::llm) mod decode;
pub(in crate::llm) mod delta;
pub(crate) mod http_date;
