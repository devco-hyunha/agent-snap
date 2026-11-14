//! `tool_call` 낮은 온도 프로파일을 요청에 합치는 자리입니다(D15, AC 7).
//!
//! 프로파일은 `temperature`만 담고, 합치는 자리에서 예약 키(`model`·`messages`·
//! `stream`·`tools`)를 덮으려는 항목이 있으면 설정 오류로 냅니다. 값 넣기는 요청 본문
//! 조립(`llm/request_body.rs`)이 이미 하므로 여기서는 검증과 합치기만 맡습니다.

use crate::config::{ModelIssue, SectionIssue, ValidationError};
use crate::config::{RESERVED_EXTRA_KEYS, is_valid_temperature};
use crate::llm::ChatRequest;

/// 프로파일 검증이 낸 설정 오류입니다. 02-2 검증 규칙을 그대로 재사용하므로 그대로 넘깁니다(DRY).
pub type ProfileError = ValidationError;

/// `tool_call` 턴의 낮은 온도 프로파일입니다. `temperature`만 받습니다.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolCallProfile {
    temperature: f64,
}

impl ToolCallProfile {
    /// 낮은 온도 값으로 프로파일을 만듭니다.
    ///
    /// # 오류
    /// 범위를 벗어난 값은 `temperature` 항목 이름을 밝히고 거부합니다(02-2 규칙 재사용).
    pub fn new(temperature: f64) -> Result<Self, ProfileError> {
        if is_valid_temperature(&temperature) {
            Ok(Self { temperature })
        } else {
            Err(SectionIssue::SamplingOutOfRange {
                item: "temperature".to_owned(),
                model: "tool_call".to_owned(),
            }
            .into())
        }
    }

    /// 합치는 자리에서 덮으면 안 되는 요청 칸 목록입니다(AC 7 검사 기준, 02-2 정본 재사용).
    #[must_use]
    pub const fn reserved_keys() -> [&'static str; 4] {
        RESERVED_EXTRA_KEYS
    }

    /// 요청 골격에 프로파일 온도를 합칩니다(AC 7, D15).
    ///
    /// 예약 키를 덮으려는 항목이 있으면 설정 오류로 내고, 없으면 요청의 `temperature` 자리에
    /// 프로파일 온도를 넣습니다. 나머지 샘플링 항목은 골격 값 그대로 둡니다.
    ///
    /// # 오류
    /// 예약 키를 덮으려는 자리면 `ExtraReservedKey`, 온도가 범위 밖이면 `SamplingOutOfRange`입니다.
    pub fn apply(&self, request: &mut ChatRequest) -> Result<(), ProfileError> {
        for key in Self::reserved_keys() {
            if request.extra.contains_key(key) {
                return Err(ModelIssue::ExtraReservedKey {
                    model: "tool_call".to_owned(),
                    key: (*key).to_owned(),
                }
                .into());
            }
        }
        request.sampling.temperature = Some(self.temperature);
        Ok(())
    }

    /// 프로파일의 온도 값입니다.
    #[must_use]
    pub const fn temperature(&self) -> f64 {
        self.temperature
    }
}
