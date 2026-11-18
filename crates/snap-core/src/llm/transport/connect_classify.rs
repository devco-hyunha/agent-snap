//! 헤더 전 연결 오류를 6종 세부로 가르는 분류.
//!
//! io 종류·원시 코드·오류 문자열 열쇠를 앞 열부터 살핌. 텍스트보다 원시 값을 먼저 봄.

use std::io::ErrorKind;

use crate::llm::retry::PreHeaderConnectKind;

use super::os_error_codes::{
    EAI_AGAIN, EAI_FAIL, EAI_HOST_NOT_FOUND, EAI_NO_DATA, WSA_HOST_NOT_FOUND,
    WSA_HOST_NOT_FOUND_DATA,
};

/// 헤더 전 연결 오류를 6종 세부로 가르는 열쇠 표.
///
/// 앞 열부터 살펴 첫 번째로 걸리는 열쇠를 고름.
const CONNECT_KIND_KEYS: &[&[&str]] = &[
    &[
        "dns",
        "getaddrinfo",
        "lookup",
        "no such host",
        "no such name",
        "name: name",
        "server failure",
    ],
    &["refused", "거부"],
    &["timed", "timeout"],
    &["reset", "broken pipe"],
    &["tls", "ssl", "handshake", "certificate", "cert"],
    &["proxy", "프록시"],
];

/// 열쇠 표의 줄 번호를 헤더 전 연결 세부 6종과 연결한 표.
///
/// 시험이 같은 표를 직접 읽게 `pub(crate)`로 둠.
pub(crate) const CONNECT_KIND_BY_ROW: [PreHeaderConnectKind; 6] = [
    PreHeaderConnectKind::DnsFailure,
    PreHeaderConnectKind::ConnectionRefused,
    PreHeaderConnectKind::ConnectTimeout,
    PreHeaderConnectKind::ResetBeforeHeaders,
    PreHeaderConnectKind::Tls,
    PreHeaderConnectKind::Proxy,
];

/// 헤더 전 연결 오류에서 세부 종류를 고름.
///
/// io 오류의 종류와 원시 코드를 앞 열부터 살피고, 열쇠 문자열을 앞 열부터 찾음.
/// 아무것도 없으면 프록시 단계 실패. 주소는 오류에 넣지 않음.
pub(crate) fn classify_connect_error(error: &reqwest::Error) -> PreHeaderConnectKind {
    // 전체 사슬 문구(원시 io 오류 메시지 포함)를 앞 열부터 봄. reqwest의 최상위 문구만
    // 보면 원시 메시지가 안 나오는 운영체체가 있어 사슬 전체를 씀.
    let chain_text = {
        let mut text = String::new();
        let mut current: &dyn std::error::Error = error;
        loop {
            use std::fmt::Write as _;
            let _ = write!(&mut text, "{current}; ");
            match current.source() {
                Some(next) => current = next,
                None => break,
            }
        }
        text.to_lowercase()
    };
    // 원시 io 종류가 정해진 것부터 앞 열 순서. 없으면 원시 코드로 가름.
    for io in raw_io_errors(error) {
        match io.kind() {
            ErrorKind::ConnectionRefused => return PreHeaderConnectKind::ConnectionRefused,
            ErrorKind::TimedOut => return PreHeaderConnectKind::ConnectTimeout,
            ErrorKind::NotFound => return PreHeaderConnectKind::DnsFailure,
            ErrorKind::ConnectionReset | ErrorKind::BrokenPipe => {
                return PreHeaderConnectKind::ResetBeforeHeaders;
            }
            _ if matches!(
                io.raw_os_error(),
                Some(WSA_HOST_NOT_FOUND)
                    | Some(WSA_HOST_NOT_FOUND_DATA)
                    | Some(EAI_HOST_NOT_FOUND)
                    | Some(EAI_NO_DATA)
                    | Some(EAI_FAIL)
                    | Some(EAI_AGAIN)
            ) =>
            {
                return PreHeaderConnectKind::DnsFailure;
            }
            _ => {}
        }
    }
    for (row, keys) in CONNECT_KIND_KEYS.iter().enumerate() {
        if keys.iter().any(|key| chain_text.contains(key)) {
            return CONNECT_KIND_BY_ROW[row];
        }
    }
    PreHeaderConnectKind::Proxy
}

/// reqwest 오류 사슬에 있는 원시 io 오류를 순서대로 모음.
///
/// 라이브러리가 주소 문자열을 넣을 수 있어, 텍스트보다 원시 값으로 먼저 가림.
fn raw_io_errors(error: &reqwest::Error) -> Vec<&std::io::Error> {
    let mut found = Vec::new();
    let mut source = std::error::Error::source(error);
    while let Some(current) = source {
        if let Some(io) = current.downcast_ref::<std::io::Error>() {
            found.push(io);
        }
        source = current.source();
    }
    found
}
