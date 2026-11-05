#![allow(dead_code, unreachable_pub)]

//! 모의 서버가 본문을 도중에 끊을 때 쓰는 끊김 표식 타입입니다.
//!
//! 본문이 오류로 끝나면 hyper가 본문을 마무리하지 않고 연결을 닫아, 받는 쪽에서는 스트림 도중 끊김으로 보입니다.
//! 본문 스트림 구현(`handler`)이 이 값을 씁니다.

use std::error::Error;
use std::fmt;

/// 본문을 마무리 없이 끊는 표식입니다. 값 자체는 "연결 닫힘"을 나타냅니다.
pub(super) struct BodyDisconnected;

impl fmt::Display for BodyDisconnected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("모의 서버 본문 연결 끊김")
    }
}

impl fmt::Debug for BodyDisconnected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BodyDisconnected")
    }
}

impl Error for BodyDisconnected {}
