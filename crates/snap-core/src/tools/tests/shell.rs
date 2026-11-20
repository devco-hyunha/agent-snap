//! 셸 대기·표시 명령 문자열 재료 정본
//!
//! 두 운영체제 셸이 같은 의미로 기다리게 하는 명령 문자열만 담음

#![allow(dead_code)]

/// 셸이 두 운영체제에서 같은 의미로 기다리게 하는 긴 대기 명령
///
/// Windows는 `ping`, 그 밖의 운영체제는 `sleep`을 사용
#[must_use]
pub fn long_sleep_command(secs: &str) -> String {
    #[cfg(windows)]
    let command = {
        // Windows는 `ping`이 항목마다 약 1초를 기다림. 표준 입력이 없는 시험 환경에서도 그대로 대기
        format!("ping -n {secs} -w 1000 127.0.0.1 >nul")
    };
    #[cfg(not(windows))]
    let command = { format!("sleep {secs}") };
    command
}

/// 표준 출력을 내는 명령
#[must_use]
pub fn echo_command(text: &str) -> String {
    format!("echo {text}")
}

/// 약 2초 뒤에 표준 출력을 내는 명령. 상한 만료로 백그라운드로 옮겨진 뒤에도 살아 있음
#[must_use]
#[cfg(windows)]
pub fn sleep_then_echo(text: &str) -> String {
    format!("ping -n 3 -w 1000 127.0.0.1 >nul && echo {text}")
}

#[must_use]
#[cfg(not(windows))]
pub fn sleep_then_echo(text: &str) -> String {
    format!("sleep 2; echo {text}")
}

/// 작업 폴더의 마커 파일을 읽는 셸 명령
pub const READ_MARKER_COMMAND: &str = if cfg!(windows) {
    "type marker.txt"
} else {
    "cat marker.txt"
};

/// 손자가 3초 뒤에 마커를 쓰고, 셸은 6초까지 사는 2단 명령
///
/// 상한 만료로 백그라운드에 넘어간 뒤에 손자가 마커를 쓰는 상태를 만듦. Windows는 `start`로 만든 자식의 `ping`이 자식의 자식
#[must_use]
#[cfg(windows)]
pub fn grandchild_marker_command(marker: &str) -> String {
    format!(
        r#"start /b cmd /c "ping -n 4 -w 1000 127.0.0.1 >nul && echo leak > {marker}" & ping -n 7 -w 1000 127.0.0.1 >nul"#
    )
}

#[must_use]
#[cfg(not(windows))]
pub fn grandchild_marker_command(marker: &str) -> String {
    format!("sleep 3; echo leak > {marker} & sleep 6")
}
