//! 포기 이유 텍스트의 비밀 표지 확인 시험. spec AC 7의 담지 않는 값을 확인함.

use std::time::Duration;

use snap_core::llm::FailureInput;
use snap_core::llm::PreHeaderConnectKind;
use snap_core::llm::RetryReason;

/// 포기 이유를 텍스트로 나타냈을 때 키·base_url 호스트·응답 본문 표지가 없음.
#[test]
fn reason_text_has_no_secret_fields() {
    let reasons = [
        RetryReason::NotRetryable {
            input: FailureInput::Status(429),
        },
        RetryReason::NotRetryable {
            input: FailureInput::PreHeaderConnect(PreHeaderConnectKind::Tls),
        },
        RetryReason::AttemptLimit { attempt: 5 },
        RetryReason::RetryAfterCap {
            wait: Duration::from_secs(61),
        },
    ];
    for reason in reasons {
        let text = format!("{reason:?}");
        for marker in ["api_key", "Bearer", "base_url", "https://", "http://"] {
            assert!(
                !text.contains(marker),
                "포기 이유 텍스트 {text}에 {marker} 포함"
            );
        }
    }
}
