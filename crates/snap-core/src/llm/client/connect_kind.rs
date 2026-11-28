//! 헤더 전 연결 종류 표시 이름 자리. 종류 6종 매핑 한곳만 담음.

use crate::llm::PreHeaderConnectKind;
use crate::locale::transport::CONNECT_KIND_CONNECT_TIMEOUT;
use crate::locale::transport::CONNECT_KIND_DNS;
use crate::locale::transport::CONNECT_KIND_PROXY;
use crate::locale::transport::CONNECT_KIND_REFUSED;
use crate::locale::transport::CONNECT_KIND_RESET;
use crate::locale::transport::CONNECT_KIND_TLS;

/// 헤더 전 연결 종류를 오류 문구에 쓸 표시 이름으로 고르는 매핑.
pub(super) const fn display_name(kind: PreHeaderConnectKind) -> &'static str {
    match kind {
        PreHeaderConnectKind::Dns => CONNECT_KIND_DNS,
        PreHeaderConnectKind::Refused => CONNECT_KIND_REFUSED,
        PreHeaderConnectKind::ConnectTimeout => CONNECT_KIND_CONNECT_TIMEOUT,
        PreHeaderConnectKind::Reset => CONNECT_KIND_RESET,
        PreHeaderConnectKind::Tls => CONNECT_KIND_TLS,
        PreHeaderConnectKind::Proxy => CONNECT_KIND_PROXY,
    }
}
