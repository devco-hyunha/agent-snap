//! reqwest 연결 오류를 헤더 전 연결 오류 여섯 종류로 나누는 자리. 연결 단계 오류 한 책임만 담음.
//!
//! reqwest는 연결 단계 오류의 세부 종류를 공개 판정으로 주지 않아,
//! 표준 오류 사슬을 따라다니며 종류를 가린다. 사슬의 원문은 오류에 담지 않고 종류만 담는다.
//! 표준 입출력 종류는 문자열이 아니라 `ErrorKind`으로 본다(OS 언어에 상관없이 재현).

use std::error::Error;
use std::io::ErrorKind;

use crate::llm::PreHeaderConnectKind;
use crate::llm::client::error::TransportError;

use reqwest::Error as ReqwestError;

/// reqwest가 문구로만 밝히는 연결 단계 표시. TLS·프록시·재설정 모두 표준 입출력 종류가 없다.
const TLS_MARKS: [&str; 3] = ["tls", "certificate", "handshake"];
const PROXY_MARKS: [&str; 2] = ["proxy", "tunnel"];
const RESET_MARKS: [&str; 5] = [
    "connection closed",
    "connection reset",
    "broken pipe",
    "forcibly closed",
    "aborted",
];

/// reqwest 연결 오류를 헤더 전 연결 오류로 옮김.
///
/// reqwest 오류 문자열에는 주소가 담길 수 있어 그대로 넣지 않고 종류와 설정 초 값만 담음.
pub(super) fn connect_error(error: &ReqwestError, connect_seconds: f64) -> TransportError {
    TransportError::connect(classify(error), connect_seconds)
}

/// reqwest 연결 오류의 세부 종류.
///
/// reqwest 공개 판정(시간 초과·DNS)을 먼저 보고, 표준 입출력 종류(거부·재설정·파이프 절림)를 본다.
/// TLS·프록시는 표준 입출력 종류가 없어 오류 사슬 문구로 가린다.
/// 문구에 없는 마지막 실패는 spec의 「프록시 등」으로 본다.
fn classify(error: &ReqwestError) -> PreHeaderConnectKind {
    if error.is_timeout() {
        return PreHeaderConnectKind::ConnectTimeout;
    }
    if error.is_dns() {
        return PreHeaderConnectKind::Dns;
    }
    let marks = source_marks(error);
    if has_mark(&marks, &TLS_MARKS) {
        return PreHeaderConnectKind::Tls;
    }
    if has_mark(&marks, &PROXY_MARKS) {
        return PreHeaderConnectKind::Proxy;
    }
    if has_mark(&marks, &RESET_MARKS) {
        return PreHeaderConnectKind::Reset;
    }
    if let Some(kind) = io_kind(error) {
        return kind;
    }
    PreHeaderConnectKind::Proxy
}

/// 오류 사슬의 표준 입출력 종류를 헤더 전 연결 종류로 옮김.
///
/// 연결 거부·재설정·파이프 절림·시간 초과만 알고, 그 밖의 종류는 모른다고 본다.
fn io_kind(error: &ReqwestError) -> Option<PreHeaderConnectKind> {
    let mut source = Some(error as &dyn Error);
    while let Some(current) = source {
        if let Some(io) = current.downcast_ref::<std::io::Error>() {
            let kind = match io.kind() {
                ErrorKind::ConnectionRefused => PreHeaderConnectKind::Refused,
                ErrorKind::ConnectionReset
                | ErrorKind::ConnectionAborted
                | ErrorKind::BrokenPipe => PreHeaderConnectKind::Reset,
                ErrorKind::TimedOut => PreHeaderConnectKind::ConnectTimeout,
                _ => return None,
            };
            return Some(kind);
        }
        source = current.source();
    }
    None
}

/// 표준 오류 사슬을 따라다니며 각 단계의 표시 문구를 소문자로 모음.
fn source_marks(error: &ReqwestError) -> Vec<String> {
    let mut marks = Vec::new();
    let mut source = Some(error as &dyn Error);
    while let Some(current) = source {
        marks.push(current.to_string().to_ascii_lowercase());
        source = current.source();
    }
    marks
}

/// 모은 문구 중 하나가 표시를 담았는지.
fn has_mark(marks: &[String], needles: &[&str]) -> bool {
    marks
        .iter()
        .any(|mark| needles.iter().any(|needle| mark.contains(needle)))
}
