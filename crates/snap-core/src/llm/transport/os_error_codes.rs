//! OS가 헤더 전 연결 단계에서 내는 원시 오류 코드 상수입니다(03-6 R2 감사 분리).
//!
//! 코드 값은 OS별로 바뀌므로 판별([`super::connect_classify`])과 따로 둡니다(dev-principles 5절).

/// Windows DNS 조회 실패 원시 코드입니다.
pub(crate) const WSA_HOST_NOT_FOUND: i32 = 11001;
/// Windows 주소는 있으나 데이터가 없는 원시 코드입니다.
pub(crate) const WSA_HOST_NOT_FOUND_DATA: i32 = 11004;
// Linux에서 주소 이름 풀이가 실패하면 원시 코드가 없이 종류만 없는 io 오류로 온다(실측: 111·5·7·4).
/// Linux DNS 풀이 실패 원시 코드입니다.
pub(crate) const EAI_HOST_NOT_FOUND: i32 = 111;
/// Linux 데이터 없음 원시 코드입니다.
pub(crate) const EAI_NO_DATA: i32 = 5;
/// Linux 풀이 자체 실패 원시 코드입니다.
pub(crate) const EAI_FAIL: i32 = 4;
/// Linux 잠시 후 재시도 원시 코드입니다.
pub(crate) const EAI_AGAIN: i32 = 7;
