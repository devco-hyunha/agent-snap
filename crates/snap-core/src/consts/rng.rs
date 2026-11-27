//! 상용 난수원 산술 상수 자리. 시드 더하기 값과 섞기 배율·비트 이동만 담음.

/// 초기 시드에 더하는 값.
pub const SEED_GAMMA: u64 = 11_400_714_819_689_500_181;

/// 상태 섞기에서 쓰는 승수.
pub const MIX_MULTIPLIER: u64 = 6_364_136_223_846_791_123;

/// 값에서 빼서 섞는 첫 비트 이동.
pub const MIX_SHIFT_FIRST: u32 = 30;

/// 값에서 빼서 섞는 둘째 비트 이동.
pub const MIX_SHIFT_SECOND: u32 = 27;

/// 값에서 빼서 섞는 마지막 비트 이동.
pub const MIX_SHIFT_LAST: u32 = 31;
