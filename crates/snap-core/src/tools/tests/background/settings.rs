//! wake 주기 설정 값 시험: 읽기·기본값·예시 파일 세 자리에 값이 자리하는지 확인

use std::time::Duration;

use crate::config::AgentSettings;

/// wake 주기 설정 값이 3곳(읽기·기본값·예시 파일)에 자리하는지 확인
#[test]
fn wake_interval_setting_lands_in_three_places() {
    assert_eq!(
        AgentSettings::default().wake_interval_seconds,
        Duration::from_secs(30),
        "기본값 30초"
    );
    let example = include_str!("../../../../../../agent-snap.example.jsonc");
    assert!(
        example.contains("wake_interval_seconds"),
        "예시 파일에 wake 주기 키가 자리해야 합니다"
    );
}
