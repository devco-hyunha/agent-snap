//! 한국어 전송 오류 문구.

/// 헤더를 받기 전 연결 단계의 실패 문구. 종류 이름만 붙임.
pub const CONNECT_FAILED: &str = "헤더를 받기 전 연결에 실패했습니다";

/// 연결 실패 종류 표시. DNS 해석 실패.
pub const CONNECT_KIND_DNS: &str = "dns";

/// 연결 실패 종류 표시. 서버가 연결을 거부함.
pub const CONNECT_KIND_REFUSED: &str = "refused";

/// 연결 실패 종류 표시. 연결 시도 시간 초과.
pub const CONNECT_KIND_CONNECT_TIMEOUT: &str = "connect-timeout";

/// 연결 실패 종류 표시. 헤더를 받기 전에 연결이 끊김.
pub const CONNECT_KIND_RESET: &str = "reset";

/// 연결 실패 종류 표시. TLS 연결 실패.
pub const CONNECT_KIND_TLS: &str = "tls";

/// 연결 실패 종류 표시. 프록시 경유 실패.
pub const CONNECT_KIND_PROXY: &str = "proxy";

/// 서버가 잘못된 HTTP 상태로 응답했을 때 문구. 상태 코드만 붙임.
pub const STATUS_FAILED: &str = "서버가 잘못된 상태로 응답했습니다";

/// 응답 헤더 대기 시간 초과 문구. 설정 초 값만 붙임.
pub const HEADER_WAIT_TIMEOUT: &str = "응답 헤더를 기다리다 시간 초과했습니다";

/// 응답 스트림 유휴 시간 초과 문구. 설정 초 값만 붙임.
pub const STREAM_IDLE_TIMEOUT: &str = "응답 스트림이 멈춰서 시간 초과했습니다";

/// 헤더를 받은 뒤 응답 스트림 조각 읽기 실패 문구. 연결 설정 초 값만 붙임.
pub const STREAM_READ_FAILED: &str = "헤더를 받은 뒤 응답 스트림을 읽다 실패했습니다";

/// 서버가 보낸 오류 청크 문구. 가린 요약만 붙임.
pub const SERVER_ERROR_CHUNK: &str = "서버가 오류를 보냈습니다";

/// 요청 취소 문구.
pub const REQUEST_CANCELLED: &str = "요청을 취소했습니다";

/// 끝난 스트림에 대한 추가 조각 거부 문구.
pub const STREAM_CLOSED: &str = "스트림이 끝남";
